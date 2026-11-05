//! Aerospace Engineering's drawings for the visitor's inputs: orbits to scale, shocks and expansion fans, nozzles
//! with the Mach number along them, rocket stages, actuator disks, level turns, the neutral point and the wind
//! triangle. Lengths and angles come from the model's answer (the Earth's radius, 6378.1366 km, is stated for scale);
//! curves are further solves of the model.

use crate::figs_aero_kit::{self as a, dashed, ser};
use crate::figs_common::{self as c, area, arc, poly, qv, varrow};
use crate::figs_more::Plot;
use ax_engine::figkit::{self as k, num, Canvas, Solved};
use ax_engine::figures::Figure;
use ax_engine::Input;
use std::f64::consts::PI;

/// The Earth's equatorial radius the model uses (m), stated for drawing orbits to scale.
const RE: f64 = 6378136.6;

pub fn live(s: &Solved) -> Vec<Figure> {
    let v: Vec<Option<Figure>> = match s.key {
        "aero_circular_orbit" | "aero_altitude_for_period" => vec![circle_orbit(s)],
        "aero_bielliptic_transfer" => vec![bielliptic_bars(s), bielliptic_orbit(s)],
        "aero_lambert_problem" => vec![lambert(s)],
        "aero_satellite_coverage" => vec![coverage(s)],
        "aero_normal_shock" | "aero_normal_shock_from_pressure_ratio" => vec![normal_shock(s)],
        "aero_oblique_shock_from_wave_angle" => vec![wave_angle_shock(s)],
        "aero_oblique_shock" => vec![oblique(s)],
        "aero_expansion_fan" => vec![fan(s)],
        "aero_mach_from_area_ratio" | "aero_nozzle_normal_shock" | "aero_rocket_engine_performance" => vec![nozzle(s)],
        "aero_multistage_delta_v" => vec![multistage(s)],
        "aero_optimal_staging" => vec![optimal_stack(s)],
        "aero_propellant_for_delta_v" | "aero_rocket_delta_v" => vec![single_stage(s)],
        "aero_actuator_disk_forward" | "aero_actuator_disk_hover" => vec![disk(s)],
        "aero_level_turn_bank" | "aero_level_turn_load_factor" => vec![turn(s)],
        "aero_neutral_point" => vec![neutral(s)],
        "aero_wind_triangle" => vec![wind_triangle(s)],
        _ => vec![],
    };
    v.into_iter().flatten().collect()
}

fn ring(cv: &mut Canvas, c0: (f64, f64), r: f64, class: &str) {
    cv.raw(&format!("<circle class=\"{class}\" fill=\"none\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"{r:.1}\"/>", c0.0, c0.1));
}

fn earth(cv: &mut Canvas, c0: (f64, f64), r: f64) {
    cv.raw(&format!("<circle class=\"axp-s2\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"{r:.1}\"/>", c0.0, c0.1));
    ring(cv, c0, r, "axp-c2");
    if r > 22.0 {
        cv.bold(c0.0, c0.1 + 5.0, "middle", "Earth");
    }
}

// ------------------------------------------------------------------------------------------------ orbits
fn circle_orbit(s: &Solved) -> Option<Figure> {
    let (h, v, t) = if s.key == "aero_circular_orbit" {
        (c::inp(s, "h")?, s.out("v")?, s.out("T")?)
    } else {
        (s.out("h")?, s.out("v")?, c::inp(s, "T")?)
    };
    if !(h > -RE) {
        return None;
    }
    let r = RE + h;
    let mut cv = Canvas::new(360.0);
    let c0 = (190.0, 182.0);
    let sc = 150.0 / r.max(RE);
    earth(&mut cv, c0, RE * sc);
    ring(&mut cv, c0, r * sc, "axp-c");
    let sat = (c0.0 + r * sc * (PI / 4.0).cos(), c0.1 - r * sc * (PI / 4.0).sin());
    cv.raw(&format!("<circle class=\"axp-d\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"5\"/>", sat.0, sat.1));
    // velocity along the orbit (anticlockwise)
    let dir = (-(PI / 4.0).sin(), -(PI / 4.0).cos());
    varrow(&mut cv, sat, (sat.0 + dir.0 * 50.0, sat.1 + dir.1 * 50.0), "axp-c3");
    // altitude dimension along the radius to the right
    let (e0, e1) = ((c0.0 + RE * sc, c0.1), (c0.0 + r * sc, c0.1));
    cv.dashed(c0, e1);
    if e1.0 - e0.0 > 8.0 {
        cv.line((e0.0, c0.1 - 6.0), (e0.0, c0.1 + 6.0));
        cv.line((e1.0, c0.1 - 6.0), (e1.0, c0.1 + 6.0));
    }
    let x0 = 372.0;
    cv.bold(x0, 60.0, "start", "circular orbit");
    cv.value(x0, 86.0, "start", &format!("h = {}", a::km(h)));
    cv.value(x0, 106.0, "start", &format!("v = {} km/s", num(v / 1000.0)));
    cv.value(x0, 126.0, "start", &format!("T = {}", a::dur(t)));
    cv.name(x0, 330.0, "start", "Earth radius 6378 km");
    cv.name(x0, 346.0, "start", "(stated), to scale");
    let cap = format!("A circular orbit drawn to scale around the Earth (radius 6378.14 km, stated) at altitude h = {}. Marked: \
                       the model's orbital speed v = {} and period T = {} ({} min).", qv(h, "m"), qv(v, "m/s"), qv(t, "s"),
                      num(t / 60.0));
    let title = if s.key == "aero_circular_orbit" { "Circular orbit to scale with speed and period" }
                else { "The circular orbit with a given period" };
    Some(s.figure("orbit", title, &cap, cv.body(), cv.h))
}

fn bielliptic_bars(s: &Solved) -> Option<Figure> {
    let (h1, h2, hb) = (c::inp(s, "h1")?, c::inp(s, "h2")?, c::inp(s, "hb")?);
    let hoh = s.out("dv_hohmann")?;
    let mut hbs: Vec<f64> = [1.0, 1.5, 2.0, 3.0, 5.0].iter().map(|f| h2 * f).collect();
    if !hbs.iter().any(|x| (x - hb).abs() <= 1e-9 * hb.abs().max(1.0)) {
        hbs.push(hb);
    }
    hbs.sort_by(|x, y| x.partial_cmp(y).unwrap());
    let mut cols = vec![];
    let mut notes = vec![];
    let mut tot_note = vec![];
    for x in &hbs {
        let Some(ans) = s.with(&[("hb", *x), ("h1", h1), ("h2", h2)]) else { continue };
        let (d1, d2, d3, dv) = (k::num_of(&ans, "dv1")?, k::num_of(&ans, "dv2")?, k::num_of(&ans, "dv3")?, k::num_of(&ans, "dv")?);
        let lab = if (x - hb).abs() <= 1e-9 * hb.abs().max(1.0) { format!("{}*", tick_km(*x)) } else { tick_km(*x) };
        tot_note.push(format!("hb = {}: {}", qv(*x, "m"), qv(dv, "m/s")));
        cols.push((lab, vec![(d1 / 1000.0, "axp-b", String::new()), (d2 / 1000.0, "axp-b2", String::new()),
                             (d3 / 1000.0, "axp-b3", String::new())]));
        notes.push(num(dv / 1000.0));
    }
    if cols.len() < 2 {
        return None;
    }
    cols.push(("Hohmann".to_string(), vec![(hoh / 1000.0, "axp-b4", String::new())]));
    notes.push(num(hoh / 1000.0));
    let body = a::stacked(&cols, &[("first burn", "axp-b"), ("second", "axp-b2"), ("third", "axp-b3"), ("Hohmann", "axp-b4")],
                          "Δv (km/s)", &format!("Hohmann {}", qv(hoh, "m/s")), &notes)?;
    let mut body = c::fit_right(&body);
    body.push_str(&format!("<text class=\"axp-l\" x=\"300\" y=\"326\" text-anchor=\"middle\">intermediate apoapsis altitude hb (km), * given</text>"));
    let cap = format!("The three burns of bi-elliptic transfers from a circular orbit at h1 = {} to h2 = {}, stacked, for {} \
                       intermediate apoapses, against the Hohmann transfer's {}; every column a solve of the model; totals \
                       {}.", qv(h1, "m"), qv(h2, "m"), cols.len() - 1, qv(hoh, "m/s"), tot_note.join("; "));
    Some(s.chart("bars", "Bi-elliptic burns against a Hohmann transfer", &cap, &body))
}

