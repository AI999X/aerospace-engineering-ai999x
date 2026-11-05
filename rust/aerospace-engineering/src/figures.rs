//! Aerospace Engineering's own live figures (ax_engine::figkit), drawn for the visitor's inputs from what the model
//! computes (docs/FIGURE_PARITY.md): values over two inputs, curve families, the standard atmosphere, climb
//! envelopes, compressible-flow charts and polars here; orbits, shocks, nozzles, rockets and flight schematics in
//! figs_aero_draw.rs. Helpers: figs_common.rs, figs_more.rs, figs_aero_kit.rs.

use crate::figs_aero_kit::{self as a, dashed, dotted, ser, Ser};
use crate::figs_common::{self as c, qv};
use crate::figs_more::{self as m, dec, fx, Plot};
use ax_engine::figkit::{self as k, num, Solved};
use ax_engine::figures::Figure;
use ax_engine::Input;

/// The figures for one solved request (the dispatcher ax_engine calls: Pack::figures).
pub fn live(s: &Solved) -> Vec<Figure> {
    let v: Vec<Option<Figure>> = match s.key {
        // values over two inputs
        "aero_calibrated_airspeed" => vec![speed_alt_field(s, "V", "hp", "CAS")],
        "aero_true_airspeed_from_calibrated" => vec![speed_alt_field(s, "CAS", "hp", "TAS")],
        "aero_flight_condition" => vec![speed_alt_field(s, "V", "h", "M")],
        // each solve iterates (supersonic Rayleigh pitot relation): a coarser grid keeps the redraw quick
        "aero_mach_from_pitot" => vec![c::field_map_n(s, "field", "p", "pt", "M", (0.5, 1.5), (0.5, 1.5), 11)],
        "aero_rocket_thrust" => vec![m::field_half(s, "field", "mdot", "ve", "F")],
        "aero_wing_loading" => vec![m::field_half(s, "field", "W", "S", "wing_loading")],
        "aero_total_temperature" => vec![m::field_half(s, "field", "V", "T", "T0")],
        "aero_j2_nodal_regression" => vec![j2_field(s)],
        // curve families
        "aero_level_flight_lift_coefficient" => vec![c::family_std(s, "curve-2", "V", "CL", "W", 0.5, 1.5, false)],
        "aero_min_drag_power_speeds" => vec![c::family_std(s, "curve-2", "S", "V_md", "W", 0.5, 1.5, false)],
        "aero_stall_speed" => vec![c::family_std(s, "curve-2", "S", "V_stall", "W", 0.5, 1.5, false)],
        // the standard atmosphere
        "aero_isa_properties" => vec![isa_temperature(s), isa_three(s)],
        "aero_pressure_altitude" => vec![pressure_profile(s)],
        "aero_density_altitude" => vec![density_profile(s)],
        // climb, entry, glide
        "aero_absolute_ceiling_jet" | "aero_service_ceiling_jet" => vec![envelope(s)],
        "aero_ballistic_entry_velocity" => vec![entry_profile(s)],
        "aero_glide_range" => vec![glide_path(s)],
        // compressible flow and polars
        "aero_isentropic_flow" => vec![isentropic_chart(s)],
        "aero_max_deflection_angle" => vec![theta_beta(s)],
        "aero_drag_polar_optimum" => vec![polar(s, false), polar(s, true)],
        "aero_wind_triangle" => vec![wind_polar(s)],
        "aero_normal_shock" => vec![shock_table(s)],
        _ => vec![],
    };
    let mut out: Vec<Figure> = v.into_iter().flatten().collect();
    out.extend(crate::figs_aero_draw::live(s));
    out
}

// ------------------------------------------------------------------------------------------------ fields
/// A speed over half to one and a half times the given one, and an altitude from sea level to 11 km (or one and a
/// half times the given altitude, when higher).
fn speed_alt_field(s: &Solved, v: &str, h: &str, z: &str) -> Option<Figure> {
    let (v0, h0) = (c::inp(s, v)?, c::inp(s, h)?);
    let top = if h0 > 8000.0 { h0 * 1.5 } else { 11000.0 };
    let bot = if h0 < 0.0 { h0 * 1.5 } else { 0.0 };
    m::field_xy(s, "field", v, fx(v0, 0.5, 1.5), h, (bot, top, false), z)
}

/// The node drift over the perigee altitude (a tenth to ten times, logarithmic) and the inclination (0 to 180°).
fn j2_field(s: &Solved) -> Option<Figure> {
    let hp = c::inp(s, "hp")?;
    if !(hp > 0.0) {
        return None;
    }
    m::field_xy(s, "field", "hp", dec(hp), "i", (0.0, std::f64::consts::PI, false), "raan_rate_deg_day")
}

// ------------------------------------------------------------------------------------------------ the atmosphere
/// The standard atmosphere at an altitude (h in m): an output of the model's atmosphere capability.
fn isa(s: &Solved, h: f64, y: &str) -> Option<f64> {
    a::o(s, "aero_isa_properties", &[("h", h)], y)
}

