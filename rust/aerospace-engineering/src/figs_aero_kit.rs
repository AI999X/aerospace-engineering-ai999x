//! Aerospace Engineering's drawing helpers: curves over measured axes of their own (logarithmic either way, any
//! colour, dashed or dotted, several marked points, a legend placed inside the plot), stacked columns, a value
//! table shaded column by column, and small solve shorthands.

#![allow(dead_code)]

use crate::figs_common::esc;
use crate::figs_more::{Plot, Rg, X0, X1, Y0, Y1};
use ax_engine::figkit::{self as k, num, Solved};
use ax_engine::Input;

/// One series: its legend name ("" = none), points, colour class (axp-c, axp-c2, axp-c3, axp-c4), dashed, dots.
pub struct Ser {
    pub name: String,
    pub pts: Vec<(f64, f64)>,
    pub class: &'static str,
    pub dashed: bool,
    pub dots: bool,
}

pub fn ser(name: &str, pts: Vec<(f64, f64)>, class: &'static str) -> Ser {
    Ser { name: name.to_string(), pts, class, dashed: false, dots: false }
}
pub fn dashed(name: &str, pts: Vec<(f64, f64)>, class: &'static str) -> Ser {
    Ser { name: name.to_string(), pts, class, dashed: true, dots: false }
}
pub fn dotted(name: &str, pts: Vec<(f64, f64)>, class: &'static str) -> Ser {
    Ser { name: name.to_string(), pts, class, dashed: false, dots: true }
}

/// A range over values with a margin (a fraction of the span; on a logarithmic range a factor).
pub fn rng(vals: &[f64], pad: f64, log: bool) -> Option<Rg> {
    let v: Vec<f64> = vals.iter().copied().filter(|x| x.is_finite() && (!log || *x > 0.0)).collect();
    if v.is_empty() {
        return None;
    }
    let lo = v.iter().cloned().fold(f64::INFINITY, f64::min);
    let hi = v.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    if log {
        let (l, h) = if hi > lo { (lo, hi) } else { (lo / 2.0, hi * 2.0) };
        let f = (h / l).powf(pad);
        return Some((l / f, h * f, true));
    }
    let span = if hi > lo { hi - lo } else if lo != 0.0 { lo.abs() } else { 1.0 };
    Some((lo - span * pad, hi + span * pad, false))
}

/// Starts a range at zero when its values are all positive (a speed, a mass).
pub fn from0(r: Rg) -> Rg {
    if r.0 > 0.0 && !r.2 { (0.0, r.1, false) } else { r }
}

fn fill_of(class: &str) -> &'static str {
    match class {
        "axp-c" => "axp-d",
        "axp-c2" => "axp-d2",
        "axp-c3" => "axp-d3",
        _ => "axp-d4",
    }
}