/// Half an ellipse with periapsis radius rp at angle th0 (pixels: centre c0, scale sc), from periapsis to apoapsis.
fn tick_km(x: f64) -> String {
    let v = x / 1000.0;
    if v.abs() >= 1000.0 { format!("{:.0}", v) } else { num(v) }
}

fn half_ellipse(cv: &mut Canvas, c0: (f64, f64), sc: f64, rp: f64, ra: f64, th0: f64, class: &str) {
    let e = (ra - rp) / (ra + rp);
    let p = rp * (1.0 + e);
    let pts: Vec<(f64, f64)> = (0..=60).map(|i| {
        let nu = PI * i as f64 / 60.0;
        let r = p / (1.0 + e * nu.cos());
        (c0.0 + r * sc * (th0 + nu).cos(), c0.1 - r * sc * (th0 + nu).sin())
    }).collect();
    poly(cv, &pts, class, false);
}

fn bielliptic_orbit(s: &Solved) -> Option<Figure> {
    let (h1, h2, hb) = (c::inp(s, "h1")?, c::inp(s, "h2")?, c::inp(s, "hb")?);
    let (d1, d2, d3, dv, t, hoh) = (s.out("dv1")?, s.out("dv2")?, s.out("dv3")?, s.out("dv")?, s.out("t")?, s.out("dv_hohmann")?);
    let (r1, r2, rb) = (RE + h1, RE + h2, RE + hb);
    if !(r1 > 0.0 && r2 > 0.0 && rb > 0.0) {
        return None;
    }
    let mut cv = Canvas::new(360.0);
    let c0 = (200.0, 180.0);
    // the first ellipse reaches rb opposite the start; the drawing's width holds r1 + rb, its height 2 rb
    let sc = (150.0 / rb.max(r2)).min(300.0 / (r1 + rb.max(r2)));
    let c0 = (20.0 + rb.max(r2) * sc, c0.1);
    earth(&mut cv, c0, RE * sc);
    ring(&mut cv, c0, r1 * sc, "axp-c4");
    ring(&mut cv, c0, r2 * sc, "axp-c2");
    half_ellipse(&mut cv, c0, sc, r1, rb, 0.0, "axp-c");
    if (rb - r2).abs() > 1e-6 * rb {
        // second half: from rb (at 180°) down to r2 at 0° on the other side
        half_ellipse(&mut cv, c0, sc, r2, rb, PI, "axp-c3");
    }
    let p1 = (c0.0 + r1 * sc, c0.1);
    let pb = (c0.0 - rb * sc, c0.1);
    cv.raw(&format!("<circle class=\"axp-d\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"4.5\"/><circle class=\"axp-d\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"4.5\"/>",
                    p1.0, p1.1, pb.0, pb.1));
    let x0 = 352.0;
    cv.bold(x0, 30.0, "start", "burns (model)");
    cv.value(x0, 54.0, "start", &format!("Δv1 = {}", qv(d1, "m/s")));
    cv.value(x0, 72.0, "start", &format!("Δv2 = {}", qv(d2, "m/s")));
    cv.value(x0, 90.0, "start", &format!("Δv3 = {}", qv(d3, "m/s")));
    cv.value(x0, 114.0, "start", &format!("total {}", qv(dv, "m/s")));
    cv.value(x0, 132.0, "start", &format!("Hohmann {}", qv(hoh, "m/s")));
    cv.value(x0, 150.0, "start", &format!("time {}", a::dur(t)));
    let legend = [("axp-c4", format!("start {}", a::km(h1))), ("axp-c2", format!("final {}", a::km(h2))),
                  ("axp-c", "transfer, first half".to_string()), ("axp-c3", "second half".to_string())];
    for (i, (cl, t_)) in legend.iter().enumerate() {
        let y = 200.0 + 20.0 * i as f64;
        cv.raw(&format!("<path class=\"{cl}\" d=\"M{x0},{:.1} L{},{:.1}\"/>", y - 4.0, x0 + 18.0, y - 4.0));
        cv.name(x0 + 24.0, y, "start", t_);
    }
    cv.name(x0, 300.0, "start", "to scale; Earth");
    cv.name(x0, 316.0, "start", "radius 6378 km");
    let same = (rb - r2).abs() <= 1e-6 * rb;
    let cap = format!("Orbit transfer drawn to scale (Earth radius 6378.14 km, stated) from a circular orbit at h1 = {} to h2 = \
                       {} through the intermediate apoapsis hb = {}{}. Marked: the model's burns Δv1 = {}, Δv2 = {}, Δv3 = {}, \
                       total {} (Hohmann {}), transfer time {}.", qv(h1, "m"), qv(h2, "m"), qv(hb, "m"),
                      if same { ": with the intermediate apoapsis at the final orbit the transfer is the Hohmann transfer (third burn zero)" } else { "" },
                      qv(d1, "m/s"), qv(d2, "m/s"), qv(d3, "m/s"), qv(dv, "m/s"), qv(hoh, "m/s"), qv(t, "s"));
    Some(s.figure("orbit", "Transfer between circular orbits, to scale", &cap, cv.body(), cv.h))
}