fn isa_temperature(s: &Solved) -> Option<Figure> {
    let (h0, t0) = (c::inp(s, "h")?, s.out("T")?);
    let hs = k::span(0.0, 47000.0, 95, false);
    let pts: Vec<(f64, f64)> = hs.iter().filter_map(|h| isa(s, *h, "T").map(|t| (t, h / 1000.0))).collect();
    if pts.len() < 10 {
        return None;
    }
    let xr = a::rng(&pts.iter().map(|p| p.0).chain([t0]).collect::<Vec<_>>(), 0.06, false)?;
    let p = Plot::new(xr, (-1.0f64.min(h0 / 1000.0), 47.0f64.max(h0 / 1000.0), false));
    let t11 = isa(s, 11000.0, "T");
    let mut marks = vec![((t0, h0 / 1000.0), format!("{} at {}", qv(t0, "K"), qv(h0, "m")))];
    if let Some(t) = t11 {
        marks.push(((t, 11.0), format!("{} at 11 km", qv(t, "K"))));
    }
    let body = c::fit_right(&a::plot(&p, &[ser("temperature", pts.clone(), "axp-c")], &marks, None,
                                     "T, temperature (K)", "h, geometric altitude (km)", &c::eqs(s, "T", t0)));
    let cap = format!("Temperature of the International Standard Atmosphere from 0 to 47 km, {} altitudes each a solve of \
                       the model: {} at sea level, {} at 11 km (the tropopause), {} at 47 km. Marked: the given altitude \
                       h = {} with T = {}.", pts.len(), qv(pts[0].0, "K"),
                      t11.map(|t| qv(t, "K")).unwrap_or_else(|| "?".into()), qv(pts[pts.len() - 1].0, "K"), qv(h0, "m"),
                      qv(t0, "K"));
    Some(s.chart("profile", "The standard atmosphere: temperature with altitude", &cap, &body))
}

fn isa_three(s: &Solved) -> Option<Figure> {
    let (h0, t0, sg0, dl0) = (c::inp(s, "h")?, s.out("T")?, s.out("sigma")?, s.out("delta")?);
    let hs = k::span(0.0, 47000.0, 48, false);
    let mut tp = vec![];
    let mut sp = vec![];
    let mut dp = vec![];
    for h in &hs {
        let Some(a_) = s.solve_other("aero_isa_properties", &[("h", Input::Float(*h))]) else { continue };
        if let (Some(t), Some(sg), Some(dl)) = (k::num_of(&a_, "T"), k::num_of(&a_, "sigma"), k::num_of(&a_, "delta")) {
            tp.push((t, h / 1000.0));
            sp.push((sg, h / 1000.0));
            dp.push((dl, h / 1000.0));
        }
    }
    if tp.len() < 10 {
        return None;
    }
    let yr = (-1.0f64.min(h0 / 1000.0), 47.0f64.max(h0 / 1000.0), false);
    let p1 = Plot::new(a::rng(&tp.iter().map(|p| p.0).chain([t0]).collect::<Vec<_>>(), 0.06, false)?, yr);
    let b1 = a::plot(&p1, &[ser("temperature", tp.clone(), "axp-c")], &[((t0, h0 / 1000.0), qv(t0, "K"))], None,
                     "T, temperature (K)", "h, altitude (km)", &format!("h = {}: T = {}", qv(h0, "m"), qv(t0, "K")));
    let all: Vec<f64> = sp.iter().chain(dp.iter()).map(|p| p.0).chain([sg0, dl0]).collect();
    let p2 = Plot::new(a::rng(&all, 0.05, true)?, yr);
    let b2 = a::plot(&p2, &[ser("pressure δ = p/p₀", dp.clone(), "axp-c3"), ser("density σ = ρ/ρ₀", sp.clone(), "axp-c2")],
                     &[((sg0, h0 / 1000.0), format!("σ = {}", num(sg0)))], Some((330.0, 52.0)),
                     "ratio to sea level (logarithmic)", "h, altitude (km)", &format!("σ = {}, δ = {}", num(sg0), num(dl0)));
    let (body, h) = k::stack(&[c::fit_right(&b1), c::fit_right(&b2)]);
    let cap = format!("The International Standard Atmosphere to 47 km, {} altitudes each a solve of the model: temperature \
                       (top) and the pressure and density ratios on a logarithmic axis (bottom). Marked: the given altitude \
                       h = {}: T = {}, p = {}, ρ = {}, σ = {}, δ = {}.", tp.len(), qv(h0, "m"), qv(t0, "K"),
                      qv(s.out("p").unwrap_or(f64::NAN), "Pa"), qv(s.out("rho").unwrap_or(f64::NAN), "kg/m^3"), num(sg0),
                      num(dl0));
    Some(s.figure("profile-2", "Standard atmosphere: temperature, pressure and density", &cap, &body, h))
}