/// Curves on a plot of their own: grid, series (clipped to the box), legend at (lx, ly) inside the box (top-left
/// corner of the legend), marked points with labels (label kept inside the box), axis labels and heading.
#[allow(clippy::too_many_arguments)]
pub fn plot(p: &Plot, series: &[Ser], marks: &[((f64, f64), String)], legend: Option<(f64, f64)>, xlab: &str, ylab: &str,
            header: &str) -> String {
    let mut out = String::new();
    p.axes(&mut out, xlab, ylab, header);
    p.clip(&mut out);
    for s in series {
        let mut d = String::new();
        let mut pen = false;
        for (x, y) in &s.pts {
            let ok = x.is_finite() && y.is_finite() && (!p.xr.2 || *x > 0.0) && (!p.yr.2 || *y > 0.0);
            if !ok {
                pen = false;
                continue;
            }
            let (px, py) = p.p(*x, *y);
            d.push_str(&format!("{}{px:.1},{py:.1} ", if pen { "L" } else { "M" }));
            pen = true;
        }
        if !s.dots && !d.is_empty() {
            out.push_str(&format!("<path class=\"{}\"{} d=\"{}\"/>", s.class,
                                  if s.dashed { " stroke-dasharray=\"6 4\"" } else { "" }, d.trim_end()));
        }
        if s.dots {
            for (x, y) in s.pts.iter().filter(|q| q.0.is_finite() && q.1.is_finite()).take(200) {
                let (px, py) = p.p(*x, *y);
                out.push_str(&format!("<circle class=\"{}\" cx=\"{px:.1}\" cy=\"{py:.1}\" r=\"3.2\"/>", fill_of(s.class)));
            }
        }
    }
    out.push_str("</g>");
    if let Some((lx, ly)) = legend {
        let named: Vec<&Ser> = series.iter().filter(|s| !s.name.is_empty()).collect();
        let w = named.iter().map(|s| k::text_w(&s.name, false)).fold(0.0, f64::max) + 34.0;
        let h = 16.0 * named.len() as f64 + 6.0;
        if !named.is_empty() {
            let lx = lx.min(X1 - w - 4.0).max(X0 + 4.0);
            out.push_str(&format!("<rect class=\"axp-ka\" x=\"{lx:.1}\" y=\"{ly:.1}\" width=\"{w:.1}\" height=\"{h:.1}\"/>"));
            for (i, s) in named.iter().enumerate() {
                let y = ly + 14.0 + 16.0 * i as f64;
                if s.dots {
                    out.push_str(&format!("<circle class=\"{}\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"3.2\"/>", fill_of(s.class), lx + 15.0,
                                          y - 4.0));
                } else {
                    out.push_str(&format!("<path class=\"{}\"{} d=\"M{:.1},{:.1} L{:.1},{:.1}\"/>", s.class,
                                          if s.dashed { " stroke-dasharray=\"6 4\"" } else { "" }, lx + 6.0, y - 4.0,
                                          lx + 24.0, y - 4.0));
                }
                out.push_str(&format!("<text class=\"axp-lt\" x=\"{:.1}\" y=\"{y:.1}\">{}</text>", lx + 30.0, esc(&s.name)));
            }
        }
    }
    for (i, ((x, y), lab)) in marks.iter().enumerate() {
        if !(x.is_finite() && y.is_finite()) {
            continue;
        }
        let (px, py) = p.p(*x, *y);
        if !(p.x0 - 1.0..=p.x1 + 1.0).contains(&px) || !(p.y0 - 1.0..=p.y1 + 1.0).contains(&py) {
            continue;
        }
        out.push_str(&format!("<circle class=\"axp-m\" cx=\"{px:.1}\" cy=\"{py:.1}\" r=\"5.5\"/>"));
        if lab.is_empty() {
            continue;
        }
        let w = 9.0 * lab.chars().count() as f64;
        let (tx, anchor) = if px + 10.0 + w <= p.x1 { (px + 10.0, "start") } else if px - 10.0 - w >= p.x0 {
            (px - 10.0, "end")
        } else {
            (((p.x0 + p.x1) / 2.0).clamp(p.x0 + w / 2.0, p.x1 - w / 2.0), "middle")
        };
        let dy = if i % 2 == 0 { -9.0 } else { 18.0 };
        let ty = (py + dy).clamp(p.y0 + 14.0, p.y1 - 4.0);
        out.push_str(&format!("<text class=\"axp-mt\" x=\"{tx:.1}\" y=\"{ty:.1}\" text-anchor=\"{anchor}\">{}</text>", esc(lab)));
    }
    out
}

/// A plot in the generic place with these ranges.
pub fn frame(xr: Rg, yr: Rg) -> Plot {
    Plot::new(xr, yr)
}