fn lambert(s: &Solved) -> Option<Figure> {
    let (r1, r2) = (s.vec("r1")?, s.vec("r2")?);
    let (v1, v2, dth) = (s.out_list("v1")?, s.out_list("v2")?, s.out("dtheta_deg")?);
    if r1.len() != 3 || r2.len() != 3 || v1.len() != 3 || v2.len() != 3 {
        return None;
    }
    let norm = |v: &[f64]| (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    let (n1, n2) = (norm(&r1), norm(&r2));
    if !(n1 > 0.0 && n2 > 0.0) || r1[2].abs() > 1e-9 * n1 || r2[2].abs() > 1e-9 * n2 || v1[2].abs() > 1e-9 * norm(&v1) {
        return None; // drawn in the orbital plane only when the vectors lie in the x-y plane
    }
    // μ from the model's circular speed at |r1| (km): v² r
    let vc = a::o(s, "aero_circular_orbit", &[("h", n1 * 1000.0 - RE)], "v")? / 1000.0;
    let mu = vc * vc * n1;
    // the two-body orbit through r1 with v1 (derived): r(ν) = p / (1 + e cos(ν))
    let hz = r1[0] * v1[1] - r1[1] * v1[0];
    let vv = norm(&v1);
    let rv = r1[0] * v1[0] + r1[1] * v1[1];
    let ex = ((vv * vv - mu / n1) * r1[0] - rv * v1[0]) / mu;
    let ey = ((vv * vv - mu / n1) * r1[1] - rv * v1[1]) / mu;
    let e = (ex * ex + ey * ey).sqrt();
    let p = hz * hz / mu;
    if !(p > 0.0) || !e.is_finite() {
        return None;
    }
    let w = ey.atan2(ex);
    let th1 = r1[1].atan2(r1[0]);
    let sgn = if hz >= 0.0 { 1.0 } else { -1.0 };
    let span = dth.to_radians();
    let pts_km: Vec<(f64, f64)> = (0..=80).map(|i| {
        let th = th1 + sgn * span * i as f64 / 80.0;
        let r = p / (1.0 + e * (th - w).cos());
        (r * th.cos(), r * th.sin())
    }).collect();
    if pts_km.iter().any(|q| !q.0.is_finite() || !q.1.is_finite()) {
        return None;
    }
    let ext = pts_km.iter().map(|q| q.0.abs().max(q.1.abs())).fold(n1.max(n2), f64::max);
    let mut cv = Canvas::new(360.0);
    let c0 = (190.0, 180.0);
    let sc = 145.0 / ext;
    let pp = |x: f64, y: f64| (c0.0 + x * sc, c0.1 - y * sc);
    earth(&mut cv, c0, RE / 1000.0 * sc);
    let px: Vec<(f64, f64)> = pts_km.iter().map(|q| pp(q.0, q.1)).collect();
    poly(&mut cv, &px, "axp-c", false);
    let (a1, a2) = (pp(r1[0], r1[1]), pp(r2[0], r2[1]));
    cv.dashed(c0, a1);
    cv.dashed(c0, a2);
    cv.raw(&format!("<circle class=\"axp-d\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"4.5\"/><circle class=\"axp-d\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"4.5\"/>",
                    a1.0, a1.1, a2.0, a2.1));
    let vs = 40.0 / norm(&v1).max(norm(&v2));
    varrow(&mut cv, a1, (a1.0 + v1[0] * vs, a1.1 - v1[1] * vs), "axp-c3");
    varrow(&mut cv, a2, (a2.0 + v2[0] * vs, a2.1 - v2[1] * vs), "axp-c3");
    cv.name(a1.0 + 8.0, a1.1 + 18.0, "start", "r1");
    cv.name(a2.0 + 8.0, a2.1 - 8.0, "start", "r2");
    let x0 = 360.0;
    cv.bold(x0, 40.0, "start", "Lambert transfer");
    cv.value(x0, 66.0, "start", &format!("v1 = {} km/s", num(norm(&v1))));
    cv.value(x0, 86.0, "start", &format!("v2 = {} km/s", num(norm(&v2))));
    cv.value(x0, 106.0, "start", &format!("Δθ = {}", a::dg(dth)));
    cv.value(x0, 126.0, "start", &format!("tof = {}", a::dur(c::inp(s, "tof")?)));
    cv.name(x0, 300.0, "start", "arc: two-body orbit");
    cv.name(x0, 316.0, "start", "through r1 with v1");
    let cap = format!("Lambert's problem in the plane: from r1 = ({}) km to r2 = ({}) km in {} (Earth radius 6378.14 km, stated). \
                       Marked: the model's departure and arrival velocities v1 = ({}) km/s and v2 = ({}) km/s and transfer \
                       angle {}. The arc is the two-body orbit through r1 with velocity v1 (derived, with μ from the model's \
                       circular speed at |r1|: {} km³/s²; eccentricity {}).",
                      r1.iter().map(|x| num(*x)).collect::<Vec<_>>().join(", "),
                      r2.iter().map(|x| num(*x)).collect::<Vec<_>>().join(", "), qv(c::inp(s, "tof")?, "s"),
                      v1.iter().map(|x| num(*x)).collect::<Vec<_>>().join(", "),
                      v2.iter().map(|x| num(*x)).collect::<Vec<_>>().join(", "), a::dg(dth), num(mu), num(k::snap(e, 1.0)));
    Some(s.figure("orbit", "Lambert transfer: velocities and the transfer arc", &cap, cv.body(), cv.h))
}

fn coverage(s: &Solved) -> Option<Figure> {
    let (h, lam, eta, sr, cov) = (c::inp(s, "h")?, s.out("lambda_deg")?, s.out("nadir_deg")?, s.out("slant_range")?,
                                  s.out("coverage")?);
    if !(h > 0.0) {
        return None;
    }
    let r = RE + h;
    let mut cv = Canvas::new(360.0);
    let sc = 300.0 / (r + RE);
    let c0 = (190.0, 30.0 + r * sc);
    earth(&mut cv, c0, RE * sc);
    let sat = (c0.0, c0.1 - r * sc);
    let l = lam.to_radians();
    let e1 = (c0.0 + RE * sc * l.sin(), c0.1 - RE * sc * l.cos());
    let e2 = (c0.0 - RE * sc * l.sin(), c0.1 - RE * sc * l.cos());
    // the covered cap
    arc(&mut cv, c0, RE * sc, PI / 2.0 - l, PI / 2.0 + l, "axp-c");
    cv.line(sat, e1);
    cv.line(sat, e2);
    cv.dashed(c0, e1);
    cv.dashed(c0, sat);
    cv.raw(&format!("<circle class=\"axp-d\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"5\"/>", sat.0, sat.1));
    arc(&mut cv, c0, 26.0, PI / 2.0 - l, PI / 2.0, "axp-c3");
    cv.value(c0.0 + 14.0, c0.1 - 34.0, "start", &format!("λ = {}", a::dg(lam)));
    cv.value(sat.0 + 10.0, sat.1 + 34.0, "start", &format!("η = {}", a::dg(eta)));
    let x0 = 360.0;
    cv.bold(x0, 40.0, "start", "coverage (model)");
    cv.value(x0, 66.0, "start", &format!("h = {}", a::km(h)));
    cv.value(x0, 86.0, "start", &format!("λ = {}", a::dg(lam)));
    cv.value(x0, 106.0, "start", &format!("η = {}", a::dg(eta)));
    cv.value(x0, 126.0, "start", &format!("slant {}", a::km(sr)));
    cv.value(x0, 146.0, "start", &format!("covered {} %", num(cov * 100.0)));
    cv.name(x0, 316.0, "start", "to scale, Earth radius");
    cv.name(x0, 332.0, "start", "6378 km (stated)");
    let cap = format!("Coverage of a satellite at h = {} drawn to scale over the Earth (radius 6378.14 km, stated): the lines of \
                       sight to the edge of coverage, the Earth central angle λ and the nadir angle η. Marked, from the model: \
                       λ = {}, η = {}, slant range {}, fraction of the surface covered {}.", qv(h, "m"), a::dg(lam), a::dg(eta),
                      qv(sr, "m"), num(cov));
    Some(s.figure("orbit", "Satellite coverage cone to scale", &cap, cv.body(), cv.h))
}

// ------------------------------------------------------------------------------------------------ shocks
/// Flow arrows in a band: `n` arrows of length `len` starting at x, rows at ys.
fn flow(cv: &mut Canvas, x: f64, len: f64, ys: &[f64], class: &str) {
    for y in ys {
        varrow(cv, (x, *y), (x + len, *y), class);
    }
}

#[allow(clippy::too_many_arguments)]
fn normal_shock_body(m1: f64, m2: f64, p21: f64, t21: f64, r21: f64, p0r: f64) -> Canvas {
    let mut cv = Canvas::new(300.0);
    let xs = 250.0;
    cv.raw(&format!("<rect class=\"axp-s4\" x=\"{}\" y=\"40\" width=\"8\" height=\"210\"/>", xs - 4.0));
    cv.raw(&format!("<path class=\"axp-c\" stroke-width=\"3\" d=\"M{xs},40 L{xs},250\"/>"));
    cv.bold(xs, 30.0, "middle", "shock");
    let up = 150.0;
    let down = (up / r21).clamp(12.0, 150.0);
    flow(&mut cv, 60.0, up, &[90.0, 145.0, 200.0], "axp-c3");
    flow(&mut cv, xs + 24.0, down, &[90.0, 145.0, 200.0], "axp-c3");
    cv.bold(60.0, 60.0, "start", "upstream");
    cv.bold(xs + 24.0, 60.0, "start", "downstream");
    cv.value(60.0, 236.0, "start", &format!("M1 = {}", num(m1)));
    cv.name(60.0, 256.0, "start", "p1, T1, ρ1");
    let x2 = xs + 24.0;
    cv.value(x2, 236.0, "start", &format!("M2 = {}", num(m2)));
    cv.value(x2, 256.0, "start", &format!("p2/p1 = {}", num(p21)));
    cv.value(x2, 274.0, "start", &format!("T2/T1 = {}", num(t21)));
    cv.value(x2 + 120.0, 256.0, "start", &format!("ρ2/ρ1 = {}", num(r21)));
    cv.value(x2 + 120.0, 274.0, "start", &format!("p02/p01 = {}", num(p0r)));
    cv.name(60.0, 290.0, "start", "arrow lengths follow the speed ratio");
    cv
}

fn normal_shock(s: &Solved) -> Option<Figure> {
    let m1 = if s.key == "aero_normal_shock" { c::inp(s, "M1")? } else { s.out("M1")? };
    let (m2, t21, r21, p0r) = (s.out("M2")?, s.out("T2_T1")?, s.out("rho2_rho1")?, s.out("p02_p01")?);
    let p21 = if s.key == "aero_normal_shock" { s.out("p2_p1")? } else { c::inp(s, "p21")? };
    if !(r21 > 0.0) {
        return None;
    }
    let cv = normal_shock_body(m1, m2, p21, t21, r21, p0r);
    let lead = if s.key == "aero_normal_shock" { format!("A normal shock at M1 = {}", num(m1)) }
               else { format!("A normal shock with the given pressure ratio p2/p1 = {}; the model returns M1 = {}", num(p21), num(m1)) };
    let cap = format!("{lead}, and the downstream state the model returns. Arrow lengths follow the speed ratio, ρ1/ρ2 by \
                       continuity (derived). Marked: M2 = {}, p2/p1 = {}, T2/T1 = {}, ρ2/ρ1 = {}, stagnation-pressure ratio {}.",
                      num(m2), num(p21), num(t21), num(r21), num(p0r));
    Some(s.figure("shock", "A normal shock and the jumps across it", &cap, cv.body(), cv.h))
}

fn wave_angle_shock(s: &Solved) -> Option<Figure> {
    let (m1, beta) = (c::inp(s, "M1")?, c::inp(s, "beta")?);
    let (th, m2, p21, t21, r21, p0r) = (s.out("theta_deg")?, s.out("M2")?, s.out("p2_p1")?, s.out("T2_T1")?,
                                        s.out("rho2_rho1")?, s.out("p02_p01")?);
    if (beta.to_degrees() - 90.0).abs() < 0.01 {
        if !(r21 > 0.0) {
            return None;
        }
        let cv = normal_shock_body(m1, m2, p21, t21, r21, p0r);
        let cap = format!("An oblique shock at M1 = {} with wave angle β = 90°; the model returns the deflection θ = {}: with a \
                           wave angle of 90° the oblique shock is a normal shock. Arrow lengths follow the speed ratio, ρ1/ρ2 by \
                           continuity (derived). Marked: M2 = {}, p2/p1 = {}, T2/T1 = {}, ρ2/ρ1 = {}, stagnation-pressure ratio {}.",
                          num(m1), a::dg(k::snap(th, 1.0)), num(m2), num(p21), num(t21), num(r21), num(p0r));
        return Some(s.figure("shock", "A normal shock and the jumps across it", &cap, cv.body(), cv.h));
    }
    let cv = wedge(m1, m2, th, beta.to_degrees(), p21, t21)?;
    let cap = format!("An oblique shock at M1 = {} with wave angle β = {}: the model returns the flow deflection θ = {}, drawn as \
                       the wedge that turns the flow. Marked: M2 = {}, p2/p1 = {}, T2/T1 = {} (also ρ2/ρ1 = {}, p02/p01 = {}).",
                      num(m1), a::dg(beta.to_degrees()), a::dg(th), num(m2), num(p21), num(t21), num(r21), num(p0r));
    Some(s.figure("shock", "Oblique shock from its wave angle", &cap, cv.body(), cv.h))
}

/// A wedge of half-angle th (deg) with an attached shock at beta (deg) from its tip.
fn wedge(m1: f64, m2: f64, th: f64, beta: f64, p21: f64, t21: f64) -> Option<Canvas> {
    if !(th >= 0.0 && th < 60.0 && beta > th && beta < 90.0) {
        return None;
    }
    let mut cv = Canvas::new(300.0);
    let tip = (170.0, 250.0);
    let (tr, br) = (th.to_radians(), beta.to_radians());
    let len = 320.0;
    let end = (tip.0 + len, tip.1 - len * tr.tan());
    area(&mut cv, &[tip, end, (tip.0 + len, tip.1)], "axp-s4");
    cv.line(tip, end);
    cv.line((30.0, tip.1), (tip.0 + len, tip.1));
    let sl = (215.0 / br.sin()).min(330.0);
    let send = (tip.0 + sl * br.cos(), tip.1 - sl * br.sin());
    cv.raw(&format!("<path class=\"axp-c\" stroke-width=\"3\" d=\"M{:.1},{:.1} L{:.1},{:.1}\"/>", tip.0, tip.1, send.0, send.1));
    flow(&mut cv, 30.0, 110.0, &[110.0, 170.0, 225.0], "axp-c3");
    // turned flow arrows above the wedge, parallel to it
    for d in [40.0, 90.0] {
        let st = (tip.0 + 200.0, tip.1 - 200.0 * tr.tan() - d);
        varrow(&mut cv, st, (st.0 + 90.0 * tr.cos(), st.1 - 90.0 * tr.sin()), "axp-c3");
    }
    arc(&mut cv, tip, 60.0, 0.0, tr, "axp-c4");
    arc(&mut cv, tip, 90.0, 0.0, br, "axp-c4");
    cv.value(tip.0 + 66.0, tip.1 - 4.0, "start", &format!("θ = {}", a::dg(th)));
    cv.value(tip.0 + 92.0 * (br / 2.0).cos() + 6.0, tip.1 - 92.0 * (br / 2.0).sin(), "start", &format!("β = {}", a::dg(beta)));
    cv.value(30.0, 96.0, "start", &format!("M1 = {}", num(m1)));
    cv.value(380.0, 40.0, "start", &format!("M2 = {}", num(m2)));
    cv.value(380.0, 60.0, "start", &format!("p2/p1 = {}", num(p21)));
    cv.value(380.0, 80.0, "start", &format!("T2/T1 = {}", num(t21)));
    Some(cv)
}

fn oblique(s: &Solved) -> Option<Figure> {
    let (m1, th) = (c::inp(s, "M1")?, c::inp(s, "theta")?.to_degrees());
    let (beta, m2, p21, t21) = (s.out("beta_deg")?, s.out("M2")?, s.out("p2_p1")?, s.out("T2_T1")?);
    let cv = wedge(m1, m2, th, beta, p21, t21)?;
    let cap = format!("Supersonic flow at M1 = {} over a wedge of θ = {}. The shock leaves the wedge tip at the model's wave angle \
                       β = {} and turns the flow by θ, parallel to the wedge. Marked: M2 = {}, p2/p1 = {}, T2/T1 = {} (also \
                       ρ2/ρ1 = {}, p02/p01 = {}, normal Mach {}).", num(m1), a::dg(th), a::dg(beta), num(m2), num(p21), num(t21),
                      num(s.out("rho2_rho1")?), num(s.out("p02_p01")?), num(s.out("Mn1")?));
    Some(s.figure("shock", "Oblique shock on a wedge", &cap, cv.body(), cv.h))
}

fn fan(s: &Solved) -> Option<Figure> {
    let (m1, th) = (c::inp(s, "M1")?, c::inp(s, "theta")?);
    let (m2, nu1, nu2, p21, t21) = (s.out("M2")?, s.out("nu1_deg")?, s.out("nu2_deg")?, s.out("p2_p1")?, s.out("T2_T1")?);
    if !(m1 >= 1.0 && m2 >= 1.0 && th > 0.0 && th < PI / 2.0) {
        return None;
    }
    // Mach angles asin(1/M) (derived): the leading wave to the upstream flow, the trailing wave to the turned flow
    let (mu1, mu2) = ((1.0 / m1).asin(), (1.0 / m2).asin());
    let mut cv = Canvas::new(300.0);
    let corner = (230.0, 200.0);
    let wall_end = (corner.0 + 260.0 * th.cos(), corner.1 + 260.0 * th.sin());
    area(&mut cv, &[(20.0, corner.1), corner, wall_end, (wall_end.0, 300.0), (20.0, 300.0)], "axp-s4");
    cv.line((20.0, corner.1), corner);
    cv.line(corner, wall_end);
    let l = 190.0;
    let a1 = mu1; // above the horizontal
    let a2 = mu2 - th; // trailing wave angle above the horizontal
    let mut d = String::new();
    for i in 0..=6 {
        let ang = a1 + (a2 - a1) * i as f64 / 6.0;
        d.push_str(&format!("M{:.1},{:.1} L{:.1},{:.1} ", corner.0, corner.1, corner.0 + l * ang.cos(), corner.1 - l * ang.sin()));
    }
    cv.raw(&format!("<path class=\"axp-c\" stroke-width=\"1.4\" d=\"{}\"/>", d.trim_end()));
    flow(&mut cv, 30.0, 110.0, &[120.0, 165.0], "axp-c3");
    for dy in [50.0, 95.0] {
        let st = (corner.0 + 120.0, corner.1 + 120.0 * th.tan() - dy);
        varrow(&mut cv, st, (st.0 + 90.0 * th.cos(), st.1 + 90.0 * th.sin()), "axp-c3");
    }
    cv.value(30.0, 104.0, "start", &format!("M1 = {}", num(m1)));
    cv.value(390.0, 40.0, "start", &format!("M2 = {}", num(m2)));
    cv.value(390.0, 60.0, "start", &format!("θ = {}", a::dg(th.to_degrees())));
    cv.value(30.0, 236.0, "start", &format!("ν1 = {}, ν2 = {}", a::dg(nu1), a::dg(nu2)));
    cv.value(30.0, 256.0, "start", &format!("p2/p1 = {}", num(p21)));
    cv.value(30.0, 276.0, "start", &format!("T2/T1 = {}", num(t21)));
    let cap = format!("A Prandtl–Meyer expansion round a corner turning the flow by θ = {} from M1 = {}. The fan spreads from the \
                       leading Mach wave at μ1 = {} to the upstream flow to the trailing wave at μ2 = {} to the turned flow (Mach \
                       angles asin(1/M), derived). Marked: the model's M2 = {}, ν1 = {}, ν2 = {}, p2/p1 = {}, T2/T1 = {}, ρ2/ρ1 = {}.",
                      a::dg(th.to_degrees()), num(m1), a::dg(mu1.to_degrees()), a::dg(mu2.to_degrees()), num(m2), a::dg(nu1),
                      a::dg(nu2), num(p21), num(t21), num(s.out("rho2_rho1")?));
    Some(s.figure("shock", "Prandtl–Meyer expansion fan at a corner", &cap, cv.body(), cv.h))
}

// ------------------------------------------------------------------------------------------------ nozzles
/// The model's Mach number for an area ratio on a branch (the area–Mach capability).
fn mach_of(s: &Solved, ar: f64, sup: bool, g: Option<f64>) -> Option<f64> {
    let mut set = vec![("ar", Input::Float(ar.max(1.0))), ("regime", Input::Text(if sup { "supersonic" } else { "subsonic" }.into()))];
    if let Some(gv) = g {
        set.push(("gamma", Input::Float(gv)));
    }
    let ans = s.solve_other("aero_mach_from_area_ratio", &set)?;
    k::num_of(&ans, "M")
}

/// A converging-diverging nozzle 12 throat radii long (throat at 3), its outline from the area ratio (radius ∝
/// √(A/At)), over the Mach number along it.
fn nozzle(s: &Solved) -> Option<Figure> {
    let g = c::inp(s, "gamma");
    let (eps, inlet, shock): (f64, f64, Option<(f64, f64, f64, f64)>) = match s.key {
        "aero_mach_from_area_ratio" => (c::inp(s, "ar")?, 3.0, None),
        "aero_nozzle_normal_shock" => (c::inp(s, "eps")?, 3.0,
                                       Some((s.out("As_At")?, s.out("M1")?, s.out("M2")?, s.out("p02_p01")?))),
        _ => (c::inp(s, "eps")?, 4.0, None),
    };
    if !(eps >= 1.0) || eps > 400.0 {
        return None;
    }
    let sub_exit = s.key == "aero_mach_from_area_ratio" && s.txt("regime").as_deref() == Some("subsonic");
    let xt = 3.0;
    let ar_at = |x: f64| if x <= xt { inlet + (1.0 - inlet) * (x / xt) } else { 1.0 + (eps - 1.0) * ((x - xt) / (12.0 - xt)) };
    let xs = k::span(0.0, 12.0, 41, false);
    let mut pts = vec![];
    for x in &xs {
        let ar = ar_at(*x);
        let mv = if *x <= xt { mach_of(s, ar, false, g) } else {
            match shock {
                Some((as_, _, _, p0r)) if ar > as_ => mach_of(s, ar * p0r, false, g),
                _ => mach_of(s, ar, !sub_exit, g),
            }
        };
        if let Some(mv) = mv {
            pts.push((*x, mv));
        }
    }
    if pts.len() < 10 {
        return None;
    }
    let mut series = vec![ser("", pts.clone(), "axp-c")];
    let mut marks = vec![];
    let xshock = shock.map(|(as_, ..)| xt + (as_ - 1.0) / (eps - 1.0).max(1e-9) * (12.0 - xt));
    let (exit_label, exit_m) = match s.key {
        "aero_mach_from_area_ratio" => (format!("exit M = {}", num(s.out("M")?)), s.out("M")?),
        "aero_nozzle_normal_shock" => (format!("Me = {}", num(s.out("Me")?)), s.out("Me")?),
        _ => (format!("Me = {}", num(s.out("Me")?)), s.out("Me")?),
    };
    marks.push(((12.0, exit_m), exit_label.clone()));
    if let (Some(xsk), Some((_, m1, m2, _))) = (xshock, shock) {
        series.push(dashed("", vec![(xsk, m1), (xsk, m2)], "axp-c3"));
        marks.push(((xsk, m1), format!("M1 = {}", num(m1))));
        marks.push(((xsk, m2), format!("M2 = {}", num(m2))));
    }
    let ymax = pts.iter().map(|p| p.1).fold(exit_m, f64::max) * 1.12;
    let p = Plot::new((0.0, 12.0, false), (0.0, ymax.max(1.2), false));
    let chart = c::fit_right(&a::plot(&p, &series, &marks, None, "x, distance along the nozzle (throat radii)",
                                      "M, Mach number on the axis", ""));
    // the nozzle outline above the chart, same horizontal scale
    let mut cv = Canvas::new(470.0);
    let (ya, rs) = (70.0, 50.0 / inlet.max(eps).sqrt());
    let px = |x: f64| p.px(x);
    let upper: Vec<(f64, f64)> = (0..=60).map(|i| { let x = 12.0 * i as f64 / 60.0; (px(x), ya - rs * ar_at(x).sqrt()) }).collect();
    let lower: Vec<(f64, f64)> = upper.iter().map(|q| (q.0, 2.0 * ya - q.1)).collect();
    let mut region = upper.clone();
    region.extend(lower.iter().rev());
    area(&mut cv, &region, "axp-s3");
    poly(&mut cv, &upper, "axp-k", false);
    poly(&mut cv, &lower, "axp-k", false);
    cv.dashed((px(xt), ya - 56.0), (px(xt), ya + 56.0));
    cv.name(px(xt), ya + 72.0, "middle", "throat");
    if let Some(xsk) = xshock {
        cv.raw(&format!("<path class=\"axp-c\" stroke-width=\"3\" d=\"M{:.1},{:.1} L{:.1},{:.1}\"/>", px(xsk),
                        ya - rs * ar_at(xsk).sqrt(), px(xsk), ya + rs * ar_at(xsk).sqrt()));
        cv.name(px(xsk), ya + rs * ar_at(xsk).sqrt() + 14.0, "middle", "shock");
    }
    let head = match s.key {
        "aero_rocket_engine_performance" => format!("F = {}, Isp = {}", qv(s.out("F")?, "N"), qv(s.out("Isp")?, "s")),
        "aero_nozzle_normal_shock" => format!("shock at A/At = {}", num(shock?.0)),
        _ => format!("A/A* = {} → M = {}", num(eps), num(s.out("M")?)),
    };
    cv.bold(16.0, 20.0, "start", &head);
    cv.raw(&format!("<g transform=\"translate(0,130)\">{chart}</g>"));
    let cap = match s.key {
        "aero_mach_from_area_ratio" => format!("A converging–diverging nozzle (drawn 12 throat radii long, inlet area ratio 3 \
                                                stated for the drawing) with the given exit area ratio A/A* = {}, radius ∝ √(A/A*), \
                                                over the Mach number along it: at {} stations the model's Mach number for the \
                                                local area ratio (subsonic before the throat, the given branch after). Marked: \
                                                exit M = {}, where it returns p/p0 = {} and T/T0 = {}.", num(eps), pts.len(),
                                               num(exit_m), num(s.out("p_p0")?), num(s.out("T_T0")?)),
        "aero_nozzle_normal_shock" => {
            let (as_, m1, m2, p0r) = shock?;
            format!("A normal shock standing in a nozzle of expansion ratio {} with chamber pressure {} and back pressure {} (inlet \
                     ratio 3 stated for the drawing). The model places the shock at A/At = {} with M1 = {}, M2 = {} and exit Mach \
                     {}. The Mach number along the nozzle is the model's area–Mach relation at {} stations: supersonic to the \
                     shock, then subsonic with the sonic area grown by the stagnation-pressure loss (A*₂ = A*/(p02/p01), p02/p01 \
                     = {}, derived).", num(eps), qv(c::inp(s, "pc")?, "Pa"), qv(c::inp(s, "pb")?, "Pa"), num(as_), num(m1),
                    num(m2), num(exit_m), pts.len(), num(p0r))
        }
        _ => format!("A rocket engine (chamber {} and {}, expansion ratio {}, ambient {}; inlet area ratio 4 stated for the \
                      drawing). Marked, from the model: thrust F = {}, mass flow {}, Isp = {}, CF = {}, c* = {}, exit pressure \
                      {}, exit speed {}, Me = {}; the Mach number along the nozzle from its area–Mach capability at {} stations.",
                     qv(c::inp(s, "pc")?, "Pa"), qv(c::inp(s, "Tc")?, "K"), num(eps), qv(c::inp(s, "pa")?, "Pa"),
                     qv(s.out("F")?, "N"), qv(s.out("mdot")?, "kg/s"), qv(s.out("Isp")?, "s"), num(s.out("CF")?),
                     qv(s.out("c_star")?, "m/s"), qv(s.out("pe")?, "Pa"), qv(s.out("ve")?, "m/s"), num(exit_m), pts.len()),
    };
    let title = match s.key {
        "aero_mach_from_area_ratio" => "Mach number along a converging–diverging nozzle",
        "aero_nozzle_normal_shock" => "Normal shock inside a nozzle and the Mach number along it",
        _ => "Rocket nozzle: chamber to exit",
    };
    Some(s.figure("nozzle", title, &cap, cv.body(), cv.h))
}

// ------------------------------------------------------------------------------------------------ rockets
fn multistage(s: &Solved) -> Option<Figure> {
    let (isp, m0, mf) = (s.vec("Isp")?, s.vec("m0")?, s.vec("mf")?);
    let dvs = s.out_list("dv_stages")?;
    let n = m0.len();
    if mf.len() != n || dvs.len() != n || isp.len() != n || n > 6 {
        return None;
    }
    let cols: Vec<(String, Vec<(f64, &'static str, String)>)> = (0..n).map(|i| {
        (format!("stage {}", i + 1), vec![(mf[i], "axp-b4", a::kg(mf[i])), ((m0[i] - mf[i]).max(0.0), "axp-b", a::kg(m0[i] - mf[i]))])
    }).collect();
    let notes: Vec<String> = dvs.iter().map(|d| format!("Δv = {}", qv(*d, "m/s"))).collect();
    let body = a::stacked(&cols, &[("burnout mass", "axp-b4"), ("propellant burned", "axp-b")], "mass (kg)",
                          &format!("total Δv = {}", qv(s.out("dv")?, "m/s")), &notes)?;
    let cap = format!("Stages of a multistage rocket (specific impulses {} s, ignition masses {} kg, burnout masses {} kg), each \
                       column the mass a stage starts with, split into burnout mass and propellant. Marked: the model's stage Δv \
                       {} m/s, total {}.", isp.iter().map(|x| num(*x)).collect::<Vec<_>>().join(", "),
                      m0.iter().map(|x| num(*x)).collect::<Vec<_>>().join(", "),
                      mf.iter().map(|x| num(*x)).collect::<Vec<_>>().join(", "),
                      dvs.iter().map(|x| num(*x)).collect::<Vec<_>>().join(", "), qv(s.out("dv")?, "m/s"));
    Some(s.chart("staging", "Multistage rocket: Δv stage by stage", &cap, &c::fit_right(&body)))
}

fn optimal_stack(s: &Solved) -> Option<Figure> {
    let (mpl, m0, pf) = (c::inp(s, "m_pl")?, s.out("m0")?, s.out("payload_fraction")?);
    let (stg, rr) = (s.out_list("stage_mass")?, s.out_list("R")?);
    if stg.is_empty() || stg.len() > 6 {
        return None;
    }
    let classes = ["axp-b", "axp-b2", "axp-b3", "axp-b", "axp-b2", "axp-b3"];
    let mut parts: Vec<(f64, &'static str, String)> = stg.iter().enumerate()
        .map(|(i, m_)| (*m_, classes[i], format!("stage {}: {}", i + 1, a::kg(*m_)))).collect();
    parts.push((mpl, "axp-b4", format!("payload {}", a::kg(mpl))));
    let cols = vec![(format!("lift-off mass {}", a::kg(m0)), parts)];
    let body = a::stacked(&cols, &[("stages, bottom first", "axp-b"), ("payload", "axp-b4")], "mass (kg)",
                          &format!("payload fraction {}", num(pf)), &[format!("payload {} on top; mass ratios {}", a::kg(mpl), rr.iter().map(|x| num(*x)).collect::<Vec<_>>().join(", "))])?;
    let cap = format!("Optimally staged rocket for Δv = {} with a payload of {} (specific impulses {} s, structural fractions {}), \
                       drawn as a stack: stage 1 at the bottom, payload on top. Marked, from the model: stage masses {} kg, \
                       lift-off mass {}, payload fraction {}, stage mass ratios {}.", qv(c::inp(s, "dv")?, "m/s"), qv(mpl, "kg"),
                      s.vec("Isp")?.iter().map(|x| num(*x)).collect::<Vec<_>>().join(", "),
                      s.vec("eps")?.iter().map(|x| num(*x)).collect::<Vec<_>>().join(", "),
                      stg.iter().map(|x| num(*x)).collect::<Vec<_>>().join(", "), qv(m0, "kg"), num(pf),
                      rr.iter().map(|x| num(*x)).collect::<Vec<_>>().join(", "));
    Some(s.chart("staging", "Optimal staging: the rocket's mass stack", &cap, &c::fit_right(&body)))
}

fn single_stage(s: &Solved) -> Option<Figure> {
    let (m0, mf, dv) = if s.key == "aero_rocket_delta_v" {
        (c::inp(s, "m0")?, c::inp(s, "mf")?, s.out("dv")?)
    } else {
        (s.out("m0")?, c::inp(s, "mf")?, c::inp(s, "dv")?)
    };
    let isp = c::inp(s, "Isp")?;
    if !(m0 >= mf && mf > 0.0) {
        return None;
    }
    let cols = vec![("at ignition".to_string(), vec![(mf, "axp-b4", a::kg(mf)), (m0 - mf, "axp-b", a::kg(m0 - mf))]),
                    ("at burnout".to_string(), vec![(mf, "axp-b4", a::kg(mf))])];
    let notes = vec![format!("Δv = {}", qv(dv, "m/s")), format!("m0/mf = {}", num(m0 / mf))];
    let body = a::stacked(&cols, &[("dry (final) mass", "axp-b4"), ("propellant", "axp-b")], "mass (kg)",
                          &format!("Isp = {}", qv(isp, "s")), &notes)?;
    let what = if s.key == "aero_rocket_delta_v" { format!("the model's Δv = {}", qv(dv, "m/s")) }
               else { format!("the model's initial mass m0 = {}, propellant {}, mass ratio {}", qv(m0, "kg"), qv(m0 - mf, "kg"),
                              num(s.out("R")?)) };
    let cap = format!("A single stage with specific impulse {}: mass at ignition and at burnout, the propellant burned stacked on \
                       the final mass (m0 = {}, mf = {}, Δv = {}). Marked: {what}.", qv(isp, "s"), qv(m0, "kg"), qv(mf, "kg"),
                      qv(dv, "m/s"));
    Some(s.chart("staging", "Rocket stage: masses and Δv", &cap, &c::fit_right(&body)))
}

// ------------------------------------------------------------------------------------------------ flight schematics
fn disk(s: &Solved) -> Option<Figure> {
    let fwd = s.key == "aero_actuator_disk_forward";
    let (t, d, rho) = (c::inp(s, "T")?, c::inp(s, "D")?, c::inp(s, "rho")?);
    let v = if fwd { c::inp(s, "V")? } else { 0.0 };
    let (vi, w, pw) = (s.out("v_i")?, s.out("w")?, s.out("P_ideal")?);
    if !(v + vi > 0.0 && v + w > 0.0 && d > 0.0) {
        return None;
    }
    // stream-tube radii from continuity with the returned speeds (derived)
    let rd = 60.0;
    let rw = rd * ((v + vi) / (v + w)).sqrt();
    let r0 = if v > 0.0 { (rd * ((v + vi) / v).sqrt()).min(120.0) } else { 120.0 };
    let mut cv = Canvas::new(330.0);
    let (xd, yc) = (200.0, 165.0);
    let _ = r0;
    let radius = |x: f64| -> f64 {
        // the axial velocity grows from V far ahead to V + w far behind, V + v_i at the disk (v_i = w/2 by the model)
        let xi = x - xd;
        let u = (w / 2.0) * (1.0 + xi / (xi * xi + rd * rd).sqrt());
        rd * ((v + vi) / (v + u).max(1e-9)).sqrt()
    };
    let xs: Vec<f64> = (0..=60).map(|i| 30.0 + 440.0 * i as f64 / 60.0).collect();
    let top: Vec<(f64, f64)> = xs.iter().map(|x| (*x, yc - radius(*x).min(118.0))).collect();
    let bot: Vec<(f64, f64)> = top.iter().map(|p| (p.0, 2.0 * yc - p.1)).collect();
    let mut region = top.clone();
    region.extend(bot.iter().rev());
    area(&mut cv, &region, "axp-s3");
    poly(&mut cv, &top, "axp-c3", false);
    poly(&mut cv, &bot, "axp-c3", false);
    cv.raw(&format!("<path class=\"axp-k\" stroke-width=\"5\" d=\"M{xd},{:.1} L{xd},{:.1}\"/>", yc - rd, yc + rd));
    let sc = 70.0 / (v + w);
    if v > 0.0 {
        varrow(&mut cv, (40.0, yc), (40.0 + v * sc, yc), "axp-c");
    }
    varrow(&mut cv, (xd - 40.0, yc), (xd - 40.0 + (v + vi) * sc, yc), "axp-c");
    varrow(&mut cv, (380.0, yc), (380.0 + (v + w) * sc, yc), "axp-c");
    cv.value(40.0, yc - 14.0, "start", &format!("V = {}", qv(v, "m/s")));
    cv.value(xd + 8.0, yc - rd - 8.0, "start", &format!("at the disk V + v_i, v_i = {}", qv(vi, "m/s")));
    cv.value(360.0, yc + rw + 24.0, "start", &format!("wake: V + w, w = {}", qv(w, "m/s")));
    cv.bold(16.0, 20.0, "start", &format!("T = {}, D = {}, ρ = {}", qv(t, "N"), qv(d, "m"), qv(rho, "kg/m^3")));
    cv.value(16.0, 318.0, "start", &format!("ideal power {}", qv(pw, "W")));
    if fwd {
        cv.value(260.0, 318.0, "start", &format!("ideal efficiency {}", num(s.out("eta_ideal")?)));
    }
    let cap = format!("Momentum (actuator-disk) theory {}: thrust {} on a disk of diameter {} in air of density {}{}. The stream \
                       tube contracts through the disk; its outline is drawn from continuity with the returned speeds (far-wake \
                       radius {}, derived). Marked, from the model: induced speed at the disk v_i = {}, far-wake increment w = {}, \
                       ideal power {}{}.", if fwd { "in forward (axial) flight" } else { "in hover" }, qv(t, "N"), qv(d, "m"),
                      qv(rho, "kg/m^3"), if fwd { format!(", free stream {}", qv(v, "m/s")) } else { String::new() },
                      qv(d / 2.0 * ((v + vi) / (v + w)).sqrt(), "m"), qv(vi, "m/s"), qv(w, "m/s"), qv(pw, "W"),
                      if fwd { format!(", ideal efficiency {}", num(s.out("eta_ideal")?)) } else { String::new() });
    let title = if fwd { "Actuator disk in forward (axial) flight: the stream tube" } else { "Actuator disk in hover: the stream tube" };
    Some(s.figure("schematic", title, &cap, cv.body(), cv.h))
}

fn turn(s: &Solved) -> Option<Figure> {
    let v = c::inp(s, "V")?;
    let (n, phi, r, om, t360) = (s.out("n")?, s.out("phi_deg")?, s.out("R")?, s.out("omega_deg")?, s.out("t_360")?);
    if !(n >= 1.0 && phi >= 0.0 && phi < 89.9) {
        return None;
    }
    let mut cv = Canvas::new(320.0);
    // rear view: wings banked by φ, lift tilted, weight down
    let c0 = (150.0, 190.0);
    let ph = phi.to_radians();
    let half = 95.0;
    let wl = (c0.0 - half * ph.cos(), c0.1 - half * ph.sin());
    let wr = (c0.0 + half * ph.cos(), c0.1 + half * ph.sin());
    cv.raw(&format!("<path class=\"axp-k\" stroke-width=\"4\" d=\"M{:.1},{:.1} L{:.1},{:.1}\"/>", wl.0, wl.1, wr.0, wr.1));
    cv.raw(&format!("<circle class=\"axp-ka\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"12\"/><circle class=\"axp-k\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"12\"/>",
                    c0.0, c0.1, c0.0, c0.1));
    let wlen = 55.0;
    let llen = (wlen * n).min(150.0);
    let lt = (c0.0 + llen * ph.sin(), c0.1 - llen * ph.cos());
    varrow(&mut cv, c0, lt, "axp-c");
    varrow(&mut cv, c0, (c0.0, c0.1 + wlen * llen / (wlen * n)), "axp-c3");
    cv.dashed(c0, (c0.0, c0.1 - 150.0));
    arc(&mut cv, c0, 46.0, PI / 2.0 - ph, PI / 2.0, "axp-c4");
    cv.value(lt.0 + 6.0, lt.1.max(40.0), "start", &format!("L = {} W", num(n)));
    cv.value(c0.0 + 8.0, c0.1 + wlen * llen / (wlen * n) + 4.0, "start", "W");
    cv.value(c0.0 - 8.0, c0.1 - 52.0, "end", &format!("φ = {}", a::dg(phi)));
    cv.name(c0.0, 304.0, "middle", "rear view");
    // top view: the turn circle
    let tc = (395.0, 160.0);
    let tr = 80.0;
    c::ellipse(&mut cv, tc, tr, tr, "axp-c2");
    cv.dot(tc);
    cv.dashed(tc, (tc.0 + tr, tc.1));
    let pl = (tc.0, tc.1 - tr);
    varrow(&mut cv, pl, (pl.0 - 40.0, pl.1), "axp-c3");
    cv.value(tc.0 + 8.0, tc.1 - 6.0, "start", &format!("R = {}", qv(r, "m")));
    cv.value(pl.0 - 44.0, pl.1 - 10.0, "end", &format!("V = {}", qv(v, "m/s")));
    cv.name(tc.0, 262.0, "middle", "top view (not to scale)");
    cv.value(300.0, 288.0, "start", &format!("turn rate {} °/s", num(om)));
    cv.value(300.0, 306.0, "start", &format!("full circle {}", qv(t360, "s")));
    cv.bold(16.0, 22.0, "start", &format!("n = {}, φ = {}", num(n), a::dg(phi)));
    let given = if s.key == "aero_level_turn_bank" { format!("bank angle {}", a::dg(phi)) } else { format!("load factor n = {}", num(n)) };
    let cap = format!("A level, coordinated turn at V = {} with {given}. Rear view: the lift, tilted by the bank angle, carries the \
                       weight and supplies the turning force (L = n·W). Top view: the turn circle. Marked, from the model: n = {}, \
                       φ = {}, radius R = {}, turn rate {} °/s, time for 360° {}.", qv(v, "m/s"), num(n), a::dg(phi), qv(r, "m"),
                      num(om), qv(t360, "s"));
    Some(s.figure("schematic", "Level turn: bank, load factor and radius", &cap, cv.body(), cv.h))
}

fn neutral(s: &Solved) -> Option<Figure> {
    let (hac, hcg, hn, sm) = (c::inp(s, "h_ac")?, c::inp(s, "h_cg")?, s.out("h_n")?, s.out("SM")?);
    let lo = 0.0f64.min(hac).min(hcg).min(hn);
    let hi = 1.0f64.max(hac).max(hcg).max(hn);
    let (x0, x1) = (70.0, 470.0);
    let px = |f: f64| x0 + (f - lo) / (hi - lo) * (x1 - x0);
    let mut cv = Canvas::new(260.0);
    let (le, te) = (px(0.0), px(1.0));
    let yc = 140.0;
    // a schematic aerofoil over the chord
    let n = 40;
    let upper: Vec<(f64, f64)> = (0..=n).map(|i| {
        let t = i as f64 / n as f64;
        (le + (te - le) * t, yc - 36.0 * (2.0 * (t.sqrt() - t)).max(0.0) * 1.6)
    }).collect();
    let lower: Vec<(f64, f64)> = (0..=n).map(|i| {
        let t = i as f64 / n as f64;
        (le + (te - le) * t, yc + 12.0 * (2.0 * (t.sqrt() - t)).max(0.0))
    }).collect();
    let mut region = upper.clone();
    region.extend(lower.iter().rev());
    area(&mut cv, &region, "axp-s4");
    poly(&mut cv, &upper, "axp-k", false);
    poly(&mut cv, &lower, "axp-k", false);
    cv.dashed((le, yc), (te, yc));
    cv.name(le, yc + 34.0, "middle", "LE");
    cv.name(te, yc + 34.0, "middle", "TE");
    let mark = |cv: &mut Canvas, f: f64, lab: String, up: bool, class: &str| {
        let x = px(f);
        cv.raw(&format!("<path class=\"{class}\" stroke-width=\"2.4\" d=\"M{x:.1},{:.1} L{x:.1},{:.1}\"/>", yc - 50.0, yc + 50.0));
        if up {
            cv.value(x, yc - 58.0, "middle", &lab);
        } else {
            cv.value(x, yc + 68.0, "middle", &lab);
        }
    };
    mark(&mut cv, hac, format!("wing ac {}", num(hac)), true, "axp-c4");
    mark(&mut cv, hcg, format!("c.g. {}", num(hcg)), false, "axp-c3");
    mark(&mut cv, hn, format!("neutral point {}", num(hn)), true, "axp-c");
    // the static margin bracket
    let (a1, a2) = (px(hcg), px(hn));
    let yb = yc + 92.0;
    cv.line((a1, yb - 6.0), (a1, yb + 6.0));
    cv.line((a2, yb - 6.0), (a2, yb + 6.0));
    cv.line((a1, yb), (a2, yb));
    cv.value((a1 + a2) / 2.0, yb + 20.0, "middle", &format!("static margin {} c", num(sm)));
    cv.bold(16.0, 22.0, "start", if sm > 0.0 { "statically stable (c.g. ahead of the neutral point)" } else { "statically unstable (c.g. behind the neutral point)" });
    let cap = format!("Longitudinal static stability along the mean chord (fractions of the chord from the leading edge LE to the \
                       trailing edge TE; the aerofoil outline is schematic): wing aerodynamic centre {} and centre of gravity {} \
                       as given. Marked: the model's neutral point {} c and static margin {} c (with CL_α = {}, Cm_α = {}).",
                      num(hac), num(hcg), num(hn), num(sm), num(s.out("CL_alpha")?), num(s.out("Cm_alpha")?));
    Some(s.figure("diagram", "Neutral point and static margin on the chord", &cap, cv.body(), cv.h))
}

fn wind_triangle(s: &Solved) -> Option<Figure> {
    let (tas, crs, wf, ws) = (c::inp(s, "TAS")?, c::inp(s, "course")?, c::inp(s, "wind_from")?, c::inp(s, "wind_speed")?);
    let (hdg, wca, gs) = (s.out("heading_deg")?, s.out("wca_deg")?, s.out("GS")?);
    // vectors (east, north): air along the heading, wind towards wind_from + 180°, ground along the course
    let hr = hdg.to_radians();
    let air = (tas * hr.sin(), tas * hr.cos());
    let wto = wf + PI;
    let wind = (ws * wto.sin(), ws * wto.cos());
    let ground = (gs * crs.sin(), gs * crs.cos());
    let pts = [(0.0, 0.0), air, (air.0 + wind.0, air.1 + wind.1), ground];
    let (xl, xh) = pts.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |a_, p| (a_.0.min(p.0), a_.1.max(p.0)));
    let (yl, yh) = pts.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |a_, p| (a_.0.min(p.1), a_.1.max(p.1)));
    let span = (xh - xl).max(yh - yl).max(1e-9);
    let sc = 220.0 / span;
    let mut cv = Canvas::new(330.0);
    let org = (60.0 + (0.0 - xl) * sc + (220.0 - (xh - xl) * sc) / 2.0, 290.0 - (0.0 - yl) * sc - (220.0 - (yh - yl) * sc) / 2.0);
    let pp = |v: (f64, f64)| (org.0 + v.0 * sc, org.1 - v.1 * sc);
    // collinear vectors are drawn side by side (offset across them) so each stays visible
    let collinear = wca.abs() < 0.05;
    let off = if collinear { (8.0 * crs.cos(), 8.0 * crs.sin()) } else { (0.0, 0.0) };
    let a0 = pp((0.0, 0.0));
    let a1 = pp(air);
    varrow(&mut cv, a0, a1, "axp-c");
    varrow(&mut cv, (a1.0 + off.0, a1.1 + off.1), (pp((air.0 + wind.0, air.1 + wind.1)).0 + off.0, pp((air.0 + wind.0, air.1 + wind.1)).1 + off.1), "axp-c3");
    varrow(&mut cv, (a0.0 - off.0, a0.1 - off.1), (pp(ground).0 - off.0, pp(ground).1 - off.1), "axp-c2");
    // north arrow
    varrow(&mut cv, (470.0, 290.0), (470.0, 250.0), "axp-k");
    cv.bold(470.0, 242.0, "middle", "N");
    let rows = [("axp-c", format!("air {}, heading {}", qv(tas, "m/s"), a::dg(hdg))),
                ("axp-c3", format!("wind {} from {}", qv(ws, "m/s"), a::dg(wf.to_degrees()))),
                ("axp-c2", format!("ground {}, course {}", qv(gs, "m/s"), a::dg(crs.to_degrees())))];
    // the legend starts where its longest line still ends inside the canvas (its key line just left of it)
    let widest = rows.iter().map(|r| ax_engine::figkit::text_w(&r.1, true)).fold(0.0, f64::max);
    let x0 = (512.0 - widest).min(318.0);
    for (i, (cl, t)) in rows.iter().enumerate() {
        let y = 40.0 + 22.0 * i as f64;
        cv.raw(&format!("<path class=\"{cl}\" d=\"M{:.1},{:.1} L{:.1},{:.1}\"/>", x0 - 26.0, y - 4.0, x0 - 6.0, y - 4.0));
        cv.value(x0, y, "start", t);
    }
    cv.value(x0, 120.0, "start", &format!("WCA {}", a::dg(wca)));
    if collinear {
        cv.name(x0, 150.0, "start", "wind along the course:");
        cv.name(x0, 166.0, "start", "vectors side by side");
    }
    cv.bold(16.0, 22.0, "start", "air + wind = ground");
    let cap = format!("Wind triangle for a true airspeed of {} on a course of {} with a {} wind from {}. Air vector + wind vector = \
                       ground vector, to scale. Marked, from the model: heading {}, wind correction angle {}, ground speed {} ({} kt).",
                      qv(tas, "m/s"), a::dg(crs.to_degrees()), qv(ws, "m/s"), a::dg(wf.to_degrees()), a::dg(hdg), a::dg(wca),
                      qv(gs, "m/s"), num(s.out("GS_kt")?));
    Some(s.figure("triangle", "Wind triangle: heading and ground speed", &cap, cv.body(), cv.h))
}