fn pressure_profile(s: &Solved) -> Option<Figure> {
    let (p0, h0) = (c::inp(s, "p")?, s.out("h")?);
    let hs = k::span(0.0, 40000.0f64.max(h0 * 1.1), 41, false);
    let pts: Vec<(f64, f64)> = hs.iter().filter_map(|h| isa(s, *h, "p").map(|p| (p / 1000.0, h / 1000.0))).collect();
    if pts.len() < 10 {
        return None;
    }
    let xr = a::rng(&pts.iter().map(|q| q.0).chain([p0 / 1000.0]).collect::<Vec<_>>(), 0.04, true)?;
    let yr = (0.0f64.min(h0 / 1000.0 - 1.0), pts[pts.len() - 1].1.max(h0 / 1000.0), false);
    let p = Plot::new(xr, yr);
    let body = c::fit_right(&a::plot(&p, &[ser("standard pressure with altitude", pts.clone(), "axp-c")],
                                     &[((p0 / 1000.0, h0 / 1000.0), format!("h = {}", qv(h0, "m")))], Some((300.0, 52.0)),
                                     "p, static pressure (kPa, logarithmic)", "h, altitude (km)",
                                     &format!("p = {} → h = {}", qv(p0, "Pa"), qv(h0, "m"))));
    let cap = format!("Pressure altitude read off the standard atmosphere: the model's standard pressure at {} altitudes to \
                       {} (logarithmic pressure axis). Marked: the given pressure {} gives the pressure altitude h = {} \
                       ({} ft), standard temperature {}.", pts.len(), qv(hs[hs.len() - 1], "m"), qv(p0, "Pa"), qv(h0, "m"),
                      num(s.out("h_ft").unwrap_or(f64::NAN)), qv(s.out("T_std").unwrap_or(f64::NAN), "K"));
    Some(s.chart("profile", "Pressure altitude on the standard atmosphere", &cap, &body))
}

fn density_profile(s: &Solved) -> Option<Figure> {
    let (sg, hr) = (s.out("sigma")?, s.out("h_rho")?);
    let top = 20000.0f64.max(hr * 1.2);
    let hs = k::span(0.0f64.min(hr * 1.2), top, 41, false);
    let pts: Vec<(f64, f64)> = hs.iter().filter_map(|h| isa(s, *h, "sigma").map(|v| (v, h / 1000.0))).collect();
    if pts.len() < 10 {
        return None;
    }
    let xr = a::rng(&pts.iter().map(|q| q.0).chain([sg]).collect::<Vec<_>>(), 0.05, false)?;
    let p = Plot::new(xr, (hs[0] / 1000.0, top / 1000.0, false));
    let body = c::fit_right(&a::plot(&p, &[ser("standard density ratio with altitude", pts.clone(), "axp-c")],
                                     &[((sg, hr / 1000.0), format!("density altitude {}", qv(hr, "m")))], Some((300.0, 52.0)),
                                     "σ, density ratio ρ/ρ₀", "h, altitude (km)", &format!("σ = {}", num(sg))));
    let cap = format!("Density altitude: the standard altitude with the same air density. The curve is the model's standard \
                       density ratio at {} altitudes to {}. Marked: the given conditions give ρ = {}, σ = {} and the \
                       density altitude {} ({} ft).", pts.len(), qv(top, "m"), qv(s.out("rho").unwrap_or(f64::NAN), "kg/m^3"),
                      num(sg), qv(hr, "m"), num(s.out("h_rho_ft").unwrap_or(f64::NAN)));
    Some(s.chart("profile", "Density altitude on the standard atmosphere", &cap, &body))
}