/// Columns of stacked parts (bottom first): labels under the columns, parts (value, class, text inside), a value
/// axis from zero; the legend names each class once. Returns the body.
pub fn stacked(cols: &[(String, Vec<(f64, &'static str, String)>)], legend: &[(&str, &'static str)], ylab: &str,
               header: &str, top_notes: &[String]) -> Option<String> {
    let tot = cols.iter().map(|c| c.1.iter().map(|x| x.0).sum::<f64>()).fold(0.0, f64::max);
    if !(tot > 0.0) || cols.is_empty() {
        return None;
    }
    let p = Plot::new((0.0, 1.0, false), (0.0, tot * 1.18, false));
    let mut out = String::new();
    // value axis only
    for t in crate::figs_more::ticks(p.yr, 5) {
        let y = p.py(t);
        out.push_str(&format!("<line class=\"axp-g\" x1=\"{X0}\" y1=\"{y:.1}\" x2=\"{X1}\" y2=\"{y:.1}\"/>\
                               <text class=\"axp-t\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"end\">{}</text>", X0 - 6.0, y + 5.0,
                              esc(&tick(t))));
    }
    out.push_str(&format!("<rect class=\"axp-f\" x=\"{X0}\" y=\"{Y0}\" width=\"{}\" height=\"{}\"/>", X1 - X0, Y1 - Y0));
    let n = cols.len() as f64;
    let slot = (X1 - X0) / n;
    let bw = (slot * 0.55).min(110.0);
    for (i, (lab, parts)) in cols.iter().enumerate() {
        let cx = X0 + slot * (i as f64 + 0.5);
        let mut base = 0.0;
        for (v, class, txt) in parts {
            let (ya, yb) = (p.py(base), p.py(base + v));
            out.push_str(&format!("<rect class=\"{class}\" x=\"{:.1}\" y=\"{yb:.1}\" width=\"{bw:.1}\" height=\"{:.1}\"/>",
                                  cx - bw / 2.0, (ya - yb).max(0.0)));
            if !txt.is_empty() && ya - yb > 16.0 {
                out.push_str(&format!("<text class=\"axp-bt\" x=\"{cx:.1}\" y=\"{:.1}\" text-anchor=\"middle\">{}</text>",
                                      (ya + yb) / 2.0 + 5.0, esc(txt)));
            }
            base += v;
        }
        if let Some(t) = top_notes.get(i) {
            out.push_str(&format!("<text class=\"axp-vt\" x=\"{cx:.1}\" y=\"{:.1}\" text-anchor=\"middle\">{}</text>",
                                  p.py(base) - 8.0, esc(t)));
        }
        out.push_str(&format!("<text class=\"axp-lt\" x=\"{cx:.1}\" y=\"{:.1}\" text-anchor=\"middle\">{}</text>", Y1 + 18.0,
                              esc(lab)));
    }
    let mut lx = X1;
    for (name, class) in legend.iter().rev() {
        let w = k::text_w(name, false) + 24.0;
        lx -= w;
        out.push_str(&format!("<rect class=\"{class}\" x=\"{lx:.1}\" y=\"28\" width=\"12\" height=\"10\"/>\
                               <text class=\"axp-lt\" x=\"{:.1}\" y=\"37\">{}</text>", lx + 16.0, esc(name)));
    }
    out.push_str(&format!("<text class=\"axp-l\" transform=\"translate(16,{:.1}) rotate(-90)\" text-anchor=\"middle\">{}</text>",
                          (Y0 + Y1) / 2.0, esc(ylab)));
    if !header.is_empty() {
        out.push_str(&format!("<text class=\"axp-mt\" x=\"{X0}\" y=\"20\">{}</text>", esc(header)));
    }
    Some(out)
}

/// A table of values, each column shaded by its own largest value: rows (label, values), column names.
pub fn shaded_table(rows: &[(String, Vec<f64>)], cols: &[String], mark: Option<usize>, header: &str) -> Option<(String, f64)> {
    let nc = cols.len();
    if rows.is_empty() || nc == 0 || rows.iter().any(|r| r.1.len() != nc || r.1.iter().any(|v| !v.is_finite())) {
        return None;
    }
    let left = 92.0;
    let cw = ((520.0 - left - 14.0) / nc as f64).min(84.0);
    let rh = 34.0;
    let x0 = left;
    let y0 = 62.0;
    let mut out = String::new();
    if !header.is_empty() {
        out.push_str(&format!("<text class=\"axp-mt\" x=\"16\" y=\"22\">{}</text>", esc(header)));
    }
    for (j, cname) in cols.iter().enumerate() {
        out.push_str(&format!("<text class=\"axp-bt\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\">{}</text>",
                              x0 + cw * (j as f64 + 0.5), y0 - 10.0, esc(cname)));
        let mx = rows.iter().map(|r| r.1[j].abs()).fold(0.0, f64::max);
        for (i, (_, vals)) in rows.iter().enumerate() {
            let v = vals[j];
            let op = if mx > 0.0 { 0.1 + 0.7 * v.abs() / mx } else { 0.1 };
            let (x, y) = (x0 + cw * j as f64, y0 + rh * i as f64);
            out.push_str(&format!("<rect class=\"axp-hp\" x=\"{x:.1}\" y=\"{y:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
                                   fill-opacity=\"{op:.2}\"/><text class=\"axp-ht\" x=\"{:.1}\" y=\"{:.1}\" \
                                   text-anchor=\"middle\" font-size=\"13\">{}</text>", cw - 2.0, rh - 2.0, x + cw / 2.0,
                                  y + rh / 2.0 + 4.0, esc(&num(v))));
        }
    }
    for (i, (lab, _)) in rows.iter().enumerate() {
        let y = y0 + rh * i as f64;
        out.push_str(&format!("<text class=\"axp-lt\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"end\">{}</text>", x0 - 8.0,
                              y + rh / 2.0 + 4.0, esc(lab)));
        if mark == Some(i) {
            out.push_str(&format!("<rect class=\"axp-k\" x=\"{:.1}\" y=\"{y:.1}\" width=\"{:.1}\" height=\"{:.1}\"/>", x0 - 2.0,
                                  cw * nc as f64 + 2.0, rh - 2.0));
        }
    }
    Some((out, y0 + rh * rows.len() as f64 + 16.0))
}

/// A number output of another capability solved with SI numbers.
pub fn o(s: &Solved, key: &str, set: &[(&str, f64)], y: &str) -> Option<f64> {
    if set.iter().any(|x| !x.1.is_finite()) {
        return None;
    }
    let v: Vec<(&str, Input)> = set.iter().map(|(a, x)| (*a, Input::Float(*x))).collect();
    let a = s.solve_other(key, &v)?;
    k::num_of(&a, y)
}

/// Another capability solved with inputs of any kind; its answer.
pub fn other(s: &Solved, key: &str, set: Vec<(&str, Input)>) -> Option<ax_engine::Answer> {
    s.solve_other(key, &set)
}

pub fn deg(r: f64) -> f64 {
    r.to_degrees()
}

/// A text with a degree value: "26.38°".
pub fn dg(x: f64) -> String {
    format!("{}°", num(x))
}

/// A mass in whole kilograms when large ("30000 kg", not "3e+04 kg").
pub fn kg(x: f64) -> String {
    if x.abs() >= 1000.0 && x.abs() < 1e9 { format!("{:.0} kg", x) } else { crate::figs_common::qv(x, "kg") }
}

/// A length in kilometres ("19134 km").
pub fn km(x: f64) -> String {
    let v = x / 1000.0;
    if v.abs() >= 1000.0 && v.abs() < 1e9 { format!("{:.0} km", v) } else { format!("{} km", num(v)) }
}

/// A duration in seconds with minutes or hours beside it when long.
pub fn dur(t: f64) -> String {
    if t.abs() >= 7200.0 { format!("{} h", num(t / 3600.0)) } else if t.abs() >= 120.0 { format!("{} min", num(t / 60.0)) }
    else { format!("{} s", num(t)) }
}

/// A tick label: whole numbers written out up to a billion.
pub fn tick(t: f64) -> String {
    if t.abs() >= 1000.0 && t.abs() < 1e9 && t.fract() == 0.0 { format!("{:.0}", t) } else { num(t) }
}