// ------------------------------------------------------------------------------------------------ climb, entry, glide
/// Best rate of climb with altitude, the thrust lapsing with the standard density ratio (as the ceiling capabilities
/// assume): each point the model's density ratio and its maximum rate of climb.
fn envelope(s: &Solved) -> Option<Figure> {
    let ins: Vec<f64> = ["W", "S", "T0", "CD0", "AR", "e"].iter().map(|x| c::inp(s, x)).collect::<Option<Vec<_>>>()?;
    let service = s.key == "aero_service_ceiling_jet";
    let hc = if service { s.out("h_s")? } else { s.out("h_abs")? };
    let rcs = if service { c::inp(s, "RCs").unwrap_or(0.508) } else { 0.0 };
    let habs = if service {
        a::o(s, "aero_absolute_ceiling_jet", &[("W", ins[0]), ("S", ins[1]), ("T0", ins[2]), ("CD0", ins[3]),
                                               ("AR", ins[4]), ("e", ins[5])], "h_abs")
    } else {
        Some(hc)
    };
    let top = (habs.unwrap_or(hc) * 1.12).max(2000.0).min(47000.0);
    let rc_at = |h: f64| -> Option<f64> {
        let sg = isa(s, h, "sigma")?;
        a::o(s, "aero_max_rate_of_climb_jet", &[("W", ins[0]), ("S", ins[1]), ("h", h), ("T", ins[2] * sg),
                                                ("CD0", ins[3]), ("AR", ins[4]), ("e", ins[5])], "RC_max")
    };
    let pts: Vec<(f64, f64)> = k::span(0.0, top, 41, false).iter().filter_map(|h| rc_at(*h).map(|r| (r, h / 1000.0))).collect();
    if pts.len() < 5 {
        return None;
    }
    let xr = a::from0(a::rng(&pts.iter().map(|p| p.0).chain([0.0]).collect::<Vec<_>>(), 0.05, false)?);
    let p = Plot::new(xr, (0.0, top / 1000.0, false));
    let mut series = vec![ser("best climb rate, thrust ∝ σ", pts.clone(), "axp-c")];
    if service {
        series.push(dashed(&format!("RC = {}", qv(rcs, "m/s")), vec![(rcs, 0.0), (rcs, top / 1000.0)], "axp-c4"));
    }
    let what = if service { "service ceiling" } else { "absolute ceiling" };
    let mut marks = vec![((rcs, hc / 1000.0), format!("{what} {}", qv(hc, "m")))];
    if let (true, Some(ha)) = (service, habs) {
        marks.push(((0.0, ha / 1000.0), format!("absolute {}", qv(ha, "m"))));
    }
    let body = c::fit_right(&a::plot(&p, &series, &marks, Some((110.0, 222.0)), "RC_max, best rate of climb (m/s)",
                                     "h, altitude (km)", &format!("{what}: {}", qv(hc, "m"))));
    let cap = format!("Climb performance with altitude of a jet (W = {}, S = {} m², sea-level thrust {}, C_D0 = {}, AR = {}, \
                       e = {}): at {} altitudes the model's best rate of climb with the thrust lapsing in proportion to the \
                       standard density ratio (also from the model), as its ceiling capabilities assume. Marked: the {what} \
                       {}{}.", qv(ins[0], "N"), num(ins[1]), qv(ins[2], "N"), num(ins[3]), num(ins[4]), num(ins[5]), pts.len(),
                      qv(hc, "m"), match (service, habs) {
                          (true, Some(ha)) => format!(" and the absolute ceiling of the same aircraft, {}", qv(ha, "m")),
                          _ => String::new(),
                      });
    Some(s.chart("envelope", "Rate of climb with altitude and the ceilings", &cap, &body))
}

/// Speed against altitude of a ballistic entry: the model at 91 altitudes; the peak deceleration from its
/// neighbouring capability.
fn entry_profile(s: &Solved) -> Option<Figure> {
    let (h0, v0, n0, ve) = (c::inp(s, "h")?, s.out("V")?, s.out("n")?, c::inp(s, "VE")?);
    let top = 100000.0f64.max(h0 * 1.1);
    let pts: Vec<(f64, f64)> = k::span(0.0, top, 91, false).iter()
        .filter_map(|h| c::at(s, &[("h", *h)], "V").map(|v| (v / 1000.0, h / 1000.0))).collect();
    if pts.len() < 10 {
        return None;
    }
    let mut set: Vec<(&str, Input)> = ["m", "CD", "A", "VE", "gamma_e", "H", "rho0"].iter()
        .filter_map(|x| c::inp(s, x).map(|v| (*x, Input::Float(v)))).collect();
    set.retain(|x| !matches!(x.1, Input::Float(v) if !v.is_finite()));
    let peak = a::other(s, "aero_ballistic_entry_peak_deceleration", set);
    let xr = a::from0(a::rng(&pts.iter().map(|p| p.0).chain([ve / 1000.0]).collect::<Vec<_>>(), 0.05, false)?);
    let p = Plot::new(xr, (0.0, top / 1000.0, false));
    let mut marks = vec![((v0 / 1000.0, h0 / 1000.0), format!("{} g at {}", num(n0), qv(h0, "m")))];
    let mut series = vec![ser("speed", pts.clone(), "axp-c"),
                          dashed("entry speed", vec![(ve / 1000.0, 0.0), (ve / 1000.0, top / 1000.0)], "axp-c4")];
    let mut peak_note = String::new();
    if let Some(pk) = &peak {
        if let (Some(hp), Some(vp), Some(np)) = (k::num_of(pk, "h_peak"), k::num_of(pk, "V_peak"), k::num_of(pk, "n_max")) {
            marks.push(((vp / 1000.0, hp / 1000.0), format!("peak {} g", num(np))));
            series.push(dotted("", vec![(vp / 1000.0, hp / 1000.0)], "axp-c3"));
            peak_note = format!(" Also marked: the peak deceleration {} g at h = {}, V = {} (the model's peak-deceleration \
                                 capability).", num(np), qv(hp, "m"), qv(vp, "m/s"));
        }
    }
    let body = c::fit_right(&a::plot(&p, &series, &marks, Some((110.0, 52.0)), "V, speed (km/s)", "h, altitude (km)",
                                     &format!("V = {} at h = {}", qv(v0, "m/s"), qv(h0, "m"))));
    let cap = format!("Speed against altitude of a ballistic entry (m = {} kg, entry speed {}), {} altitudes each a solve of \
                       the model. Marked: at the given altitude h = {} the speed V = {} and deceleration {} g.{peak_note}",
                      num(c::inp(s, "m")?), qv(ve, "m/s"), pts.len(), qv(h0, "m"), qv(v0, "m/s"), num(n0));
    Some(s.chart("profile", "Ballistic entry: speed with altitude", &cap, &body))
}

fn glide_path(s: &Solved) -> Option<Figure> {
    let (dh, ld, r, ang) = (c::inp(s, "dh")?, c::inp(s, "LD")?, s.out("range")?, s.out("glide_angle_deg")?);
    if !(dh > 0.0 && r > 0.0) {
        return None;
    }
    let p = Plot::new((0.0, r / 1000.0 * 1.08, false), (0.0, dh / 1000.0 * 1.25, false));
    let series = vec![ser(&format!("glide path, L/D = {}", num(ld)), vec![(0.0, dh / 1000.0), (r / 1000.0, 0.0)], "axp-c"),
                      dashed("", vec![(0.0, 0.0), (r / 1000.0, 0.0)], "axp-c4")];
    let marks = vec![((r / 1000.0, 0.0), format!("range {}", qv(r, "m"))),
                     ((0.0, dh / 1000.0), format!("height lost {}", qv(dh, "m")))];
    let body = a::plot(&p, &series, &marks, Some((300.0, 52.0)), "distance (km)", "height (km), axis stretched",
                       &format!("glide angle {}", a::dg(ang)));
    let cap = format!("A still-air glide from {} with lift-to-drag ratio {}: the straight path to the ground (the vertical \
                       axis is stretched). Marked: the model's range {} and glide angle {}.", qv(dh, "m"), num(ld), qv(r, "m"),
                      a::dg(ang));
    Some(s.chart("path", "Glide path: range from height and L/D", &cap, &c::fit_right(&body)))
}

// ------------------------------------------------------------------------------------------------ compressible flow
fn isentropic_chart(s: &Solved) -> Option<Figure> {
    let m0 = c::inp(s, "M")?;
    let g = c::inp(s, "gamma");
    let top = 4.0f64.max(m0 * 1.2).min(10.0);
    let ms = k::span(0.1f64.min(m0), top, 61, false);
    let names = [("A_Astar", "A/A*", "axp-c"), ("p0_p", "p₀/p", "axp-c2"), ("rho0_rho", "ρ₀/ρ", "axp-c3"), ("T0_T", "T₀/T", "axp-c4")];
    let mut cols: Vec<Vec<(f64, f64)>> = vec![vec![]; 4];
    for mv in &ms {
        let mut set = vec![("M", *mv)];
        if let Some(gv) = g {
            set.push(("gamma", gv));
        }
        let Some(ans) = s.with(&set) else { continue };
        for (i, (sym, _, _)) in names.iter().enumerate() {
            if let Some(v) = k::num_of(&ans, sym) {
                cols[i].push((*mv, v));
            }
        }
    }
    if cols[0].len() < 10 {
        return None;
    }
    let all: Vec<f64> = cols.iter().flatten().map(|p| p.1).collect();
    let yr = a::rng(&all, 0.03, true)?;
    let p = Plot::new((0.0, top, false), (yr.0.max(0.8), yr.1, true));
    let series: Vec<Ser> = names.iter().zip(cols.iter()).map(|((_, n, cl), pts)| ser(n, pts.clone(), cl)).collect();
    let ar = s.out("A_Astar")?;
    let body = c::fit_right(&a::plot(&p, &series, &[((m0, ar), format!("A/A* = {}", num(ar)))], Some((230.0, 52.0)),
                                     "M, Mach number", "ratio (logarithmic)", &format!("M = {}", num(m0))));
    let cap = format!("Isentropic flow (γ = {}): area, pressure, density and temperature ratios against Mach number, {} solves \
                       of the model from M = {} to {}, logarithmic axis; A/A* is least, 1, at M = 1. Marked: the given M = {}: \
                       A/A* = {}, p0/p = {}, ρ0/ρ = {}, T0/T = {}.", num(g.unwrap_or(1.4)), cols[0].len(), num(ms[0]), num(top),
                      num(m0), num(ar), num(s.out("p0_p")?), num(s.out("rho0_rho")?), num(s.out("T0_T")?));
    Some(s.chart("chart", "Isentropic flow table as curves", &cap, &body))
}

/// θ against β for several upstream Mach numbers (the model's oblique shock from the wave angle), the given Mach
/// number's maximum deflection marked.
fn theta_beta(s: &Solved) -> Option<Figure> {
    let (m1, tmax, bmax) = (c::inp(s, "M1")?, s.out("theta_max_deg")?, s.out("beta_deg")?);
    let g = c::inp(s, "gamma");
    let mut machs = vec![m1];
    for v in [1.5, 3.0, 5.0] {
        if machs.iter().all(|x| (x - v).abs() > 0.25) {
            machs.push(v);
        }
    }
    let classes = ["axp-c", "axp-c2", "axp-c3", "axp-c4"];
    let betas = k::span(1.0, 90.0, 60, false);
    let mut series = vec![];
    for (i, mv) in machs.iter().take(4).enumerate() {
        let mut pts = vec![];
        for b in &betas {
            let mut set: Vec<(&str, Input)> = vec![("M1", Input::Float(*mv)), ("beta", Input::Float(b.to_radians()))];
            if let Some(gv) = g {
                set.push(("gamma", Input::Float(gv)));
            }
            if let Some(ans) = s.solve_other("aero_oblique_shock_from_wave_angle", &set) {
                if let Some(t) = k::num_of(&ans, "theta_deg") {
                    if t >= 0.0 {
                        pts.push((*b, t));
                    }
                }
            }
        }
        if pts.len() >= 3 {
            let name = if i == 0 { format!("M1 = {} (given)", num(*mv)) } else { format!("M1 = {}", num(*mv)) };
            series.push(ser(&name, pts, classes[i]));
        }
    }
    if series.is_empty() {
        return None;
    }
    let ymax = series.iter().flat_map(|s_| s_.pts.iter().map(|p| p.1)).fold(tmax, f64::max);
    let p = Plot::new((0.0, 90.0, false), (0.0, ymax * 1.12, false));
    let body = c::fit_right(&a::plot(&p, &series, &[((bmax, tmax), format!("θmax = {}", a::dg(tmax)))], Some((110.0, 52.0)),
                                     "β, wave angle (°)", "θ, flow deflection (°)",
                                     &format!("M1 = {}: θmax = {} at β = {}", num(m1), a::dg(tmax), a::dg(bmax))));
    let cap = format!("The θ–β–M chart: flow deflection against shock wave angle for upstream Mach numbers {}, each point the \
                       model's oblique shock at that wave angle (wave angles below the Mach angle are refused and left out). \
                       Marked: the model's maximum deflection for M1 = {}, θmax = {} at β = {} (there M2 = {}, p2/p1 = {}); a \
                       larger wedge detaches the shock.", machs.iter().take(4).map(|x| num(*x)).collect::<Vec<_>>().join(", "),
                      num(m1), a::dg(tmax), a::dg(bmax), num(s.out("M2")?), num(s.out("p2_p1")?));
    Some(s.chart("chart", "θ–β–M chart with the maximum deflection", &cap, &body))
}

/// The normal-shock table for upstream Mach numbers around the given one, each row a solve of the model.
fn shock_table(s: &Solved) -> Option<Figure> {
    let m1 = c::inp(s, "M1")?;
    let g = c::inp(s, "gamma");
    let mut machs: Vec<f64> = vec![1.5, 2.0, 3.0, 4.0, 5.0];
    if !machs.iter().any(|x| (x - m1).abs() < 1e-9) {
        machs.push(m1);
        machs.sort_by(|x, y| x.partial_cmp(y).unwrap());
    }
    let syms = ["M2", "p2_p1", "rho2_rho1", "T2_T1", "p02_p01"];
    let mut rows = vec![];
    let mut mark = None;
    for mv in &machs {
        let mut set = vec![("M1", *mv)];
        if let Some(gv) = g {
            set.push(("gamma", gv));
        }
        let Some(ans) = s.with(&set) else { continue };
        let vals: Option<Vec<f64>> = syms.iter().map(|y| k::num_of(&ans, y)).collect();
        if let Some(v) = vals {
            if (mv - m1).abs() < 1e-9 {
                mark = Some(rows.len());
            }
            rows.push((format!("M₁ = {}", num(*mv)), v));
        }
    }
    let cols: Vec<String> = ["M₂", "p₂/p₁", "ρ₂/ρ₁", "T₂/T₁", "p₀₂/p₀₁"].iter().map(|x| x.to_string()).collect();
    let (body, h) = a::shaded_table(&rows, &cols, mark, &format!("M1 = {}: M2 = {}", num(m1), num(s.out("M2")?)))?;
    let cap = format!("The normal-shock table (γ = {}): for upstream Mach numbers {} (rows) the downstream Mach number and the \
                       pressure, density, temperature and stagnation-pressure ratios, every row a solve of the model and each \
                       column shaded by its own largest value; outlined: the given M1 = {}: M2 = {}, p2/p1 = {}, ρ2/ρ1 = {}, \
                       T2/T1 = {}, p02/p01 = {}.", num(g.unwrap_or(1.4)),
                      rows.iter().map(|r| r.0.replace("M₁ = ", "")).collect::<Vec<_>>().join(", "), num(m1),
                      num(s.out("M2")?), num(s.out("p2_p1")?), num(s.out("rho2_rho1")?), num(s.out("T2_T1")?),
                      num(s.out("p02_p01")?));
    Some(s.figure("heatmap", "The normal-shock table", &cap, &body, h))
}

// ------------------------------------------------------------------------------------------------ polars
/// The drag polar from the model's induced drag (W = 10 kN, S = 16 m², sea level, stated) and its optimum points.
fn polar(s: &Solved, scatter: bool) -> Option<Figure> {
    let (cd0, ar, e) = (c::inp(s, "CD0")?, c::inp(s, "AR")?, c::inp(s, "e")?);
    let (clmd, ldmax) = (s.out("CL_md")?, s.out("LD_max")?);
    let n = if scatter { 12 } else { 46 };
    let mut pts = vec![];
    for v in k::span(24.0, 110.0, n, true) {
        let Some(ans) = s.solve_other("aero_induced_drag", &[("W", Input::Float(10000.0)), ("S", Input::Float(16.0)),
                                                             ("V", Input::Float(v)), ("h", Input::Float(0.0)),
                                                             ("AR", Input::Float(ar)), ("e", Input::Float(e))]) else { continue };
        if let (Some(cl), Some(cdi)) = (k::num_of(&ans, "CL"), k::num_of(&ans, "CDi")) {
            pts.push((cd0 + cdi, cl));
        }
    }
    if pts.len() < 6 {
        return None;
    }
    pts.sort_by(|x, y| x.1.partial_cmp(&y.1).unwrap());
    let cdmd = clmd / ldmax;
    let xmax = pts.iter().map(|p| p.0).fold(cdmd, f64::max) * 1.08;
    let ymax = pts.iter().map(|p| p.1).fold(clmd, f64::max) * 1.08;
    let p = Plot::new((0.0, xmax, false), (0.0, ymax, false));
    let tangent = vec![(0.0, 0.0), (xmax, xmax * ldmax)];
    let mut series = vec![if scatter { dotted(&format!("{} computed points", pts.len()), pts.clone(), "axp-c") }
                          else { ser("drag polar", pts.clone(), "axp-c") },
                          dashed(&format!("tangent, L/D {}", num(ldmax)), tangent, "axp-c2"),
                          dashed(&format!("C_D0 = {}", num(cd0)), vec![(cd0, 0.0), (cd0, ymax)], "axp-c4")];
    let mut marks = vec![((cdmd, clmd), format!("min drag C_L {}", num(clmd)))];
    let mut note = String::new();
    if !scatter {
        let (clmp, ldmp, clmr, ldmr) = (s.out("CL_mp")?, s.out("LD_mp")?, s.out("CL_mr")?, s.out("LD_mr")?);
        let pts3 = vec![(clmp / ldmp, clmp), (clmr / ldmr, clmr)];
        series.push(dotted("", pts3, "axp-c3"));
        marks.push(((clmp / ldmp, clmp), format!("min power C_L {}", num(clmp))));
        marks.push(((clmr / ldmr, clmr), format!("max range C_L {}", num(clmr))));
        note = format!(", minimum power C_L = {} (L/D {}), best jet range C_L = {} (L/D {})", num(clmp), num(ldmp), num(clmr),
                       num(ldmr));
    }
    let leg = if scatter { (330.0, 215.0) } else { (160.0, 52.0) };
    let body = c::fit_right(&a::plot(&p, &series, &marks, Some(leg), "C_D, drag coefficient",
                                     "C_L, lift coefficient", &format!("(L/D)max = {} at C_L = {}", num(ldmax), num(clmd))));
    let (kind, title) = if scatter { ("scatter", "Drag polar and the best lift-to-drag ratio") }
                        else { ("polar", "Drag polar with its optimum points") };
    let cap = format!("Parabolic drag polar for C_D0 = {}, aspect ratio {}, Oswald factor {}: C_D = C_D0 + C_Di with the \
                       induced drag from the model at {} speeds (W = 10 kN, S = 16 m², sea level, stated); the tangent from the \
                       origin touches at the best lift-to-drag point. Marked, each at C_D = C_L/(L/D) of the returned values: \
                       minimum drag C_L = {} with (L/D)max = {}{note}.", num(cd0), num(ar), num(e), pts.len(), num(clmd),
                      num(ldmax));
    Some(s.chart(kind, title, &cap, &body))
}

/// Ground speed for every course (5° steps) through the given wind, each solved by the model: a polar plot, north
/// up.
fn wind_polar(s: &Solved) -> Option<Figure> {
    let (tas, wf, ws) = (c::inp(s, "TAS")?, c::inp(s, "wind_from")?, c::inp(s, "wind_speed")?);
    let mut pts = vec![];
    for i in 0..72 {
        let crs = (i as f64 * 5.0).to_radians();
        if let Some(gs) = c::at(s, &[("course", crs)], "GS") {
            pts.push((crs, gs));
        }
    }
    if pts.len() < 24 {
        return None;
    }
    let gmax = pts.iter().map(|p| p.1).fold(0.0, f64::max);
    let best = pts.iter().cloned().fold((0.0, 0.0), |a_, b| if b.1 > a_.1 { b } else { a_ });
    let worst = pts.iter().cloned().fold((0.0, f64::INFINITY), |a_, b| if b.1 < a_.1 { b } else { a_ });
    let (cx, cy, rad) = (180.0, 178.0, 128.0);
    let sc = rad / (gmax * 1.05);
    let at = |crs: f64, r: f64| (cx + r * sc * crs.sin(), cy - r * sc * crs.cos());
    let mut cv = k::Canvas::new(360.0);
    let step = c::nice_levels(0.0, gmax, 3);
    for r in step.iter().filter(|r| **r > 0.0) {
        m_ring(&mut cv, (cx, cy), r * sc);
        cv.name(cx + 4.0, cy - r * sc - 3.0, "start", &qv(*r, "m/s"));
    }
    for (i, lab) in ["N", "E", "S", "W"].iter().enumerate() {
        let th = (i as f64 * 90.0).to_radians();
        let (x, y) = (cx + (rad + 14.0) * th.sin(), cy - (rad + 14.0) * th.cos() + 5.0);
        cv.bold(x, y, "middle", lab);
        cv.raw(&format!("<path class=\"axp-g\" d=\"M{cx:.1},{cy:.1} L{:.1},{:.1}\"/>", cx + rad * th.sin(), cy - rad * th.cos()));
    }
    let mut d = String::new();
    for (i, (crs, gs)) in pts.iter().enumerate() {
        let (x, y) = at(*crs, *gs);
        d.push_str(&format!("{}{x:.1},{y:.1} ", if i == 0 { "M" } else { "L" }));
    }
    cv.raw(&format!("<path class=\"axp-c\" d=\"{}Z\"/>", d.trim_end()));
    // the airspeed circle and the wind arrow (blowing towards wind_from + 180°)
    m_ring_class(&mut cv, (cx, cy), tas * sc, "axp-c4");
    let tip = (cx + ws * sc * (wf + std::f64::consts::PI).sin(), cy - ws * sc * (wf + std::f64::consts::PI).cos());
    crate::figs_common::varrow(&mut cv, (cx, cy), tip, "axp-c3");
    let (bx, by) = at(best.0, best.1);
    cv.raw(&format!("<circle class=\"axp-m\" cx=\"{bx:.1}\" cy=\"{by:.1}\" r=\"5.5\"/>"));
    let x0 = 352.0;
    cv.bold(x0, 40.0, "start", "ground speed");
    cv.raw(&format!("<path class=\"axp-c\" d=\"M{x0},60 L{},60\"/>", x0 + 20.0));
    cv.name(x0 + 26.0, 64.0, "start", "by course");
    cv.raw(&format!("<path class=\"axp-c4\" d=\"M{x0},82 L{},82\"/>", x0 + 20.0));
    cv.name(x0 + 26.0, 86.0, "start", &format!("air {}", qv(tas, "m/s")));
    cv.raw(&format!("<path class=\"axp-c3\" d=\"M{x0},104 L{},104\"/>", x0 + 20.0));
    cv.name(x0 + 26.0, 108.0, "start", &format!("wind {}", qv(ws, "m/s")));
    cv.value(x0, 140.0, "start", &format!("fastest {}", qv(best.1, "m/s")));
    cv.value(x0, 158.0, "start", &format!("  course {}", a::dg(best.0.to_degrees())));
    cv.value(x0, 182.0, "start", &format!("slowest {}", qv(worst.1, "m/s")));
    cv.value(x0, 200.0, "start", &format!("  course {}", a::dg(worst.0.to_degrees())));
    cv.name(x0, 236.0, "start", &format!("wind from {}", a::dg(wf.to_degrees())));
    cv.name(x0, 330.0, "start", "north up");
    let cap = format!("Ground speed for every course (5° steps, {} solves of the model's wind triangle) of an aircraft flying {} \
                       through a {} wind from {}: slowest {} on course {}, fastest {} on course {} (marked).", pts.len(),
                      qv(tas, "m/s"), qv(ws, "m/s"), a::dg(wf.to_degrees()), qv(worst.1, "m/s"), a::dg(worst.0.to_degrees()),
                      qv(best.1, "m/s"), a::dg(best.0.to_degrees()));
    Some(s.figure("polar", "Ground speed around the compass in a wind", &cap, cv.body(), cv.h))
}

fn m_ring(cv: &mut k::Canvas, c0: (f64, f64), r: f64) {
    m_ring_class(cv, c0, r, "axp-g");
}

fn m_ring_class(cv: &mut k::Canvas, c0: (f64, f64), r: f64, class: &str) {
    cv.raw(&format!("<circle class=\"{class}\" fill=\"none\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"{r:.1}\"/>", c0.0, c0.1));
}
