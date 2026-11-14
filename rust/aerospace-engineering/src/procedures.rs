//! aerospace_engineering: procedures translated from the package's Python by tools/py2rust.py (do not edit; regenerate).
//! Source: models/aerospace_engineering/capabilities/procedures.py. Each keeps Python's number rules through
//! ax_engine::py and is checked against the package by tools/rust_accuracy.py before the build is listed.
#![allow(clippy::all, non_snake_case, unused_parens, unused_braces, unused_mut, unused_variables, unreachable_code)]

use ax_engine::py::{self, Py, PyErr, R};
use ax_engine::Val;
use serde_json::Value;
use std::collections::HashMap;

/// The procedures this crate implements (tools/export_rust_pack.py reads this list).
pub const NATIVE: &[&str] = &[
    "aero_absolute_ceiling_jet",
    "aero_actuator_disk_forward",
    "aero_actuator_disk_hover",
    "aero_altitude_for_period",
    "aero_ballistic_coefficient",
    "aero_ballistic_entry_peak_deceleration",
    "aero_ballistic_entry_velocity",
    "aero_bielliptic_transfer",
    "aero_breguet_endurance_jet",
    "aero_breguet_range_jet",
    "aero_breguet_range_prop",
    "aero_calibrated_airspeed",
    "aero_characteristic_velocity",
    "aero_choked_mass_flow",
    "aero_circular_orbit",
    "aero_critical_mach",
    "aero_delta_v_gravity_loss",
    "aero_density_altitude",
    "aero_drag_polar_optimum",
    "aero_elliptic_wing_lifting_line",
    "aero_escape_speed",
    "aero_expansion_fan",
    "aero_fanno_flow",
    "aero_flight_condition",
    "aero_geopotential_altitude",
    "aero_glide_range",
    "aero_ideal_exhaust_velocity",
    "aero_identical_stages_payload",
    "aero_induced_drag",
    "aero_isa_properties",
    "aero_isentropic_flow",
    "aero_j2_apsidal_rotation",
    "aero_j2_nodal_regression",
    "aero_jet_fuel_for_range",
    "aero_jet_range_constant_altitude",
    "aero_lambert_problem",
    "aero_landing_ground_roll",
    "aero_level_flight_lift_coefficient",
    "aero_level_turn_bank",
    "aero_level_turn_load_factor",
    "aero_lift_drag_forces",
    "aero_mach_from_area_ratio",
    "aero_mach_from_pitot",
    "aero_mach_from_prandtl_meyer",
    "aero_mach_from_pressure_ratio",
    "aero_max_climb_angle_jet",
    "aero_max_deflection_angle",
    "aero_max_rate_of_climb_jet",
    "aero_max_rate_of_climb_prop",
    "aero_min_drag_power_speeds",
    "aero_multistage_delta_v",
    "aero_naca4_thin_airfoil",
    "aero_neutral_point",
    "aero_normal_shock",
    "aero_normal_shock_from_pressure_ratio",
    "aero_nozzle_normal_shock",
    "aero_oblique_shock",
    "aero_oblique_shock_from_wave_angle",
    "aero_optimal_staging",
    "aero_optimum_area_ratio",
    "aero_prandtl_meyer",
    "aero_pressure_altitude",
    "aero_prop_endurance_constant_altitude",
    "aero_propellant_for_delta_v",
    "aero_rayleigh_flow",
    "aero_rocket_delta_v",
    "aero_rocket_engine_performance",
    "aero_rocket_thrust",
    "aero_satellite_coverage",
    "aero_service_ceiling_jet",
    "aero_stall_speed",
    "aero_sun_synchronous_inclination",
    "aero_takeoff_ground_roll",
    "aero_throat_area_for_thrust",
    "aero_thrust_coefficient",
    "aero_time_to_climb",
    "aero_total_temperature",
    "aero_true_airspeed_from_calibrated",
    "aero_vertical_rocket_trajectory",
    "aero_wind_triangle",
    "aero_wing_loading",
];

type Out = Result<(Vec<(String, Value)>, f64), String>;

/// The run's result as the engine's answer: a dict of outputs and the residual of the package's own check; a
/// ValueError is the refusal's reason; any other error is an internal error (None).
fn finish(r: R, residual: impl Fn(&Py, &Py) -> f64, v: &HashMap<String, Val>) -> Option<Out> {
    let o = match r {
        Ok(o) => o,
        Err(PyErr::Value(m)) => return Some(Err(m)),
        Err(e) => {
            if std::env::var_os("AX_DEBUG").is_some() {        // the error behind an internal error, for diagnosis
                eprintln!("internal error: {e:?}");
            }
            return None;
        }
    };
    let Py::Dict(d) = &o else { return None };
    let outs: Vec<(String, Value)> = d.borrow().iter().map(|(k, x)| (py::to_str(k), py::to_json(x))).collect();
    let i = Py::dict(v.iter().map(|(k, x)| (Py::str(k), py::from_val(x))).collect());
    let res = residual(&i, &o);
    if std::env::var_os("AX_DEBUG").is_some() && !(res <= 1e-9) {
        eprintln!("check failed: residual {res}");
    }
    Some(Ok((outs, res)))
}

/// The engine's entry point (ax_engine::pack::NativeFn): None for a key that is not one of these procedures.
pub fn native(key: &str, v: &HashMap<String, Val>) -> Option<Out> {
    match key {
        "aero_absolute_ceiling_jet" => {
            let r = f_absolute_ceiling_jet(match v.get("W") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("CD0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("AR") { Some(x) => py::from_val(x), None => { return None } }, match v.get("e") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_39(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_actuator_disk_forward" => {
            let r = f_actuator_disk_forward(match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rho") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_76(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_actuator_disk_hover" => {
            let r = f_actuator_disk_hover(match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rho") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_113(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_altitude_for_period" => {
            let r = f_altitude_for_period(match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_154(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_ballistic_coefficient" => {
            let r = f_ballistic_coefficient(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("CD") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_193(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_ballistic_entry_peak_deceleration" => {
            let r = f_entry_peak(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("CD") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("VE") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gamma_e") { Some(x) => py::from_val(x), None => { return None } }, match v.get("H") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(7200.0)) })().ok()? } }, match v.get("rho0") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.225)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_237(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_ballistic_entry_velocity" => {
            let r = f_entry_velocity(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("CD") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("VE") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gamma_e") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } }, match v.get("H") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(7200.0)) })().ok()? } }, match v.get("rho0") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.225)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_283(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_bielliptic_transfer" => {
            let r = f_bielliptic(match v.get("h1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("hb") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_324(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_breguet_endurance_jet" => {
            let r = f_breguet_endurance_jet(match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("LD") { Some(x) => py::from_val(x), None => { return None } }, match v.get("W0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("W1") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_371(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_breguet_range_jet" => {
            let r = f_breguet_jet(match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("LD") { Some(x) => py::from_val(x), None => { return None } }, match v.get("W0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("W1") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_418(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_breguet_range_prop" => {
            let r = f_breguet_prop(match v.get("eta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("LD") { Some(x) => py::from_val(x), None => { return None } }, match v.get("W0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("W1") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_469(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_calibrated_airspeed" => {
            let r = f_calibrated_airspeed(match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("hp") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_530(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_characteristic_velocity" => {
            let r = f_characteristic_velocity(match v.get("Tc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Mw") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gamma") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_575(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_choked_mass_flow" => {
            let r = f_choked_mass_flow(match v.get("p0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gamma") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.4)) })().ok()? } }, match v.get("R") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(c_R_AIR()?) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_616(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_circular_orbit" => {
            let r = f_circular_orbit(match v.get("h") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_652(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_critical_mach" => {
            let r = f_critical_mach(match v.get("Cp0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rule") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_719(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_delta_v_gravity_loss" => {
            let r = f_gravity_loss(match v.get("Isp") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mf") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tb") { Some(x) => py::from_val(x), None => { return None } }, match v.get("g") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(c_G0()?) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_765(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_density_altitude" => {
            let r = f_density_altitude(match v.get("hp") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_830(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_drag_polar_optimum" => {
            let r = f_drag_polar_optimum(match v.get("CD0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("AR") { Some(x) => py::from_val(x), None => { return None } }, match v.get("e") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_875(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_elliptic_wing_lifting_line" => {
            let r = f_elliptic_wing(match v.get("AR") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alpha") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a0") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_921(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_escape_speed" => {
            let r = f_escape_speed(match v.get("h") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_958(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_expansion_fan" => {
            let r = f_expansion_fan(match v.get("M1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("theta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gamma") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.4)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1030(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_fanno_flow" => {
            let r = f_fanno(match v.get("M") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gamma") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.4)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1073(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_flight_condition" => {
            let r = f_flight_condition(match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1127(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_geopotential_altitude" => {
            let r = f_geopotential_altitude(match v.get("h") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1163(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_glide_range" => {
            let r = f_glide(match v.get("dh") { Some(x) => py::from_val(x), None => { return None } }, match v.get("LD") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1205(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_ideal_exhaust_velocity" => {
            let r = f_ideal_exhaust_velocity(match v.get("Tc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Mw") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gamma") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pe") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1256(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_identical_stages_payload" => {
            let r = f_identical_stages(match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dv") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Isp") { Some(x) => py::from_val(x), None => { return None } }, match v.get("eps") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1307(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_induced_drag" => {
            let r = f_induced_drag(match v.get("W") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } }, match v.get("AR") { Some(x) => py::from_val(x), None => { return None } }, match v.get("e") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1366(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_isa_properties" => {
            let r = f_isa_properties(match v.get("h") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1412(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_isentropic_flow" => {
            let r = f_isentropic_flow(match v.get("M") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gamma") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.4)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1452(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_j2_apsidal_rotation" => {
            let r = f_apsidal_rotation(match v.get("hp") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ha") { Some(x) => py::from_val(x), None => { return None } }, match v.get("i") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1499(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_j2_nodal_regression" => {
            let r = f_nodal_regression(match v.get("hp") { Some(x) => py::from_val(x), None => { return None } }, match v.get("i") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ha") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1546(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_jet_fuel_for_range" => {
            let r = f_jet_fuel_for_range(match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("LD") { Some(x) => py::from_val(x), None => { return None } }, match v.get("W0") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1593(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_jet_range_constant_altitude" => {
            let r = f_jet_range_constant_altitude(match v.get("W0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("W1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } }, match v.get("CL") { Some(x) => py::from_val(x), None => { return None } }, match v.get("CD") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1660(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_lambert_problem" => {
            let r = f_lambert(match v.get("r1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tof") { Some(x) => py::from_val(x), None => { return None } }, match v.get("direction") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("prograde")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1775(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_landing_ground_roll" => {
            let r = f_landing_roll(match v.get("W") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } }, match v.get("VTD") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("CL") { Some(x) => py::from_val(x), None => { return None } }, match v.get("CD") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Tr") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1851(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_level_flight_lift_coefficient" => {
            let r = f_level_flight_cl(match v.get("W") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1902(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_level_turn_bank" => {
            let r = f_level_turn_bank(match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("phi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("g") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(c_G0()?) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1943(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_level_turn_load_factor" => {
            let r = f_level_turn_load_factor(match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("g") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(c_G0()?) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1984(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_lift_drag_forces" => {
            let r = f_lift_drag_forces(match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("CL") { Some(x) => py::from_val(x), None => { return None } }, match v.get("CD0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("AR") { Some(x) => py::from_val(x), None => { return None } }, match v.get("e") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rho") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("h") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2060(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_mach_from_area_ratio" => {
            let r = f_mach_from_area_ratio(match v.get("ar") { Some(x) => py::from_val(x), None => { return None } }, match v.get("regime") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gamma") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.4)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2138(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_mach_from_pitot" => {
            let r = f_mach_from_pitot(match v.get("p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pt") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gamma") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.4)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2210(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_mach_from_prandtl_meyer" => {
            let r = f_mach_from_prandtl_meyer(match v.get("nu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gamma") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.4)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2279(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_mach_from_pressure_ratio" => {
            let r = f_mach_from_pressure_ratio(match v.get("pr") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gamma") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.4)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2321(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_max_climb_angle_jet" => {
            let r = f_max_climb_angle_jet(match v.get("W") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("CD0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("AR") { Some(x) => py::from_val(x), None => { return None } }, match v.get("e") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2386(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_max_deflection_angle" => {
            let r = f_max_deflection(match v.get("M1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gamma") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.4)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2428(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_max_rate_of_climb_jet" => {
            let r = f_max_rc_jet(match v.get("W") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("CD0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("AR") { Some(x) => py::from_val(x), None => { return None } }, match v.get("e") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2495(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_max_rate_of_climb_prop" => {
            let r = f_max_rc_prop(match v.get("W") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("eta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("CD0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("AR") { Some(x) => py::from_val(x), None => { return None } }, match v.get("e") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2565(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_min_drag_power_speeds" => {
            let r = f_min_drag_power_speeds(match v.get("W") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } }, match v.get("CD0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("AR") { Some(x) => py::from_val(x), None => { return None } }, match v.get("e") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2627(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_multistage_delta_v" => {
            let r = f_multistage_delta_v(match v.get("Isp") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mf") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2676(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_naca4_thin_airfoil" => {
            let r = f_naca4_thin_airfoil(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alpha") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2735(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_neutral_point" => {
            let r = f_neutral_point(match v.get("h_ac") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a_w") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a_t") { Some(x) => py::from_val(x), None => { return None } }, match v.get("St_S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lt_c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("deps") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h_cg") { Some(x) => py::from_val(x), None => { return None } }, match v.get("eta_t") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2775(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_normal_shock" => {
            let r = f_normal_shock(match v.get("M1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gamma") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.4)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2816(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_normal_shock_from_pressure_ratio" => {
            let r = f_normal_shock_from_pressure_ratio(match v.get("p21") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gamma") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.4)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2857(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_nozzle_normal_shock" => {
            let r = f_nozzle_normal_shock(match v.get("pc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pb") { Some(x) => py::from_val(x), None => { return None } }, match v.get("eps") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gamma") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.4)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2938(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_oblique_shock" => {
            let r = f_oblique_shock(match v.get("M1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("theta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("branch") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("weak")) })().ok()? } }, match v.get("gamma") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.4)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3014(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_oblique_shock_from_wave_angle" => {
            let r = f_oblique_from_wave_angle(match v.get("M1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("beta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gamma") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.4)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3061(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_optimal_staging" => {
            let r = f_optimal_staging(match v.get("Isp") { Some(x) => py::from_val(x), None => { return None } }, match v.get("eps") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dv") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m_pl") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3155(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_optimum_area_ratio" => {
            let r = f_optimum_area_ratio(match v.get("gamma") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pa") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3201(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_prandtl_meyer" => {
            let r = f_prandtl_meyer(match v.get("M") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gamma") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.4)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3241(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_pressure_altitude" => {
            let r = f_pressure_altitude(match v.get("p") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3296(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_prop_endurance_constant_altitude" => {
            let r = f_prop_endurance(match v.get("eta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("CL") { Some(x) => py::from_val(x), None => { return None } }, match v.get("CD") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } }, match v.get("W0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("W1") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3367(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_propellant_for_delta_v" => {
            let r = f_propellant_for_delta_v(match v.get("dv") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Isp") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mf") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3410(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_rayleigh_flow" => {
            let r = f_rayleigh(match v.get("M") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gamma") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.4)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3450(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_rocket_delta_v" => {
            let r = f_rocket_delta_v(match v.get("Isp") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mf") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3493(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_rocket_engine_performance" => {
            let r = f_rocket_performance(match v.get("pc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Tc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Mw") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gamma") { Some(x) => py::from_val(x), None => { return None } }, match v.get("At") { Some(x) => py::from_val(x), None => { return None } }, match v.get("eps") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pa") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3582(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_rocket_thrust" => {
            let r = f_rocket_thrust(match v.get("mdot") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ve") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pe") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("pa") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("Ae") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3630(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_satellite_coverage" => {
            let r = f_satellite_coverage(match v.get("h") { Some(x) => py::from_val(x), None => { return None } }, match v.get("eps_min") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3670(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_service_ceiling_jet" => {
            let r = f_service_ceiling_jet(match v.get("W") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("CD0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("AR") { Some(x) => py::from_val(x), None => { return None } }, match v.get("e") { Some(x) => py::from_val(x), None => { return None } }, match v.get("RCs") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.508)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3763(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_stall_speed" => {
            let r = f_stall_speed(match v.get("W") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("CLmax") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3818(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_sun_synchronous_inclination" => {
            let r = f_sun_synchronous(match v.get("h") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3856(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_takeoff_ground_roll" => {
            let r = f_takeoff_roll(match v.get("W") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("CL") { Some(x) => py::from_val(x), None => { return None } }, match v.get("CD") { Some(x) => py::from_val(x), None => { return None } }, match v.get("VLO") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3929(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_throat_area_for_thrust" => {
            let r = f_throat_area_for_thrust(match v.get("F") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Tc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Mw") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gamma") { Some(x) => py::from_val(x), None => { return None } }, match v.get("eps") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pa") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4020(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_thrust_coefficient" => {
            let r = f_thrust_coefficient(match v.get("gamma") { Some(x) => py::from_val(x), None => { return None } }, match v.get("eps") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pa") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4105(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_time_to_climb" => {
            let r = f_time_to_climb(match v.get("RC0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Hc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4148(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_total_temperature" => {
            let r = f_total_temperature(match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4190(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_true_airspeed_from_calibrated" => {
            let r = f_true_airspeed_from_cas(match v.get("CAS") { Some(x) => py::from_val(x), None => { return None } }, match v.get("hp") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4251(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_vertical_rocket_trajectory" => {
            let r = f_vertical_rocket(match v.get("Isp") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mf") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tb") { Some(x) => py::from_val(x), None => { return None } }, match v.get("g") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(c_G0()?) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4299(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_wind_triangle" => {
            let r = f_wind_triangle(match v.get("TAS") { Some(x) => py::from_val(x), None => { return None } }, match v.get("course") { Some(x) => py::from_val(x), None => { return None } }, match v.get("wind_from") { Some(x) => py::from_val(x), None => { return None } }, match v.get("wind_speed") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4349(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "aero_wing_loading" => {
            let r = f_wing_loading(match v.get("W") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4386(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        _ => None,
    }
}

#[allow(non_snake_case)]
fn c_A_SL() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(340.2941077869353)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_DAY() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(86400.0)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_DEG() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(0.017453292519943295)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_FT() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(0.3048)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_G0() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(9.80665)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_GAMMA_AIR() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(1.4)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_H_MAX() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(47000.0)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_H_MIN() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(-1000.0)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_J2_E() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(0.00108263)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_KT() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(0.5144444444444445)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_LBF() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(4.4482216152605)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_MU_E() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(398600441800000.0)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_P_MAX() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(113931.16143967435)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_P_MIN() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(115.85111376529373)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_P_SL() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(101325.0)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_R0_GEO() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(6356766.0)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_RHO_MAX() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(1.3470148166735143)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_RHO_MIN() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(0.0014965203349559914)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_RHO_SL() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(1.2249991558877125)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_RU() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(8.314462618)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_R_AIR() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(287.05307204706463)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_R_E() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(6378136.6)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_SSO_RATE() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(1.9910638534437194e-07)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_T_SL() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(288.15)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__HB() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::list(vec![Py::Float(0.0), Py::Float(11000.0), Py::Float(20000.0), Py::Float(32000.0), Py::Float(47000.0)])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__LB() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::list(vec![Py::Float(-0.0065), Py::Float(0.0), Py::Float(0.001), Py::Float(0.0028), Py::Float(0.0)])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__MU_KM() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(398600.4418)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__PB() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::list(vec![Py::Float(101325.0), Py::Float(22632.06397346291), Py::Float(5474.8886696777745), Py::Float(868.0186847552279), Py::Float(110.90630555496608)])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__TB() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::list(vec![Py::Float(288.15), Py::Float(216.65), Py::Float(216.65), Py::Float(228.65), Py::Float(270.65)])) })());
    V.with(|v| v.clone())
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__H_from_p(mut v_p: Py) -> R {
    let mut v_T: Py = Py::None;
    let mut v_i: Py = Py::None;
    v_i = Py::Int(0i64);
    while ({ let t3276 = { let t3272 = py::add(&v_i.clone(), &Py::Int(1i64))?; Py::Bool({ let t3273 = py::len(&c__HB()?)?; if py::lt(&t3272, &t3273)? { true } else { false } }) }; if !t3276.truthy() { t3276 } else { { let t3274 = v_p.clone(); Py::Bool({ let t3275 = py::getitem(&c__PB()?, &py::add(&v_i.clone(), &Py::Int(1i64))?)?; if py::lt(&t3274, &t3275)? { true } else { false } }) } } }).truthy() {
        v_i = py::iadd(&v_i, &Py::Int(1i64))?;
    }
    if ({ let t3277 = py::getitem(&c__LB()?, &v_i.clone())?; Py::Bool({ let t3278 = Py::Float(0.0); if py::eq(&t3277, &t3278) { true } else { false } }) }).truthy() {
        return Ok(py::add(&py::getitem(&c__HB()?, &v_i.clone())?, &py::mul(&py::truediv(&py::mul(&c_R_AIR()?, &py::getitem(&c__TB()?, &v_i.clone())?)?, &c_G0()?)?, &py::math1("log", &py::truediv(&py::getitem(&c__PB()?, &v_i.clone())?, &v_p.clone())?)?)?)?);
    }
    v_T = py::mul(&py::getitem(&c__TB()?, &v_i.clone())?, &py::pow(&py::truediv(&v_p.clone(), &py::getitem(&c__PB()?, &v_i.clone())?)?, &py::truediv(&py::mul(&py::neg(&c_R_AIR()?)?, &py::getitem(&c__LB()?, &v_i.clone())?)?, &c_G0()?)?)?)?;
    return Ok(py::add(&py::getitem(&c__HB()?, &v_i.clone())?, &py::truediv(&py::sub(&v_T.clone(), &py::getitem(&c__TB()?, &v_i.clone())?)?, &py::getitem(&c__LB()?, &v_i.clone())?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__H_from_rho(mut v_rho: Py) -> R {
    let mut v_T: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_rb: Py = Py::None;
    v_rb = { let mut out = vec![]; for t813 in py::range_iter(&[(py::len(&c__HB()?)?).i()?]) { let mut v_i: Py; 
v_i = t813;
out.push(py::truediv(&py::getitem(&c__PB()?, &v_i.clone())?, &py::mul(&c_R_AIR()?, &py::getitem(&c__TB()?, &v_i.clone())?)?)?); } Py::list(out) };
    v_i = Py::Int(0i64);
    while ({ let t818 = { let t814 = py::add(&v_i.clone(), &Py::Int(1i64))?; Py::Bool({ let t815 = py::len(&c__HB()?)?; if py::lt(&t814, &t815)? { true } else { false } }) }; if !t818.truthy() { t818 } else { { let t816 = v_rho.clone(); Py::Bool({ let t817 = py::getitem(&v_rb.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?; if py::lt(&t816, &t817)? { true } else { false } }) } } }).truthy() {
        v_i = py::iadd(&v_i, &Py::Int(1i64))?;
    }
    if ({ let t819 = py::getitem(&c__LB()?, &v_i.clone())?; Py::Bool({ let t820 = Py::Float(0.0); if py::eq(&t819, &t820) { true } else { false } }) }).truthy() {
        return Ok(py::add(&py::getitem(&c__HB()?, &v_i.clone())?, &py::mul(&py::truediv(&py::mul(&c_R_AIR()?, &py::getitem(&c__TB()?, &v_i.clone())?)?, &c_G0()?)?, &py::math1("log", &py::truediv(&py::getitem(&v_rb.clone(), &v_i.clone())?, &v_rho.clone())?)?)?)?);
    }
    v_n = py::truediv(&c_G0()?, &py::mul(&c_R_AIR()?, &py::getitem(&c__LB()?, &v_i.clone())?)?)?;
    v_T = py::mul(&py::getitem(&c__TB()?, &v_i.clone())?, &py::pow(&py::truediv(&v_rho.clone(), &py::getitem(&v_rb.clone(), &v_i.clone())?)?, &py::truediv(&py::neg(&Py::Float(1.0))?, &py::add(&v_n.clone(), &Py::Float(1.0))?)?)?)?;
    return Ok(py::add(&py::getitem(&c__HB()?, &v_i.clone())?, &py::truediv(&py::sub(&v_T.clone(), &py::getitem(&c__TB()?, &v_i.clone())?)?, &py::getitem(&c__LB()?, &v_i.clone())?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__J(mut v_n: Py, mut v_a: Py, mut v_b: Py) -> R {
    if ({ let t2716 = v_n.clone(); Py::Bool({ let t2717 = Py::Int(0i64); if py::eq(&t2716, &t2717) { true } else { false } }) }).truthy() {
        return Ok(py::sub(&v_b.clone(), &v_a.clone())?);
    }
    return Ok(py::truediv(&py::sub(&py::math1("sin", &py::mul(&v_n.clone(), &v_b.clone())?)?, &py::math1("sin", &py::mul(&v_n.clone(), &v_a.clone())?)?)?, &v_n.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__K(mut v_AR: Py, mut v_e: Py) -> R {
    let _ = f__need({ let t3705 = Py::Bool(matches!(v_AR.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t3705.truthy() { t3705 } else { { let t3702 = Py::Float(1.0); Py::Bool({ let t3703 = v_AR.clone(); if py::le(&t3702, &t3703)? { { let t3704 = Py::Float(50.0); if py::le(&t3703, &t3704)? { true } else { false } } } else { false } }) } } }, Py::str("the aspect ratio must lie between 1 and 50"))?;
    let _ = f__need({ let t3709 = Py::Bool(matches!(v_e.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t3709.truthy() { t3709 } else { { let t3706 = Py::Float(0.3); Py::Bool({ let t3707 = v_e.clone(); if py::le(&t3706, &t3707)? { { let t3708 = Py::Float(1.0); if py::le(&t3707, &t3708)? { true } else { false } } } else { false } }) } } }, Py::str("the Oswald efficiency factor must lie between 0.3 and 1"))?;
    return Ok(py::truediv(&Py::Float(1.0), &py::mul(&py::mul(&Py::Float(std::f64::consts::PI), &v_e.clone())?, &v_AR.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__M_of_area(mut v_ar: Py, mut v_g: Py, mut v_supersonic: Py) -> R {
    let mut v_hi: Py = Py::None;
    let mut v_lo: Py = Py::None;
    if ({ let t4060 = v_ar.clone(); Py::Bool({ let t4061 = Py::Float(1.0); if py::eq(&t4060, &t4061) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(1.0));
    }
    if (v_supersonic.clone()).truthy() {
        v_hi = Py::Float(2.0);
        while ({ let t4062 = f__area(v_hi.clone(), v_g.clone())?; Py::Bool({ let t4063 = v_ar.clone(); if py::lt(&t4062, &t4063)? { true } else { false } }) }).truthy() {
            v_hi = py::mul(&v_hi, &Py::Float(2.0))?;
        }
        return Ok(f__bisect({ let v_ar = v_ar.clone(); let v_g = v_g.clone(); let v_hi = v_hi.clone(); let v_supersonic = v_supersonic.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_M: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&f__area(v_M.clone(), v_g.clone())?, &v_ar.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, Py::Float(1.0), v_hi.clone(), Py::Int(200i64))?);
    }
    v_lo = Py::Float(0.5);
    while ({ let t4089 = f__area(v_lo.clone(), v_g.clone())?; Py::Bool({ let t4090 = v_ar.clone(); if py::lt(&t4089, &t4090)? { true } else { false } }) }).truthy() {
        v_lo = py::mul(&v_lo, &Py::Float(0.5))?;
    }
    return Ok(f__bisect({ let v_ar = v_ar.clone(); let v_g = v_g.clone(); let v_hi = v_hi.clone(); let v_lo = v_lo.clone(); let v_supersonic = v_supersonic.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_M: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&f__area(v_M.clone(), v_g.clone())?, &v_ar.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, v_lo.clone(), Py::Float(1.0), Py::Int(200i64))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__M_of_nu(mut v_nu: Py, mut v_g: Py) -> R {
    let mut v_hi: Py = Py::None;
    v_hi = Py::Float(2.0);
    while ({ let t2252 = f__nu(v_hi.clone(), v_g.clone())?; Py::Bool({ let t2253 = v_nu.clone(); if py::lt(&t2252, &t2253)? { true } else { false } }) }).truthy() {
        v_hi = py::mul(&v_hi, &Py::Float(2.0))?;
    }
    return Ok(f__bisect({ let v_g = v_g.clone(); let v_hi = v_hi.clone(); let v_nu = v_nu.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_M: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&f__nu(v_M.clone(), v_g.clone())?, &v_nu.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, Py::Float(1.0), v_hi.clone(), Py::Int(200i64))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__area(mut v_M: Py, mut v_g: Py) -> R {
    return Ok(py::truediv(&py::pow(&py::mul(&py::truediv(&Py::Float(2.0), &py::add(&v_g.clone(), &Py::Float(1.0))?)?, &f__t0t(v_M.clone(), v_g.clone())?)?, &py::truediv(&py::add(&v_g.clone(), &Py::Float(1.0))?, &py::mul(&Py::Float(2.0), &py::sub(&v_g.clone(), &Py::Float(1.0))?)?)?)?, &v_M.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__beta_max(mut v_M1: Py, mut v_g: Py) -> R {
    let mut v_m2: Py = Py::None;
    let mut v_s2: Py = Py::None;
    v_m2 = py::mul(&v_M1.clone(), &v_M1.clone())?;
    v_s2 = py::truediv(&py::add(&py::sub(&py::truediv(&py::mul(&py::add(&v_g.clone(), &Py::Float(1.0))?, &v_m2.clone())?, &Py::Float(4.0))?, &Py::Float(1.0))?, &py::math1("sqrt", &py::mul(&py::add(&v_g.clone(), &Py::Float(1.0))?, &py::add(&py::add(&Py::Float(1.0), &py::mul(&py::mul(&Py::Float(0.5), &py::sub(&v_g.clone(), &Py::Float(1.0))?)?, &v_m2.clone())?)?, &py::truediv(&py::mul(&py::mul(&py::add(&v_g.clone(), &Py::Float(1.0))?, &v_m2.clone())?, &v_m2.clone())?, &Py::Float(16.0))?)?)?)?)?, &py::mul(&v_g.clone(), &v_m2.clone())?)?;
    return Ok(py::math1("asin", &py::math1("sqrt", &v_s2.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__bisect(mut v_f: Py, mut v_a: Py, mut v_b: Py, mut v_n: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_fa: Py = Py::None;
    let mut v_fb: Py = Py::None;
    let mut v_fm: Py = Py::None;
    let mut v_m: Py = Py::None;
    v_fa = py::call(&v_f.clone(), &[v_a.clone()])?;
    v_fb = py::call(&v_f.clone(), &[v_b.clone()])?;
    if ({ let t4064 = v_fa.clone(); Py::Bool({ let t4065 = Py::Int(0i64); if py::eq(&t4064, &t4065) { true } else { false } }) }).truthy() {
        return Ok(v_a.clone());
    }
    if ({ let t4066 = v_fb.clone(); Py::Bool({ let t4067 = Py::Int(0i64); if py::eq(&t4066, &t4067) { true } else { false } }) }).truthy() {
        return Ok(v_b.clone());
    }
    if ({ let t4068 = { let t4070 = v_fa.clone(); Py::Bool({ let t4071 = Py::Int(0i64); if py::gt(&t4070, &t4071)? { true } else { false } }) }; Py::Bool({ let t4069 = { let t4072 = v_fb.clone(); Py::Bool({ let t4073 = Py::Int(0i64); if py::gt(&t4072, &t4073)? { true } else { false } }) }; if py::eq(&t4068, &t4069) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("no root in the interval"))));
    }
    for t4074 in py::range_iter(&[(v_n.clone()).i()?]) {
        v__ = t4074;
        v_m = py::mul(&Py::Float(0.5), &py::add(&v_a.clone(), &v_b.clone())?)?;
        if ({ let t4079 = { let t4075 = v_m.clone(); Py::Bool({ let t4076 = v_a.clone(); if py::eq(&t4075, &t4076) { true } else { false } }) }; if t4079.truthy() { t4079 } else { { let t4077 = v_m.clone(); Py::Bool({ let t4078 = v_b.clone(); if py::eq(&t4077, &t4078) { true } else { false } }) } } }).truthy() {
            break;
        }
        v_fm = py::call(&v_f.clone(), &[v_m.clone()])?;
        if ({ let t4080 = v_fm.clone(); Py::Bool({ let t4081 = Py::Int(0i64); if py::eq(&t4080, &t4081) { true } else { false } }) }).truthy() {
            return Ok(v_m.clone());
        }
        if ({ let t4082 = { let t4084 = v_fm.clone(); Py::Bool({ let t4085 = Py::Int(0i64); if py::gt(&t4084, &t4085)? { true } else { false } }) }; Py::Bool({ let t4083 = { let t4086 = v_fa.clone(); Py::Bool({ let t4087 = Py::Int(0i64); if py::gt(&t4086, &t4087)? { true } else { false } }) }; if py::eq(&t4082, &t4083) { true } else { false } }) }).truthy() {
            let t4088 = (Py::tuple(vec![v_m.clone(), v_fm.clone()])).items()?;
            if t4088.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_a = t4088[0].clone();
            v_fa = t4088[1].clone();
        } else {
            v_b = v_m.clone();
        }
    }
    return Ok(py::mul(&Py::Float(0.5), &py::add(&v_a.clone(), &v_b.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__bsfc_in(mut v_c: Py) -> R {
    let _ = f__need({ let t3326 = Py::Bool(matches!(v_c.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t3326.truthy() { t3326 } else { { let t3323 = Py::Float(1e-09); Py::Bool({ let t3324 = v_c.clone(); if py::le(&t3323, &t3324)? { { let t3325 = Py::Float(1e-05); if py::le(&t3324, &t3325)? { true } else { false } } } else { false } }) } } }, Py::str("the power-specific fuel consumption must lie between 1e-9 and 1e-5 kg/J"))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__camber_int(mut v_n: Py, mut v_m: Py, mut v_p: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c0: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_total: Py = Py::None;
    let mut v_tp: Py = Py::None;
    if ({ let t2713 = v_m.clone(); Py::Bool({ let t2714 = Py::Int(0i64); if py::eq(&t2713, &t2714) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    v_tp = py::math1("acos", &py::sub(&Py::Int(1i64), &py::mul(&Py::Int(2i64), &v_p.clone())?)?)?;
    v_total = Py::Float(0.0);
    for t2715 in py::iter(&(Py::tuple(vec![Py::tuple(vec![py::truediv(&py::mul(&Py::Int(2i64), &v_m.clone())?, &py::pow(&v_p.clone(), &Py::Int(2i64))?)?, Py::Float(0.0), v_tp.clone()]), Py::tuple(vec![py::truediv(&py::mul(&Py::Int(2i64), &v_m.clone())?, &py::pow(&py::sub(&Py::Int(1i64), &v_p.clone())?, &Py::Int(2i64))?)?, v_tp.clone(), Py::Float(std::f64::consts::PI)])])))? {
        let t2718 = (t2715).items()?;
        if t2718.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2718[0].clone();
        v_a = t2718[1].clone();
        v_b = t2718[2].clone();
        v_c0 = py::sub(&v_p.clone(), &Py::Float(0.5))?;
        v_total = py::iadd(&v_total, &py::mul(&v_k.clone(), &py::add(&py::mul(&v_c0.clone(), &f__J(v_n.clone(), v_a.clone(), v_b.clone())?)?, &py::mul(&Py::Float(0.25), &py::add(&f__J(py::abs(&py::sub(&v_n.clone(), &Py::Int(1i64))?)?, v_a.clone(), v_b.clone())?, &f__J(py::add(&v_n.clone(), &Py::Int(1i64))?, v_a.clone(), v_b.clone())?)?)?)?)?)?;
    }
    return Ok(v_total.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__cd0(mut v_CD0: Py) -> R {
    let _ = f__need({ let t3701 = Py::Bool(matches!(v_CD0.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t3701.truthy() { t3701 } else { { let t3698 = Py::Float(0.002); Py::Bool({ let t3699 = v_CD0.clone(); if py::le(&t3698, &t3699)? { { let t3700 = Py::Float(0.2); if py::le(&t3699, &t3700)? { true } else { false } } } else { false } }) } } }, Py::str("the zero-lift drag coefficient must lie between 0.002 and 0.2"))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__clcd_in(mut v_CL: Py, mut v_CD: Py) -> R {
    let _ = f__need({ let t3330 = Py::Bool(matches!(v_CL.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t3330.truthy() { t3330 } else { { let t3327 = Py::Float(0.05); Py::Bool({ let t3328 = v_CL.clone(); if py::le(&t3327, &t3328)? { { let t3329 = Py::Float(3.0); if py::le(&t3328, &t3329)? { true } else { false } } } else { false } }) } } }, Py::str("the lift coefficient must lie between 0.05 and 3"))?;
    let _ = f__need({ let t3337 = Py::Bool(matches!(v_CD.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t3337.truthy() { t3337 } else { { let t3336 = { let t3331 = Py::Float(0.003); Py::Bool({ let t3332 = v_CD.clone(); if py::le(&t3331, &t3332)? { { let t3333 = Py::Float(0.5); if py::le(&t3332, &t3333)? { true } else { false } } } else { false } }) }; if !t3336.truthy() { t3336 } else { { let t3334 = v_CD.clone(); Py::Bool({ let t3335 = v_CL.clone(); if py::lt(&t3334, &t3335)? { true } else { false } }) } } } } }, Py::str("the drag coefficient must lie between 0.003 and 0.5 and below CL"))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__disk_in(mut v_T: Py, mut v_D: Py, mut v_rho: Py) -> R {
    let _ = f__pos(Py::str("the thrust, the disk diameter and the air density"), Py::tuple(vec![v_T.clone(), v_D.clone(), v_rho.clone()]))?;
    return Ok(py::truediv(&py::mul(&py::mul(&Py::Float(std::f64::consts::PI), &v_D.clone())?, &v_D.clone())?, &Py::Int(4i64))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__entry_in(mut v_m: Py, mut v_CD: Py, mut v_A: Py, mut v_VE: Py, mut v_gam: Py, mut v_H: Py, mut v_rho0: Py) -> R {
    let _ = f__pos(Py::str("the mass, drag coefficient, area, entry speed, scale height and surface density"), Py::tuple(vec![v_m.clone(), v_CD.clone(), v_A.clone(), v_VE.clone(), v_H.clone(), v_rho0.clone()]))?;
    let _ = f__need({ let t265 = Py::Int(0i64); Py::Bool({ let t266 = v_gam.clone(); if py::lt(&t265, &t266)? { { let t267 = py::truediv(&Py::Float(std::f64::consts::PI), &Py::Int(2i64))?; if py::le(&t266, &t267)? { true } else { false } } } else { false } }) }, Py::str("the entry flight-path angle (below the horizontal) must lie between 0 and 90 degrees"))?;
    let _ = f__need({ let t268 = v_gam.clone(); Py::Bool({ let t269 = py::mul(&Py::Int(2i64), &c_DEG()?)?; if py::ge(&t268, &t269)? { true } else { false } }) }, Py::str("the straight-line ballistic (Allen-Eggers) model needs an entry angle of at least 2 degrees"))?;
    return Ok(py::truediv(&v_m.clone(), &py::mul(&v_CD.clone(), &v_A.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__eta_in(mut v_eta: Py) -> R {
    let _ = f__need({ let t3322 = Py::Bool(matches!(v_eta.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t3322.truthy() { t3322 } else { { let t3319 = Py::Float(0.1); Py::Bool({ let t3320 = v_eta.clone(); if py::le(&t3319, &t3320)? { { let t3321 = Py::Float(1.0); if py::le(&t3320, &t3321)? { true } else { false } } } else { false } }) } } }, Py::str("the propeller efficiency must lie between 0.1 and 1"))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__gam(mut v_gamma: Py) -> R {
    let _ = f__need({ let t4046 = Py::Bool(matches!(v_gamma.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t4046.truthy() { t4046 } else { { let t4043 = Py::Float(1.05); Py::Bool({ let t4044 = v_gamma.clone(); if py::le(&t4043, &t4044)? { { let t4045 = Py::Float(1.7); if py::le(&t4044, &t4045)? { true } else { false } } } else { false } }) } } }, Py::str("the ratio of specific heats gamma must lie between 1.05 and 1.7"))?;
    return Ok(py::float(&v_gamma.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__geom(mut v_H: Py) -> R {
    return Ok(py::truediv(&py::mul(&c_R0_GEO()?, &v_H.clone())?, &py::sub(&c_R0_GEO()?, &v_H.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__geopot(mut v_h: Py) -> R {
    return Ok(py::truediv(&py::mul(&c_R0_GEO()?, &v_h.clone())?, &py::add(&c_R0_GEO()?, &v_h.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__isa(mut v_h: Py) -> R {
    let mut v_T: Py = Py::None;
    let mut v_p: Py = Py::None;
    let _ = f__need({ let t4223 = Py::Bool(matches!(v_h.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t4223.truthy() { t4223 } else { { let t4220 = c_H_MIN()?; Py::Bool({ let t4221 = v_h.clone(); if py::le(&t4220, &t4221)? { { let t4222 = c_H_MAX()?; if py::le(&t4221, &t4222)? { true } else { false } } } else { false } }) } } }, Py::str("the altitude must lie between -1 km and 47 km (standard atmosphere layers to 47 km)"))?;
    let t4232 = (f__isa_H(f__geopot(v_h.clone())?)?).items()?;
    if t4232.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_T = t4232[0].clone();
    v_p = t4232[1].clone();
    return Ok(Py::tuple(vec![v_T.clone(), v_p.clone(), py::truediv(&v_p.clone(), &py::mul(&c_R_AIR()?, &v_T.clone())?)?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__isa_H(mut v_H: Py) -> R {
    let mut v_T: Py = Py::None;
    let mut v_i: Py = Py::None;
    v_i = f__layer(v_H.clone())?;
    v_T = py::add(&py::getitem(&c__TB()?, &v_i.clone())?, &py::mul(&py::getitem(&c__LB()?, &v_i.clone())?, &py::sub(&v_H.clone(), &py::getitem(&c__HB()?, &v_i.clone())?)?)?)?;
    return Ok(Py::tuple(vec![v_T.clone(), f__layer_p(py::getitem(&c__PB()?, &v_i.clone())?, py::getitem(&c__TB()?, &v_i.clone())?, py::getitem(&c__LB()?, &v_i.clone())?, py::sub(&v_H.clone(), &py::getitem(&c__HB()?, &v_i.clone())?)?)?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__isp_in(mut v_Isp: Py) -> R {
    let _ = f__need({ let t4277 = Py::Bool(matches!(v_Isp.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t4277.truthy() { t4277 } else { { let t4274 = Py::Float(1.0); Py::Bool({ let t4275 = v_Isp.clone(); if py::le(&t4274, &t4275)? { { let t4276 = Py::Float(10000.0); if py::le(&t4275, &t4276)? { true } else { false } } } else { false } }) } } }, Py::str("the specific impulse must lie between 1 s and 10000 s"))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__j2_elements(mut v_hp: Py, mut v_ha: Py, mut v_i: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_ra: Py = Py::None;
    let mut v_rp: Py = Py::None;
    let _ = f__orb_alt(v_hp.clone(), Py::Float(100000.0), Py::Float(100000000.0))?;
    v_ha = (if ({ let t1526 = v_ha.clone(); Py::Bool({ let t1527 = Py::None; if py::is(&t1526, &t1527) { true } else { false } }) }).truthy() { v_hp.clone() } else { v_ha.clone() });
    let _ = f__orb_alt(v_ha.clone(), Py::Float(100000.0), Py::Float(100000000.0))?;
    let _ = f__need({ let t1528 = v_ha.clone(); Py::Bool({ let t1529 = v_hp.clone(); if py::ge(&t1528, &t1529)? { true } else { false } }) }, Py::str("the apogee altitude must not be below the perigee altitude"))?;
    let _ = f__need({ let t1530 = c_DEG()?; Py::Bool({ let t1531 = v_i.clone(); if py::le(&t1530, &t1531)? { { let t1532 = py::mul(&Py::Int(179i64), &c_DEG()?)?; if py::le(&t1531, &t1532)? { true } else { false } } } else { false } }) }, Py::str("the inclination must lie between 1 and 179 degrees (the node and perigee are undefined for an equatorial orbit)"))?;
    let t1533 = (Py::tuple(vec![py::add(&c_R_E()?, &v_hp.clone())?, py::add(&c_R_E()?, &v_ha.clone())?])).items()?;
    if t1533.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_rp = t1533[0].clone();
    v_ra = t1533[1].clone();
    v_a = py::truediv(&py::add(&v_rp.clone(), &v_ra.clone())?, &Py::Int(2i64))?;
    v_e = py::truediv(&py::sub(&v_ra.clone(), &v_rp.clone())?, &py::add(&v_ra.clone(), &v_rp.clone())?)?;
    v_p = py::mul(&v_a.clone(), &py::sub(&Py::Int(1i64), &py::mul(&v_e.clone(), &v_e.clone())?)?)?;
    v_n = py::math1("sqrt", &py::truediv(&c_MU_E()?, &py::pow(&v_a.clone(), &Py::Int(3i64))?)?)?;
    return Ok(Py::tuple(vec![v_a.clone(), v_e.clone(), v_p.clone(), v_n.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__layer(mut v_H: Py) -> R {
    let mut v_hb: Py = Py::None;
    let mut v_i: Py = Py::None;
    if ({ let t4224 = v_H.clone(); Py::Bool({ let t4225 = Py::Float(0.0); if py::le(&t4224, &t4225)? { true } else { false } }) }).truthy() {
        return Ok(Py::Int(0i64));
    }
    for t4226 in py::iter(&(py::enumerate(&c__HB()?, 0)?))? {
        let t4229 = (t4226).items()?;
        if t4229.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_i = t4229[0].clone();
        v_hb = t4229[1].clone();
        if ({ let t4227 = v_hb.clone(); Py::Bool({ let t4228 = v_H.clone(); if py::ge(&t4227, &t4228)? { true } else { false } }) }).truthy() {
            return Ok(py::sub(&v_i.clone(), &Py::Int(1i64))?);
        }
    }
    return Ok(py::sub(&py::len(&c__HB()?)?, &Py::Int(1i64))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__layer_p(mut v_pb: Py, mut v_tb: Py, mut v_lr: Py, mut v_dh: Py) -> R {
    if ({ let t4230 = v_lr.clone(); Py::Bool({ let t4231 = Py::Float(0.0); if py::eq(&t4230, &t4231) { true } else { false } }) }).truthy() {
        return Ok(py::mul(&v_pb.clone(), &py::math1("exp", &py::truediv(&py::mul(&py::neg(&c_G0()?)?, &v_dh.clone())?, &py::mul(&c_R_AIR()?, &v_tb.clone())?)?)?)?);
    }
    return Ok(py::mul(&v_pb.clone(), &py::pow(&py::truediv(&v_tb.clone(), &py::add(&v_tb.clone(), &py::mul(&v_lr.clone(), &v_dh.clone())?)?)?, &py::truediv(&c_G0()?, &py::mul(&c_R_AIR()?, &v_lr.clone())?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__ld_in(mut v_LD: Py) -> R {
    let _ = f__need({ let t1581 = Py::Bool(matches!(v_LD.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t1581.truthy() { t1581 } else { { let t1578 = Py::Float(1.0); Py::Bool({ let t1579 = v_LD.clone(); if py::le(&t1578, &t1579)? { { let t1580 = Py::Float(60.0); if py::le(&t1579, &t1580)? { true } else { false } } } else { false } }) } } }, Py::str("the lift-to-drag ratio must lie between 1 and 60"))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__mach_in(mut v_M: Py, mut v_lo: Py, mut v_hi: Py) -> R {
    let _ = f__need({ let t1442 = Py::Bool(matches!(v_M.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t1442.truthy() { t1442 } else { { let t1439 = v_lo.clone(); Py::Bool({ let t1440 = v_M.clone(); if py::le(&t1439, &t1440)? { { let t1441 = v_hi.clone(); if py::le(&t1440, &t1441)? { true } else { false } } } else { false } }) } } }, Py::str(&["the Mach number must lie between ".to_string(), py::format(&v_lo.clone(), "g")?, " and ".to_string(), py::format(&v_hi.clone(), "g")?].concat()))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__need(mut v_ok: Py, mut v_msg: Py) -> R {
    if (py::not(&v_ok.clone())).truthy() {
        return Err(py::value_error(py::to_str(&v_msg.clone())));
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__nu(mut v_M: Py, mut v_g: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_s: Py = Py::None;
    v_a = py::math1("sqrt", &py::truediv(&py::add(&v_g.clone(), &Py::Float(1.0))?, &py::sub(&v_g.clone(), &Py::Float(1.0))?)?)?;
    v_s = py::math1("sqrt", &py::sub(&py::mul(&v_M.clone(), &v_M.clone())?, &Py::Float(1.0))?)?;
    return Ok(py::sub(&py::mul(&v_a.clone(), &py::math1("atan", &py::truediv(&v_s.clone(), &v_a.clone())?)?)?, &py::math1("atan", &v_s.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__oblique_from_beta(mut v_M1: Py, mut v_b: Py, mut v_g: Py) -> R {
    let mut v_M2: Py = Py::None;
    let mut v_Mn1: Py = Py::None;
    let mut v_Mn2: Py = Py::None;
    let mut v_p0r: Py = Py::None;
    let mut v_p21: Py = Py::None;
    let mut v_r21: Py = Py::None;
    let mut v_th: Py = Py::None;
    v_Mn1 = py::mul(&v_M1.clone(), &py::math1("sin", &v_b.clone())?)?;
    v_th = f__theta_of_beta(v_M1.clone(), v_b.clone(), v_g.clone())?;
    let t3048 = (f__shock(v_Mn1.clone(), v_g.clone())?).items()?;
    if t3048.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_Mn2 = t3048[0].clone();
    v_p21 = t3048[1].clone();
    v_r21 = t3048[2].clone();
    v_p0r = t3048[3].clone();
    v_M2 = py::truediv(&v_Mn2.clone(), &py::math1("sin", &py::sub(&v_b.clone(), &v_th.clone())?)?)?;
    return Ok(Py::tuple(vec![v_th.clone(), v_Mn1.clone(), v_M2.clone(), v_p21.clone(), v_r21.clone(), v_p0r.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__orb_alt(mut v_h: Py, mut v_lo: Py, mut v_hi: Py) -> R {
    let _ = f__need({ let t3844 = Py::Bool(matches!(v_h.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t3844.truthy() { t3844 } else { { let t3841 = v_lo.clone(); Py::Bool({ let t3842 = v_h.clone(); if py::le(&t3841, &t3842)? { { let t3843 = v_hi.clone(); if py::le(&t3842, &t3843)? { true } else { false } } } else { false } }) } } }, Py::str(&["the altitude must lie between ".to_string(), py::format(&py::truediv(&v_lo.clone(), &Py::Float(1000.0))?, "g")?, " km and ".to_string(), py::format(&py::truediv(&v_hi.clone(), &Py::Float(1000.0))?, "g")?, " km".to_string()].concat()))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__out(mut v_kw: Py) -> R {
    return Ok({ let mut out = vec![]; for t4377 in py::iter(&(py::method(&v_kw.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t4378 = (t4377).items()?;
if t4378.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t4378[0].clone();
v_v = t4378[1].clone();
out.push((v_k.clone(), f__r(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__p0p(mut v_M: Py, mut v_g: Py) -> R {
    return Ok(py::pow(&f__t0t(v_M.clone(), v_g.clone())?, &py::truediv(&v_g.clone(), &py::sub(&v_g.clone(), &Py::Float(1.0))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pitot_ratio_super(mut v_M: Py, mut v_g: Py) -> R {
    let mut v_m2: Py = Py::None;
    v_m2 = py::mul(&v_M.clone(), &v_M.clone())?;
    return Ok(py::truediv(&py::mul(&py::pow(&py::truediv(&py::mul(&py::pow(&py::add(&v_g.clone(), &Py::Float(1.0))?, &Py::Int(2i64))?, &v_m2.clone())?, &py::sub(&py::mul(&py::mul(&Py::Float(4.0), &v_g.clone())?, &v_m2.clone())?, &py::mul(&Py::Float(2.0), &py::sub(&v_g.clone(), &Py::Float(1.0))?)?)?)?, &py::truediv(&v_g.clone(), &py::sub(&v_g.clone(), &Py::Float(1.0))?)?)?, &py::add(&py::sub(&Py::Float(1.0), &v_g.clone())?, &py::mul(&py::mul(&Py::Float(2.0), &v_g.clone())?, &v_m2.clone())?)?)?, &py::add(&v_g.clone(), &Py::Float(1.0))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pos(mut v_name: Py, mut v_vals: Py) -> R {
    let mut v_v: Py = Py::None;
    for t4372 in py::iter(&(v_vals.clone()))? {
        v_v = t4372;
        if (py::not(&{ let t4376 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t4376.truthy() { t4376 } else { { let t4375 = py::math1("isfinite", &v_v.clone())?; if !t4375.truthy() { t4375 } else { { let t4373 = v_v.clone(); Py::Bool({ let t4374 = Py::Int(0i64); if py::gt(&t4373, &t4374)? { true } else { false } }) } } } } })).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must be a positive number".to_string()].concat()))));
        }
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__r(mut v_x: Py) -> R {
    if ({ let t4379 = Py::Bool(matches!(v_x.clone(), Py::Bool(_))); if t4379.truthy() { t4379 } else { Py::Bool(matches!(v_x.clone(), Py::Str(_))) } }).truthy() {
        return Ok(v_x.clone());
    }
    if (Py::Bool(matches!(v_x.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_)))).truthy() {
        return Ok(v_x.clone());
    }
    if (Py::Bool(matches!(v_x.clone(), Py::List(_) | Py::Tuple(_)))).truthy() {
        return Ok({ let mut out = vec![]; for t4380 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t4380;
out.push(f__r(v_v.clone())?); } Py::list(out) });
    }
    if ({ let t4383 = { let t4381 = v_x.clone(); Py::Bool({ let t4382 = Py::Int(0i64); if py::eq(&t4381, &t4382) { true } else { false } }) }; if t4383.truthy() { t4383 } else { py::not(&py::math1("isfinite", &v_x.clone())?) } }).truthy() {
        return Ok((if ({ let t4384 = v_x.clone(); Py::Bool({ let t4385 = Py::Int(0i64); if py::eq(&t4384, &t4385) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { v_x.clone() }));
    }
    return Ok(py::add(&py::float(&Py::str(&[py::format(&v_x.clone(), ".12g")?].concat()))?, &Py::Float(0.0))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__rc_jet(mut v_W: Py, mut v_S: Py, mut v_rho: Py, mut v_T: Py, mut v_CD0: Py, mut v_K: Py) -> R {
    let mut v_V: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_rc: Py = Py::None;
    v_a = py::mul(&py::mul(&py::mul(&Py::Float(0.5), &v_rho.clone())?, &v_S.clone())?, &v_CD0.clone())?;
    v_b = py::truediv(&py::mul(&py::mul(&py::mul(&Py::Int(2i64), &v_K.clone())?, &v_W.clone())?, &v_W.clone())?, &py::mul(&v_rho.clone(), &v_S.clone())?)?;
    v_V = py::math1("sqrt", &py::truediv(&py::add(&v_T.clone(), &py::math1("sqrt", &py::add(&py::mul(&v_T.clone(), &v_T.clone())?, &py::mul(&py::mul(&Py::Int(12i64), &v_a.clone())?, &v_b.clone())?)?)?)?, &py::mul(&Py::Int(6i64), &v_a.clone())?)?)?;
    v_rc = py::truediv(&py::mul(&v_V.clone(), &py::sub(&py::sub(&v_T.clone(), &py::mul(&py::mul(&v_a.clone(), &v_V.clone())?, &v_V.clone())?)?, &py::truediv(&v_b.clone(), &py::mul(&v_V.clone(), &v_V.clone())?)?)?)?, &v_W.clone())?;
    return Ok(Py::tuple(vec![v_V.clone(), v_rc.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__rel(mut v_a: Py, mut v_b: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_s: Py = Py::None;
    if ({ let t4392 = Py::Bool(matches!(v_a.clone(), Py::List(_) | Py::Tuple(_))); if t4392.truthy() { t4392 } else { Py::Bool(matches!(v_b.clone(), Py::List(_) | Py::Tuple(_))) } }).truthy() {
        if ({ let t4396 = py::not(&{ let t4393 = Py::Bool(matches!(v_a.clone(), Py::List(_) | Py::Tuple(_))); if !t4393.truthy() { t4393 } else { Py::Bool(matches!(v_b.clone(), Py::List(_) | Py::Tuple(_))) } }); if t4396.truthy() { t4396 } else { { let t4394 = py::len(&v_a.clone())?; Py::Bool({ let t4395 = py::len(&v_b.clone())?; if py::ne(&t4394, &t4395) { true } else { false } }) } } }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        return Ok(py::minmax(&[py::add(&{ let mut out = vec![]; for t4397 in py::iter(&(py::zip(&[v_a.clone(), v_b.clone()])?))? { let mut v_x: Py; let mut v_y: Py; 
let t4398 = (t4397).items()?;
if t4398.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t4398[0].clone();
v_y = t4398[1].clone();
out.push(f__rel(v_x.clone(), v_y.clone())?); } Py::list(out) }, &Py::list(vec![Py::Float(0.0)]))?], true)?);
    }
    if ({ let t4399 = Py::Bool(matches!(v_a.clone(), Py::Str(_) | Py::Bool(_))); if t4399.truthy() { t4399 } else { Py::Bool(matches!(v_b.clone(), Py::Str(_) | Py::Bool(_))) } }).truthy() {
        return Ok((if ({ let t4400 = v_a.clone(); Py::Bool({ let t4401 = v_b.clone(); if py::eq(&t4400, &t4401) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(f64::INFINITY) }));
    }
    let t4403: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            let t4402 = (Py::tuple(vec![py::float(&v_a.clone())?, py::float(&v_b.clone())?])).items()?;
            if t4402.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_a = t4402[0].clone();
            v_b = t4402[1].clone();
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4403 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_d = py::abs(&py::sub(&v_a.clone(), &v_b.clone())?)?;
    if ({ let t4404 = v_d.clone(); Py::Bool({ let t4405 = Py::Int(0i64); if py::eq(&t4404, &t4405) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    v_s = py::minmax(&[py::abs(&v_a.clone())?, py::abs(&v_b.clone())?], true)?;
    return Ok((if ({ let t4406 = v_s.clone(); Py::Bool({ let t4407 = Py::Int(0i64); if py::gt(&t4406, &t4407)? { true } else { false } }) }).truthy() { py::truediv(&v_d.clone(), &v_s.clone())? } else { Py::Float(f64::INFINITY) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__rho_at(mut v_h: Py) -> R {
    return Ok(py::getitem(&f__isa(v_h.clone())?, &Py::Int(2i64))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__rkt_in(mut v_pc: Py, mut v_Tc: Py, mut v_gamma: Py, mut v_Mw: Py) -> R {
    let mut v_g: Py = Py::None;
    v_g = f__gam(v_gamma.clone())?;
    let _ = f__pos(Py::str("the chamber pressure, chamber temperature and molar mass"), Py::tuple(vec![v_pc.clone(), v_Tc.clone(), v_Mw.clone()]))?;
    let _ = f__need({ let t3961 = Py::Float(0.001); Py::Bool({ let t3962 = v_Mw.clone(); if py::le(&t3961, &t3962)? { { let t3963 = Py::Float(0.3); if py::le(&t3962, &t3963)? { true } else { false } } } else { false } }) }, Py::str("the molar mass must lie between 1 and 300 g/mol"))?;
    return Ok(Py::tuple(vec![v_g.clone(), py::truediv(&c_RU()?, &v_Mw.clone())?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__roll_terms(mut v_W: Py, mut v_S: Py, mut v_h: Py, mut v_mu: Py, mut v_CL: Py, mut v_CD: Py) -> R {
    let mut v_rho: Py = Py::None;
    let _ = f__ws_in(v_W.clone(), v_S.clone())?;
    let _ = f__need({ let t3887 = Py::Bool(matches!(v_mu.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t3887.truthy() { t3887 } else { { let t3884 = Py::Int(0i64); Py::Bool({ let t3885 = v_mu.clone(); if py::le(&t3884, &t3885)? { { let t3886 = Py::Float(0.8); if py::le(&t3885, &t3886)? { true } else { false } } } else { false } }) } } }, Py::str("the friction coefficient must lie between 0 and 0.8"))?;
    let _ = f__need({ let t3891 = Py::Bool(matches!(v_CL.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t3891.truthy() { t3891 } else { { let t3888 = Py::Int(0i64); Py::Bool({ let t3889 = v_CL.clone(); if py::le(&t3888, &t3889)? { { let t3890 = Py::Float(3.0); if py::le(&t3889, &t3890)? { true } else { false } } } else { false } }) } } }, Py::str("the ground-run lift coefficient must lie between 0 and 3"))?;
    let _ = f__need({ let t3895 = Py::Bool(matches!(v_CD.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t3895.truthy() { t3895 } else { { let t3892 = Py::Float(0.005); Py::Bool({ let t3893 = v_CD.clone(); if py::le(&t3892, &t3893)? { { let t3894 = Py::Float(1.0); if py::le(&t3893, &t3894)? { true } else { false } } } else { false } }) } } }, Py::str("the ground-run drag coefficient must lie between 0.005 and 1"))?;
    v_rho = f__rho_at(v_h.clone())?;
    return Ok(py::truediv(&py::mul(&py::mul(&py::mul(&c_G0()?, &v_rho.clone())?, &v_S.clone())?, &py::sub(&v_CD.clone(), &py::mul(&v_mu.clone(), &v_CL.clone())?)?)?, &py::mul(&Py::Int(2i64), &v_W.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__sep_check(mut v_pe: Py, mut v_pa: Py) -> R {
    let _ = f__need({ let t4095 = { let t4091 = v_pa.clone(); Py::Bool({ let t4092 = Py::Int(0i64); if py::eq(&t4091, &t4092) { true } else { false } }) }; if t4095.truthy() { t4095 } else { { let t4093 = v_pe.clone(); Py::Bool({ let t4094 = py::mul(&Py::Float(0.4), &v_pa.clone())?; if py::ge(&t4093, &t4094)? { true } else { false } }) } } }, Py::str("the nozzle is so overexpanded (pe < 0.4 pa) that the flow is likely to separate; ideal thrust does not apply"))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__shock(mut v_M1: Py, mut v_g: Py) -> R {
    let mut v_M2: Py = Py::None;
    let mut v_m2: Py = Py::None;
    let mut v_p0r: Py = Py::None;
    let mut v_p21: Py = Py::None;
    let mut v_r21: Py = Py::None;
    v_m2 = py::mul(&v_M1.clone(), &v_M1.clone())?;
    v_p21 = py::add(&Py::Float(1.0), &py::mul(&py::truediv(&py::mul(&Py::Float(2.0), &v_g.clone())?, &py::add(&v_g.clone(), &Py::Float(1.0))?)?, &py::sub(&v_m2.clone(), &Py::Float(1.0))?)?)?;
    v_r21 = py::truediv(&py::mul(&py::add(&v_g.clone(), &Py::Float(1.0))?, &v_m2.clone())?, &py::add(&py::mul(&py::sub(&v_g.clone(), &Py::Float(1.0))?, &v_m2.clone())?, &Py::Float(2.0))?)?;
    v_M2 = py::math1("sqrt", &py::truediv(&py::add(&Py::Float(1.0), &py::mul(&py::mul(&Py::Float(0.5), &py::sub(&v_g.clone(), &Py::Float(1.0))?)?, &v_m2.clone())?)?, &py::sub(&py::mul(&v_g.clone(), &v_m2.clone())?, &py::mul(&Py::Float(0.5), &py::sub(&v_g.clone(), &Py::Float(1.0))?)?)?)?)?;
    v_p0r = py::truediv(&py::mul(&v_p21.clone(), &f__p0p(v_M2.clone(), v_g.clone())?)?, &f__p0p(v_M1.clone(), v_g.clone())?)?;
    return Ok(Py::tuple(vec![v_M2.clone(), v_p21.clone(), v_r21.clone(), v_p0r.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__stack(mut v_R: Py, mut v_eps: Py, mut v_mpl: Py) -> R {
    let mut v_Ri: Py = Py::None;
    let mut v_above: Py = Py::None;
    let mut v_ei: Py = Py::None;
    let mut v_mi: Py = Py::None;
    let mut v_stages: Py = Py::None;
    let t3142 = (Py::tuple(vec![v_mpl.clone(), Py::list(vec![])])).items()?;
    if t3142.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_above = t3142[0].clone();
    v_stages = t3142[1].clone();
    for t3143 in py::iter(&(py::zip(&[py::reversed(&v_R.clone())?, py::reversed(&v_eps.clone())?])?))? {
        let t3144 = (t3143).items()?;
        if t3144.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_Ri = t3144[0].clone();
        v_ei = t3144[1].clone();
        v_mi = py::truediv(&py::mul(&v_above.clone(), &py::sub(&v_Ri.clone(), &Py::Float(1.0))?)?, &py::sub(&Py::Float(1.0), &py::mul(&v_Ri.clone(), &v_ei.clone())?)?)?;
        let _ = py::method(&v_stages.clone(), "append", &[v_mi.clone()])?;
        v_above = py::iadd(&v_above, &v_mi.clone())?;
    }
    return Ok(Py::tuple(vec![v_above.clone(), Py::list((py::reversed(&v_stages.clone())?).items()?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__static(mut v_hp: Py, mut v_T: Py) -> R {
    let mut v_Tstd: Py = Py::None;
    let mut v__rho: Py = Py::None;
    let mut v_p: Py = Py::None;
    let t4233 = (f__isa(v_hp.clone())?).items()?;
    if t4233.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_Tstd = t4233[0].clone();
    v_p = t4233[1].clone();
    v__rho = t4233[2].clone();
    v_T = (if ({ let t4234 = v_T.clone(); Py::Bool({ let t4235 = Py::None; if py::is(&t4234, &t4235) { true } else { false } }) }).truthy() { v_Tstd.clone() } else { v_T.clone() });
    let _ = f__need({ let t4236 = Py::Float(150.0); Py::Bool({ let t4237 = v_T.clone(); if py::le(&t4236, &t4237)? { { let t4238 = Py::Float(350.0); if py::le(&t4237, &t4238)? { true } else { false } } } else { false } }) }, Py::str("the outside air temperature must lie between 150 K and 350 K"))?;
    return Ok(Py::tuple(vec![v_p.clone(), v_T.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__stumpff(mut v_z: Py) -> R {
    let mut v_s: Py = Py::None;
    if ({ let t1717 = py::abs(&v_z.clone())?; Py::Bool({ let t1718 = Py::Float(0.001); if py::lt(&t1717, &t1718)? { true } else { false } }) }).truthy() {
        return Ok(Py::tuple(vec![py::sub(&py::add(&py::sub(&Py::Float(0.5), &py::truediv(&v_z.clone(), &Py::Int(24i64))?)?, &py::truediv(&py::mul(&v_z.clone(), &v_z.clone())?, &Py::Int(720i64))?)?, &py::truediv(&py::pow(&v_z.clone(), &Py::Int(3i64))?, &Py::Int(40320i64))?)?, py::sub(&py::add(&py::sub(&py::truediv(&Py::Int(1i64), &Py::Int(6i64))?, &py::truediv(&v_z.clone(), &Py::Int(120i64))?)?, &py::truediv(&py::mul(&v_z.clone(), &v_z.clone())?, &Py::Int(5040i64))?)?, &py::truediv(&py::pow(&v_z.clone(), &Py::Int(3i64))?, &Py::Int(362880i64))?)?]));
    }
    if ({ let t1719 = v_z.clone(); Py::Bool({ let t1720 = Py::Int(0i64); if py::gt(&t1719, &t1720)? { true } else { false } }) }).truthy() {
        v_s = py::math1("sqrt", &v_z.clone())?;
        return Ok(Py::tuple(vec![py::truediv(&py::mul(&Py::Int(2i64), &py::pow(&py::math1("sin", &py::truediv(&v_s.clone(), &Py::Int(2i64))?)?, &Py::Int(2i64))?)?, &v_z.clone())?, py::truediv(&py::sub(&v_s.clone(), &py::math1("sin", &v_s.clone())?)?, &py::pow(&v_s.clone(), &Py::Int(3i64))?)?]));
    }
    v_s = py::math1("sqrt", &py::neg(&v_z.clone())?)?;
    return Ok(Py::tuple(vec![py::truediv(&py::mul(&Py::Int(2i64), &py::pow(&py::math1("sinh", &py::truediv(&v_s.clone(), &Py::Int(2i64))?)?, &Py::Int(2i64))?)?, &py::neg(&v_z.clone())?)?, py::truediv(&py::sub(&py::math1("sinh", &v_s.clone())?, &v_s.clone())?, &py::pow(&v_s.clone(), &Py::Int(3i64))?)?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__sutherland(mut v_T: Py) -> R {
    return Ok(py::truediv(&py::mul(&py::mul(&Py::Float(1.458e-06), &v_T.clone())?, &py::math1("sqrt", &v_T.clone())?)?, &py::add(&v_T.clone(), &Py::Float(110.4))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__t0t(mut v_M: Py, mut v_g: Py) -> R {
    return Ok(py::add(&Py::Float(1.0), &py::mul(&py::mul(&py::mul(&Py::Float(0.5), &py::sub(&v_g.clone(), &Py::Float(1.0))?)?, &v_M.clone())?, &v_M.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__theta_of_beta(mut v_M1: Py, mut v_b: Py, mut v_g: Py) -> R {
    let mut v_m2s: Py = Py::None;
    v_m2s = py::pow(&py::mul(&v_M1.clone(), &py::math1("sin", &v_b.clone())?)?, &Py::Int(2i64))?;
    return Ok(py::atan2(&py::truediv(&py::mul(&Py::Float(2.0), &py::sub(&v_m2s.clone(), &Py::Float(1.0))?)?, &py::math1("tan", &v_b.clone())?)?, &py::add(&py::mul(&py::mul(&v_M1.clone(), &v_M1.clone())?, &py::add(&v_g.clone(), &py::math1("cos", &py::mul(&Py::Float(2.0), &v_b.clone())?)?)?)?, &Py::Float(2.0))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__tsfc_in(mut v_c: Py) -> R {
    let _ = f__need({ let t1637 = Py::Bool(matches!(v_c.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t1637.truthy() { t1637 } else { { let t1634 = Py::Float(1e-07); Py::Bool({ let t1635 = v_c.clone(); if py::le(&t1634, &t1635)? { { let t1636 = Py::Float(0.001); if py::le(&t1635, &t1636)? { true } else { false } } } else { false } }) } } }, Py::str("the thrust-specific fuel consumption must lie between 1e-7 and 1e-3 kg/(N s)"))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__turn_out(mut v_V: Py, mut v_n: Py, mut v_g: Py) -> R {
    let mut v_w: Py = Py::None;
    v_w = py::truediv(&py::mul(&v_g.clone(), &py::math1("sqrt", &py::sub(&py::mul(&v_n.clone(), &v_n.clone())?, &Py::Int(1i64))?)?)?, &v_V.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("n"), v_n.clone()), (Py::str("phi_deg"), py::math1("degrees", &py::math1("acos", &py::truediv(&Py::Float(1.0), &v_n.clone())?)?)?), (Py::str("R"), py::truediv(&v_V.clone(), &v_w.clone())?), (Py::str("omega"), v_w.clone()), (Py::str("omega_deg"), py::truediv(&v_w.clone(), &c_DEG()?)?), (Py::str("t_360"), py::truediv(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_w.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__vdk(mut v_g: Py) -> R {
    return Ok(py::mul(&py::math1("sqrt", &v_g.clone())?, &py::pow(&py::truediv(&Py::Float(2.0), &py::add(&v_g.clone(), &Py::Float(1.0))?)?, &py::truediv(&py::add(&v_g.clone(), &Py::Float(1.0))?, &py::mul(&Py::Float(2.0), &py::sub(&v_g.clone(), &Py::Float(1.0))?)?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__vert_in(mut v_Isp: Py, mut v_m0: Py, mut v_mf: Py, mut v_tb: Py, mut v_g: Py) -> R {
    let mut v_c: Py = Py::None;
    let _ = f__isp_in(v_Isp.clone())?;
    let _ = f__pos(Py::str("the masses, burn time and gravity"), Py::tuple(vec![v_m0.clone(), v_mf.clone(), v_tb.clone(), v_g.clone()]))?;
    let _ = f__need({ let t4283 = v_mf.clone(); Py::Bool({ let t4284 = v_m0.clone(); if py::lt(&t4283, &t4284)? { true } else { false } }) }, Py::str("the burnout mass must be smaller than the initial mass"))?;
    v_c = py::mul(&v_Isp.clone(), &c_G0()?)?;
    let _ = f__need({ let t4285 = py::truediv(&py::mul(&v_c.clone(), &py::sub(&v_m0.clone(), &v_mf.clone())?)?, &v_tb.clone())?; Py::Bool({ let t4286 = py::mul(&py::mul(&Py::Float(1.001), &v_m0.clone())?, &v_g.clone())?; if py::gt(&t4285, &t4286)? { true } else { false } }) }, Py::str("the thrust must exceed the initial weight (thrust-to-weight above 1)"))?;
    return Ok(Py::tuple(vec![v_c.clone(), py::truediv(&v_m0.clone(), &v_mf.clone())?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__w01(mut v_W0: Py, mut v_W1: Py) -> R {
    let _ = f__pos(Py::str("the initial and final weights"), Py::tuple(vec![v_W0.clone(), v_W1.clone()]))?;
    let _ = f__need({ let t3343 = v_W1.clone(); Py::Bool({ let t3344 = v_W0.clone(); if py::lt(&t3343, &t3344)? { true } else { false } }) }, Py::str("the final weight must be below the initial weight"))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__ws_in(mut v_W: Py, mut v_S: Py) -> R {
    let _ = f__pos(Py::str("the weight and the wing area"), Py::tuple(vec![v_W.clone(), v_S.clone()]))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_absolute_ceiling_jet(mut v_W: Py, mut v_S: Py, mut v_T0: Py, mut v_CD0: Py, mut v_AR: Py, mut v_e: Py) -> R {
    let mut v_K: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_rho: Py = Py::None;
    let mut v_sig: Py = Py::None;
    let _ = f__ws_in(v_W.clone(), v_S.clone())?;
    let _ = f__cd0(v_CD0.clone())?;
    v_K = f__K(v_AR.clone(), v_e.clone())?;
    let _ = f__pos(Py::str("the sea-level thrust"), Py::tuple(vec![v_T0.clone()]))?;
    v_sig = py::truediv(&py::mul(&py::mul(&Py::Int(2i64), &v_W.clone())?, &py::math1("sqrt", &py::mul(&v_K.clone(), &v_CD0.clone())?)?)?, &v_T0.clone())?;
    let _ = f__need({ let t18 = v_sig.clone(); Py::Bool({ let t19 = Py::Float(1.0); if py::lt(&t18, &t19)? { true } else { false } }) }, Py::str("the sea-level thrust is below the minimum drag: the aircraft cannot fly level even at sea level"))?;
    v_rho = py::mul(&v_sig.clone(), &c_RHO_SL()?)?;
    let _ = f__need({ let t20 = v_rho.clone(); Py::Bool({ let t21 = c_RHO_MIN()?; if py::ge(&t20, &t21)? { true } else { false } }) }, Py::str("the ceiling lies above 47 km"))?;
    v_h = f__geom(f__H_from_rho(v_rho.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("h_abs"), v_h.clone()), (Py::str("h_abs_ft"), py::truediv(&v_h.clone(), &c_FT()?)?), (Py::str("sigma"), v_sig.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_actuator_disk_forward(mut v_T: Py, mut v_V: Py, mut v_D: Py, mut v_rho: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_A = f__disk_in(v_T.clone(), v_D.clone(), v_rho.clone())?;
    let _ = f__pos(Py::str("the flight speed"), Py::tuple(vec![v_V.clone()]))?;
    v_k = py::truediv(&v_T.clone(), &py::mul(&py::mul(&Py::Int(2i64), &v_rho.clone())?, &v_A.clone())?)?;
    v_v = py::truediv(&v_k.clone(), &py::add(&py::truediv(&v_V.clone(), &Py::Int(2i64))?, &py::math1("sqrt", &py::add(&py::truediv(&py::mul(&v_V.clone(), &v_V.clone())?, &Py::Int(4i64))?, &v_k.clone())?)?)?)?;
    v_P = py::mul(&v_T.clone(), &py::add(&v_V.clone(), &v_v.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("v_i"), v_v.clone()), (Py::str("P_ideal"), v_P.clone()), (Py::str("eta_ideal"), py::truediv(&py::mul(&v_T.clone(), &v_V.clone())?, &v_P.clone())?), (Py::str("w"), py::mul(&Py::Int(2i64), &v_v.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_actuator_disk_hover(mut v_T: Py, mut v_D: Py, mut v_rho: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_A = f__disk_in(v_T.clone(), v_D.clone(), v_rho.clone())?;
    v_v = py::math1("sqrt", &py::truediv(&v_T.clone(), &py::mul(&py::mul(&Py::Int(2i64), &v_rho.clone())?, &v_A.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("v_i"), v_v.clone()), (Py::str("P_ideal"), py::mul(&v_T.clone(), &v_v.clone())?), (Py::str("w"), py::mul(&Py::Int(2i64), &v_v.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_altitude_for_period(mut v_T: Py) -> R {
    let mut v_h: Py = Py::None;
    let mut v_r: Py = Py::None;
    let _ = f__pos(Py::str("the period"), Py::tuple(vec![v_T.clone()]))?;
    v_r = py::pow(&py::mul(&c_MU_E()?, &py::pow(&py::truediv(&v_T.clone(), &py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?)?, &Py::Int(2i64))?)?, &py::truediv(&Py::Float(1.0), &Py::Float(3.0))?)?;
    v_h = py::sub(&v_r.clone(), &c_R_E()?)?;
    let _ = f__orb_alt(v_h.clone(), Py::Float(100000.0), Py::Float(1000000000.0))?;
    return Ok(f__out(Py::dict(vec![(Py::str("h"), v_h.clone()), (Py::str("h_km"), py::truediv(&v_h.clone(), &Py::Float(1000.0))?), (Py::str("v"), py::math1("sqrt", &py::truediv(&c_MU_E()?, &v_r.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_apsidal_rotation(mut v_hp: Py, mut v_ha: Py, mut v_i: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_rate: Py = Py::None;
    let t1487 = (f__j2_elements(v_hp.clone(), v_ha.clone(), v_i.clone())?).items()?;
    if t1487.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_a = t1487[0].clone();
    v_e = t1487[1].clone();
    v_p = t1487[2].clone();
    v_n = t1487[3].clone();
    let _ = f__need({ let t1488 = v_e.clone(); Py::Bool({ let t1489 = Py::Float(0.0001); if py::ge(&t1488, &t1489)? { true } else { false } }) }, Py::str("the orbit must be eccentric (apogee above perigee) for its perigee to be defined"))?;
    v_rate = py::mul(&py::mul(&py::mul(&py::mul(&Py::Float(0.75), &v_n.clone())?, &c_J2_E()?)?, &py::pow(&py::truediv(&c_R_E()?, &v_p.clone())?, &Py::Int(2i64))?)?, &py::sub(&py::mul(&Py::Int(5i64), &py::pow(&py::math1("cos", &v_i.clone())?, &Py::Int(2i64))?)?, &Py::Int(1i64))?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("argp_rate_deg_day"), py::mul(&py::truediv(&v_rate.clone(), &c_DEG()?)?, &c_DAY()?)?), (Py::str("argp_rate"), v_rate.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ballistic_coefficient(mut v_m: Py, mut v_CD: Py, mut v_A: Py) -> R {
    let _ = f__pos(Py::str("the mass, drag coefficient and reference area"), Py::tuple(vec![v_m.clone(), v_CD.clone(), v_A.clone()]))?;
    let _ = f__need({ let t182 = v_CD.clone(); Py::Bool({ let t183 = Py::Int(5i64); if py::le(&t182, &t183)? { true } else { false } }) }, Py::str("the drag coefficient must be at most 5"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("beta"), py::truediv(&v_m.clone(), &py::mul(&v_CD.clone(), &v_A.clone())?)?), (Py::str("beta_psf"), py::truediv(&py::mul(&py::truediv(&v_m.clone(), &py::mul(&v_CD.clone(), &v_A.clone())?)?, &c_G0()?)?, &py::truediv(&c_LBF()?, &py::pow(&c_FT()?, &Py::Int(2i64))?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bielliptic(mut v_h1: Py, mut v_h2: Py, mut v_hb: Py) -> R {
    let mut v_a1: Py = Py::None;
    let mut v_a2: Py = Py::None;
    let mut v_ah: Py = Py::None;
    let mut v_dv1: Py = Py::None;
    let mut v_dv2: Py = Py::None;
    let mut v_dv3: Py = Py::None;
    let mut v_dvh: Py = Py::None;
    let mut v_mu: Py = Py::None;
    let mut v_r1: Py = Py::None;
    let mut v_r2: Py = Py::None;
    let mut v_rb: Py = Py::None;
    let mut v_t: Py = Py::None;
    let _ = f__orb_alt(v_h1.clone(), Py::Float(100000.0), Py::Float(1000000000.0))?;
    let _ = f__orb_alt(v_h2.clone(), Py::Float(100000.0), Py::Float(1000000000.0))?;
    let _ = f__orb_alt(v_hb.clone(), Py::Float(100000.0), Py::Float(1000000000.0))?;
    let t310 = (Py::tuple(vec![py::add(&c_R_E()?, &v_h1.clone())?, py::add(&c_R_E()?, &v_h2.clone())?, py::add(&c_R_E()?, &v_hb.clone())?])).items()?;
    if t310.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_r1 = t310[0].clone();
    v_r2 = t310[1].clone();
    v_rb = t310[2].clone();
    let _ = f__need({ let t311 = v_r1.clone(); Py::Bool({ let t312 = v_r2.clone(); if py::lt(&t311, &t312)? { { let t313 = v_rb.clone(); if py::le(&t312, &t313)? { true } else { false } } } else { false } }) }, Py::str("the bi-elliptic transfer here is outward: h1 < h2 <= hb"))?;
    v_mu = c_MU_E()?;
    let t314 = (Py::tuple(vec![py::truediv(&py::add(&v_r1.clone(), &v_rb.clone())?, &Py::Int(2i64))?, py::truediv(&py::add(&v_r2.clone(), &v_rb.clone())?, &Py::Int(2i64))?])).items()?;
    if t314.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a1 = t314[0].clone();
    v_a2 = t314[1].clone();
    v_dv1 = py::sub(&py::math1("sqrt", &py::mul(&v_mu.clone(), &py::sub(&py::truediv(&Py::Int(2i64), &v_r1.clone())?, &py::truediv(&Py::Int(1i64), &v_a1.clone())?)?)?)?, &py::math1("sqrt", &py::truediv(&v_mu.clone(), &v_r1.clone())?)?)?;
    v_dv2 = py::sub(&py::math1("sqrt", &py::mul(&v_mu.clone(), &py::sub(&py::truediv(&Py::Int(2i64), &v_rb.clone())?, &py::truediv(&Py::Int(1i64), &v_a2.clone())?)?)?)?, &py::math1("sqrt", &py::mul(&v_mu.clone(), &py::sub(&py::truediv(&Py::Int(2i64), &v_rb.clone())?, &py::truediv(&Py::Int(1i64), &v_a1.clone())?)?)?)?)?;
    v_dv3 = py::sub(&py::math1("sqrt", &py::mul(&v_mu.clone(), &py::sub(&py::truediv(&Py::Int(2i64), &v_r2.clone())?, &py::truediv(&Py::Int(1i64), &v_a2.clone())?)?)?)?, &py::math1("sqrt", &py::truediv(&v_mu.clone(), &v_r2.clone())?)?)?;
    v_t = py::mul(&Py::Float(std::f64::consts::PI), &py::add(&py::math1("sqrt", &py::truediv(&py::pow(&v_a1.clone(), &Py::Int(3i64))?, &v_mu.clone())?)?, &py::math1("sqrt", &py::truediv(&py::pow(&v_a2.clone(), &Py::Int(3i64))?, &v_mu.clone())?)?)?)?;
    v_ah = py::truediv(&py::add(&v_r1.clone(), &v_r2.clone())?, &Py::Int(2i64))?;
    v_dvh = py::add(&py::sub(&py::math1("sqrt", &py::mul(&v_mu.clone(), &py::sub(&py::truediv(&Py::Int(2i64), &v_r1.clone())?, &py::truediv(&Py::Int(1i64), &v_ah.clone())?)?)?)?, &py::math1("sqrt", &py::truediv(&v_mu.clone(), &v_r1.clone())?)?)?, &py::sub(&py::math1("sqrt", &py::truediv(&v_mu.clone(), &v_r2.clone())?)?, &py::math1("sqrt", &py::mul(&v_mu.clone(), &py::sub(&py::truediv(&Py::Int(2i64), &v_r2.clone())?, &py::truediv(&Py::Int(1i64), &v_ah.clone())?)?)?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("dv1"), v_dv1.clone()), (Py::str("dv2"), v_dv2.clone()), (Py::str("dv3"), v_dv3.clone()), (Py::str("dv"), py::add(&py::add(&v_dv1.clone(), &v_dv2.clone())?, &v_dv3.clone())?), (Py::str("t"), v_t.clone()), (Py::str("t_days"), py::truediv(&v_t.clone(), &c_DAY()?)?), (Py::str("dv_hohmann"), v_dvh.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_breguet_endurance_jet(mut v_c: Py, mut v_LD: Py, mut v_W0: Py, mut v_W1: Py) -> R {
    let mut v_E: Py = Py::None;
    let _ = f__tsfc_in(v_c.clone())?;
    let _ = f__ld_in(v_LD.clone())?;
    let _ = f__w01(v_W0.clone(), v_W1.clone())?;
    v_E = py::mul(&py::truediv(&v_LD.clone(), &py::mul(&c_G0()?, &v_c.clone())?)?, &py::math1("log", &py::truediv(&v_W0.clone(), &v_W1.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("E"), v_E.clone()), (Py::str("E_h"), py::truediv(&v_E.clone(), &Py::Float(3600.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_breguet_jet(mut v_V: Py, mut v_c: Py, mut v_LD: Py, mut v_W0: Py, mut v_W1: Py) -> R {
    let mut v_R: Py = Py::None;
    let _ = f__pos(Py::str("the flight speed"), Py::tuple(vec![v_V.clone()]))?;
    let _ = f__tsfc_in(v_c.clone())?;
    let _ = f__ld_in(v_LD.clone())?;
    let _ = f__w01(v_W0.clone(), v_W1.clone())?;
    v_R = py::mul(&py::truediv(&py::mul(&v_V.clone(), &v_LD.clone())?, &py::mul(&c_G0()?, &v_c.clone())?)?, &py::math1("log", &py::truediv(&v_W0.clone(), &v_W1.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("R"), v_R.clone()), (Py::str("R_km"), py::truediv(&v_R.clone(), &Py::Float(1000.0))?), (Py::str("t"), py::truediv(&v_R.clone(), &v_V.clone())?), (Py::str("t_h"), py::truediv(&py::truediv(&v_R.clone(), &v_V.clone())?, &Py::Float(3600.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_breguet_prop(mut v_eta: Py, mut v_c: Py, mut v_LD: Py, mut v_W0: Py, mut v_W1: Py) -> R {
    let mut v_R: Py = Py::None;
    let _ = f__eta_in(v_eta.clone())?;
    let _ = f__bsfc_in(v_c.clone())?;
    let _ = f__ld_in(v_LD.clone())?;
    let _ = f__w01(v_W0.clone(), v_W1.clone())?;
    v_R = py::mul(&py::mul(&py::truediv(&v_eta.clone(), &py::mul(&c_G0()?, &v_c.clone())?)?, &v_LD.clone())?, &py::math1("log", &py::truediv(&v_W0.clone(), &v_W1.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("R"), v_R.clone()), (Py::str("R_km"), py::truediv(&v_R.clone(), &Py::Float(1000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_calibrated_airspeed(mut v_V: Py, mut v_hp: Py, mut v_T: Py) -> R {
    let mut v_M: Py = Py::None;
    let mut v_cas: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_qc: Py = Py::None;
    let _ = f__pos(Py::str("the true airspeed"), Py::tuple(vec![v_V.clone()]))?;
    let t516 = (f__static(v_hp.clone(), v_T.clone())?).items()?;
    if t516.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_p = t516[0].clone();
    v_T = t516[1].clone();
    v_M = py::truediv(&v_V.clone(), &py::math1("sqrt", &py::mul(&py::mul(&c_GAMMA_AIR()?, &c_R_AIR()?)?, &v_T.clone())?)?)?;
    let _ = f__need({ let t517 = v_M.clone(); Py::Bool({ let t518 = Py::Float(1.0); if py::lt(&t517, &t518)? { true } else { false } }) }, Py::str("the flight Mach number must be subsonic (below 1) for the subsonic pitot formula"))?;
    v_qc = py::mul(&v_p.clone(), &py::sub(&f__p0p(v_M.clone(), c_GAMMA_AIR()?)?, &Py::Float(1.0))?)?;
    v_cas = py::mul(&c_A_SL()?, &py::math1("sqrt", &py::mul(&Py::Float(5.0), &py::sub(&py::pow(&py::add(&py::truediv(&v_qc.clone(), &c_P_SL()?)?, &Py::Float(1.0))?, &py::truediv(&Py::Float(2.0), &Py::Float(7.0))?)?, &Py::Float(1.0))?)?)?)?;
    let _ = f__need({ let t519 = v_cas.clone(); Py::Bool({ let t520 = c_A_SL()?; if py::lt(&t519, &t520)? { true } else { false } }) }, Py::str("the calibrated airspeed would exceed the sea-level speed of sound"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("CAS"), v_cas.clone()), (Py::str("CAS_kt"), py::truediv(&v_cas.clone(), &c_KT()?)?), (Py::str("qc"), v_qc.clone()), (Py::str("pt"), py::add(&v_p.clone(), &v_qc.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_characteristic_velocity(mut v_Tc: Py, mut v_Mw: Py, mut v_gamma: Py) -> R {
    let mut v_R: Py = Py::None;
    let mut v_g: Py = Py::None;
    let t565 = (f__rkt_in(Py::Float(1.0), v_Tc.clone(), v_gamma.clone(), v_Mw.clone())?).items()?;
    if t565.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_g = t565[0].clone();
    v_R = t565[1].clone();
    return Ok(f__out(Py::dict(vec![(Py::str("c_star"), py::truediv(&py::math1("sqrt", &py::mul(&v_R.clone(), &v_Tc.clone())?)?, &f__vdk(v_g.clone())?)?), (Py::str("Gamma"), f__vdk(v_g.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_choked_mass_flow(mut v_p0: Py, mut v_T0: Py, mut v_A: Py, mut v_gamma: Py, mut v_R: Py) -> R {
    let mut v_g: Py = Py::None;
    let mut v_t: Py = Py::None;
    v_g = f__gam(v_gamma.clone())?;
    let _ = f__pos(Py::str("the stagnation pressure, stagnation temperature, throat area and gas constant"), Py::tuple(vec![v_p0.clone(), v_T0.clone(), v_A.clone(), v_R.clone()]))?;
    v_t = py::truediv(&Py::Float(2.0), &py::add(&v_g.clone(), &Py::Float(1.0))?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("mdot"), py::truediv(&py::mul(&py::mul(&f__vdk(v_g.clone())?, &v_p0.clone())?, &v_A.clone())?, &py::math1("sqrt", &py::mul(&v_R.clone(), &v_T0.clone())?)?)?), (Py::str("p_star"), py::mul(&v_p0.clone(), &py::pow(&v_t.clone(), &py::truediv(&v_g.clone(), &py::sub(&v_g.clone(), &Py::Float(1.0))?)?)?)?), (Py::str("T_star"), py::mul(&v_T0.clone(), &v_t.clone())?), (Py::str("V_star"), py::math1("sqrt", &py::mul(&py::mul(&py::mul(&v_g.clone(), &v_R.clone())?, &v_T0.clone())?, &v_t.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_circular_orbit(mut v_h: Py) -> R {
    let mut v_T: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_v: Py = Py::None;
    let _ = f__orb_alt(v_h.clone(), Py::Float(100000.0), Py::Float(1000000000.0))?;
    v_r = py::add(&c_R_E()?, &v_h.clone())?;
    v_v = py::math1("sqrt", &py::truediv(&c_MU_E()?, &v_r.clone())?)?;
    v_T = py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &py::math1("sqrt", &py::truediv(&py::pow(&v_r.clone(), &Py::Int(3i64))?, &c_MU_E()?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("v"), v_v.clone()), (Py::str("T"), v_T.clone()), (Py::str("T_min"), py::truediv(&v_T.clone(), &Py::Float(60.0))?), (Py::str("revs_per_day"), py::truediv(&c_DAY()?, &v_T.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_critical_mach(mut v_Cp0: Py, mut v_rule: Py) -> R {
    let mut v_M: Py = Py::None;
    let mut v_cp: Py = Py::None;
    let mut v_cp_cr: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_top: Py = Py::None;
    let _ = f__need({ let t678 = Py::Bool(matches!(v_Cp0.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t678.truthy() { t678 } else { { let t675 = py::neg(&Py::Float(3.0))?; Py::Bool({ let t676 = v_Cp0.clone(); if py::le(&t675, &t676)? { { let t677 = py::neg(&Py::Float(0.01))?; if py::lt(&t676, &t677)? { true } else { false } } } else { false } }) } } }, Py::str("the incompressible minimum pressure coefficient must lie between -3 and -0.01"))?;
    let _ = f__need({ let t679 = v_rule.clone(); Py::Bool({ let t680 = Py::tuple(vec![Py::str("prandtl-glauert"), Py::str("karman-tsien")]); if py::contains(&t680, &t679)? { true } else { false } }) }, Py::str("the compressibility rule must be prandtl-glauert or karman-tsien"))?;
    v_g = c_GAMMA_AIR()?;
    v_cp_cr = { let v_Cp0 = v_Cp0.clone(); let v_cp_cr = v_cp_cr.clone(); let v_g = v_g.clone(); let v_rule = v_rule.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_M: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::mul(&py::truediv(&Py::Float(2.0), &py::mul(&py::mul(&v_g.clone(), &v_M.clone())?, &v_M.clone())?)?, &py::sub(&py::pow(&py::truediv(&f__t0t(v_M.clone(), v_g.clone())?, &f__t0t(Py::Float(1.0), v_g.clone())?)?, &py::truediv(&v_g.clone(), &py::sub(&v_g.clone(), &Py::Float(1.0))?)?)?, &Py::Float(1.0))?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_cp = { let v_Cp0 = v_Cp0.clone(); let v_cp = v_cp.clone(); let v_cp_cr = v_cp_cr.clone(); let v_g = v_g.clone(); let v_rule = v_rule.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_M: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_b: Py = Py::None;
            v_b = py::math1("sqrt", &py::sub(&Py::Float(1.0), &py::mul(&v_M.clone(), &v_M.clone())?)?)?;
            return Ok((if ({ let t681 = v_rule.clone(); Py::Bool({ let t682 = Py::str("prandtl-glauert"); if py::eq(&t681, &t682) { true } else { false } }) }).truthy() { py::truediv(&v_Cp0.clone(), &v_b.clone())? } else { py::truediv(&v_Cp0.clone(), &py::add(&v_b.clone(), &py::truediv(&py::mul(&py::truediv(&py::mul(&v_M.clone(), &v_M.clone())?, &py::add(&Py::Float(1.0), &v_b.clone())?)?, &v_Cp0.clone())?, &Py::Float(2.0))?)?)? }));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_top = py::sub(&Py::Float(1.0), &Py::Float(1e-12))?;
    if ({ let t683 = v_rule.clone(); Py::Bool({ let t684 = Py::str("karman-tsien"); if py::eq(&t683, &t684) { true } else { false } }) }).truthy() {
        v_top = py::mul(&f__bisect({ let v_Cp0 = v_Cp0.clone(); let v_cp = v_cp.clone(); let v_cp_cr = v_cp_cr.clone(); let v_g = v_g.clone(); let v_rule = v_rule.clone(); let v_top = v_top.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_m: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::add(&py::math1("sqrt", &py::sub(&Py::Float(1.0), &py::mul(&v_m.clone(), &v_m.clone())?)?)?, &py::truediv(&py::mul(&py::truediv(&py::mul(&v_m.clone(), &v_m.clone())?, &py::add(&Py::Float(1.0), &py::math1("sqrt", &py::sub(&Py::Float(1.0), &py::mul(&v_m.clone(), &v_m.clone())?)?)?)?)?, &v_Cp0.clone())?, &Py::Float(2.0))?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, Py::Float(0.001), Py::Float(1.0), Py::Int(200i64))?, &py::sub(&Py::Float(1.0), &Py::Float(1e-12))?)?;
    }
    v_M = f__bisect({ let v_Cp0 = v_Cp0.clone(); let v_cp = v_cp.clone(); let v_cp_cr = v_cp_cr.clone(); let v_g = v_g.clone(); let v_rule = v_rule.clone(); let v_top = v_top.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_m: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&py::call(&v_cp.clone(), &[v_m.clone()])?, &py::call(&v_cp_cr.clone(), &[v_m.clone()])?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, Py::Float(0.001), v_top.clone(), Py::Int(200i64))?;
    return Ok(f__out(Py::dict(vec![(Py::str("M_cr"), v_M.clone()), (Py::str("Cp_cr"), py::call(&v_cp_cr.clone(), &[v_M.clone()])?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_density_altitude(mut v_hp: Py, mut v_T: Py) -> R {
    let mut v_H: Py = Py::None;
    let mut v__Tstd: Py = Py::None;
    let mut v__rho: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_rho: Py = Py::None;
    let _ = f__pos(Py::str("the outside air temperature"), Py::tuple(vec![v_T.clone()]))?;
    let _ = f__need({ let t793 = Py::Float(150.0); Py::Bool({ let t794 = v_T.clone(); if py::le(&t793, &t794)? { { let t795 = Py::Float(350.0); if py::le(&t794, &t795)? { true } else { false } } } else { false } }) }, Py::str("the outside air temperature must lie between 150 K and 350 K"))?;
    let t809 = (f__isa(v_hp.clone())?).items()?;
    if t809.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v__Tstd = t809[0].clone();
    v_p = t809[1].clone();
    v__rho = t809[2].clone();
    v_rho = py::truediv(&v_p.clone(), &py::mul(&c_R_AIR()?, &v_T.clone())?)?;
    let _ = f__need({ let t810 = c_RHO_MIN()?; Py::Bool({ let t811 = v_rho.clone(); if py::le(&t810, &t811)? { { let t812 = c_RHO_MAX()?; if py::le(&t811, &t812)? { true } else { false } } } else { false } }) }, Py::str("the density lies outside the standard atmosphere from -1 km to 47 km"))?;
    v_H = f__H_from_rho(v_rho.clone())?;
    v_h = f__geom(v_H.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("rho"), v_rho.clone()), (Py::str("sigma"), py::truediv(&v_rho.clone(), &c_RHO_SL()?)?), (Py::str("h_rho"), v_h.clone()), (Py::str("h_rho_ft"), py::truediv(&v_h.clone(), &c_FT()?)?), (Py::str("p"), v_p.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_drag_polar_optimum(mut v_CD0: Py, mut v_AR: Py, mut v_e: Py) -> R {
    let mut v_K: Py = Py::None;
    let mut v_cl_md: Py = Py::None;
    let mut v_cl_mp: Py = Py::None;
    let mut v_cl_mr: Py = Py::None;
    let _ = f__cd0(v_CD0.clone())?;
    v_K = f__K(v_AR.clone(), v_e.clone())?;
    let t865 = (Py::tuple(vec![py::math1("sqrt", &py::truediv(&v_CD0.clone(), &v_K.clone())?)?, py::math1("sqrt", &py::truediv(&py::mul(&Py::Int(3i64), &v_CD0.clone())?, &v_K.clone())?)?, py::math1("sqrt", &py::truediv(&v_CD0.clone(), &py::mul(&Py::Int(3i64), &v_K.clone())?)?)?])).items()?;
    if t865.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_cl_md = t865[0].clone();
    v_cl_mp = t865[1].clone();
    v_cl_mr = t865[2].clone();
    return Ok(f__out(Py::dict(vec![(Py::str("CL_md"), v_cl_md.clone()), (Py::str("LD_max"), py::truediv(&Py::Float(1.0), &py::mul(&Py::Float(2.0), &py::math1("sqrt", &py::mul(&v_CD0.clone(), &v_K.clone())?)?)?)?), (Py::str("CL_mp"), v_cl_mp.clone()), (Py::str("LD_mp"), py::truediv(&v_cl_mp.clone(), &py::mul(&Py::Float(4.0), &v_CD0.clone())?)?), (Py::str("CL_mr"), v_cl_mr.clone()), (Py::str("LD_mr"), py::truediv(&v_cl_mr.clone(), &py::truediv(&py::mul(&v_CD0.clone(), &Py::Float(4.0))?, &Py::Float(3.0))?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_elliptic_wing(mut v_AR: Py, mut v_alpha: Py, mut v_a0: Py) -> R {
    let mut v_CL: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_out: Py = Py::None;
    let _ = f__need({ let t901 = Py::Bool(matches!(v_AR.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t901.truthy() { t901 } else { { let t898 = Py::Float(2.0); Py::Bool({ let t899 = v_AR.clone(); if py::le(&t898, &t899)? { { let t900 = Py::Float(40.0); if py::le(&t899, &t900)? { true } else { false } } } else { false } }) } } }, Py::str("the aspect ratio must lie between 2 and 40 (lifting-line theory)"))?;
    let _ = f__need({ let t905 = Py::Bool(matches!(v_a0.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t905.truthy() { t905 } else { { let t902 = Py::Float(3.0); Py::Bool({ let t903 = v_a0.clone(); if py::le(&t902, &t903)? { { let t904 = Py::Float(7.5); if py::le(&t903, &t904)? { true } else { false } } } else { false } }) } } }, Py::str("the section lift slope must lie between 3 and 7.5 per radian"))?;
    let _ = f__need({ let t908 = Py::Bool(matches!(v_alpha.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t908.truthy() { t908 } else { { let t906 = py::abs(&v_alpha.clone())?; Py::Bool({ let t907 = py::mul(&Py::Int(15i64), &c_DEG()?)?; if py::le(&t906, &t907)? { true } else { false } }) } } }, Py::str("the angle of attack (from zero lift) must lie within +-15 degrees"))?;
    v_a = py::truediv(&v_a0.clone(), &py::add(&Py::Int(1i64), &py::truediv(&v_a0.clone(), &py::mul(&Py::Float(std::f64::consts::PI), &v_AR.clone())?)?)?)?;
    v_CL = py::mul(&v_a.clone(), &v_alpha.clone())?;
    v_out = f__out(Py::dict(vec![(Py::str("CL_alpha"), v_a.clone()), (Py::str("CL"), v_CL.clone()), (Py::str("CDi"), py::truediv(&py::mul(&v_CL.clone(), &v_CL.clone())?, &py::mul(&Py::Float(std::f64::consts::PI), &v_AR.clone())?)?), (Py::str("alpha_i_deg"), py::truediv(&py::truediv(&v_CL.clone(), &py::mul(&Py::Float(std::f64::consts::PI), &v_AR.clone())?)?, &c_DEG()?)?)]))?;
    if ({ let t918 = v_alpha.clone(); Py::Bool({ let t919 = Py::Int(0i64); if py::eq(&t918, &t919) { true } else { false } }) }).truthy() {
        let _ = { let t920 = v_out.clone(); py::setitem(&t920, &Py::str("CL"), Py::Float(0.0))?; py::setitem(&t920, &Py::str("CDi"), Py::Float(0.0))?; py::setitem(&t920, &Py::str("alpha_i_deg"), Py::Float(0.0))?; Py::None };
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_entry_peak(mut v_m: Py, mut v_CD: Py, mut v_A: Py, mut v_VE: Py, mut v_gamma_e: Py, mut v_H: Py, mut v_rho0: Py) -> R {
    let mut v_amax: Py = Py::None;
    let mut v_beta: Py = Py::None;
    let mut v_hs: Py = Py::None;
    let mut v_s: Py = Py::None;
    v_beta = f__entry_in(v_m.clone(), v_CD.clone(), v_A.clone(), v_VE.clone(), v_gamma_e.clone(), v_H.clone(), v_rho0.clone())?;
    v_s = py::math1("sin", &v_gamma_e.clone())?;
    v_hs = py::mul(&v_H.clone(), &py::math1("log", &py::truediv(&py::mul(&v_rho0.clone(), &v_H.clone())?, &py::mul(&v_beta.clone(), &v_s.clone())?)?)?)?;
    let _ = f__need({ let t226 = v_hs.clone(); Py::Bool({ let t227 = Py::Int(0i64); if py::gt(&t226, &t227)? { true } else { false } }) }, Py::str("the deceleration would peak below sea level (the body reaches the ground first)"))?;
    v_amax = py::truediv(&py::mul(&py::mul(&v_VE.clone(), &v_VE.clone())?, &v_s.clone())?, &py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::E))?, &v_H.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("n_max"), py::truediv(&v_amax.clone(), &c_G0()?)?), (Py::str("a_max"), v_amax.clone()), (Py::str("h_peak"), v_hs.clone()), (Py::str("V_peak"), py::mul(&v_VE.clone(), &py::math1("exp", &py::neg(&Py::Float(0.5))?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_entry_velocity(mut v_m: Py, mut v_CD: Py, mut v_A: Py, mut v_VE: Py, mut v_gamma_e: Py, mut v_h: Py, mut v_H: Py, mut v_rho0: Py) -> R {
    let mut v_V: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_beta: Py = Py::None;
    let mut v_rho: Py = Py::None;
    let mut v_s: Py = Py::None;
    v_beta = f__entry_in(v_m.clone(), v_CD.clone(), v_A.clone(), v_VE.clone(), v_gamma_e.clone(), v_H.clone(), v_rho0.clone())?;
    let _ = f__need({ let t273 = Py::Bool(matches!(v_h.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t273.truthy() { t273 } else { { let t270 = Py::Int(0i64); Py::Bool({ let t271 = v_h.clone(); if py::le(&t270, &t271)? { { let t272 = py::mul(&Py::Int(40i64), &v_H.clone())?; if py::le(&t271, &t272)? { true } else { false } } } else { false } }) } } }, Py::str("the altitude must lie between sea level and 40 scale heights"))?;
    v_s = py::math1("sin", &v_gamma_e.clone())?;
    v_rho = py::mul(&v_rho0.clone(), &py::math1("exp", &py::truediv(&py::neg(&v_h.clone())?, &v_H.clone())?)?)?;
    v_V = py::mul(&v_VE.clone(), &py::math1("exp", &py::truediv(&py::mul(&py::neg(&v_rho.clone())?, &v_H.clone())?, &py::mul(&py::mul(&Py::Int(2i64), &v_beta.clone())?, &v_s.clone())?)?)?)?;
    v_a = py::truediv(&py::mul(&py::mul(&v_rho.clone(), &v_V.clone())?, &v_V.clone())?, &py::mul(&Py::Int(2i64), &v_beta.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("V"), v_V.clone()), (Py::str("n"), py::truediv(&v_a.clone(), &c_G0()?)?), (Py::str("a"), v_a.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_escape_speed(mut v_h: Py) -> R {
    let mut v_r: Py = Py::None;
    let mut v_vc: Py = Py::None;
    let mut v_ve: Py = Py::None;
    let _ = f__need({ let t947 = Py::Bool(matches!(v_h.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t947.truthy() { t947 } else { { let t944 = Py::Int(0i64); Py::Bool({ let t945 = v_h.clone(); if py::le(&t944, &t945)? { { let t946 = Py::Float(1000000000.0); if py::le(&t945, &t946)? { true } else { false } } } else { false } }) } } }, Py::str("the altitude must lie between 0 and 1e6 km"))?;
    v_r = py::add(&c_R_E()?, &v_h.clone())?;
    let t948 = (Py::tuple(vec![py::math1("sqrt", &py::truediv(&py::mul(&Py::Int(2i64), &c_MU_E()?)?, &v_r.clone())?)?, py::math1("sqrt", &py::truediv(&c_MU_E()?, &v_r.clone())?)?])).items()?;
    if t948.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_ve = t948[0].clone();
    v_vc = t948[1].clone();
    return Ok(f__out(Py::dict(vec![(Py::str("v_esc"), v_ve.clone()), (Py::str("v_circ"), v_vc.clone()), (Py::str("dv"), py::sub(&v_ve.clone(), &v_vc.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_expansion_fan(mut v_M1: Py, mut v_theta: Py, mut v_gamma: Py) -> R {
    let mut v_M2: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_nu1: Py = Py::None;
    let mut v_nu2: Py = Py::None;
    let mut v_pr: Py = Py::None;
    v_g = f__gam(v_gamma.clone())?;
    let _ = f__need({ let t988 = Py::Bool(matches!(v_M1.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t988.truthy() { t988 } else { { let t985 = Py::Float(1.0); Py::Bool({ let t986 = v_M1.clone(); if py::le(&t985, &t986)? { { let t987 = Py::Float(20.0); if py::le(&t986, &t987)? { true } else { false } } } else { false } }) } } }, Py::str("the upstream Mach number must lie between 1 and 20"))?;
    let _ = f__need({ let t989 = Py::Int(0i64); Py::Bool({ let t990 = v_theta.clone(); if py::lt(&t989, &t990)? { { let t991 = py::truediv(&Py::Float(std::f64::consts::PI), &Py::Int(2i64))?; if py::lt(&t990, &t991)? { true } else { false } } } else { false } }) }, Py::str("the turning angle must lie between 0 and 90 degrees"))?;
    v_nu1 = f__nu(v_M1.clone(), v_g.clone())?;
    v_nu2 = py::add(&v_nu1.clone(), &v_theta.clone())?;
    let _ = f__need({ let t992 = v_nu2.clone(); Py::Bool({ let t993 = f__nu(Py::Float(50.0), v_g.clone())?; if py::lt(&t992, &t993)? { true } else { false } }) }, Py::str("the turn exceeds what the gas can expand through (downstream Mach above 50)"))?;
    v_M2 = f__M_of_nu(v_nu2.clone(), v_g.clone())?;
    v_pr = py::pow(&py::truediv(&f__t0t(v_M1.clone(), v_g.clone())?, &f__t0t(v_M2.clone(), v_g.clone())?)?, &py::truediv(&v_g.clone(), &py::sub(&v_g.clone(), &Py::Float(1.0))?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("M2"), v_M2.clone()), (Py::str("nu1_deg"), py::truediv(&v_nu1.clone(), &c_DEG()?)?), (Py::str("nu2_deg"), py::truediv(&v_nu2.clone(), &c_DEG()?)?), (Py::str("p2_p1"), v_pr.clone()), (Py::str("T2_T1"), py::truediv(&f__t0t(v_M1.clone(), v_g.clone())?, &f__t0t(v_M2.clone(), v_g.clone())?)?), (Py::str("rho2_rho1"), py::pow(&v_pr.clone(), &py::truediv(&Py::Float(1.0), &v_g.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_fanno(mut v_M: Py, mut v_gamma: Py) -> R {
    let mut v_fld: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_m2: Py = Py::None;
    let mut v_tr: Py = Py::None;
    v_g = f__gam(v_gamma.clone())?;
    let _ = f__need({ let t1063 = Py::Bool(matches!(v_M.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t1063.truthy() { t1063 } else { { let t1062 = { let t1057 = Py::Float(0.05); Py::Bool({ let t1058 = v_M.clone(); if py::le(&t1057, &t1058)? { { let t1059 = Py::Float(10.0); if py::le(&t1058, &t1059)? { true } else { false } } } else { false } }) }; if !t1062.truthy() { t1062 } else { { let t1060 = v_M.clone(); Py::Bool({ let t1061 = Py::Float(1.0); if py::ne(&t1060, &t1061) { true } else { false } }) } } } } }, Py::str("the Mach number must lie between 0.05 and 10 (not 1)"))?;
    v_m2 = py::mul(&v_M.clone(), &v_M.clone())?;
    v_tr = py::truediv(&py::add(&v_g.clone(), &Py::Float(1.0))?, &py::add(&Py::Float(2.0), &py::mul(&py::sub(&v_g.clone(), &Py::Float(1.0))?, &v_m2.clone())?)?)?;
    v_fld = py::add(&py::truediv(&py::sub(&Py::Float(1.0), &v_m2.clone())?, &py::mul(&v_g.clone(), &v_m2.clone())?)?, &py::mul(&py::truediv(&py::add(&v_g.clone(), &Py::Float(1.0))?, &py::mul(&Py::Float(2.0), &v_g.clone())?)?, &py::math1("log", &py::truediv(&py::mul(&py::add(&v_g.clone(), &Py::Float(1.0))?, &v_m2.clone())?, &py::add(&Py::Float(2.0), &py::mul(&py::sub(&v_g.clone(), &Py::Float(1.0))?, &v_m2.clone())?)?)?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("fLD"), v_fld.clone()), (Py::str("p_pstar"), py::truediv(&py::math1("sqrt", &v_tr.clone())?, &v_M.clone())?), (Py::str("T_Tstar"), v_tr.clone()), (Py::str("rho_rhostar"), py::truediv(&Py::Float(1.0), &py::mul(&v_M.clone(), &py::math1("sqrt", &v_tr.clone())?)?)?), (Py::str("p0_p0star"), py::truediv(&py::pow(&py::truediv(&Py::Float(1.0), &v_tr.clone())?, &py::truediv(&py::add(&v_g.clone(), &Py::Float(1.0))?, &py::mul(&Py::Float(2.0), &py::sub(&v_g.clone(), &Py::Float(1.0))?)?)?)?, &v_M.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_flight_condition(mut v_V: Py, mut v_h: Py, mut v_L: Py) -> R {
    let mut v_T: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_rho: Py = Py::None;
    let _ = f__pos(Py::str("the true airspeed"), Py::tuple(vec![v_V.clone()]))?;
    let t1114 = (f__isa(v_h.clone())?).items()?;
    if t1114.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_T = t1114[0].clone();
    v_p = t1114[1].clone();
    v_rho = t1114[2].clone();
    v_a = py::math1("sqrt", &py::mul(&py::mul(&c_GAMMA_AIR()?, &c_R_AIR()?)?, &v_T.clone())?)?;
    v_out = py::dict_from(&Py::dict(vec![]), vec![(Py::str("M"), py::truediv(&v_V.clone(), &v_a.clone())?), (Py::str("q"), py::mul(&py::mul(&py::mul(&Py::Float(0.5), &v_rho.clone())?, &v_V.clone())?, &v_V.clone())?), (Py::str("EAS"), py::mul(&v_V.clone(), &py::math1("sqrt", &py::truediv(&v_rho.clone(), &c_RHO_SL()?)?)?)?), (Py::str("EAS_kt"), py::truediv(&py::mul(&v_V.clone(), &py::math1("sqrt", &py::truediv(&v_rho.clone(), &c_RHO_SL()?)?)?)?, &c_KT()?)?), (Py::str("a"), v_a.clone()), (Py::str("rho"), v_rho.clone())])?;
    if ({ let t1115 = v_L.clone(); Py::Bool({ let t1116 = Py::None; if !py::is(&t1115, &t1116) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("the reference length"), Py::tuple(vec![v_L.clone()]))?;
        py::setitem(&v_out.clone(), &Py::str("Re"), py::truediv(&py::mul(&py::mul(&v_rho.clone(), &v_V.clone())?, &v_L.clone())?, &f__sutherland(v_T.clone())?)?)?;
    }
    return Ok(f__out(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_geopotential_altitude(mut v_h: Py) -> R {
    let mut v_x: Py = Py::None;
    let _ = f__need({ let t1153 = Py::Bool(matches!(v_h.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t1153.truthy() { t1153 } else { { let t1150 = Py::Int(0i64); Py::Bool({ let t1151 = v_h.clone(); if py::le(&t1150, &t1151)? { { let t1152 = Py::Float(1000000.0); if py::le(&t1151, &t1152)? { true } else { false } } } else { false } }) } } }, Py::str("the altitude must lie between 0 and 1000 km"))?;
    v_x = py::truediv(&c_R0_GEO()?, &py::add(&c_R0_GEO()?, &v_h.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("H"), py::truediv(&py::mul(&c_R0_GEO()?, &v_h.clone())?, &py::add(&c_R0_GEO()?, &v_h.clone())?)?), (Py::str("g"), py::mul(&py::mul(&c_G0()?, &v_x.clone())?, &v_x.clone())?), (Py::str("dH"), py::sub(&v_h.clone(), &py::truediv(&py::mul(&c_R0_GEO()?, &v_h.clone())?, &py::add(&c_R0_GEO()?, &v_h.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_glide(mut v_dh: Py, mut v_LD: Py, mut v_V: Py) -> R {
    let mut v_gamma: Py = Py::None;
    let mut v_out: Py = Py::None;
    let _ = f__pos(Py::str("the height and the lift-to-drag ratio"), Py::tuple(vec![v_dh.clone(), v_LD.clone()]))?;
    let _ = f__need({ let t1191 = v_LD.clone(); Py::Bool({ let t1192 = Py::Int(100i64); if py::le(&t1191, &t1192)? { true } else { false } }) }, Py::str("the lift-to-drag ratio must be at most 100"))?;
    v_gamma = py::math1("atan", &py::truediv(&Py::Float(1.0), &v_LD.clone())?)?;
    v_out = py::dict_from(&Py::dict(vec![]), vec![(Py::str("range"), py::mul(&v_dh.clone(), &v_LD.clone())?), (Py::str("range_km"), py::truediv(&py::mul(&v_dh.clone(), &v_LD.clone())?, &Py::Float(1000.0))?), (Py::str("glide_angle_deg"), py::truediv(&v_gamma.clone(), &c_DEG()?)?)])?;
    if ({ let t1193 = v_V.clone(); Py::Bool({ let t1194 = Py::None; if !py::is(&t1193, &t1194) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("the airspeed"), Py::tuple(vec![v_V.clone()]))?;
        py::setitem(&v_out.clone(), &Py::str("sink_rate"), py::mul(&v_V.clone(), &py::math1("sin", &v_gamma.clone())?)?)?;
        py::setitem(&v_out.clone(), &Py::str("time"), py::truediv(&v_dh.clone(), &py::mul(&v_V.clone(), &py::math1("sin", &v_gamma.clone())?)?)?)?;
    }
    return Ok(f__out(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_gravity_loss(mut v_Isp: Py, mut v_m0: Py, mut v_mf: Py, mut v_tb: Py, mut v_g: Py) -> R {
    let mut v_R: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_dvi: Py = Py::None;
    let t755 = (f__vert_in(v_Isp.clone(), v_m0.clone(), v_mf.clone(), v_tb.clone(), v_g.clone())?).items()?;
    if t755.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_c = t755[0].clone();
    v_R = t755[1].clone();
    v_dvi = py::mul(&v_c.clone(), &py::math1("log", &v_R.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("v_b"), py::sub(&v_dvi.clone(), &py::mul(&v_g.clone(), &v_tb.clone())?)?), (Py::str("dv_ideal"), v_dvi.clone()), (Py::str("loss"), py::mul(&v_g.clone(), &v_tb.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ideal_exhaust_velocity(mut v_Tc: Py, mut v_Mw: Py, mut v_gamma: Py, mut v_pc: Py, mut v_pe: Py) -> R {
    let mut v_Me: Py = Py::None;
    let mut v_R: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_ve: Py = Py::None;
    let mut v_x: Py = Py::None;
    let t1240 = (f__rkt_in(v_pc.clone(), v_Tc.clone(), v_gamma.clone(), v_Mw.clone())?).items()?;
    if t1240.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_g = t1240[0].clone();
    v_R = t1240[1].clone();
    let _ = f__need({ let t1244 = Py::Bool(matches!(v_pe.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t1244.truthy() { t1244 } else { { let t1241 = Py::Int(0i64); Py::Bool({ let t1242 = v_pe.clone(); if py::le(&t1241, &t1242)? { { let t1243 = v_pc.clone(); if py::lt(&t1242, &t1243)? { true } else { false } } } else { false } }) } } }, Py::str("the exit pressure must be at least 0 and below the chamber pressure"))?;
    v_x = py::truediv(&v_pe.clone(), &v_pc.clone())?;
    v_ve = py::math1("sqrt", &py::mul(&py::mul(&py::mul(&py::truediv(&py::mul(&Py::Float(2.0), &v_g.clone())?, &py::sub(&v_g.clone(), &Py::Float(1.0))?)?, &v_R.clone())?, &v_Tc.clone())?, &py::sub(&Py::Float(1.0), &py::pow(&v_x.clone(), &py::truediv(&py::sub(&v_g.clone(), &Py::Float(1.0))?, &v_g.clone())?)?)?)?)?;
    v_Me = (if ({ let t1245 = v_x.clone(); Py::Bool({ let t1246 = Py::Int(0i64); if py::gt(&t1245, &t1246)? { true } else { false } }) }).truthy() { py::math1("sqrt", &py::mul(&py::truediv(&Py::Float(2.0), &py::sub(&v_g.clone(), &Py::Float(1.0))?)?, &py::sub(&py::pow(&v_x.clone(), &py::truediv(&py::neg(&py::sub(&v_g.clone(), &Py::Float(1.0))?)?, &v_g.clone())?)?, &Py::Float(1.0))?)?)? } else { Py::Float(f64::INFINITY) });
    let _ = f__need(py::math1("isfinite", &v_Me.clone())?, Py::str("an exit pressure of zero gives an infinite exit Mach number; use a positive exit pressure"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("ve"), v_ve.clone()), (Py::str("Isp"), py::truediv(&v_ve.clone(), &c_G0()?)?), (Py::str("Me"), v_Me.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_identical_stages(mut v_n: Py, mut v_dv: Py, mut v_Isp: Py, mut v_eps: Py) -> R {
    let mut v_R: Py = Py::None;
    let mut v_lam: Py = Py::None;
    let mut v_pi: Py = Py::None;
    let _ = f__isp_in(v_Isp.clone())?;
    let _ = f__pos(Py::str("the velocity change"), Py::tuple(vec![v_dv.clone()]))?;
    let _ = f__need({ let t1291 = Py::Bool(matches!(v_n.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t1291.truthy() { t1291 } else { { let t1288 = Py::Int(1i64); Py::Bool({ let t1289 = v_n.clone(); if py::le(&t1288, &t1289)? { { let t1290 = Py::Int(6i64); if py::le(&t1289, &t1290)? { true } else { false } } } else { false } }) } } }, Py::str("the number of stages must be 1 to 6"))?;
    let _ = f__need({ let t1295 = Py::Bool(matches!(v_eps.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t1295.truthy() { t1295 } else { { let t1292 = Py::Float(0.01); Py::Bool({ let t1293 = v_eps.clone(); if py::le(&t1292, &t1293)? { { let t1294 = Py::Float(0.5); if py::le(&t1293, &t1294)? { true } else { false } } } else { false } }) } } }, Py::str("the structural coefficient must lie between 0.01 and 0.5"))?;
    v_R = py::math1("exp", &py::truediv(&v_dv.clone(), &py::mul(&py::mul(&v_n.clone(), &v_Isp.clone())?, &c_G0()?)?)?)?;
    let _ = f__need({ let t1296 = py::mul(&v_eps.clone(), &v_R.clone())?; Py::Bool({ let t1297 = py::sub(&Py::Float(1.0), &Py::Float(1e-06))?; if py::lt(&t1296, &t1297)? { true } else { false } }) }, Py::str("the velocity change is not reachable with this structural coefficient (eps R >= 1)"))?;
    v_lam = py::truediv(&py::sub(&Py::Float(1.0), &py::mul(&v_eps.clone(), &v_R.clone())?)?, &py::sub(&v_R.clone(), &Py::Float(1.0))?)?;
    v_pi = py::pow(&py::truediv(&v_lam.clone(), &py::add(&Py::Float(1.0), &v_lam.clone())?)?, &v_n.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("lam"), v_lam.clone()), (Py::str("payload_fraction"), v_pi.clone()), (Py::str("R"), v_R.clone()), (Py::str("gross_per_payload"), py::truediv(&Py::Float(1.0), &v_pi.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_induced_drag(mut v_W: Py, mut v_S: Py, mut v_V: Py, mut v_h: Py, mut v_AR: Py, mut v_e: Py) -> R {
    let mut v_CDi: Py = Py::None;
    let mut v_CL: Py = Py::None;
    let mut v_Di: Py = Py::None;
    let mut v_K: Py = Py::None;
    let mut v_T: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_rho: Py = Py::None;
    let _ = f__ws_in(v_W.clone(), v_S.clone())?;
    let _ = f__pos(Py::str("the true airspeed"), Py::tuple(vec![v_V.clone()]))?;
    v_K = f__K(v_AR.clone(), v_e.clone())?;
    let t1356 = (f__isa(v_h.clone())?).items()?;
    if t1356.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_T = t1356[0].clone();
    v_p = t1356[1].clone();
    v_rho = t1356[2].clone();
    v_q = py::mul(&py::mul(&py::mul(&Py::Float(0.5), &v_rho.clone())?, &v_V.clone())?, &v_V.clone())?;
    v_CL = py::truediv(&v_W.clone(), &py::mul(&v_q.clone(), &v_S.clone())?)?;
    v_CDi = py::mul(&py::mul(&v_K.clone(), &v_CL.clone())?, &v_CL.clone())?;
    v_Di = py::mul(&py::mul(&v_q.clone(), &v_S.clone())?, &v_CDi.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("CL"), v_CL.clone()), (Py::str("CDi"), v_CDi.clone()), (Py::str("Di"), v_Di.clone()), (Py::str("P_induced"), py::mul(&v_Di.clone(), &v_V.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_isa_properties(mut v_h: Py) -> R {
    let mut v_T: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_rho: Py = Py::None;
    let t1402 = (f__isa(v_h.clone())?).items()?;
    if t1402.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_T = t1402[0].clone();
    v_p = t1402[1].clone();
    v_rho = t1402[2].clone();
    return Ok(f__out(Py::dict(vec![(Py::str("T"), v_T.clone()), (Py::str("p"), v_p.clone()), (Py::str("rho"), v_rho.clone()), (Py::str("a"), py::math1("sqrt", &py::mul(&py::mul(&c_GAMMA_AIR()?, &c_R_AIR()?)?, &v_T.clone())?)?), (Py::str("mu"), f__sutherland(v_T.clone())?), (Py::str("sigma"), py::truediv(&v_rho.clone(), &c_RHO_SL()?)?), (Py::str("delta"), py::truediv(&v_p.clone(), &c_P_SL()?)?), (Py::str("theta"), py::truediv(&v_T.clone(), &c_T_SL()?)?), (Py::str("H"), f__geopot(v_h.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_isentropic_flow(mut v_M: Py, mut v_gamma: Py) -> R {
    let mut v_g: Py = Py::None;
    let mut v_t: Py = Py::None;
    v_g = f__gam(v_gamma.clone())?;
    let _ = f__mach_in(v_M.clone(), Py::Float(0.01), Py::Float(10.0))?;
    v_t = f__t0t(v_M.clone(), v_g.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("T0_T"), v_t.clone()), (Py::str("p0_p"), py::pow(&v_t.clone(), &py::truediv(&v_g.clone(), &py::sub(&v_g.clone(), &Py::Float(1.0))?)?)?), (Py::str("rho0_rho"), py::pow(&v_t.clone(), &py::truediv(&Py::Float(1.0), &py::sub(&v_g.clone(), &Py::Float(1.0))?)?)?), (Py::str("A_Astar"), f__area(v_M.clone(), v_g.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_jet_fuel_for_range(mut v_R: Py, mut v_V: Py, mut v_c: Py, mut v_LD: Py, mut v_W0: Py) -> R {
    let mut v_W1: Py = Py::None;
    let mut v_Wf: Py = Py::None;
    let mut v_x: Py = Py::None;
    let _ = f__pos(Py::str("the range and the speed"), Py::tuple(vec![v_R.clone(), v_V.clone()]))?;
    let _ = f__tsfc_in(v_c.clone())?;
    let _ = f__ld_in(v_LD.clone())?;
    let _ = f__pos(Py::str("the initial weight"), Py::tuple(vec![v_W0.clone()]))?;
    v_x = py::truediv(&py::mul(&py::mul(&v_R.clone(), &c_G0()?)?, &v_c.clone())?, &py::mul(&v_V.clone(), &v_LD.clone())?)?;
    v_W1 = py::mul(&v_W0.clone(), &py::math1("exp", &py::neg(&v_x.clone())?)?)?;
    let _ = f__need({ let t1582 = v_W1.clone(); Py::Bool({ let t1583 = py::mul(&Py::Float(0.2), &v_W0.clone())?; if py::ge(&t1582, &t1583)? { true } else { false } }) }, Py::str("the fuel needed would exceed 80% of the initial weight"))?;
    v_Wf = py::mul(&py::neg(&v_W0.clone())?, &py::math1("expm1", &py::neg(&v_x.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("W1"), v_W1.clone()), (Py::str("Wf"), v_Wf.clone()), (Py::str("mf"), py::truediv(&v_Wf.clone(), &c_G0()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_jet_range_constant_altitude(mut v_W0: Py, mut v_W1: Py, mut v_S: Py, mut v_h: Py, mut v_CL: Py, mut v_CD: Py, mut v_c: Py) -> R {
    let mut v_R: Py = Py::None;
    let mut v_rho: Py = Py::None;
    let mut v_t: Py = Py::None;
    let _ = f__w01(v_W0.clone(), v_W1.clone())?;
    let _ = f__pos(Py::str("the wing area"), Py::tuple(vec![v_S.clone()]))?;
    let _ = f__clcd_in(v_CL.clone(), v_CD.clone())?;
    let _ = f__tsfc_in(v_c.clone())?;
    v_rho = f__rho_at(v_h.clone())?;
    v_R = py::mul(&py::truediv(&py::mul(&py::mul(&py::truediv(&Py::Float(2.0), &py::mul(&c_G0()?, &v_c.clone())?)?, &py::math1("sqrt", &py::truediv(&Py::Float(2.0), &py::mul(&v_rho.clone(), &v_S.clone())?)?)?)?, &py::math1("sqrt", &v_CL.clone())?)?, &v_CD.clone())?, &py::sub(&py::math1("sqrt", &v_W0.clone())?, &py::math1("sqrt", &v_W1.clone())?)?)?;
    v_t = py::mul(&py::truediv(&py::truediv(&v_CL.clone(), &v_CD.clone())?, &py::mul(&c_G0()?, &v_c.clone())?)?, &py::math1("log", &py::truediv(&v_W0.clone(), &v_W1.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("R"), v_R.clone()), (Py::str("R_km"), py::truediv(&v_R.clone(), &Py::Float(1000.0))?), (Py::str("t"), v_t.clone()), (Py::str("t_h"), py::truediv(&v_t.clone(), &Py::Float(3600.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_lambert(mut v_r1: Py, mut v_r2: Py, mut v_tof: Py, mut v_direction: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_F: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_cz: Py = Py::None;
    let mut v_dth: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_gd: Py = Py::None;
    let mut v_hi: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let mut v_n1: Py = Py::None;
    let mut v_n2: Py = Py::None;
    let mut v_smu: Py = Py::None;
    let mut v_snap: Py = Py::None;
    let mut v_v1: Py = Py::None;
    let mut v_v2: Py = Py::None;
    let mut v_y: Py = Py::None;
    let mut v_yz: Py = Py::None;
    let mut v_z: Py = Py::None;
    let _ = f__need({ let t1687 = { let t1683 = py::len(&v_r1.clone())?; Py::Bool({ let t1684 = Py::Int(3i64); if py::eq(&t1683, &t1684) { true } else { false } }) }; if !t1687.truthy() { t1687 } else { { let t1685 = py::len(&v_r2.clone())?; Py::Bool({ let t1686 = Py::Int(3i64); if py::eq(&t1685, &t1686) { true } else { false } }) } } }, Py::str("the positions must be vectors of three components (km)"))?;
    let _ = f__pos(Py::str("the time of flight"), Py::tuple(vec![v_tof.clone()]))?;
    let _ = f__need({ let t1693 = v_direction.clone(); Py::Bool({ let t1694 = Py::tuple(vec![Py::str("prograde"), Py::str("retrograde")]); if py::contains(&t1694, &t1693)? { true } else { false } }) }, Py::str("the direction must be prograde or retrograde"))?;
    let t1697 = (Py::tuple(vec![py::math1("sqrt", &py::sum(&{ let mut out = vec![]; for t1695 in py::iter(&(v_r1.clone()))? { let mut v_x: Py; 
v_x = t1695;
out.push(py::mul(&v_x.clone(), &v_x.clone())?); } Py::list(out) }, None)?)?, py::math1("sqrt", &py::sum(&{ let mut out = vec![]; for t1696 in py::iter(&(v_r2.clone()))? { let mut v_x: Py; 
v_x = t1696;
out.push(py::mul(&v_x.clone(), &v_x.clone())?); } Py::list(out) }, None)?)?])).items()?;
    if t1697.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n1 = t1697[0].clone();
    v_n2 = t1697[1].clone();
    let _ = f__need({ let t1702 = { let t1698 = v_n1.clone(); Py::Bool({ let t1699 = Py::Int(0i64); if py::gt(&t1698, &t1699)? { true } else { false } }) }; if !t1702.truthy() { t1702 } else { { let t1700 = v_n2.clone(); Py::Bool({ let t1701 = Py::Int(0i64); if py::gt(&t1700, &t1701)? { true } else { false } }) } } }, Py::str("the positions must not be at the centre of the Earth"))?;
    v_cz = py::sub(&py::mul(&py::getitem(&v_r1.clone(), &Py::Int(0i64))?, &py::getitem(&v_r2.clone(), &Py::Int(1i64))?)?, &py::mul(&py::getitem(&v_r1.clone(), &Py::Int(1i64))?, &py::getitem(&v_r2.clone(), &Py::Int(0i64))?)?)?;
    v_c = py::minmax(&[py::neg(&Py::Float(1.0))?, py::minmax(&[Py::Float(1.0), py::truediv(&py::sum(&{ let mut out = vec![]; for t1703 in py::iter(&(py::zip(&[v_r1.clone(), v_r2.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t1704 = (t1703).items()?;
if t1704.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t1704[0].clone();
v_b = t1704[1].clone();
out.push(py::mul(&v_a.clone(), &v_b.clone())?); } Py::list(out) }, None)?, &py::mul(&v_n1.clone(), &v_n2.clone())?)?], false)?], true)?;
    v_dth = py::math1("acos", &v_c.clone())?;
    if ({ let t1705 = { let t1707 = v_direction.clone(); Py::Bool({ let t1708 = Py::str("prograde"); if py::eq(&t1707, &t1708) { true } else { false } }) }; Py::Bool({ let t1706 = { let t1709 = v_cz.clone(); Py::Bool({ let t1710 = Py::Int(0i64); if py::ge(&t1709, &t1710)? { true } else { false } }) }; if py::ne(&t1705, &t1706) { true } else { false } }) }).truthy() {
        v_dth = py::sub(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_dth.clone())?;
    }
    let _ = f__need({ let t1716 = { let t1711 = py::math1("radians", &Py::Int(2i64))?; Py::Bool({ let t1712 = v_dth.clone(); if py::le(&t1711, &t1712)? { { let t1713 = py::math1("radians", &Py::Int(358i64))?; if py::le(&t1712, &t1713)? { true } else { false } } } else { false } }) }; if !t1716.truthy() { t1716 } else { { let t1714 = py::abs(&py::sub(&v_dth.clone(), &Py::Float(std::f64::consts::PI))?)?; Py::Bool({ let t1715 = py::math1("radians", &Py::Int(2i64))?; if py::ge(&t1714, &t1715)? { true } else { false } }) } } }, Py::str("the transfer angle must be between 2 and 358 degrees and not within 2 degrees of 180 (the transfer plane is undefined)"))?;
    v_A = py::mul(&py::math1("sin", &v_dth.clone())?, &py::math1("sqrt", &py::truediv(&py::mul(&v_n1.clone(), &v_n2.clone())?, &py::sub(&Py::Int(1i64), &py::math1("cos", &v_dth.clone())?)?)?)?)?;
    v_smu = py::math1("sqrt", &c__MU_KM()?)?;
    v_y = { let v_A = v_A.clone(); let v_c = v_c.clone(); let v_cz = v_cz.clone(); let v_direction = v_direction.clone(); let v_dth = v_dth.clone(); let v_n1 = v_n1.clone(); let v_n2 = v_n2.clone(); let v_r1 = v_r1.clone(); let v_r2 = v_r2.clone(); let v_smu = v_smu.clone(); let v_tof = v_tof.clone(); let v_y = v_y.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_z: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_C: Py = Py::None;
            let mut v_S: Py = Py::None;
            let t1721 = (f__stumpff(v_z.clone())?).items()?;
            if t1721.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_C = t1721[0].clone();
            v_S = t1721[1].clone();
            return Ok(py::add(&py::add(&v_n1.clone(), &v_n2.clone())?, &py::truediv(&py::mul(&v_A.clone(), &py::sub(&py::mul(&v_z.clone(), &v_S.clone())?, &Py::Int(1i64))?)?, &py::math1("sqrt", &v_C.clone())?)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_F = { let v_A = v_A.clone(); let v_F = v_F.clone(); let v_c = v_c.clone(); let v_cz = v_cz.clone(); let v_direction = v_direction.clone(); let v_dth = v_dth.clone(); let v_n1 = v_n1.clone(); let v_n2 = v_n2.clone(); let v_r1 = v_r1.clone(); let v_r2 = v_r2.clone(); let v_smu = v_smu.clone(); let v_tof = v_tof.clone(); let v_y = v_y.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_z: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_C: Py = Py::None;
            let mut v_S: Py = Py::None;
            let mut v_yz: Py = Py::None;
            let t1722 = (f__stumpff(v_z.clone())?).items()?;
            if t1722.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_C = t1722[0].clone();
            v_S = t1722[1].clone();
            v_yz = py::call(&v_y.clone(), &[v_z.clone()])?;
            if ({ let t1723 = v_yz.clone(); Py::Bool({ let t1724 = Py::Int(0i64); if py::le(&t1723, &t1724)? { true } else { false } }) }).truthy() {
                return Ok(py::sub(&py::mul(&py::neg(&v_smu.clone())?, &v_tof.clone())?, &Py::Float(1.0))?);
            }
            return Ok(py::sub(&py::add(&py::mul(&py::pow(&py::truediv(&v_yz.clone(), &v_C.clone())?, &Py::Float(1.5))?, &v_S.clone())?, &py::mul(&v_A.clone(), &py::math1("sqrt", &v_yz.clone())?)?)?, &py::mul(&v_smu.clone(), &v_tof.clone())?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_hi = py::pow(&py::sub(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &Py::Float(0.0001))?, &Py::Int(2i64))?;
    let _ = f__need({ let t1725 = py::call(&v_F.clone(), &[v_hi.clone()])?; Py::Bool({ let t1726 = Py::Int(0i64); if py::gt(&t1725, &t1726)? { true } else { false } }) }, Py::str("the time of flight is too long for a zero-revolution transfer"))?;
    v_lo = py::neg(&Py::Float(4.0))?;
    while ({ let t1729 = py::call(&v_F.clone(), &[v_lo.clone()])?; Py::Bool({ let t1730 = Py::Int(0i64); if py::gt(&t1729, &t1730)? { true } else { false } }) }).truthy() {
        v_lo = py::mul(&v_lo, &Py::Int(2i64))?;
        let _ = f__need({ let t1727 = v_lo.clone(); Py::Bool({ let t1728 = py::neg(&Py::Float(10000000.0))?; if py::gt(&t1727, &t1728)? { true } else { false } }) }, Py::str("no zero-revolution solution found"))?;
    }
    v_z = f__bisect(v_F.clone(), v_lo.clone(), v_hi.clone(), Py::Int(400i64))?;
    v_yz = py::call(&v_y.clone(), &[v_z.clone()])?;
    let _ = f__need({ let t1756 = v_yz.clone(); Py::Bool({ let t1757 = Py::Int(0i64); if py::gt(&t1756, &t1757)? { true } else { false } }) }, Py::str("no zero-revolution solution found"))?;
    v_f = py::sub(&Py::Int(1i64), &py::truediv(&v_yz.clone(), &v_n1.clone())?)?;
    v_g = py::mul(&v_A.clone(), &py::math1("sqrt", &py::truediv(&v_yz.clone(), &c__MU_KM()?)?)?)?;
    v_gd = py::sub(&Py::Int(1i64), &py::truediv(&v_yz.clone(), &v_n2.clone())?)?;
    v_v1 = { let mut out = vec![]; for t1758 in py::iter(&(py::zip(&[v_r1.clone(), v_r2.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t1759 = (t1758).items()?;
if t1759.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t1759[0].clone();
v_b = t1759[1].clone();
out.push(py::truediv(&py::sub(&v_b.clone(), &py::mul(&v_f.clone(), &v_a.clone())?)?, &v_g.clone())?); } Py::list(out) };
    v_v2 = { let mut out = vec![]; for t1760 in py::iter(&(py::zip(&[v_r1.clone(), v_r2.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t1761 = (t1760).items()?;
if t1761.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t1761[0].clone();
v_b = t1761[1].clone();
out.push(py::truediv(&py::sub(&py::mul(&v_gd.clone(), &v_b.clone())?, &v_a.clone())?, &v_g.clone())?); } Py::list(out) };
    v_snap = { let v_A = v_A.clone(); let v_F = v_F.clone(); let v_c = v_c.clone(); let v_cz = v_cz.clone(); let v_direction = v_direction.clone(); let v_dth = v_dth.clone(); let v_f = v_f.clone(); let v_g = v_g.clone(); let v_gd = v_gd.clone(); let v_hi = v_hi.clone(); let v_lo = v_lo.clone(); let v_n1 = v_n1.clone(); let v_n2 = v_n2.clone(); let v_r1 = v_r1.clone(); let v_r2 = v_r2.clone(); let v_smu = v_smu.clone(); let v_tof = v_tof.clone(); let v_v1 = v_v1.clone(); let v_v2 = v_v2.clone(); let v_y = v_y.clone(); let v_yz = v_yz.clone(); let v_z = v_z.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_v: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok({ let mut out = vec![]; for t1762 in py::iter(&(v_v.clone()))? { let mut v_x: Py; 
v_x = t1762;
out.push((if ({ let t1763 = py::abs(&v_x.clone())?; Py::Bool({ let t1764 = py::mul(&Py::Float(1e-13), &py::minmax(&[{ let mut out = vec![]; for t1765 in py::iter(&(v_v.clone()))? { let mut v_w: Py; 
v_w = t1765;
out.push(py::abs(&v_w.clone())?); } Py::list(out) }], true)?)?; if py::le(&t1763, &t1764)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { v_x.clone() })); } Py::list(out) });
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    return Ok(f__out(Py::dict(vec![(Py::str("v1"), py::call(&v_snap.clone(), &[v_v1.clone()])?), (Py::str("v2"), py::call(&v_snap.clone(), &[v_v2.clone()])?), (Py::str("dtheta_deg"), py::truediv(&v_dth.clone(), &c_DEG()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_landing_roll(mut v_W: Py, mut v_S: Py, mut v_h: Py, mut v_VTD: Py, mut v_mu: Py, mut v_CL: Py, mut v_CD: Py, mut v_Tr: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_B = f__roll_terms(v_W.clone(), v_S.clone(), v_h.clone(), v_mu.clone(), v_CL.clone(), v_CD.clone())?;
    let _ = f__pos(Py::str("the touchdown speed"), Py::tuple(vec![v_VTD.clone()]))?;
    let _ = f__need({ let t1830 = Py::Bool(matches!(v_Tr.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t1830.truthy() { t1830 } else { { let t1828 = v_Tr.clone(); Py::Bool({ let t1829 = Py::Int(0i64); if py::ge(&t1828, &t1829)? { true } else { false } }) } } }, Py::str("the reverse thrust must not be negative"))?;
    v_A = py::mul(&c_G0()?, &py::add(&py::truediv(&v_Tr.clone(), &v_W.clone())?, &v_mu.clone())?)?;
    let _ = f__need({ let t1835 = { let t1831 = v_A.clone(); Py::Bool({ let t1832 = Py::Int(0i64); if py::gt(&t1831, &t1832)? { true } else { false } }) }; if !t1835.truthy() { t1835 } else { { let t1833 = py::add(&v_A.clone(), &py::mul(&py::mul(&v_B.clone(), &v_VTD.clone())?, &v_VTD.clone())?)?; Py::Bool({ let t1834 = Py::Int(0i64); if py::gt(&t1833, &t1834)? { true } else { false } }) } } }, Py::str("the aircraft would not decelerate (no braking, reverse thrust or drag)"))?;
    v_x = py::truediv(&py::mul(&py::mul(&v_B.clone(), &v_VTD.clone())?, &v_VTD.clone())?, &v_A.clone())?;
    v_s = (if ({ let t1836 = py::abs(&v_x.clone())?; Py::Bool({ let t1837 = Py::Float(1e-08); if py::gt(&t1836, &t1837)? { true } else { false } }) }).truthy() { py::truediv(&py::math1("log1p", &v_x.clone())?, &py::mul(&Py::Int(2i64), &v_B.clone())?)? } else { py::mul(&py::truediv(&py::mul(&v_VTD.clone(), &v_VTD.clone())?, &py::mul(&Py::Int(2i64), &v_A.clone())?)?, &py::add(&py::sub(&Py::Int(1i64), &py::truediv(&v_x.clone(), &Py::Int(2i64))?)?, &py::truediv(&py::mul(&v_x.clone(), &v_x.clone())?, &Py::Int(3i64))?)?)? });
    if ({ let t1838 = py::abs(&v_x.clone())?; Py::Bool({ let t1839 = Py::Float(1e-06); if py::lt(&t1838, &t1839)? { true } else { false } }) }).truthy() {
        v_t = py::mul(&py::truediv(&v_VTD.clone(), &v_A.clone())?, &py::add(&py::sub(&Py::Int(1i64), &py::truediv(&v_x.clone(), &Py::Int(3i64))?)?, &py::truediv(&py::mul(&v_x.clone(), &v_x.clone())?, &Py::Int(5i64))?)?)?;
    } else {
        if ({ let t1840 = v_B.clone(); Py::Bool({ let t1841 = Py::Int(0i64); if py::gt(&t1840, &t1841)? { true } else { false } }) }).truthy() {
            v_t = py::truediv(&py::math1("atan", &py::math1("sqrt", &v_x.clone())?)?, &py::math1("sqrt", &py::mul(&v_A.clone(), &v_B.clone())?)?)?;
        } else {
            v_t = py::truediv(&py::math1("atanh", &py::math1("sqrt", &py::neg(&v_x.clone())?)?)?, &py::math1("sqrt", &py::mul(&py::neg(&v_A.clone())?, &v_B.clone())?)?)?;
        }
    }
    return Ok(f__out(Py::dict(vec![(Py::str("s"), v_s.clone()), (Py::str("t"), v_t.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_level_flight_cl(mut v_W: Py, mut v_S: Py, mut v_V: Py, mut v_h: Py) -> R {
    let mut v_T: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_rho: Py = Py::None;
    let _ = f__ws_in(v_W.clone(), v_S.clone())?;
    let _ = f__pos(Py::str("the true airspeed"), Py::tuple(vec![v_V.clone()]))?;
    let t1892 = (f__isa(v_h.clone())?).items()?;
    if t1892.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_T = t1892[0].clone();
    v_p = t1892[1].clone();
    v_rho = t1892[2].clone();
    v_q = py::mul(&py::mul(&py::mul(&Py::Float(0.5), &v_rho.clone())?, &v_V.clone())?, &v_V.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("CL"), py::truediv(&v_W.clone(), &py::mul(&v_q.clone(), &v_S.clone())?)?), (Py::str("q"), v_q.clone()), (Py::str("wing_loading"), py::truediv(&v_W.clone(), &v_S.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_level_turn_bank(mut v_V: Py, mut v_phi: Py, mut v_g: Py) -> R {
    let _ = f__pos(Py::str("the speed and gravity"), Py::tuple(vec![v_V.clone(), v_g.clone()]))?;
    let _ = f__need({ let t1933 = Py::Bool(matches!(v_phi.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t1933.truthy() { t1933 } else { { let t1930 = py::mul(&Py::Int(1i64), &c_DEG()?)?; Py::Bool({ let t1931 = v_phi.clone(); if py::le(&t1930, &t1931)? { { let t1932 = py::mul(&Py::Int(85i64), &c_DEG()?)?; if py::le(&t1931, &t1932)? { true } else { false } } } else { false } }) } } }, Py::str("the bank angle must lie between 1 and 85 degrees"))?;
    return Ok(f__turn_out(v_V.clone(), py::truediv(&Py::Float(1.0), &py::math1("cos", &v_phi.clone())?)?, v_g.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_level_turn_load_factor(mut v_V: Py, mut v_n: Py, mut v_g: Py) -> R {
    let _ = f__pos(Py::str("the speed and gravity"), Py::tuple(vec![v_V.clone(), v_g.clone()]))?;
    let _ = f__need({ let t1974 = Py::Bool(matches!(v_n.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t1974.truthy() { t1974 } else { { let t1971 = Py::Float(1.0002); Py::Bool({ let t1972 = v_n.clone(); if py::le(&t1971, &t1972)? { { let t1973 = Py::Float(11.5); if py::le(&t1972, &t1973)? { true } else { false } } } else { false } }) } } }, Py::str("the load factor must lie between 1.0002 and 11.5"))?;
    return Ok(f__turn_out(v_V.clone(), v_n.clone(), v_g.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_lift_drag_forces(mut v_V: Py, mut v_S: Py, mut v_CL: Py, mut v_CD0: Py, mut v_AR: Py, mut v_e: Py, mut v_rho: Py, mut v_h: Py) -> R {
    let mut v_CDi: Py = Py::None;
    let mut v_D0: Py = Py::None;
    let mut v_Di: Py = Py::None;
    let mut v_K: Py = Py::None;
    let mut v_dens: Py = Py::None;
    let mut v_q: Py = Py::None;
    let _ = f__pos(Py::str("the airspeed and the wing area"), Py::tuple(vec![v_V.clone(), v_S.clone()]))?;
    let _ = f__need({ let t2015 = Py::Bool(matches!(v_CL.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t2015.truthy() { t2015 } else { { let t2012 = Py::Int(0i64); Py::Bool({ let t2013 = v_CL.clone(); if py::lt(&t2012, &t2013)? { { let t2014 = Py::Float(3.0); if py::le(&t2013, &t2014)? { true } else { false } } } else { false } }) } } }, Py::str("the lift coefficient must lie in (0, 3]"))?;
    let _ = f__cd0(v_CD0.clone())?;
    v_K = f__K(v_AR.clone(), v_e.clone())?;
    let _ = f__need({ let t2028 = { let t2030 = v_rho.clone(); Py::Bool({ let t2031 = Py::None; if py::is(&t2030, &t2031) { true } else { false } }) }; Py::Bool({ let t2029 = { let t2032 = v_h.clone(); Py::Bool({ let t2033 = Py::None; if py::is(&t2032, &t2033) { true } else { false } }) }; if py::ne(&t2028, &t2029) { true } else { false } }) }, Py::str("give the air density or the altitude (one of them)"))?;
    if ({ let t2034 = v_rho.clone(); Py::Bool({ let t2035 = Py::None; if !py::is(&t2034, &t2035) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("the air density"), Py::tuple(vec![v_rho.clone()]))?;
        let _ = f__need({ let t2036 = v_rho.clone(); Py::Bool({ let t2037 = Py::Float(2.0); if py::le(&t2036, &t2037)? { true } else { false } }) }, Py::str("the air density must be at most 2 kg/m^3"))?;
        v_dens = v_rho.clone();
    } else {
        v_dens = py::getitem(&f__isa(v_h.clone())?, &Py::Int(2i64))?;
    }
    v_q = py::mul(&py::mul(&py::mul(&Py::Float(0.5), &v_dens.clone())?, &v_V.clone())?, &v_V.clone())?;
    v_CDi = py::mul(&py::mul(&v_K.clone(), &v_CL.clone())?, &v_CL.clone())?;
    v_D0 = py::mul(&py::mul(&v_q.clone(), &v_S.clone())?, &v_CD0.clone())?;
    v_Di = py::mul(&py::mul(&v_q.clone(), &v_S.clone())?, &v_CDi.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("q"), v_q.clone()), (Py::str("CD"), py::add(&v_CD0.clone(), &v_CDi.clone())?), (Py::str("L"), py::mul(&py::mul(&v_q.clone(), &v_S.clone())?, &v_CL.clone())?), (Py::str("D"), py::add(&v_D0.clone(), &v_Di.clone())?), (Py::str("D0"), v_D0.clone()), (Py::str("Di"), v_Di.clone()), (Py::str("LD"), py::truediv(&v_CL.clone(), &py::add(&v_CD0.clone(), &v_CDi.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mach_from_area_ratio(mut v_ar: Py, mut v_regime: Py, mut v_gamma: Py) -> R {
    let mut v_M: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_t: Py = Py::None;
    v_g = f__gam(v_gamma.clone())?;
    let _ = f__need({ let t2090 = Py::Bool(matches!(v_ar.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t2090.truthy() { t2090 } else { { let t2087 = Py::Float(1.0); Py::Bool({ let t2088 = v_ar.clone(); if py::le(&t2087, &t2088)? { { let t2089 = Py::Float(10000.0); if py::le(&t2088, &t2089)? { true } else { false } } } else { false } }) } } }, Py::str("the area ratio A/A* must lie between 1 and 10000"))?;
    let _ = f__need({ let t2091 = v_regime.clone(); Py::Bool({ let t2092 = Py::tuple(vec![Py::str("subsonic"), Py::str("supersonic")]); if py::contains(&t2092, &t2091)? { true } else { false } }) }, Py::str("the regime must be subsonic or supersonic"))?;
    v_M = f__M_of_area(v_ar.clone(), v_g.clone(), { let t2124 = v_regime.clone(); Py::Bool({ let t2125 = Py::str("supersonic"); if py::eq(&t2124, &t2125) { true } else { false } }) })?;
    let _ = f__need({ let t2126 = Py::Float(0.0001); Py::Bool({ let t2127 = v_M.clone(); if py::le(&t2126, &t2127)? { { let t2128 = Py::Float(100.0); if py::le(&t2127, &t2128)? { true } else { false } } } else { false } }) }, Py::str("the Mach number lies outside 1e-4 to 100"))?;
    v_t = f__t0t(v_M.clone(), v_g.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("M"), v_M.clone()), (Py::str("p_p0"), py::pow(&v_t.clone(), &py::truediv(&py::neg(&v_g.clone())?, &py::sub(&v_g.clone(), &Py::Float(1.0))?)?)?), (Py::str("T_T0"), py::truediv(&Py::Float(1.0), &v_t.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mach_from_pitot(mut v_p: Py, mut v_pt: Py, mut v_gamma: Py) -> R {
    let mut v_M: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_r_sonic: Py = Py::None;
    v_g = f__gam(v_gamma.clone())?;
    let _ = f__pos(Py::str("the static and pitot pressures"), Py::tuple(vec![v_p.clone(), v_pt.clone()]))?;
    v_r = py::truediv(&v_pt.clone(), &v_p.clone())?;
    let _ = f__need({ let t2170 = v_r.clone(); Py::Bool({ let t2171 = Py::Float(1.0); if py::gt(&t2170, &t2171)? { true } else { false } }) }, Py::str("the pitot (total) pressure must exceed the static pressure"))?;
    v_r_sonic = f__p0p(Py::Float(1.0), v_g.clone())?;
    if ({ let t2172 = v_r.clone(); Py::Bool({ let t2173 = v_r_sonic.clone(); if py::le(&t2172, &t2173)? { true } else { false } }) }).truthy() {
        v_M = py::math1("sqrt", &py::mul(&py::truediv(&Py::Float(2.0), &py::sub(&v_g.clone(), &Py::Float(1.0))?)?, &py::sub(&py::pow(&v_r.clone(), &py::truediv(&py::sub(&v_g.clone(), &Py::Float(1.0))?, &v_g.clone())?)?, &Py::Float(1.0))?)?)?;
    } else {
        let _ = f__need({ let t2174 = v_r.clone(); Py::Bool({ let t2175 = f__pitot_ratio_super(Py::Float(20.0), v_g.clone())?; if py::le(&t2174, &t2175)? { true } else { false } }) }, Py::str("the pressure ratio corresponds to a Mach number above 20"))?;
        v_M = f__bisect({ let v_M = v_M.clone(); let v_g = v_g.clone(); let v_gamma = v_gamma.clone(); let v_p = v_p.clone(); let v_pt = v_pt.clone(); let v_r = v_r.clone(); let v_r_sonic = v_r_sonic.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_m: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&f__pitot_ratio_super(v_m.clone(), v_g.clone())?, &v_r.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, Py::Float(1.0), Py::Float(20.0), Py::Int(200i64))?;
    }
    return Ok(f__out(Py::dict(vec![(Py::str("M"), v_M.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mach_from_prandtl_meyer(mut v_nu: Py, mut v_gamma: Py) -> R {
    let mut v_g: Py = Py::None;
    let mut v_numax: Py = Py::None;
    v_g = f__gam(v_gamma.clone())?;
    v_numax = py::truediv(&py::mul(&py::sub(&py::math1("sqrt", &py::truediv(&py::add(&v_g.clone(), &Py::Float(1.0))?, &py::sub(&v_g.clone(), &Py::Float(1.0))?)?)?, &Py::Float(1.0))?, &Py::Float(std::f64::consts::PI))?, &Py::Int(2i64))?;
    let _ = f__need({ let t2240 = Py::Bool(matches!(v_nu.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t2240.truthy() { t2240 } else { { let t2237 = Py::Int(0i64); Py::Bool({ let t2238 = v_nu.clone(); if py::le(&t2237, &t2238)? { { let t2239 = f__nu(Py::Float(50.0), v_g.clone())?; if py::lt(&t2238, &t2239)? { true } else { false } } } else { false } }) } } }, Py::str("the Prandtl-Meyer angle must lie between 0 and nu(50)"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("M"), (if ({ let t2250 = v_nu.clone(); Py::Bool({ let t2251 = Py::Int(0i64); if py::gt(&t2250, &t2251)? { true } else { false } }) }).truthy() { f__M_of_nu(v_nu.clone(), v_g.clone())? } else { Py::Float(1.0) })), (Py::str("nu_max_deg"), py::truediv(&v_numax.clone(), &c_DEG()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mach_from_pressure_ratio(mut v_pr: Py, mut v_gamma: Py) -> R {
    let mut v_M: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_t: Py = Py::None;
    v_g = f__gam(v_gamma.clone())?;
    let _ = f__need({ let t2309 = Py::Bool(matches!(v_pr.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t2309.truthy() { t2309 } else { { let t2306 = Py::Float(1.0); Py::Bool({ let t2307 = v_pr.clone(); if py::lt(&t2306, &t2307)? { { let t2308 = Py::Float(1000000.0); if py::le(&t2307, &t2308)? { true } else { false } } } else { false } }) } } }, Py::str("the pressure ratio p0/p must lie between 1 and 1e6"))?;
    v_t = py::pow(&v_pr.clone(), &py::truediv(&py::sub(&v_g.clone(), &Py::Float(1.0))?, &v_g.clone())?)?;
    v_M = py::math1("sqrt", &py::mul(&py::truediv(&Py::Float(2.0), &py::sub(&v_g.clone(), &Py::Float(1.0))?)?, &py::sub(&v_t.clone(), &Py::Float(1.0))?)?)?;
    let _ = f__need({ let t2310 = v_M.clone(); Py::Bool({ let t2311 = Py::Float(20.0); if py::le(&t2310, &t2311)? { true } else { false } }) }, Py::str("the Mach number would exceed 20"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("M"), v_M.clone()), (Py::str("T0_T"), v_t.clone()), (Py::str("A_Astar"), f__area(v_M.clone(), v_g.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_max_climb_angle_jet(mut v_W: Py, mut v_S: Py, mut v_h: Py, mut v_T: Py, mut v_CD0: Py, mut v_AR: Py, mut v_e: Py) -> R {
    let mut v_K: Py = Py::None;
    let mut v_V: Py = Py::None;
    let mut v_dmin: Py = Py::None;
    let mut v_rho: Py = Py::None;
    let mut v_s: Py = Py::None;
    let _ = f__ws_in(v_W.clone(), v_S.clone())?;
    let _ = f__cd0(v_CD0.clone())?;
    v_K = f__K(v_AR.clone(), v_e.clone())?;
    let _ = f__pos(Py::str("the thrust"), Py::tuple(vec![v_T.clone()]))?;
    v_dmin = py::mul(&py::mul(&Py::Int(2i64), &v_W.clone())?, &py::math1("sqrt", &py::mul(&v_K.clone(), &v_CD0.clone())?)?)?;
    let _ = f__need({ let t2361 = v_dmin.clone(); Py::Bool({ let t2362 = v_T.clone(); if py::lt(&t2361, &t2362)? { { let t2363 = py::mul(&Py::Float(0.8), &v_W.clone())?; if py::lt(&t2362, &t2363)? { true } else { false } } } else { false } }) }, Py::str("the thrust must exceed the minimum drag and stay below 0.8 W"))?;
    v_rho = f__rho_at(v_h.clone())?;
    v_V = py::math1("sqrt", &py::mul(&py::truediv(&py::mul(&Py::Int(2i64), &v_W.clone())?, &py::mul(&v_rho.clone(), &v_S.clone())?)?, &py::math1("sqrt", &py::truediv(&v_K.clone(), &v_CD0.clone())?)?)?)?;
    v_s = py::truediv(&py::sub(&v_T.clone(), &v_dmin.clone())?, &v_W.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("gamma_max_deg"), py::math1("degrees", &py::math1("asin", &v_s.clone())?)?), (Py::str("V"), v_V.clone()), (Py::str("RC"), py::mul(&v_V.clone(), &v_s.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_max_deflection(mut v_M1: Py, mut v_gamma: Py) -> R {
    let mut v_M2: Py = Py::None;
    let mut v_Mn1: Py = Py::None;
    let mut v_bm: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_p0r: Py = Py::None;
    let mut v_p21: Py = Py::None;
    let mut v_r21: Py = Py::None;
    let mut v_th: Py = Py::None;
    v_g = f__gam(v_gamma.clone())?;
    let _ = f__need({ let t2416 = Py::Bool(matches!(v_M1.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t2416.truthy() { t2416 } else { { let t2413 = Py::Float(1.01); Py::Bool({ let t2414 = v_M1.clone(); if py::le(&t2413, &t2414)? { { let t2415 = Py::Float(20.0); if py::le(&t2414, &t2415)? { true } else { false } } } else { false } }) } } }, Py::str("the upstream Mach number must lie between 1.01 and 20"))?;
    v_bm = f__beta_max(v_M1.clone(), v_g.clone())?;
    let t2418 = (f__oblique_from_beta(v_M1.clone(), v_bm.clone(), v_g.clone())?).items()?;
    if t2418.len() != 6 { return Err(PyErr::Value("unpack".into())); }
    v_th = t2418[0].clone();
    v_Mn1 = t2418[1].clone();
    v_M2 = t2418[2].clone();
    v_p21 = t2418[3].clone();
    v_r21 = t2418[4].clone();
    v_p0r = t2418[5].clone();
    return Ok(f__out(Py::dict(vec![(Py::str("theta_max_deg"), py::truediv(&v_th.clone(), &c_DEG()?)?), (Py::str("beta_deg"), py::truediv(&v_bm.clone(), &c_DEG()?)?), (Py::str("M2"), v_M2.clone()), (Py::str("p2_p1"), v_p21.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_max_rc_jet(mut v_W: Py, mut v_S: Py, mut v_h: Py, mut v_T: Py, mut v_CD0: Py, mut v_AR: Py, mut v_e: Py) -> R {
    let mut v_K: Py = Py::None;
    let mut v_V: Py = Py::None;
    let mut v_rc: Py = Py::None;
    let _ = f__ws_in(v_W.clone(), v_S.clone())?;
    let _ = f__cd0(v_CD0.clone())?;
    v_K = f__K(v_AR.clone(), v_e.clone())?;
    let _ = f__pos(Py::str("the thrust"), Py::tuple(vec![v_T.clone()]))?;
    let _ = f__need({ let t2468 = v_T.clone(); Py::Bool({ let t2469 = py::mul(&py::mul(&Py::Int(2i64), &v_W.clone())?, &py::math1("sqrt", &py::mul(&v_K.clone(), &v_CD0.clone())?)?)?; if py::gt(&t2468, &t2469)? { true } else { false } }) }, Py::str("the thrust is below the minimum drag: the aircraft cannot hold level flight"))?;
    let _ = f__need({ let t2470 = v_T.clone(); Py::Bool({ let t2471 = py::mul(&Py::Float(0.8), &v_W.clone())?; if py::lt(&t2470, &t2471)? { true } else { false } }) }, Py::str("the thrust-to-weight ratio must be below 0.8 (small climb angle, lift = weight)"))?;
    let t2485 = (f__rc_jet(v_W.clone(), v_S.clone(), f__rho_at(v_h.clone())?, v_T.clone(), v_CD0.clone(), v_K.clone())?).items()?;
    if t2485.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_V = t2485[0].clone();
    v_rc = t2485[1].clone();
    return Ok(f__out(Py::dict(vec![(Py::str("RC_max"), v_rc.clone()), (Py::str("V"), v_V.clone()), (Py::str("gamma_deg"), py::math1("degrees", &py::math1("asin", &py::truediv(&v_rc.clone(), &v_V.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_max_rc_prop(mut v_W: Py, mut v_S: Py, mut v_h: Py, mut v_P: Py, mut v_eta: Py, mut v_CD0: Py, mut v_AR: Py, mut v_e: Py) -> R {
    let mut v_K: Py = Py::None;
    let mut v_V: Py = Py::None;
    let mut v_preq: Py = Py::None;
    let mut v_rc: Py = Py::None;
    let mut v_rho: Py = Py::None;
    let _ = f__ws_in(v_W.clone(), v_S.clone())?;
    let _ = f__cd0(v_CD0.clone())?;
    v_K = f__K(v_AR.clone(), v_e.clone())?;
    let _ = f__eta_in(v_eta.clone())?;
    let _ = f__pos(Py::str("the shaft power"), Py::tuple(vec![v_P.clone()]))?;
    v_rho = f__rho_at(v_h.clone())?;
    v_V = py::mul(&py::math1("sqrt", &py::truediv(&py::mul(&Py::Int(2i64), &v_W.clone())?, &py::mul(&v_rho.clone(), &v_S.clone())?)?)?, &py::pow(&py::truediv(&v_K.clone(), &py::mul(&Py::Int(3i64), &v_CD0.clone())?)?, &Py::Float(0.25))?)?;
    v_preq = py::mul(&py::add(&py::mul(&py::mul(&py::mul(&py::mul(&py::mul(&Py::Float(0.5), &v_rho.clone())?, &v_V.clone())?, &v_V.clone())?, &v_S.clone())?, &v_CD0.clone())?, &py::truediv(&py::mul(&py::mul(&v_K.clone(), &v_W.clone())?, &v_W.clone())?, &py::mul(&py::mul(&py::mul(&py::mul(&Py::Float(0.5), &v_rho.clone())?, &v_V.clone())?, &v_V.clone())?, &v_S.clone())?)?)?, &v_V.clone())?;
    v_rc = py::truediv(&py::sub(&py::mul(&v_eta.clone(), &v_P.clone())?, &v_preq.clone())?, &v_W.clone())?;
    let _ = f__need({ let t2552 = v_rc.clone(); Py::Bool({ let t2553 = Py::Int(0i64); if py::gt(&t2552, &t2553)? { true } else { false } }) }, Py::str("the available power is below the minimum power required"))?;
    let _ = f__need({ let t2554 = py::truediv(&v_rc.clone(), &v_V.clone())?; Py::Bool({ let t2555 = Py::Float(0.5); if py::lt(&t2554, &t2555)? { true } else { false } }) }, Py::str("the climb would be steeper than 30 degrees (the small-angle model does not apply)"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("RC_max"), v_rc.clone()), (Py::str("V"), v_V.clone()), (Py::str("gamma_deg"), py::math1("degrees", &py::math1("asin", &py::truediv(&v_rc.clone(), &v_V.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_min_drag_power_speeds(mut v_W: Py, mut v_S: Py, mut v_h: Py, mut v_CD0: Py, mut v_AR: Py, mut v_e: Py) -> R {
    let mut v_K: Py = Py::None;
    let mut v_drag: Py = Py::None;
    let mut v_rho: Py = Py::None;
    let mut v_vmd: Py = Py::None;
    let mut v_vmp: Py = Py::None;
    let _ = f__ws_in(v_W.clone(), v_S.clone())?;
    let _ = f__cd0(v_CD0.clone())?;
    v_K = f__K(v_AR.clone(), v_e.clone())?;
    v_rho = f__rho_at(v_h.clone())?;
    v_vmd = py::math1("sqrt", &py::mul(&py::truediv(&py::mul(&Py::Int(2i64), &v_W.clone())?, &py::mul(&v_rho.clone(), &v_S.clone())?)?, &py::math1("sqrt", &py::truediv(&v_K.clone(), &v_CD0.clone())?)?)?)?;
    v_vmp = py::truediv(&v_vmd.clone(), &py::pow(&Py::Int(3i64), &Py::Float(0.25))?)?;
    v_drag = { let v_AR = v_AR.clone(); let v_CD0 = v_CD0.clone(); let v_K = v_K.clone(); let v_S = v_S.clone(); let v_W = v_W.clone(); let v_drag = v_drag.clone(); let v_e = v_e.clone(); let v_h = v_h.clone(); let v_rho = v_rho.clone(); let v_vmd = v_vmd.clone(); let v_vmp = v_vmp.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_V: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_q: Py = Py::None;
            v_q = py::mul(&py::mul(&py::mul(&Py::Float(0.5), &v_rho.clone())?, &v_V.clone())?, &v_V.clone())?;
            return Ok(py::add(&py::mul(&py::mul(&v_q.clone(), &v_S.clone())?, &v_CD0.clone())?, &py::truediv(&py::mul(&py::mul(&v_K.clone(), &v_W.clone())?, &v_W.clone())?, &py::mul(&v_q.clone(), &v_S.clone())?)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    return Ok(f__out(Py::dict(vec![(Py::str("V_md"), v_vmd.clone()), (Py::str("D_min"), py::mul(&py::mul(&Py::Int(2i64), &v_W.clone())?, &py::math1("sqrt", &py::mul(&v_K.clone(), &v_CD0.clone())?)?)?), (Py::str("V_mp"), v_vmp.clone()), (Py::str("P_min"), py::mul(&py::call(&v_drag.clone(), &[v_vmp.clone()])?, &v_vmp.clone())?), (Py::str("V_mr"), py::mul(&v_vmd.clone(), &py::pow(&Py::Int(3i64), &Py::Float(0.25))?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_multistage_delta_v(mut v_Isp: Py, mut v_m0: Py, mut v_mf: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_dvs: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_s: Py = Py::None;
    let _ = f__need({ let t2650 = py::len(&v_Isp.clone())?; Py::Bool({ let t2651 = py::len(&v_m0.clone())?; if py::eq(&t2650, &t2651) { { let t2652 = py::len(&v_mf.clone())?; if py::eq(&t2651, &t2652) { true } else { false } } } else { false } }) }, Py::str("give the same number of stages in Isp, m0 and mf"))?;
    v_dvs = Py::list(vec![]);
    for t2653 in py::iter(&(py::enumerate(&py::zip(&[v_Isp.clone(), v_m0.clone(), v_mf.clone()])?, 0)?))? {
        let t2665 = (t2653).items()?;
        if t2665.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_i = t2665[0].clone();
        let t2666 = (t2665[1].clone()).items()?;
        if t2666.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_s = t2666[0].clone();
        v_a = t2666[1].clone();
        v_b = t2666[2].clone();
        let _ = f__isp_in(v_s.clone())?;
        let _ = f__need({ let t2658 = Py::Int(0i64); Py::Bool({ let t2659 = v_b.clone(); if py::lt(&t2658, &t2659)? { { let t2660 = v_a.clone(); if py::lt(&t2659, &t2660)? { true } else { false } } } else { false } }) }, Py::str(&["stage ".to_string(), py::format(&py::add(&v_i.clone(), &Py::Int(1i64))?, "")?, ": the burnout mass must be positive and below the ignition mass".to_string()].concat()))?;
        if ({ let t2661 = v_i.clone(); Py::Bool({ let t2662 = Py::Int(0i64); if py::gt(&t2661, &t2662)? { true } else { false } }) }).truthy() {
            let _ = f__need({ let t2663 = v_a.clone(); Py::Bool({ let t2664 = py::getitem(&v_mf.clone(), &py::sub(&v_i.clone(), &Py::Int(1i64))?)?; if py::lt(&t2663, &t2664)? { true } else { false } }) }, Py::str(&["stage ".to_string(), py::format(&py::add(&v_i.clone(), &Py::Int(1i64))?, "")?, ": its ignition mass must be below the burnout mass of stage ".to_string(), py::format(&v_i.clone(), "")?, " (a stage is dropped)".to_string()].concat()))?;
        }
        let _ = py::method(&v_dvs.clone(), "append", &[py::mul(&py::mul(&v_s.clone(), &c_G0()?)?, &py::math1("log", &py::truediv(&v_a.clone(), &v_b.clone())?)?)?])?;
    }
    return Ok(f__out(Py::dict(vec![(Py::str("dv"), py::sum(&v_dvs.clone(), None)?), (Py::str("dv_stages"), v_dvs.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_naca4_thin_airfoil(mut v_m: Py, mut v_p: Py, mut v_alpha: Py) -> R {
    let mut v_A1: Py = Py::None;
    let mut v_A2: Py = Py::None;
    let mut v_CL: Py = Py::None;
    let mut v_I0: Py = Py::None;
    let mut v_I1: Py = Py::None;
    let mut v_I2: Py = Py::None;
    let mut v_aL0: Py = Py::None;
    let mut v_cm4: Py = Py::None;
    let mut v_out: Py = Py::None;
    let _ = f__need({ let t2702 = Py::Bool(matches!(v_m.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t2702.truthy() { t2702 } else { { let t2699 = Py::Int(0i64); Py::Bool({ let t2700 = v_m.clone(); if py::le(&t2699, &t2700)? { { let t2701 = Py::Float(0.095); if py::le(&t2700, &t2701)? { true } else { false } } } else { false } }) } } }, Py::str("the maximum camber must lie between 0 and 0.095 (0 to 9.5% chord)"))?;
    let _ = f__need({ let t2709 = { let t2703 = v_m.clone(); Py::Bool({ let t2704 = Py::Int(0i64); if py::eq(&t2703, &t2704) { true } else { false } }) }; if t2709.truthy() { t2709 } else { { let t2708 = Py::Bool(matches!(v_p.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t2708.truthy() { t2708 } else { { let t2705 = Py::Float(0.05); Py::Bool({ let t2706 = v_p.clone(); if py::le(&t2705, &t2706)? { { let t2707 = Py::Float(0.95); if py::le(&t2706, &t2707)? { true } else { false } } } else { false } }) } } } } }, Py::str("the position of maximum camber must lie between 0.05 and 0.95 chord"))?;
    let _ = f__need({ let t2712 = Py::Bool(matches!(v_alpha.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t2712.truthy() { t2712 } else { { let t2710 = py::abs(&v_alpha.clone())?; Py::Bool({ let t2711 = py::mul(&Py::Int(15i64), &c_DEG()?)?; if py::le(&t2710, &t2711)? { true } else { false } }) } } }, Py::str("the angle of attack must lie within +-15 degrees (thin-airfoil theory, unstalled)"))?;
    let t2719 = (Py::tuple(vec![f__camber_int(Py::Int(0i64), v_m.clone(), v_p.clone())?, f__camber_int(Py::Int(1i64), v_m.clone(), v_p.clone())?, f__camber_int(Py::Int(2i64), v_m.clone(), v_p.clone())?])).items()?;
    if t2719.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_I0 = t2719[0].clone();
    v_I1 = t2719[1].clone();
    v_I2 = t2719[2].clone();
    v_aL0 = py::truediv(&py::neg(&py::sub(&v_I1.clone(), &v_I0.clone())?)?, &Py::Float(std::f64::consts::PI))?;
    let t2720 = (Py::tuple(vec![py::truediv(&py::mul(&Py::Int(2i64), &v_I1.clone())?, &Py::Float(std::f64::consts::PI))?, py::truediv(&py::mul(&Py::Int(2i64), &v_I2.clone())?, &Py::Float(std::f64::consts::PI))?])).items()?;
    if t2720.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_A1 = t2720[0].clone();
    v_A2 = t2720[1].clone();
    v_CL = py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &py::sub(&v_alpha.clone(), &v_aL0.clone())?)?;
    v_cm4 = py::mul(&py::truediv(&Py::Float(std::f64::consts::PI), &Py::Int(4i64))?, &py::sub(&v_A2.clone(), &v_A1.clone())?)?;
    v_out = f__out(Py::dict(vec![(Py::str("alpha_L0_deg"), py::truediv(&v_aL0.clone(), &c_DEG()?)?), (Py::str("CL"), v_CL.clone()), (Py::str("Cm_c4"), v_cm4.clone()), (Py::str("Cm_le"), py::neg(&py::add(&py::truediv(&v_CL.clone(), &Py::Int(4i64))?, &py::mul(&py::truediv(&Py::Float(std::f64::consts::PI), &Py::Int(4i64))?, &py::sub(&v_A1.clone(), &v_A2.clone())?)?)?)?)]))?;
    if ({ let t2730 = v_m.clone(); Py::Bool({ let t2731 = Py::Int(0i64); if py::eq(&t2730, &t2731) { true } else { false } }) }).truthy() {
        let t2732 = (Py::tuple(vec![Py::Float(0.0), Py::Float(0.0)])).items()?;
        if t2732.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        py::setitem(&v_out.clone(), &Py::str("alpha_L0_deg"), t2732[0].clone())?;
        py::setitem(&v_out.clone(), &Py::str("Cm_c4"), t2732[1].clone())?;
    }
    if ({ let t2733 = py::abs(&py::getitem(&v_out.clone(), &Py::str("CL"))?)?; Py::Bool({ let t2734 = Py::Float(1e-12); if py::lt(&t2733, &t2734)? { true } else { false } }) }).truthy() {
        py::setitem(&v_out.clone(), &Py::str("CL"), Py::Float(0.0))?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_neutral_point(mut v_h_ac: Py, mut v_a_w: Py, mut v_a_t: Py, mut v_St_S: Py, mut v_lt_c: Py, mut v_deps: Py, mut v_h_cg: Py, mut v_eta_t: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_hi: Py = Py::None;
    let mut v_hn: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let mut v_nm: Py = Py::None;
    let mut v_sm: Py = Py::None;
    let mut v_v: Py = Py::None;
    for t2758 in py::iter(&(Py::tuple(vec![Py::tuple(vec![Py::str("the wing aerodynamic-centre position"), v_h_ac.clone(), py::neg(&Py::Float(1.0))?, Py::Float(2.0)]), Py::tuple(vec![Py::str("the wing lift slope"), v_a_w.clone(), Py::Float(0.5), Py::Float(8.0)]), Py::tuple(vec![Py::str("the tail lift slope"), v_a_t.clone(), Py::Float(0.5), Py::Float(8.0)]), Py::tuple(vec![Py::str("the tail-to-wing area ratio"), v_St_S.clone(), Py::Float(0.01), Py::Float(1.0)]), Py::tuple(vec![Py::str("the tail arm"), v_lt_c.clone(), Py::Float(0.5), Py::Float(15.0)]), Py::tuple(vec![Py::str("the downwash gradient"), v_deps.clone(), Py::Float(0.0), Py::Float(0.9)]), Py::tuple(vec![Py::str("the centre-of-gravity position"), v_h_cg.clone(), py::neg(&Py::Float(1.0))?, Py::Float(2.0)]), Py::tuple(vec![Py::str("the tail efficiency"), v_eta_t.clone(), Py::Float(0.5), Py::Float(1.2)])])))? {
        let t2763 = (t2758).items()?;
        if t2763.len() != 4 { return Err(PyErr::Value("unpack".into())); }
        v_nm = t2763[0].clone();
        v_v = t2763[1].clone();
        v_lo = t2763[2].clone();
        v_hi = t2763[3].clone();
        let _ = f__need({ let t2762 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t2762.truthy() { t2762 } else { { let t2759 = v_lo.clone(); Py::Bool({ let t2760 = v_v.clone(); if py::le(&t2759, &t2760)? { { let t2761 = v_hi.clone(); if py::le(&t2760, &t2761)? { true } else { false } } } else { false } }) } } }, Py::str(&[py::format(&v_nm.clone(), "")?, " must lie between ".to_string(), py::format(&v_lo.clone(), "g")?, " and ".to_string(), py::format(&v_hi.clone(), "g")?].concat()))?;
    }
    v_b = py::mul(&py::mul(&py::mul(&v_eta_t.clone(), &v_St_S.clone())?, &v_a_t.clone())?, &py::sub(&Py::Int(1i64), &v_deps.clone())?)?;
    v_a = py::add(&v_a_w.clone(), &v_b.clone())?;
    v_hn = py::add(&v_h_ac.clone(), &py::truediv(&py::mul(&v_b.clone(), &v_lt_c.clone())?, &v_a.clone())?)?;
    v_sm = py::sub(&v_hn.clone(), &v_h_cg.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("h_n"), v_hn.clone()), (Py::str("SM"), v_sm.clone()), (Py::str("CL_alpha"), v_a.clone()), (Py::str("Cm_alpha"), (if ({ let t2773 = py::abs(&v_sm.clone())?; Py::Bool({ let t2774 = Py::Float(1e-12); if py::lt(&t2773, &t2774)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { py::mul(&py::neg(&v_a.clone())?, &v_sm.clone())? }))]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_nodal_regression(mut v_hp: Py, mut v_i: Py, mut v_ha: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_rate: Py = Py::None;
    let t1534 = (f__j2_elements(v_hp.clone(), v_ha.clone(), v_i.clone())?).items()?;
    if t1534.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_a = t1534[0].clone();
    v_e = t1534[1].clone();
    v_p = t1534[2].clone();
    v_n = t1534[3].clone();
    v_rate = py::mul(&py::mul(&py::mul(&py::mul(&py::neg(&Py::Float(1.5))?, &v_n.clone())?, &c_J2_E()?)?, &py::pow(&py::truediv(&c_R_E()?, &v_p.clone())?, &Py::Int(2i64))?)?, &py::math1("cos", &v_i.clone())?)?;
    if ({ let t1535 = py::abs(&py::math1("cos", &v_i.clone())?)?; Py::Bool({ let t1536 = Py::Float(1e-12); if py::lt(&t1535, &t1536)? { true } else { false } }) }).truthy() {
        v_rate = Py::Float(0.0);
    }
    return Ok(f__out(Py::dict(vec![(Py::str("raan_rate_deg_day"), py::mul(&py::truediv(&v_rate.clone(), &c_DEG()?)?, &c_DAY()?)?), (Py::str("raan_rate"), v_rate.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_normal_shock(mut v_M1: Py, mut v_gamma: Py) -> R {
    let mut v_M2: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_p0r: Py = Py::None;
    let mut v_p21: Py = Py::None;
    let mut v_r21: Py = Py::None;
    v_g = f__gam(v_gamma.clone())?;
    let _ = f__need({ let t2805 = Py::Bool(matches!(v_M1.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t2805.truthy() { t2805 } else { { let t2802 = Py::Float(1.0); Py::Bool({ let t2803 = v_M1.clone(); if py::lt(&t2802, &t2803)? { { let t2804 = Py::Float(20.0); if py::le(&t2803, &t2804)? { true } else { false } } } else { false } }) } } }, Py::str("the upstream Mach number must lie between 1 and 20"))?;
    let t2806 = (f__shock(v_M1.clone(), v_g.clone())?).items()?;
    if t2806.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_M2 = t2806[0].clone();
    v_p21 = t2806[1].clone();
    v_r21 = t2806[2].clone();
    v_p0r = t2806[3].clone();
    return Ok(f__out(Py::dict(vec![(Py::str("M2"), v_M2.clone()), (Py::str("p2_p1"), v_p21.clone()), (Py::str("T2_T1"), py::truediv(&v_p21.clone(), &v_r21.clone())?), (Py::str("rho2_rho1"), v_r21.clone()), (Py::str("p02_p01"), v_p0r.clone()), (Py::str("p02_p1"), py::mul(&v_p21.clone(), &f__p0p(v_M2.clone(), v_g.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_normal_shock_from_pressure_ratio(mut v_p21: Py, mut v_gamma: Py) -> R {
    let mut v_M1: Py = Py::None;
    let mut v_M2: Py = Py::None;
    let mut v__p: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_p0r: Py = Py::None;
    let mut v_r21: Py = Py::None;
    v_g = f__gam(v_gamma.clone())?;
    let _ = f__need({ let t2846 = Py::Bool(matches!(v_p21.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t2846.truthy() { t2846 } else { { let t2843 = Py::Float(1.0); Py::Bool({ let t2844 = v_p21.clone(); if py::lt(&t2843, &t2844)? { { let t2845 = Py::Float(500.0); if py::le(&t2844, &t2845)? { true } else { false } } } else { false } }) } } }, Py::str("the static pressure ratio p2/p1 must lie between 1 and 500"))?;
    v_M1 = py::math1("sqrt", &py::add(&py::truediv(&py::mul(&py::sub(&v_p21.clone(), &Py::Float(1.0))?, &py::add(&v_g.clone(), &Py::Float(1.0))?)?, &py::mul(&Py::Float(2.0), &v_g.clone())?)?, &Py::Float(1.0))?)?;
    let t2847 = (f__shock(v_M1.clone(), v_g.clone())?).items()?;
    if t2847.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_M2 = t2847[0].clone();
    v__p = t2847[1].clone();
    v_r21 = t2847[2].clone();
    v_p0r = t2847[3].clone();
    return Ok(f__out(Py::dict(vec![(Py::str("M1"), v_M1.clone()), (Py::str("M2"), v_M2.clone()), (Py::str("T2_T1"), py::truediv(&v_p21.clone(), &v_r21.clone())?), (Py::str("rho2_rho1"), v_r21.clone()), (Py::str("p02_p01"), v_p0r.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_nozzle_normal_shock(mut v_pc: Py, mut v_pb: Py, mut v_eps: Py, mut v_gamma: Py) -> R {
    let mut v_M1: Py = Py::None;
    let mut v_M2: Py = Py::None;
    let mut v_Me: Py = Py::None;
    let mut v_Msub: Py = Py::None;
    let mut v_Msup: Py = Py::None;
    let mut v__M2e: Py = Py::None;
    let mut v__p0r: Py = Py::None;
    let mut v__r21: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_p0r: Py = Py::None;
    let mut v_p21e: Py = Py::None;
    let mut v_p_hi: Py = Py::None;
    let mut v_p_lo: Py = Py::None;
    let mut v_target: Py = Py::None;
    v_g = f__gam(v_gamma.clone())?;
    let _ = f__pos(Py::str("the chamber and back pressures"), Py::tuple(vec![v_pc.clone(), v_pb.clone()]))?;
    let _ = f__need({ let t2892 = Py::Bool(matches!(v_eps.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t2892.truthy() { t2892 } else { { let t2889 = Py::Float(1.05); Py::Bool({ let t2890 = v_eps.clone(); if py::le(&t2889, &t2890)? { { let t2891 = Py::Float(100.0); if py::le(&t2890, &t2891)? { true } else { false } } } else { false } }) } } }, Py::str("the nozzle area ratio must lie between 1.05 and 100"))?;
    let t2924 = (Py::tuple(vec![f__M_of_area(v_eps.clone(), v_g.clone(), Py::Bool(false))?, f__M_of_area(v_eps.clone(), v_g.clone(), Py::Bool(true))?])).items()?;
    if t2924.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Msub = t2924[0].clone();
    v_Msup = t2924[1].clone();
    v_p_hi = py::truediv(&v_pc.clone(), &f__p0p(v_Msub.clone(), v_g.clone())?)?;
    let t2925 = (f__shock(v_Msup.clone(), v_g.clone())?).items()?;
    if t2925.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v__M2e = t2925[0].clone();
    v_p21e = t2925[1].clone();
    v__r21 = t2925[2].clone();
    v__p0r = t2925[3].clone();
    v_p_lo = py::mul(&py::truediv(&v_pc.clone(), &f__p0p(v_Msup.clone(), v_g.clone())?)?, &v_p21e.clone())?;
    let _ = f__need({ let t2926 = v_p_lo.clone(); Py::Bool({ let t2927 = v_pb.clone(); if py::lt(&t2926, &t2927)? { { let t2928 = v_p_hi.clone(); if py::lt(&t2927, &t2928)? { true } else { false } } } else { false } }) }, Py::str("a normal shock stands inside the nozzle only for back pressures between the exit-shock and the critical subsonic pressures"))?;
    v_target = py::mul(&py::truediv(&v_pb.clone(), &v_pc.clone())?, &v_eps.clone())?;
    v_Me = f__bisect({ let v_Msub = v_Msub.clone(); let v_Msup = v_Msup.clone(); let v__M2e = v__M2e.clone(); let v__p0r = v__p0r.clone(); let v__r21 = v__r21.clone(); let v_eps = v_eps.clone(); let v_g = v_g.clone(); let v_gamma = v_gamma.clone(); let v_p21e = v_p21e.clone(); let v_p_hi = v_p_hi.clone(); let v_p_lo = v_p_lo.clone(); let v_pb = v_pb.clone(); let v_pc = v_pc.clone(); let v_target = v_target.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_M: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&py::truediv(&f__area(v_M.clone(), v_g.clone())?, &f__p0p(v_M.clone(), v_g.clone())?)?, &v_target.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, Py::Float(1e-09), Py::Float(1.0), Py::Int(200i64))?;
    v_p0r = py::mul(&py::truediv(&v_pb.clone(), &v_pc.clone())?, &f__p0p(v_Me.clone(), v_g.clone())?)?;
    v_M1 = f__bisect({ let v_Me = v_Me.clone(); let v_Msub = v_Msub.clone(); let v_Msup = v_Msup.clone(); let v__M2e = v__M2e.clone(); let v__p0r = v__p0r.clone(); let v__r21 = v__r21.clone(); let v_eps = v_eps.clone(); let v_g = v_g.clone(); let v_gamma = v_gamma.clone(); let v_p0r = v_p0r.clone(); let v_p21e = v_p21e.clone(); let v_p_hi = v_p_hi.clone(); let v_p_lo = v_p_lo.clone(); let v_pb = v_pb.clone(); let v_pc = v_pc.clone(); let v_target = v_target.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_M: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&py::getitem(&f__shock(v_M.clone(), v_g.clone())?, &Py::Int(3i64))?, &v_p0r.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, Py::Float(1.0), v_Msup.clone(), Py::Int(200i64))?;
    v_M2 = py::getitem(&f__shock(v_M1.clone(), v_g.clone())?, &Py::Int(0i64))?;
    return Ok(f__out(Py::dict(vec![(Py::str("As_At"), f__area(v_M1.clone(), v_g.clone())?), (Py::str("M1"), v_M1.clone()), (Py::str("M2"), v_M2.clone()), (Py::str("Me"), v_Me.clone()), (Py::str("p02_p01"), v_p0r.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_oblique_from_wave_angle(mut v_M1: Py, mut v_beta: Py, mut v_gamma: Py) -> R {
    let mut v_M2: Py = Py::None;
    let mut v_Mn1: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_mu: Py = Py::None;
    let mut v_p0r: Py = Py::None;
    let mut v_p21: Py = Py::None;
    let mut v_r21: Py = Py::None;
    let mut v_th: Py = Py::None;
    v_g = f__gam(v_gamma.clone())?;
    let _ = f__need({ let t3044 = Py::Bool(matches!(v_M1.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t3044.truthy() { t3044 } else { { let t3041 = Py::Float(1.0); Py::Bool({ let t3042 = v_M1.clone(); if py::lt(&t3041, &t3042)? { { let t3043 = Py::Float(20.0); if py::le(&t3042, &t3043)? { true } else { false } } } else { false } }) } } }, Py::str("the upstream Mach number must lie between 1 and 20"))?;
    v_mu = py::math1("asin", &py::truediv(&Py::Float(1.0), &v_M1.clone())?)?;
    let _ = f__need({ let t3045 = v_mu.clone(); Py::Bool({ let t3046 = v_beta.clone(); if py::lt(&t3045, &t3046)? { { let t3047 = py::truediv(&Py::Float(std::f64::consts::PI), &Py::Int(2i64))?; if py::le(&t3046, &t3047)? { true } else { false } } } else { false } }) }, Py::str("the wave angle must lie between the Mach angle asin(1/M1) and 90 degrees"))?;
    let t3049 = (f__oblique_from_beta(v_M1.clone(), v_beta.clone(), v_g.clone())?).items()?;
    if t3049.len() != 6 { return Err(PyErr::Value("unpack".into())); }
    v_th = t3049[0].clone();
    v_Mn1 = t3049[1].clone();
    v_M2 = t3049[2].clone();
    v_p21 = t3049[3].clone();
    v_r21 = t3049[4].clone();
    v_p0r = t3049[5].clone();
    if ({ let t3050 = v_beta.clone(); Py::Bool({ let t3051 = py::truediv(&Py::Float(std::f64::consts::PI), &Py::Int(2i64))?; if py::eq(&t3050, &t3051) { true } else { false } }) }).truthy() {
        v_th = Py::Float(0.0);
    }
    return Ok(f__out(Py::dict(vec![(Py::str("theta_deg"), py::truediv(&v_th.clone(), &c_DEG()?)?), (Py::str("M2"), v_M2.clone()), (Py::str("p2_p1"), v_p21.clone()), (Py::str("T2_T1"), py::truediv(&v_p21.clone(), &v_r21.clone())?), (Py::str("rho2_rho1"), v_r21.clone()), (Py::str("p02_p01"), v_p0r.clone()), (Py::str("Mn1"), v_Mn1.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_oblique_shock(mut v_M1: Py, mut v_theta: Py, mut v_branch: Py, mut v_gamma: Py) -> R {
    let mut v_M2: Py = Py::None;
    let mut v_Mn1: Py = Py::None;
    let mut v__th: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_bm: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_mu: Py = Py::None;
    let mut v_p0r: Py = Py::None;
    let mut v_p21: Py = Py::None;
    let mut v_r21: Py = Py::None;
    let mut v_tmax: Py = Py::None;
    v_g = f__gam(v_gamma.clone())?;
    let _ = f__need({ let t2968 = Py::Bool(matches!(v_M1.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t2968.truthy() { t2968 } else { { let t2965 = Py::Float(1.0); Py::Bool({ let t2966 = v_M1.clone(); if py::lt(&t2965, &t2966)? { { let t2967 = Py::Float(20.0); if py::le(&t2966, &t2967)? { true } else { false } } } else { false } }) } } }, Py::str("the upstream Mach number must lie between 1 and 20"))?;
    let _ = f__need({ let t2969 = v_branch.clone(); Py::Bool({ let t2970 = Py::tuple(vec![Py::str("weak"), Py::str("strong")]); if py::contains(&t2970, &t2969)? { true } else { false } }) }, Py::str("the branch must be weak or strong"))?;
    let _ = f__need({ let t2971 = Py::Int(0i64); Py::Bool({ let t2972 = v_theta.clone(); if py::lt(&t2971, &t2972)? { { let t2973 = py::truediv(&Py::Float(std::f64::consts::PI), &Py::Int(2i64))?; if py::lt(&t2972, &t2973)? { true } else { false } } } else { false } }) }, Py::str("the deflection angle must lie between 0 and 90 degrees"))?;
    v_bm = f__beta_max(v_M1.clone(), v_g.clone())?;
    v_tmax = f__theta_of_beta(v_M1.clone(), v_bm.clone(), v_g.clone())?;
    let _ = f__need({ let t2974 = v_theta.clone(); Py::Bool({ let t2975 = v_tmax.clone(); if py::lt(&t2974, &t2975)? { true } else { false } }) }, Py::str("the deflection exceeds the maximum for an attached oblique shock (the shock detaches)"))?;
    v_mu = py::math1("asin", &py::truediv(&Py::Float(1.0), &v_M1.clone())?)?;
    if ({ let t2976 = v_branch.clone(); Py::Bool({ let t2977 = Py::str("weak"); if py::eq(&t2976, &t2977) { true } else { false } }) }).truthy() {
        v_b = f__bisect({ let v_M1 = v_M1.clone(); let v_bm = v_bm.clone(); let v_branch = v_branch.clone(); let v_g = v_g.clone(); let v_gamma = v_gamma.clone(); let v_mu = v_mu.clone(); let v_theta = v_theta.clone(); let v_tmax = v_tmax.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&f__theta_of_beta(v_M1.clone(), v_x.clone(), v_g.clone())?, &v_theta.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, v_mu.clone(), v_bm.clone(), Py::Int(200i64))?;
    } else {
        v_b = f__bisect({ let v_M1 = v_M1.clone(); let v_b = v_b.clone(); let v_bm = v_bm.clone(); let v_branch = v_branch.clone(); let v_g = v_g.clone(); let v_gamma = v_gamma.clone(); let v_mu = v_mu.clone(); let v_theta = v_theta.clone(); let v_tmax = v_tmax.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&f__theta_of_beta(v_M1.clone(), v_x.clone(), v_g.clone())?, &v_theta.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, v_bm.clone(), py::truediv(&Py::Float(std::f64::consts::PI), &Py::Int(2i64))?, Py::Int(200i64))?;
    }
    let t3004 = (f__oblique_from_beta(v_M1.clone(), v_b.clone(), v_g.clone())?).items()?;
    if t3004.len() != 6 { return Err(PyErr::Value("unpack".into())); }
    v__th = t3004[0].clone();
    v_Mn1 = t3004[1].clone();
    v_M2 = t3004[2].clone();
    v_p21 = t3004[3].clone();
    v_r21 = t3004[4].clone();
    v_p0r = t3004[5].clone();
    return Ok(f__out(Py::dict(vec![(Py::str("beta_deg"), py::truediv(&v_b.clone(), &c_DEG()?)?), (Py::str("M2"), v_M2.clone()), (Py::str("p2_p1"), v_p21.clone()), (Py::str("T2_T1"), py::truediv(&v_p21.clone(), &v_r21.clone())?), (Py::str("rho2_rho1"), v_r21.clone()), (Py::str("p02_p01"), v_p0r.clone()), (Py::str("Mn1"), v_Mn1.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_optimal_staging(mut v_Isp: Py, mut v_eps: Py, mut v_dv: Py, mut v_m_pl: Py) -> R {
    let mut v_R: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_dv_of: Py = Py::None;
    let mut v_e_: Py = Py::None;
    let mut v_hi: Py = Py::None;
    let mut v_lam: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let mut v_m0: Py = Py::None;
    let mut v_s_: Py = Py::None;
    let mut v_stages: Py = Py::None;
    let _ = f__need({ let t3089 = { let t3084 = Py::Int(2i64); Py::Bool({ let t3085 = py::len(&v_Isp.clone())?; if py::le(&t3084, &t3085)? { { let t3086 = Py::Int(5i64); if py::le(&t3085, &t3086)? { true } else { false } } } else { false } }) }; if !t3089.truthy() { t3089 } else { { let t3087 = py::len(&v_Isp.clone())?; Py::Bool({ let t3088 = py::len(&v_eps.clone())?; if py::eq(&t3087, &t3088) { true } else { false } }) } } }, Py::str("give 2 to 5 stages, the same number of specific impulses and structural coefficients"))?;
    for t3090 in py::iter(&(py::zip(&[v_Isp.clone(), v_eps.clone()])?))? {
        let t3098 = (t3090).items()?;
        if t3098.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_s_ = t3098[0].clone();
        v_e_ = t3098[1].clone();
        let _ = f__isp_in(v_s_.clone())?;
        let _ = f__need({ let t3095 = Py::Float(0.01); Py::Bool({ let t3096 = v_e_.clone(); if py::le(&t3095, &t3096)? { { let t3097 = Py::Float(0.5); if py::le(&t3096, &t3097)? { true } else { false } } } else { false } }) }, Py::str("each structural coefficient must lie between 0.01 and 0.5"))?;
    }
    let _ = f__pos(Py::str("the delta-v and the payload mass"), Py::tuple(vec![v_dv.clone(), v_m_pl.clone()]))?;
    v_c = { let mut out = vec![]; for t3104 in py::iter(&(v_Isp.clone()))? { let mut v_s_: Py; 
v_s_ = t3104;
out.push(py::mul(&v_s_.clone(), &c_G0()?)?); } Py::list(out) };
    let _ = f__need({ let t3105 = v_dv.clone(); Py::Bool({ let t3106 = py::mul(&Py::Float(0.999), &py::sum(&{ let mut out = vec![]; for t3107 in py::iter(&(py::zip(&[v_c.clone(), v_eps.clone()])?))? { let mut v_ci: Py; let mut v_ei: Py; 
let t3108 = (t3107).items()?;
if t3108.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_ci = t3108[0].clone();
v_ei = t3108[1].clone();
out.push(py::mul(&v_ci.clone(), &py::math1("log", &py::truediv(&Py::Float(1.0), &v_ei.clone())?)?)?); } Py::list(out) }, None)?)?; if py::lt(&t3105, &t3106)? { true } else { false } }) }, Py::str("the delta-v is not reachable with these stages"))?;
    v_dv_of = { let v_Isp = v_Isp.clone(); let v_c = v_c.clone(); let v_dv = v_dv.clone(); let v_dv_of = v_dv_of.clone(); let v_e_ = v_e_.clone(); let v_eps = v_eps.clone(); let v_m_pl = v_m_pl.clone(); let v_s_ = v_s_.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_lam: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sum(&{ let mut out = vec![]; for t3109 in py::iter(&(py::zip(&[v_c.clone(), v_eps.clone()])?))? { let mut v_ci: Py; let mut v_ei: Py; 
let t3110 = (t3109).items()?;
if t3110.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_ci = t3110[0].clone();
v_ei = t3110[1].clone();
out.push(py::mul(&v_ci.clone(), &py::math1("log", &py::truediv(&py::sub(&py::mul(&v_lam.clone(), &v_ci.clone())?, &Py::Float(1.0))?, &py::mul(&py::mul(&v_lam.clone(), &v_ci.clone())?, &v_ei.clone())?)?)?)?); } Py::list(out) }, None)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_lo = py::mul(&py::minmax(&[{ let mut out = vec![]; for t3111 in py::iter(&(py::zip(&[v_c.clone(), v_eps.clone()])?))? { let mut v_ci: Py; let mut v_ei: Py; 
let t3112 = (t3111).items()?;
if t3112.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_ci = t3112[0].clone();
v_ei = t3112[1].clone();
out.push(py::truediv(&Py::Float(1.0), &py::mul(&v_ci.clone(), &py::sub(&Py::Float(1.0), &v_ei.clone())?)?)?); } Py::list(out) }], true)?, &py::add(&Py::Float(1.0), &Py::Float(1e-15))?)?;
    v_hi = py::mul(&v_lo.clone(), &Py::Float(2.0))?;
    while ({ let t3113 = py::call(&v_dv_of.clone(), &[v_hi.clone()])?; Py::Bool({ let t3114 = v_dv.clone(); if py::lt(&t3113, &t3114)? { true } else { false } }) }).truthy() {
        v_hi = py::mul(&v_hi, &Py::Float(2.0))?;
    }
    v_lam = f__bisect({ let v_Isp = v_Isp.clone(); let v_c = v_c.clone(); let v_dv = v_dv.clone(); let v_dv_of = v_dv_of.clone(); let v_e_ = v_e_.clone(); let v_eps = v_eps.clone(); let v_hi = v_hi.clone(); let v_lo = v_lo.clone(); let v_m_pl = v_m_pl.clone(); let v_s_ = v_s_.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&py::call(&v_dv_of.clone(), &[v_x.clone()])?, &v_dv.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, v_lo.clone(), v_hi.clone(), Py::Int(200i64))?;
    v_R = { let mut out = vec![]; for t3140 in py::iter(&(py::zip(&[v_c.clone(), v_eps.clone()])?))? { let mut v_ci: Py; let mut v_ei: Py; 
let t3141 = (t3140).items()?;
if t3141.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_ci = t3141[0].clone();
v_ei = t3141[1].clone();
out.push(py::truediv(&py::sub(&py::mul(&v_lam.clone(), &v_ci.clone())?, &Py::Float(1.0))?, &py::mul(&py::mul(&v_lam.clone(), &v_ci.clone())?, &v_ei.clone())?)?); } Py::list(out) };
    let t3145 = (f__stack(v_R.clone(), v_eps.clone(), v_m_pl.clone())?).items()?;
    if t3145.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_m0 = t3145[0].clone();
    v_stages = t3145[1].clone();
    return Ok(f__out(Py::dict(vec![(Py::str("m0"), v_m0.clone()), (Py::str("payload_fraction"), py::truediv(&v_m_pl.clone(), &v_m0.clone())?), (Py::str("R"), v_R.clone()), (Py::str("stage_mass"), v_stages.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_optimum_area_ratio(mut v_gamma: Py, mut v_pc: Py, mut v_pa: Py) -> R {
    let mut v_Me: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_g = f__gam(v_gamma.clone())?;
    let _ = f__pos(Py::str("the chamber and ambient pressures"), Py::tuple(vec![v_pc.clone(), v_pa.clone()]))?;
    let _ = f__need({ let t3191 = { let t3187 = py::truediv(&v_pc.clone(), &v_pa.clone())?; Py::Bool({ let t3188 = Py::Float(2.0); if py::ge(&t3187, &t3188)? { true } else { false } }) }; if !t3191.truthy() { t3191 } else { { let t3189 = py::truediv(&v_pc.clone(), &v_pa.clone())?; Py::Bool({ let t3190 = Py::Float(1000000.0); if py::le(&t3189, &t3190)? { true } else { false } }) } } }, Py::str("the chamber-to-ambient pressure ratio must lie between 2 and 1e6"))?;
    v_x = py::truediv(&v_pa.clone(), &v_pc.clone())?;
    v_Me = py::math1("sqrt", &py::mul(&py::truediv(&Py::Float(2.0), &py::sub(&v_g.clone(), &Py::Float(1.0))?)?, &py::sub(&py::pow(&v_x.clone(), &py::truediv(&py::neg(&py::sub(&v_g.clone(), &Py::Float(1.0))?)?, &v_g.clone())?)?, &Py::Float(1.0))?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("eps"), f__area(v_Me.clone(), v_g.clone())?), (Py::str("Me"), v_Me.clone()), (Py::str("CF"), py::mul(&f__vdk(v_g.clone())?, &py::math1("sqrt", &py::mul(&py::truediv(&py::mul(&Py::Float(2.0), &v_g.clone())?, &py::sub(&v_g.clone(), &Py::Float(1.0))?)?, &py::sub(&Py::Float(1.0), &py::pow(&v_x.clone(), &py::truediv(&py::sub(&v_g.clone(), &Py::Float(1.0))?, &v_g.clone())?)?)?)?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_prandtl_meyer(mut v_M: Py, mut v_gamma: Py) -> R {
    let mut v_g: Py = Py::None;
    v_g = f__gam(v_gamma.clone())?;
    let _ = f__need({ let t3231 = Py::Bool(matches!(v_M.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t3231.truthy() { t3231 } else { { let t3228 = Py::Float(1.0); Py::Bool({ let t3229 = v_M.clone(); if py::le(&t3228, &t3229)? { { let t3230 = Py::Float(50.0); if py::le(&t3229, &t3230)? { true } else { false } } } else { false } }) } } }, Py::str("the Mach number must lie between 1 and 50"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("nu_deg"), py::truediv(&f__nu(v_M.clone(), v_g.clone())?, &c_DEG()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pressure_altitude(mut v_p: Py) -> R {
    let mut v_H: Py = Py::None;
    let mut v_h: Py = Py::None;
    let _ = f__pos(Py::str("the static pressure"), Py::tuple(vec![v_p.clone()]))?;
    let _ = f__need({ let t3269 = c_P_MIN()?; Py::Bool({ let t3270 = v_p.clone(); if py::le(&t3269, &t3270)? { { let t3271 = c_P_MAX()?; if py::le(&t3270, &t3271)? { true } else { false } } } else { false } }) }, Py::str("the pressure must lie within the standard atmosphere from -1 km to 47 km"))?;
    v_H = f__H_from_p(v_p.clone())?;
    v_h = f__geom(v_H.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("h"), v_h.clone()), (Py::str("h_ft"), py::truediv(&v_H.clone(), &c_FT()?)?), (Py::str("H"), v_H.clone()), (Py::str("T_std"), py::getitem(&f__isa_H(v_H.clone())?, &Py::Int(0i64))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_prop_endurance(mut v_eta: Py, mut v_c: Py, mut v_CL: Py, mut v_CD: Py, mut v_S: Py, mut v_h: Py, mut v_W0: Py, mut v_W1: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_rho: Py = Py::None;
    let _ = f__eta_in(v_eta.clone())?;
    let _ = f__bsfc_in(v_c.clone())?;
    let _ = f__clcd_in(v_CL.clone(), v_CD.clone())?;
    let _ = f__pos(Py::str("the wing area"), Py::tuple(vec![v_S.clone()]))?;
    let _ = f__w01(v_W0.clone(), v_W1.clone())?;
    v_rho = f__rho_at(v_h.clone())?;
    v_E = py::mul(&py::mul(&py::truediv(&py::mul(&py::truediv(&v_eta.clone(), &py::mul(&c_G0()?, &v_c.clone())?)?, &py::pow(&v_CL.clone(), &Py::Float(1.5))?)?, &v_CD.clone())?, &py::math1("sqrt", &py::mul(&py::mul(&Py::Int(2i64), &v_rho.clone())?, &v_S.clone())?)?)?, &py::sub(&py::truediv(&Py::Int(1i64), &py::math1("sqrt", &v_W1.clone())?)?, &py::truediv(&Py::Int(1i64), &py::math1("sqrt", &v_W0.clone())?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("E"), v_E.clone()), (Py::str("E_h"), py::truediv(&v_E.clone(), &Py::Float(3600.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_propellant_for_delta_v(mut v_dv: Py, mut v_Isp: Py, mut v_mf: Py) -> R {
    let mut v_R: Py = Py::None;
    let _ = f__isp_in(v_Isp.clone())?;
    let _ = f__pos(Py::str("the velocity change and the burnout mass"), Py::tuple(vec![v_dv.clone(), v_mf.clone()]))?;
    v_R = py::math1("exp", &py::truediv(&v_dv.clone(), &py::mul(&v_Isp.clone(), &c_G0()?)?)?)?;
    let _ = f__need({ let t3399 = v_R.clone(); Py::Bool({ let t3400 = Py::Float(10000.0); if py::le(&t3399, &t3400)? { true } else { false } }) }, Py::str("the mass ratio would exceed 10000"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("m0"), py::mul(&v_mf.clone(), &v_R.clone())?), (Py::str("mp"), py::mul(&v_mf.clone(), &py::sub(&v_R.clone(), &Py::Float(1.0))?)?), (Py::str("R"), v_R.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_rayleigh(mut v_M: Py, mut v_gamma: Py) -> R {
    let mut v_g: Py = Py::None;
    let mut v_m2: Py = Py::None;
    let mut v_pr: Py = Py::None;
    v_g = f__gam(v_gamma.clone())?;
    let _ = f__need({ let t3440 = Py::Bool(matches!(v_M.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t3440.truthy() { t3440 } else { { let t3437 = Py::Float(0.05); Py::Bool({ let t3438 = v_M.clone(); if py::le(&t3437, &t3438)? { { let t3439 = Py::Float(10.0); if py::le(&t3438, &t3439)? { true } else { false } } } else { false } }) } } }, Py::str("the Mach number must lie between 0.05 and 10"))?;
    v_m2 = py::mul(&v_M.clone(), &v_M.clone())?;
    v_pr = py::truediv(&py::add(&Py::Float(1.0), &v_g.clone())?, &py::add(&Py::Float(1.0), &py::mul(&v_g.clone(), &v_m2.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("T0_T0star"), py::truediv(&py::mul(&py::mul(&py::add(&v_g.clone(), &Py::Float(1.0))?, &v_m2.clone())?, &py::add(&Py::Float(2.0), &py::mul(&py::sub(&v_g.clone(), &Py::Float(1.0))?, &v_m2.clone())?)?)?, &py::pow(&py::add(&Py::Float(1.0), &py::mul(&v_g.clone(), &v_m2.clone())?)?, &Py::Int(2i64))?)?), (Py::str("T_Tstar"), py::mul(&py::mul(&v_m2.clone(), &v_pr.clone())?, &v_pr.clone())?), (Py::str("p_pstar"), v_pr.clone()), (Py::str("rho_rhostar"), py::truediv(&Py::Float(1.0), &py::mul(&v_m2.clone(), &v_pr.clone())?)?), (Py::str("p0_p0star"), py::mul(&v_pr.clone(), &py::pow(&py::truediv(&py::add(&Py::Float(2.0), &py::mul(&py::sub(&v_g.clone(), &Py::Float(1.0))?, &v_m2.clone())?)?, &py::add(&v_g.clone(), &Py::Float(1.0))?)?, &py::truediv(&v_g.clone(), &py::sub(&v_g.clone(), &Py::Float(1.0))?)?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_rocket_delta_v(mut v_Isp: Py, mut v_m0: Py, mut v_mf: Py) -> R {
    let _ = f__isp_in(v_Isp.clone())?;
    let _ = f__pos(Py::str("the initial and final masses"), Py::tuple(vec![v_m0.clone(), v_mf.clone()]))?;
    let _ = f__need({ let t3482 = v_mf.clone(); Py::Bool({ let t3483 = v_m0.clone(); if py::lt(&t3482, &t3483)? { true } else { false } }) }, Py::str("the final (burnout) mass must be smaller than the initial mass"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("dv"), py::mul(&py::mul(&v_Isp.clone(), &c_G0()?)?, &py::math1("log", &py::truediv(&v_m0.clone(), &v_mf.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_rocket_performance(mut v_pc: Py, mut v_Tc: Py, mut v_Mw: Py, mut v_gamma: Py, mut v_At: Py, mut v_eps: Py, mut v_pa: Py) -> R {
    let mut v_F: Py = Py::None;
    let mut v_Me: Py = Py::None;
    let mut v_R: Py = Py::None;
    let mut v_cstar: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_mdot: Py = Py::None;
    let mut v_ve: Py = Py::None;
    let mut v_x: Py = Py::None;
    let t3964 = (f__rkt_in(v_pc.clone(), v_Tc.clone(), v_gamma.clone(), v_Mw.clone())?).items()?;
    if t3964.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_g = t3964[0].clone();
    v_R = t3964[1].clone();
    let _ = f__pos(Py::str("the throat area"), Py::tuple(vec![v_At.clone()]))?;
    let _ = f__need({ let t3968 = Py::Bool(matches!(v_eps.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t3968.truthy() { t3968 } else { { let t3965 = Py::Float(1.0); Py::Bool({ let t3966 = v_eps.clone(); if py::lt(&t3965, &t3966)? { { let t3967 = Py::Float(500.0); if py::le(&t3966, &t3967)? { true } else { false } } } else { false } }) } } }, Py::str("the nozzle area ratio must lie between 1 and 500"))?;
    let _ = f__need({ let t3972 = Py::Bool(matches!(v_pa.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t3972.truthy() { t3972 } else { { let t3969 = Py::Int(0i64); Py::Bool({ let t3970 = v_pa.clone(); if py::le(&t3969, &t3970)? { { let t3971 = v_pc.clone(); if py::lt(&t3970, &t3971)? { true } else { false } } } else { false } }) } } }, Py::str("the ambient pressure must be at least 0 and below the chamber pressure"))?;
    v_Me = f__M_of_area(v_eps.clone(), v_g.clone(), Py::Bool(true))?;
    v_x = py::truediv(&Py::Float(1.0), &f__p0p(v_Me.clone(), v_g.clone())?)?;
    let _ = f__sep_check(py::mul(&v_x.clone(), &v_pc.clone())?, v_pa.clone())?;
    v_cstar = py::truediv(&py::math1("sqrt", &py::mul(&v_R.clone(), &v_Tc.clone())?)?, &f__vdk(v_g.clone())?)?;
    v_mdot = py::truediv(&py::mul(&v_pc.clone(), &v_At.clone())?, &v_cstar.clone())?;
    v_ve = py::math1("sqrt", &py::mul(&py::mul(&py::mul(&py::truediv(&py::mul(&Py::Float(2.0), &v_g.clone())?, &py::sub(&v_g.clone(), &Py::Float(1.0))?)?, &v_R.clone())?, &v_Tc.clone())?, &py::sub(&Py::Float(1.0), &py::pow(&v_x.clone(), &py::truediv(&py::sub(&v_g.clone(), &Py::Float(1.0))?, &v_g.clone())?)?)?)?)?;
    v_F = py::add(&py::mul(&v_mdot.clone(), &v_ve.clone())?, &py::mul(&py::mul(&py::sub(&py::mul(&v_x.clone(), &v_pc.clone())?, &v_pa.clone())?, &v_eps.clone())?, &v_At.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("F"), v_F.clone()), (Py::str("mdot"), v_mdot.clone()), (Py::str("Isp"), py::truediv(&v_F.clone(), &py::mul(&v_mdot.clone(), &c_G0()?)?)?), (Py::str("CF"), py::truediv(&v_F.clone(), &py::mul(&v_pc.clone(), &v_At.clone())?)?), (Py::str("c_star"), v_cstar.clone()), (Py::str("pe"), py::mul(&v_x.clone(), &v_pc.clone())?), (Py::str("ve"), v_ve.clone()), (Py::str("Me"), v_Me.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_rocket_thrust(mut v_mdot: Py, mut v_ve: Py, mut v_pe: Py, mut v_pa: Py, mut v_Ae: Py) -> R {
    let mut v_F: Py = Py::None;
    let mut v_Fp: Py = Py::None;
    let mut v_given: Py = Py::None;
    let _ = f__pos(Py::str("the mass flow and the exhaust velocity"), Py::tuple(vec![v_mdot.clone(), v_ve.clone()]))?;
    v_given = { let mut out = vec![]; for t3610 in py::iter(&(Py::tuple(vec![v_pe.clone(), v_pa.clone(), v_Ae.clone()])))? { let mut v_x: Py; 
v_x = t3610;
out.push({ let t3611 = v_x.clone(); Py::Bool({ let t3612 = Py::None; if !py::is(&t3611, &t3612) { true } else { false } }) }); } Py::list(out) };
    let _ = f__need({ let t3613 = Py::Bool((v_given.clone()).items()?.iter().all(|x| x.truthy())); if t3613.truthy() { t3613 } else { py::not(&Py::Bool((v_given.clone()).items()?.iter().any(|x| x.truthy()))) } }, Py::str("give the exit pressure, ambient pressure and exit area together (or none of them)"))?;
    v_Fp = Py::Float(0.0);
    if (Py::Bool((v_given.clone()).items()?.iter().all(|x| x.truthy()))).truthy() {
        let _ = f__need({ let t3618 = { let t3614 = v_pe.clone(); Py::Bool({ let t3615 = Py::Int(0i64); if py::ge(&t3614, &t3615)? { true } else { false } }) }; if !t3618.truthy() { t3618 } else { { let t3616 = v_pa.clone(); Py::Bool({ let t3617 = Py::Int(0i64); if py::ge(&t3616, &t3617)? { true } else { false } }) } } }, Py::str("pressures must be zero or positive"))?;
        let _ = f__pos(Py::str("the exit area"), Py::tuple(vec![v_Ae.clone()]))?;
        v_Fp = py::mul(&py::sub(&v_pe.clone(), &v_pa.clone())?, &v_Ae.clone())?;
    }
    v_F = py::add(&py::mul(&v_mdot.clone(), &v_ve.clone())?, &v_Fp.clone())?;
    let _ = f__need({ let t3619 = v_F.clone(); Py::Bool({ let t3620 = Py::Int(0i64); if py::gt(&t3619, &t3620)? { true } else { false } }) }, Py::str("the thrust must be positive"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("F"), v_F.clone()), (Py::str("F_kN"), py::truediv(&v_F.clone(), &Py::Float(1000.0))?), (Py::str("Isp"), py::truediv(&v_F.clone(), &py::mul(&v_mdot.clone(), &c_G0()?)?)?), (Py::str("c_eff"), py::truediv(&v_F.clone(), &v_mdot.clone())?), (Py::str("F_pressure"), v_Fp.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_satellite_coverage(mut v_h: Py, mut v_eps_min: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_eta: Py = Py::None;
    let mut v_lam: Py = Py::None;
    let mut v_r: Py = Py::None;
    let _ = f__orb_alt(v_h.clone(), Py::Float(100000.0), Py::Float(1000000000.0))?;
    let _ = f__need({ let t3660 = Py::Bool(matches!(v_eps_min.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t3660.truthy() { t3660 } else { { let t3657 = Py::Int(0i64); Py::Bool({ let t3658 = v_eps_min.clone(); if py::le(&t3657, &t3658)? { { let t3659 = py::mul(&Py::Int(80i64), &c_DEG()?)?; if py::le(&t3658, &t3659)? { true } else { false } } } else { false } }) } } }, Py::str("the minimum elevation angle must lie between 0 and 80 degrees"))?;
    v_r = py::add(&c_R_E()?, &v_h.clone())?;
    v_eta = py::math1("asin", &py::truediv(&py::mul(&c_R_E()?, &py::math1("cos", &v_eps_min.clone())?)?, &v_r.clone())?)?;
    v_lam = py::sub(&py::sub(&py::truediv(&Py::Float(std::f64::consts::PI), &Py::Int(2i64))?, &v_eps_min.clone())?, &v_eta.clone())?;
    v_d = py::truediv(&py::mul(&c_R_E()?, &py::math1("sin", &v_lam.clone())?)?, &py::math1("sin", &v_eta.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("lambda_deg"), py::math1("degrees", &v_lam.clone())?), (Py::str("nadir_deg"), py::math1("degrees", &v_eta.clone())?), (Py::str("slant_range"), v_d.clone()), (Py::str("coverage"), py::truediv(&py::sub(&Py::Int(1i64), &py::math1("cos", &v_lam.clone())?)?, &Py::Int(2i64))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_service_ceiling_jet(mut v_W: Py, mut v_S: Py, mut v_T0: Py, mut v_CD0: Py, mut v_AR: Py, mut v_e: Py, mut v_RCs: Py) -> R {
    let mut v_K: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_rc: Py = Py::None;
    let _ = f__ws_in(v_W.clone(), v_S.clone())?;
    let _ = f__cd0(v_CD0.clone())?;
    v_K = f__K(v_AR.clone(), v_e.clone())?;
    let _ = f__pos(Py::str("the sea-level thrust and the service-ceiling rate of climb"), Py::tuple(vec![v_T0.clone(), v_RCs.clone()]))?;
    let _ = f__need({ let t3710 = v_T0.clone(); Py::Bool({ let t3711 = py::mul(&Py::Float(0.8), &v_W.clone())?; if py::lt(&t3710, &t3711)? { true } else { false } }) }, Py::str("the sea-level thrust-to-weight ratio must be below 0.8"))?;
    v_rc = { let v_AR = v_AR.clone(); let v_CD0 = v_CD0.clone(); let v_K = v_K.clone(); let v_RCs = v_RCs.clone(); let v_S = v_S.clone(); let v_T0 = v_T0.clone(); let v_W = v_W.clone(); let v_e = v_e.clone(); let v_rc = v_rc.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_h: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_rho: Py = Py::None;
            v_rho = f__rho_at(v_h.clone())?;
            return Ok(py::getitem(&f__rc_jet(v_W.clone(), v_S.clone(), v_rho.clone(), py::truediv(&py::mul(&v_T0.clone(), &v_rho.clone())?, &c_RHO_SL()?)?, v_CD0.clone(), v_K.clone())?, &Py::Int(1i64))?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    let _ = f__need({ let t3725 = py::call(&v_rc.clone(), &[Py::Float(0.0)])?; Py::Bool({ let t3726 = v_RCs.clone(); if py::gt(&t3725, &t3726)? { true } else { false } }) }, Py::str("the aircraft cannot climb at the service-ceiling rate even at sea level"))?;
    let _ = f__need({ let t3727 = py::call(&v_rc.clone(), &[c_H_MAX()?])?; Py::Bool({ let t3728 = v_RCs.clone(); if py::lt(&t3727, &t3728)? { true } else { false } }) }, Py::str("the service ceiling lies above 47 km"))?;
    v_h = f__bisect({ let v_AR = v_AR.clone(); let v_CD0 = v_CD0.clone(); let v_K = v_K.clone(); let v_RCs = v_RCs.clone(); let v_S = v_S.clone(); let v_T0 = v_T0.clone(); let v_W = v_W.clone(); let v_e = v_e.clone(); let v_rc = v_rc.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&py::call(&v_rc.clone(), &[v_x.clone()])?, &v_RCs.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, Py::Float(0.0), c_H_MAX()?, Py::Int(200i64))?;
    return Ok(f__out(Py::dict(vec![(Py::str("h_s"), v_h.clone()), (Py::str("h_s_ft"), py::truediv(&v_h.clone(), &c_FT()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_stall_speed(mut v_W: Py, mut v_S: Py, mut v_CLmax: Py, mut v_h: Py) -> R {
    let mut v_T: Py = Py::None;
    let mut v_V: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_rho: Py = Py::None;
    let _ = f__ws_in(v_W.clone(), v_S.clone())?;
    let _ = f__need({ let t3794 = Py::Bool(matches!(v_CLmax.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t3794.truthy() { t3794 } else { { let t3791 = Py::Float(0.5); Py::Bool({ let t3792 = v_CLmax.clone(); if py::le(&t3791, &t3792)? { { let t3793 = Py::Float(5.0); if py::le(&t3792, &t3793)? { true } else { false } } } else { false } }) } } }, Py::str("the maximum lift coefficient must lie between 0.5 and 5"))?;
    let t3808 = (f__isa(v_h.clone())?).items()?;
    if t3808.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_T = t3808[0].clone();
    v_p = t3808[1].clone();
    v_rho = t3808[2].clone();
    v_V = py::math1("sqrt", &py::truediv(&py::mul(&Py::Int(2i64), &v_W.clone())?, &py::mul(&py::mul(&v_rho.clone(), &v_S.clone())?, &v_CLmax.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("V_stall"), v_V.clone()), (Py::str("V_stall_kt"), py::truediv(&v_V.clone(), &c_KT()?)?), (Py::str("EAS_stall"), py::mul(&v_V.clone(), &py::math1("sqrt", &py::truediv(&v_rho.clone(), &c_RHO_SL()?)?)?)?), (Py::str("wing_loading"), py::truediv(&v_W.clone(), &v_S.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_sun_synchronous(mut v_h: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_r: Py = Py::None;
    let _ = f__orb_alt(v_h.clone(), Py::Float(100000.0), Py::Float(6000000.0))?;
    v_r = py::add(&c_R_E()?, &v_h.clone())?;
    v_n = py::math1("sqrt", &py::truediv(&c_MU_E()?, &py::pow(&v_r.clone(), &Py::Int(3i64))?)?)?;
    v_c = py::truediv(&py::neg(&c_SSO_RATE()?)?, &py::mul(&py::mul(&py::mul(&Py::Float(1.5), &v_n.clone())?, &c_J2_E()?)?, &py::pow(&py::truediv(&c_R_E()?, &v_r.clone())?, &Py::Int(2i64))?)?)?;
    let _ = f__need({ let t3845 = v_c.clone(); Py::Bool({ let t3846 = py::neg(&Py::Float(1.0))?; if py::ge(&t3845, &t3846)? { true } else { false } }) }, Py::str("no sun-synchronous circular orbit exists at this altitude (the J2 drift is too slow)"))?;
    v_i = py::math1("acos", &v_c.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("i_deg"), py::truediv(&v_i.clone(), &c_DEG()?)?), (Py::str("period_min"), py::truediv(&py::truediv(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_n.clone())?, &Py::Float(60.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_takeoff_roll(mut v_W: Py, mut v_S: Py, mut v_h: Py, mut v_T: Py, mut v_mu: Py, mut v_CL: Py, mut v_CD: Py, mut v_VLO: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_B = f__roll_terms(v_W.clone(), v_S.clone(), v_h.clone(), v_mu.clone(), v_CL.clone(), v_CD.clone())?;
    let _ = f__pos(Py::str("the thrust and the lift-off speed"), Py::tuple(vec![v_T.clone(), v_VLO.clone()]))?;
    v_A = py::mul(&c_G0()?, &py::sub(&py::truediv(&v_T.clone(), &v_W.clone())?, &v_mu.clone())?)?;
    let _ = f__need({ let t3913 = { let t3909 = v_A.clone(); Py::Bool({ let t3910 = Py::Int(0i64); if py::gt(&t3909, &t3910)? { true } else { false } }) }; if !t3913.truthy() { t3913 } else { { let t3911 = py::sub(&v_A.clone(), &py::mul(&py::mul(&v_B.clone(), &v_VLO.clone())?, &v_VLO.clone())?)?; Py::Bool({ let t3912 = Py::Int(0i64); if py::gt(&t3911, &t3912)? { true } else { false } }) } } }, Py::str("the aircraft would stop accelerating before the lift-off speed (thrust too low)"))?;
    v_x = py::truediv(&py::mul(&py::mul(&v_B.clone(), &v_VLO.clone())?, &v_VLO.clone())?, &v_A.clone())?;
    v_s = (if ({ let t3914 = py::abs(&v_x.clone())?; Py::Bool({ let t3915 = Py::Float(1e-08); if py::gt(&t3914, &t3915)? { true } else { false } }) }).truthy() { py::truediv(&py::neg(&py::math1("log1p", &py::neg(&v_x.clone())?)?)?, &py::mul(&Py::Int(2i64), &v_B.clone())?)? } else { py::mul(&py::truediv(&py::mul(&v_VLO.clone(), &v_VLO.clone())?, &py::mul(&Py::Int(2i64), &v_A.clone())?)?, &py::add(&py::add(&Py::Int(1i64), &py::truediv(&v_x.clone(), &Py::Int(2i64))?)?, &py::truediv(&py::mul(&v_x.clone(), &v_x.clone())?, &Py::Int(3i64))?)?)? });
    if ({ let t3916 = py::abs(&v_x.clone())?; Py::Bool({ let t3917 = Py::Float(1e-06); if py::lt(&t3916, &t3917)? { true } else { false } }) }).truthy() {
        v_t = py::mul(&py::truediv(&v_VLO.clone(), &v_A.clone())?, &py::add(&py::add(&Py::Int(1i64), &py::truediv(&v_x.clone(), &Py::Int(3i64))?)?, &py::truediv(&py::mul(&v_x.clone(), &v_x.clone())?, &Py::Int(5i64))?)?)?;
    } else {
        if ({ let t3918 = v_B.clone(); Py::Bool({ let t3919 = Py::Int(0i64); if py::gt(&t3918, &t3919)? { true } else { false } }) }).truthy() {
            v_t = py::truediv(&py::math1("atanh", &py::math1("sqrt", &v_x.clone())?)?, &py::math1("sqrt", &py::mul(&v_A.clone(), &v_B.clone())?)?)?;
        } else {
            v_t = py::truediv(&py::math1("atan", &py::math1("sqrt", &py::neg(&v_x.clone())?)?)?, &py::math1("sqrt", &py::mul(&py::neg(&v_A.clone())?, &v_B.clone())?)?)?;
        }
    }
    return Ok(f__out(Py::dict(vec![(Py::str("s"), v_s.clone()), (Py::str("t"), v_t.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_throat_area_for_thrust(mut v_F: Py, mut v_pc: Py, mut v_Tc: Py, mut v_Mw: Py, mut v_gamma: Py, mut v_eps: Py, mut v_pa: Py) -> R {
    let mut v_At: Py = Py::None;
    let mut v_o: Py = Py::None;
    let _ = f__pos(Py::str("the thrust"), Py::tuple(vec![v_F.clone()]))?;
    v_o = f_rocket_performance(v_pc.clone(), v_Tc.clone(), v_Mw.clone(), v_gamma.clone(), Py::Float(1.0), v_eps.clone(), v_pa.clone())?;
    let _ = f__need({ let t4018 = py::getitem(&v_o.clone(), &Py::str("CF"))?; Py::Bool({ let t4019 = Py::Int(0i64); if py::gt(&t4018, &t4019)? { true } else { false } }) }, Py::str("the thrust coefficient is not positive at this ambient pressure"))?;
    v_At = py::truediv(&v_F.clone(), &py::mul(&py::getitem(&v_o.clone(), &Py::str("CF"))?, &v_pc.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("At"), v_At.clone()), (Py::str("Ae"), py::mul(&v_At.clone(), &v_eps.clone())?), (Py::str("mdot"), py::truediv(&py::mul(&v_pc.clone(), &v_At.clone())?, &py::getitem(&v_o.clone(), &Py::str("c_star"))?)?), (Py::str("Isp"), py::getitem(&v_o.clone(), &Py::str("Isp"))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_thrust_coefficient(mut v_gamma: Py, mut v_eps: Py, mut v_pc: Py, mut v_pa: Py) -> R {
    let mut v_Me: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_mom: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_g = f__gam(v_gamma.clone())?;
    let _ = f__pos(Py::str("the chamber pressure"), Py::tuple(vec![v_pc.clone()]))?;
    let _ = f__need({ let t4055 = Py::Bool(matches!(v_eps.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t4055.truthy() { t4055 } else { { let t4052 = Py::Float(1.0); Py::Bool({ let t4053 = v_eps.clone(); if py::lt(&t4052, &t4053)? { { let t4054 = Py::Float(500.0); if py::le(&t4053, &t4054)? { true } else { false } } } else { false } }) } } }, Py::str("the nozzle area ratio must lie between 1 and 500"))?;
    let _ = f__need({ let t4059 = Py::Bool(matches!(v_pa.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t4059.truthy() { t4059 } else { { let t4056 = Py::Int(0i64); Py::Bool({ let t4057 = v_pa.clone(); if py::le(&t4056, &t4057)? { { let t4058 = v_pc.clone(); if py::lt(&t4057, &t4058)? { true } else { false } } } else { false } }) } } }, Py::str("the ambient pressure must be at least 0 and below the chamber pressure"))?;
    v_Me = f__M_of_area(v_eps.clone(), v_g.clone(), Py::Bool(true))?;
    v_x = py::pow(&f__p0p(v_Me.clone(), v_g.clone())?, &py::neg(&Py::Float(1.0))?)?;
    let _ = f__sep_check(py::mul(&v_x.clone(), &v_pc.clone())?, v_pa.clone())?;
    v_mom = py::mul(&f__vdk(v_g.clone())?, &py::math1("sqrt", &py::mul(&py::truediv(&py::mul(&Py::Float(2.0), &v_g.clone())?, &py::sub(&v_g.clone(), &Py::Float(1.0))?)?, &py::sub(&Py::Float(1.0), &py::pow(&v_x.clone(), &py::truediv(&py::sub(&v_g.clone(), &Py::Float(1.0))?, &v_g.clone())?)?)?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("CF"), py::add(&v_mom.clone(), &py::mul(&py::sub(&v_x.clone(), &py::truediv(&v_pa.clone(), &v_pc.clone())?)?, &v_eps.clone())?)?), (Py::str("pe_pc"), v_x.clone()), (Py::str("Me"), v_Me.clone()), (Py::str("CF_vac"), py::add(&v_mom.clone(), &py::mul(&v_x.clone(), &v_eps.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_time_to_climb(mut v_RC0: Py, mut v_Hc: Py, mut v_h1: Py, mut v_h2: Py) -> R {
    let mut v_t: Py = Py::None;
    let _ = f__pos(Py::str("the sea-level rate of climb and the absolute ceiling"), Py::tuple(vec![v_RC0.clone(), v_Hc.clone()]))?;
    let _ = f__need({ let t4138 = Py::Bool(matches!(v_h1.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t4138.truthy() { t4138 } else { { let t4137 = Py::Bool(matches!(v_h2.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t4137.truthy() { t4137 } else { { let t4133 = Py::Int(0i64); Py::Bool({ let t4134 = v_h1.clone(); if py::le(&t4133, &t4134)? { { let t4135 = v_h2.clone(); if py::lt(&t4134, &t4135)? { { let t4136 = v_Hc.clone(); if py::lt(&t4135, &t4136)? { true } else { false } } } else { false } } } else { false } }) } } } } }, Py::str("the altitudes must satisfy 0 <= h1 < h2 < absolute ceiling"))?;
    v_t = py::mul(&py::truediv(&v_Hc.clone(), &v_RC0.clone())?, &py::math1("log", &py::truediv(&py::sub(&v_Hc.clone(), &v_h1.clone())?, &py::sub(&v_Hc.clone(), &v_h2.clone())?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("t"), v_t.clone()), (Py::str("t_min"), py::truediv(&v_t.clone(), &Py::Float(60.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_total_temperature(mut v_V: Py, mut v_T: Py) -> R {
    let mut v_M: Py = Py::None;
    let mut v_cp: Py = Py::None;
    let mut v_dT: Py = Py::None;
    let _ = f__pos(Py::str("the true airspeed and the static temperature"), Py::tuple(vec![v_V.clone(), v_T.clone()]))?;
    let _ = f__need({ let t4176 = Py::Float(150.0); Py::Bool({ let t4177 = v_T.clone(); if py::le(&t4176, &t4177)? { { let t4178 = Py::Float(350.0); if py::le(&t4177, &t4178)? { true } else { false } } } else { false } }) }, Py::str("the static air temperature must lie between 150 K and 350 K"))?;
    v_cp = py::truediv(&py::mul(&c_GAMMA_AIR()?, &c_R_AIR()?)?, &py::sub(&c_GAMMA_AIR()?, &Py::Int(1i64))?)?;
    v_M = py::truediv(&v_V.clone(), &py::math1("sqrt", &py::mul(&py::mul(&c_GAMMA_AIR()?, &c_R_AIR()?)?, &v_T.clone())?)?)?;
    let _ = f__need({ let t4179 = v_M.clone(); Py::Bool({ let t4180 = Py::Float(5.0); if py::lt(&t4179, &t4180)? { true } else { false } }) }, Py::str("the flight Mach number must be below 5 (calorically perfect air)"))?;
    v_dT = py::truediv(&py::mul(&v_V.clone(), &v_V.clone())?, &py::mul(&Py::Int(2i64), &v_cp.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("T0"), py::add(&v_T.clone(), &v_dT.clone())?), (Py::str("dT"), v_dT.clone()), (Py::str("M"), v_M.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_true_airspeed_from_cas(mut v_CAS: Py, mut v_hp: Py, mut v_T: Py) -> R {
    let mut v_M: Py = Py::None;
    let mut v_V: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_qc: Py = Py::None;
    let _ = f__pos(Py::str("the calibrated airspeed"), Py::tuple(vec![v_CAS.clone()]))?;
    let _ = f__need({ let t4218 = v_CAS.clone(); Py::Bool({ let t4219 = c_A_SL()?; if py::lt(&t4218, &t4219)? { true } else { false } }) }, Py::str("the calibrated airspeed must be below the sea-level speed of sound (subsonic pitot formula)"))?;
    let t4239 = (f__static(v_hp.clone(), v_T.clone())?).items()?;
    if t4239.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_p = t4239[0].clone();
    v_T = t4239[1].clone();
    v_qc = py::mul(&c_P_SL()?, &py::sub(&f__p0p(py::truediv(&v_CAS.clone(), &c_A_SL()?)?, c_GAMMA_AIR()?)?, &Py::Float(1.0))?)?;
    v_M = py::math1("sqrt", &py::mul(&Py::Float(5.0), &py::sub(&py::pow(&py::add(&py::truediv(&v_qc.clone(), &v_p.clone())?, &Py::Float(1.0))?, &py::truediv(&Py::Float(2.0), &Py::Float(7.0))?)?, &Py::Float(1.0))?)?)?;
    let _ = f__need({ let t4240 = v_M.clone(); Py::Bool({ let t4241 = Py::Float(1.0); if py::lt(&t4240, &t4241)? { true } else { false } }) }, Py::str("the flight Mach number would be supersonic; the subsonic pitot formula does not apply"))?;
    v_V = py::mul(&v_M.clone(), &py::math1("sqrt", &py::mul(&py::mul(&c_GAMMA_AIR()?, &c_R_AIR()?)?, &v_T.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("TAS"), v_V.clone()), (Py::str("TAS_kt"), py::truediv(&v_V.clone(), &c_KT()?)?), (Py::str("qc"), v_qc.clone()), (Py::str("M"), v_M.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_vertical_rocket(mut v_Isp: Py, mut v_m0: Py, mut v_mf: Py, mut v_tb: Py, mut v_g: Py) -> R {
    let mut v_R: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_hb: Py = Py::None;
    let mut v_vb: Py = Py::None;
    let t4287 = (f__vert_in(v_Isp.clone(), v_m0.clone(), v_mf.clone(), v_tb.clone(), v_g.clone())?).items()?;
    if t4287.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_c = t4287[0].clone();
    v_R = t4287[1].clone();
    v_vb = py::sub(&py::mul(&v_c.clone(), &py::math1("log", &v_R.clone())?)?, &py::mul(&v_g.clone(), &v_tb.clone())?)?;
    v_hb = py::sub(&py::mul(&py::mul(&v_c.clone(), &v_tb.clone())?, &py::sub(&Py::Float(1.0), &py::truediv(&py::math1("log", &v_R.clone())?, &py::sub(&v_R.clone(), &Py::Float(1.0))?)?)?)?, &py::mul(&py::mul(&py::mul(&Py::Float(0.5), &v_g.clone())?, &v_tb.clone())?, &v_tb.clone())?)?;
    let _ = f__need({ let t4288 = v_vb.clone(); Py::Bool({ let t4289 = Py::Int(0i64); if py::gt(&t4288, &t4289)? { true } else { false } }) }, Py::str("the rocket does not climb at burnout"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("v_b"), v_vb.clone()), (Py::str("h_b"), v_hb.clone()), (Py::str("h_max"), py::add(&v_hb.clone(), &py::truediv(&py::mul(&v_vb.clone(), &v_vb.clone())?, &py::mul(&Py::Float(2.0), &v_g.clone())?)?)?), (Py::str("t_apex"), py::add(&v_tb.clone(), &py::truediv(&v_vb.clone(), &v_g.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_wind_triangle(mut v_TAS: Py, mut v_course: Py, mut v_wind_from: Py, mut v_wind_speed: Py) -> R {
    let mut v_gs: Py = Py::None;
    let mut v_hdg: Py = Py::None;
    let mut v_nm: Py = Py::None;
    let mut v_rel: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_wca: Py = Py::None;
    let mut v_x: Py = Py::None;
    let _ = f__pos(Py::str("the true airspeed"), Py::tuple(vec![v_TAS.clone()]))?;
    let _ = f__need({ let t4329 = Py::Bool(matches!(v_wind_speed.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t4329.truthy() { t4329 } else { { let t4327 = v_wind_speed.clone(); Py::Bool({ let t4328 = Py::Int(0i64); if py::ge(&t4327, &t4328)? { true } else { false } }) } } }, Py::str("the wind speed must not be negative"))?;
    for t4330 in py::iter(&(Py::tuple(vec![Py::tuple(vec![Py::str("the course"), v_course.clone()]), Py::tuple(vec![Py::str("the wind direction"), v_wind_from.clone()])])))? {
        let t4335 = (t4330).items()?;
        if t4335.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_nm = t4335[0].clone();
        v_v = t4335[1].clone();
        let _ = f__need({ let t4334 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t4334.truthy() { t4334 } else { { let t4331 = Py::Int(0i64); Py::Bool({ let t4332 = v_v.clone(); if py::le(&t4331, &t4332)? { { let t4333 = py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?; if py::le(&t4332, &t4333)? { true } else { false } } } else { false } }) } } }, Py::str(&[py::format(&v_nm.clone(), "")?, " must lie between 0 and 360 degrees".to_string()].concat()))?;
    }
    v_rel = py::sub(&v_wind_from.clone(), &v_course.clone())?;
    v_x = py::truediv(&py::mul(&v_wind_speed.clone(), &py::math1("sin", &v_rel.clone())?)?, &v_TAS.clone())?;
    let _ = f__need({ let t4336 = py::abs(&v_x.clone())?; Py::Bool({ let t4337 = Py::Float(0.99); if py::lt(&t4336, &t4337)? { true } else { false } }) }, Py::str("the crosswind is too strong for this airspeed (no heading holds the course)"))?;
    v_wca = py::math1("asin", &v_x.clone())?;
    v_gs = py::sub(&py::mul(&v_TAS.clone(), &py::math1("cos", &v_wca.clone())?)?, &py::mul(&v_wind_speed.clone(), &py::math1("cos", &v_rel.clone())?)?)?;
    let _ = f__need({ let t4338 = v_gs.clone(); Py::Bool({ let t4339 = py::mul(&Py::Float(0.01), &v_TAS.clone())?; if py::gt(&t4338, &t4339)? { true } else { false } }) }, Py::str("the headwind would stop the aircraft over the ground"))?;
    v_hdg = py::modulo(&py::math1("degrees", &py::add(&v_course.clone(), &v_wca.clone())?)?, &Py::Float(360.0))?;
    return Ok(f__out(Py::dict(vec![(Py::str("heading_deg"), v_hdg.clone()), (Py::str("wca_deg"), py::math1("degrees", &v_wca.clone())?), (Py::str("GS"), v_gs.clone()), (Py::str("GS_kt"), py::truediv(&v_gs.clone(), &c_KT()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_wing_loading(mut v_W: Py, mut v_S: Py) -> R {
    let _ = f__ws_in(v_W.clone(), v_S.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("wing_loading"), py::truediv(&v_W.clone(), &v_S.clone())?), (Py::str("wing_loading_kg_m2"), py::truediv(&v_W.clone(), &py::mul(&c_G0()?, &v_S.clone())?)?), (Py::str("wing_loading_psf"), py::truediv(&py::truediv(&v_W.clone(), &v_S.clone())?, &py::truediv(&c_LBF()?, &py::pow(&c_FT()?, &Py::Int(2i64))?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1030(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1032: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1031 = v_inputs.clone(); f_expansion_fan(py::getitem(&t1031, &Py::str("M1"))?, py::getitem(&t1031, &Py::str("theta"))?, py::method(&t1031, "get", &[Py::str("gamma"), Py::Float(1.4)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1032 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1033 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1052 = (t1033).items()?;
        if t1052.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1052[0].clone();
        v_v = t1052[1].clone();
        if ({ let t1034 = v_k.clone(); Py::Bool({ let t1035 = v_outputs.clone(); if !py::contains(&t1035, &t1034)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1073(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1075: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1074 = v_inputs.clone(); f_fanno(py::getitem(&t1074, &Py::str("M"))?, py::method(&t1074, "get", &[Py::str("gamma"), Py::Float(1.4)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1075 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1076 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1095 = (t1076).items()?;
        if t1095.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1095[0].clone();
        v_v = t1095[1].clone();
        if ({ let t1077 = v_k.clone(); Py::Bool({ let t1078 = v_outputs.clone(); if !py::contains(&t1078, &t1077)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1127(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1129: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1128 = v_inputs.clone(); f_flight_condition(py::getitem(&t1128, &Py::str("V"))?, py::getitem(&t1128, &Py::str("h"))?, py::method(&t1128, "get", &[Py::str("L"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1129 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1130 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1149 = (t1130).items()?;
        if t1149.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1149[0].clone();
        v_v = t1149[1].clone();
        if ({ let t1131 = v_k.clone(); Py::Bool({ let t1132 = v_outputs.clone(); if !py::contains(&t1132, &t1131)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_113(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t115: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t114 = v_inputs.clone(); f_actuator_disk_hover(py::getitem(&t114, &Py::str("T"))?, py::getitem(&t114, &Py::str("D"))?, py::getitem(&t114, &Py::str("rho"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t115 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t116 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t135 = (t116).items()?;
        if t135.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t135[0].clone();
        v_v = t135[1].clone();
        if ({ let t117 = v_k.clone(); Py::Bool({ let t118 = v_outputs.clone(); if !py::contains(&t118, &t117)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1163(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1165: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1164 = v_inputs.clone(); f_geopotential_altitude(py::getitem(&t1164, &Py::str("h"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1165 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1166 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1185 = (t1166).items()?;
        if t1185.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1185[0].clone();
        v_v = t1185[1].clone();
        if ({ let t1167 = v_k.clone(); Py::Bool({ let t1168 = v_outputs.clone(); if !py::contains(&t1168, &t1167)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1205(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1207: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1206 = v_inputs.clone(); f_glide(py::getitem(&t1206, &Py::str("dh"))?, py::getitem(&t1206, &Py::str("LD"))?, py::method(&t1206, "get", &[Py::str("V"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1207 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1208 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1227 = (t1208).items()?;
        if t1227.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1227[0].clone();
        v_v = t1227[1].clone();
        if ({ let t1209 = v_k.clone(); Py::Bool({ let t1210 = v_outputs.clone(); if !py::contains(&t1210, &t1209)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1256(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1258: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1257 = v_inputs.clone(); f_ideal_exhaust_velocity(py::getitem(&t1257, &Py::str("Tc"))?, py::getitem(&t1257, &Py::str("Mw"))?, py::getitem(&t1257, &Py::str("gamma"))?, py::getitem(&t1257, &Py::str("pc"))?, py::getitem(&t1257, &Py::str("pe"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1258 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1259 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1278 = (t1259).items()?;
        if t1278.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1278[0].clone();
        v_v = t1278[1].clone();
        if ({ let t1260 = v_k.clone(); Py::Bool({ let t1261 = v_outputs.clone(); if !py::contains(&t1261, &t1260)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1307(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1309: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1308 = v_inputs.clone(); f_identical_stages(py::getitem(&t1308, &Py::str("n"))?, py::getitem(&t1308, &Py::str("dv"))?, py::getitem(&t1308, &Py::str("Isp"))?, py::getitem(&t1308, &Py::str("eps"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1309 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1310 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1329 = (t1310).items()?;
        if t1329.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1329[0].clone();
        v_v = t1329[1].clone();
        if ({ let t1311 = v_k.clone(); Py::Bool({ let t1312 = v_outputs.clone(); if !py::contains(&t1312, &t1311)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1366(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1368: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1367 = v_inputs.clone(); f_induced_drag(py::getitem(&t1367, &Py::str("W"))?, py::getitem(&t1367, &Py::str("S"))?, py::getitem(&t1367, &Py::str("V"))?, py::getitem(&t1367, &Py::str("h"))?, py::getitem(&t1367, &Py::str("AR"))?, py::getitem(&t1367, &Py::str("e"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1368 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1369 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1388 = (t1369).items()?;
        if t1388.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1388[0].clone();
        v_v = t1388[1].clone();
        if ({ let t1370 = v_k.clone(); Py::Bool({ let t1371 = v_outputs.clone(); if !py::contains(&t1371, &t1370)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1412(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1414: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1413 = v_inputs.clone(); f_isa_properties(py::getitem(&t1413, &Py::str("h"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1414 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1415 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1434 = (t1415).items()?;
        if t1434.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1434[0].clone();
        v_v = t1434[1].clone();
        if ({ let t1416 = v_k.clone(); Py::Bool({ let t1417 = v_outputs.clone(); if !py::contains(&t1417, &t1416)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1452(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1454: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1453 = v_inputs.clone(); f_isentropic_flow(py::getitem(&t1453, &Py::str("M"))?, py::method(&t1453, "get", &[Py::str("gamma"), Py::Float(1.4)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1454 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1455 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1474 = (t1455).items()?;
        if t1474.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1474[0].clone();
        v_v = t1474[1].clone();
        if ({ let t1456 = v_k.clone(); Py::Bool({ let t1457 = v_outputs.clone(); if !py::contains(&t1457, &t1456)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1499(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1501: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1500 = v_inputs.clone(); f_apsidal_rotation(py::getitem(&t1500, &Py::str("hp"))?, py::getitem(&t1500, &Py::str("ha"))?, py::getitem(&t1500, &Py::str("i"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1501 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1502 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1521 = (t1502).items()?;
        if t1521.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1521[0].clone();
        v_v = t1521[1].clone();
        if ({ let t1503 = v_k.clone(); Py::Bool({ let t1504 = v_outputs.clone(); if !py::contains(&t1504, &t1503)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_154(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t156: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t155 = v_inputs.clone(); f_altitude_for_period(py::getitem(&t155, &Py::str("T"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t156 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t157 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t176 = (t157).items()?;
        if t176.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t176[0].clone();
        v_v = t176[1].clone();
        if ({ let t158 = v_k.clone(); Py::Bool({ let t159 = v_outputs.clone(); if !py::contains(&t159, &t158)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1546(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1548: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1547 = v_inputs.clone(); f_nodal_regression(py::getitem(&t1547, &Py::str("hp"))?, py::getitem(&t1547, &Py::str("i"))?, py::method(&t1547, "get", &[Py::str("ha"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1548 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1549 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1568 = (t1549).items()?;
        if t1568.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1568[0].clone();
        v_v = t1568[1].clone();
        if ({ let t1550 = v_k.clone(); Py::Bool({ let t1551 = v_outputs.clone(); if !py::contains(&t1551, &t1550)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1593(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1595: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1594 = v_inputs.clone(); f_jet_fuel_for_range(py::getitem(&t1594, &Py::str("R"))?, py::getitem(&t1594, &Py::str("V"))?, py::getitem(&t1594, &Py::str("c"))?, py::getitem(&t1594, &Py::str("LD"))?, py::getitem(&t1594, &Py::str("W0"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1595 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1596 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1615 = (t1596).items()?;
        if t1615.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1615[0].clone();
        v_v = t1615[1].clone();
        if ({ let t1597 = v_k.clone(); Py::Bool({ let t1598 = v_outputs.clone(); if !py::contains(&t1598, &t1597)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1660(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1662: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1661 = v_inputs.clone(); f_jet_range_constant_altitude(py::getitem(&t1661, &Py::str("W0"))?, py::getitem(&t1661, &Py::str("W1"))?, py::getitem(&t1661, &Py::str("S"))?, py::getitem(&t1661, &Py::str("h"))?, py::getitem(&t1661, &Py::str("CL"))?, py::getitem(&t1661, &Py::str("CD"))?, py::getitem(&t1661, &Py::str("c"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1662 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1663 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1682 = (t1663).items()?;
        if t1682.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1682[0].clone();
        v_v = t1682[1].clone();
        if ({ let t1664 = v_k.clone(); Py::Bool({ let t1665 = v_outputs.clone(); if !py::contains(&t1665, &t1664)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1775(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1777: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1776 = v_inputs.clone(); f_lambert(py::getitem(&t1776, &Py::str("r1"))?, py::getitem(&t1776, &Py::str("r2"))?, py::getitem(&t1776, &Py::str("tof"))?, py::method(&t1776, "get", &[Py::str("direction"), Py::str("prograde")])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1777 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1778 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1797 = (t1778).items()?;
        if t1797.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1797[0].clone();
        v_v = t1797[1].clone();
        if ({ let t1779 = v_k.clone(); Py::Bool({ let t1780 = v_outputs.clone(); if !py::contains(&t1780, &t1779)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1851(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1853: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1852 = v_inputs.clone(); f_landing_roll(py::getitem(&t1852, &Py::str("W"))?, py::getitem(&t1852, &Py::str("S"))?, py::getitem(&t1852, &Py::str("h"))?, py::getitem(&t1852, &Py::str("VTD"))?, py::getitem(&t1852, &Py::str("mu"))?, py::getitem(&t1852, &Py::str("CL"))?, py::getitem(&t1852, &Py::str("CD"))?, py::method(&t1852, "get", &[Py::str("Tr"), Py::Float(0.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1853 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1854 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1873 = (t1854).items()?;
        if t1873.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1873[0].clone();
        v_v = t1873[1].clone();
        if ({ let t1855 = v_k.clone(); Py::Bool({ let t1856 = v_outputs.clone(); if !py::contains(&t1856, &t1855)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1902(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1904: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1903 = v_inputs.clone(); f_level_flight_cl(py::getitem(&t1903, &Py::str("W"))?, py::getitem(&t1903, &Py::str("S"))?, py::getitem(&t1903, &Py::str("V"))?, py::getitem(&t1903, &Py::str("h"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1904 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1905 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1924 = (t1905).items()?;
        if t1924.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1924[0].clone();
        v_v = t1924[1].clone();
        if ({ let t1906 = v_k.clone(); Py::Bool({ let t1907 = v_outputs.clone(); if !py::contains(&t1907, &t1906)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_193(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t195: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t194 = v_inputs.clone(); f_ballistic_coefficient(py::getitem(&t194, &Py::str("m"))?, py::getitem(&t194, &Py::str("CD"))?, py::getitem(&t194, &Py::str("A"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t195 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t196 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t215 = (t196).items()?;
        if t215.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t215[0].clone();
        v_v = t215[1].clone();
        if ({ let t197 = v_k.clone(); Py::Bool({ let t198 = v_outputs.clone(); if !py::contains(&t198, &t197)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1943(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1945: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1944 = v_inputs.clone(); f_level_turn_bank(py::getitem(&t1944, &Py::str("V"))?, py::getitem(&t1944, &Py::str("phi"))?, py::method(&t1944, "get", &[Py::str("g"), c_G0()?])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1945 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1946 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1965 = (t1946).items()?;
        if t1965.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1965[0].clone();
        v_v = t1965[1].clone();
        if ({ let t1947 = v_k.clone(); Py::Bool({ let t1948 = v_outputs.clone(); if !py::contains(&t1948, &t1947)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1984(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1986: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1985 = v_inputs.clone(); f_level_turn_load_factor(py::getitem(&t1985, &Py::str("V"))?, py::getitem(&t1985, &Py::str("n"))?, py::method(&t1985, "get", &[Py::str("g"), c_G0()?])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1986 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1987 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2006 = (t1987).items()?;
        if t2006.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2006[0].clone();
        v_v = t2006[1].clone();
        if ({ let t1988 = v_k.clone(); Py::Bool({ let t1989 = v_outputs.clone(); if !py::contains(&t1989, &t1988)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2060(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2062: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2061 = v_inputs.clone(); f_lift_drag_forces(py::getitem(&t2061, &Py::str("V"))?, py::getitem(&t2061, &Py::str("S"))?, py::getitem(&t2061, &Py::str("CL"))?, py::getitem(&t2061, &Py::str("CD0"))?, py::getitem(&t2061, &Py::str("AR"))?, py::getitem(&t2061, &Py::str("e"))?, py::method(&t2061, "get", &[Py::str("rho"), Py::None])?, py::method(&t2061, "get", &[Py::str("h"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2062 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2063 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2082 = (t2063).items()?;
        if t2082.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2082[0].clone();
        v_v = t2082[1].clone();
        if ({ let t2064 = v_k.clone(); Py::Bool({ let t2065 = v_outputs.clone(); if !py::contains(&t2065, &t2064)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2138(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2140: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2139 = v_inputs.clone(); f_mach_from_area_ratio(py::getitem(&t2139, &Py::str("ar"))?, py::getitem(&t2139, &Py::str("regime"))?, py::method(&t2139, "get", &[Py::str("gamma"), Py::Float(1.4)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2140 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2141 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2160 = (t2141).items()?;
        if t2160.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2160[0].clone();
        v_v = t2160[1].clone();
        if ({ let t2142 = v_k.clone(); Py::Bool({ let t2143 = v_outputs.clone(); if !py::contains(&t2143, &t2142)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2210(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2212: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2211 = v_inputs.clone(); f_mach_from_pitot(py::getitem(&t2211, &Py::str("p"))?, py::getitem(&t2211, &Py::str("pt"))?, py::method(&t2211, "get", &[Py::str("gamma"), Py::Float(1.4)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2212 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2213 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2232 = (t2213).items()?;
        if t2232.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2232[0].clone();
        v_v = t2232[1].clone();
        if ({ let t2214 = v_k.clone(); Py::Bool({ let t2215 = v_outputs.clone(); if !py::contains(&t2215, &t2214)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2279(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2281: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2280 = v_inputs.clone(); f_mach_from_prandtl_meyer(py::getitem(&t2280, &Py::str("nu"))?, py::method(&t2280, "get", &[Py::str("gamma"), Py::Float(1.4)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2281 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2282 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2301 = (t2282).items()?;
        if t2301.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2301[0].clone();
        v_v = t2301[1].clone();
        if ({ let t2283 = v_k.clone(); Py::Bool({ let t2284 = v_outputs.clone(); if !py::contains(&t2284, &t2283)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2321(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2323: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2322 = v_inputs.clone(); f_mach_from_pressure_ratio(py::getitem(&t2322, &Py::str("pr"))?, py::method(&t2322, "get", &[Py::str("gamma"), Py::Float(1.4)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2323 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2324 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2343 = (t2324).items()?;
        if t2343.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2343[0].clone();
        v_v = t2343[1].clone();
        if ({ let t2325 = v_k.clone(); Py::Bool({ let t2326 = v_outputs.clone(); if !py::contains(&t2326, &t2325)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_237(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t239: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t238 = v_inputs.clone(); f_entry_peak(py::getitem(&t238, &Py::str("m"))?, py::getitem(&t238, &Py::str("CD"))?, py::getitem(&t238, &Py::str("A"))?, py::getitem(&t238, &Py::str("VE"))?, py::getitem(&t238, &Py::str("gamma_e"))?, py::method(&t238, "get", &[Py::str("H"), Py::Float(7200.0)])?, py::method(&t238, "get", &[Py::str("rho0"), Py::Float(1.225)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t239 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t240 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t259 = (t240).items()?;
        if t259.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t259[0].clone();
        v_v = t259[1].clone();
        if ({ let t241 = v_k.clone(); Py::Bool({ let t242 = v_outputs.clone(); if !py::contains(&t242, &t241)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2386(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2388: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2387 = v_inputs.clone(); f_max_climb_angle_jet(py::getitem(&t2387, &Py::str("W"))?, py::getitem(&t2387, &Py::str("S"))?, py::getitem(&t2387, &Py::str("h"))?, py::getitem(&t2387, &Py::str("T"))?, py::getitem(&t2387, &Py::str("CD0"))?, py::getitem(&t2387, &Py::str("AR"))?, py::getitem(&t2387, &Py::str("e"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2388 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2389 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2408 = (t2389).items()?;
        if t2408.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2408[0].clone();
        v_v = t2408[1].clone();
        if ({ let t2390 = v_k.clone(); Py::Bool({ let t2391 = v_outputs.clone(); if !py::contains(&t2391, &t2390)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2428(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2430: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2429 = v_inputs.clone(); f_max_deflection(py::getitem(&t2429, &Py::str("M1"))?, py::method(&t2429, "get", &[Py::str("gamma"), Py::Float(1.4)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2430 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2431 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2450 = (t2431).items()?;
        if t2450.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2450[0].clone();
        v_v = t2450[1].clone();
        if ({ let t2432 = v_k.clone(); Py::Bool({ let t2433 = v_outputs.clone(); if !py::contains(&t2433, &t2432)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2495(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2497: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2496 = v_inputs.clone(); f_max_rc_jet(py::getitem(&t2496, &Py::str("W"))?, py::getitem(&t2496, &Py::str("S"))?, py::getitem(&t2496, &Py::str("h"))?, py::getitem(&t2496, &Py::str("T"))?, py::getitem(&t2496, &Py::str("CD0"))?, py::getitem(&t2496, &Py::str("AR"))?, py::getitem(&t2496, &Py::str("e"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2497 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2498 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2517 = (t2498).items()?;
        if t2517.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2517[0].clone();
        v_v = t2517[1].clone();
        if ({ let t2499 = v_k.clone(); Py::Bool({ let t2500 = v_outputs.clone(); if !py::contains(&t2500, &t2499)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2565(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2567: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2566 = v_inputs.clone(); f_max_rc_prop(py::getitem(&t2566, &Py::str("W"))?, py::getitem(&t2566, &Py::str("S"))?, py::getitem(&t2566, &Py::str("h"))?, py::getitem(&t2566, &Py::str("P"))?, py::getitem(&t2566, &Py::str("eta"))?, py::getitem(&t2566, &Py::str("CD0"))?, py::getitem(&t2566, &Py::str("AR"))?, py::getitem(&t2566, &Py::str("e"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2567 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2568 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2587 = (t2568).items()?;
        if t2587.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2587[0].clone();
        v_v = t2587[1].clone();
        if ({ let t2569 = v_k.clone(); Py::Bool({ let t2570 = v_outputs.clone(); if !py::contains(&t2570, &t2569)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2627(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2629: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2628 = v_inputs.clone(); f_min_drag_power_speeds(py::getitem(&t2628, &Py::str("W"))?, py::getitem(&t2628, &Py::str("S"))?, py::getitem(&t2628, &Py::str("h"))?, py::getitem(&t2628, &Py::str("CD0"))?, py::getitem(&t2628, &Py::str("AR"))?, py::getitem(&t2628, &Py::str("e"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2629 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2630 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2649 = (t2630).items()?;
        if t2649.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2649[0].clone();
        v_v = t2649[1].clone();
        if ({ let t2631 = v_k.clone(); Py::Bool({ let t2632 = v_outputs.clone(); if !py::contains(&t2632, &t2631)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2676(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2678: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2677 = v_inputs.clone(); f_multistage_delta_v(py::getitem(&t2677, &Py::str("Isp"))?, py::getitem(&t2677, &Py::str("m0"))?, py::getitem(&t2677, &Py::str("mf"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2678 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2679 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2698 = (t2679).items()?;
        if t2698.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2698[0].clone();
        v_v = t2698[1].clone();
        if ({ let t2680 = v_k.clone(); Py::Bool({ let t2681 = v_outputs.clone(); if !py::contains(&t2681, &t2680)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2735(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2737: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2736 = v_inputs.clone(); f_naca4_thin_airfoil(py::getitem(&t2736, &Py::str("m"))?, py::getitem(&t2736, &Py::str("p"))?, py::getitem(&t2736, &Py::str("alpha"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2737 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2738 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2757 = (t2738).items()?;
        if t2757.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2757[0].clone();
        v_v = t2757[1].clone();
        if ({ let t2739 = v_k.clone(); Py::Bool({ let t2740 = v_outputs.clone(); if !py::contains(&t2740, &t2739)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2775(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2777: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2776 = v_inputs.clone(); f_neutral_point(py::getitem(&t2776, &Py::str("h_ac"))?, py::getitem(&t2776, &Py::str("a_w"))?, py::getitem(&t2776, &Py::str("a_t"))?, py::getitem(&t2776, &Py::str("St_S"))?, py::getitem(&t2776, &Py::str("lt_c"))?, py::getitem(&t2776, &Py::str("deps"))?, py::getitem(&t2776, &Py::str("h_cg"))?, py::method(&t2776, "get", &[Py::str("eta_t"), Py::Float(1.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2777 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2778 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2797 = (t2778).items()?;
        if t2797.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2797[0].clone();
        v_v = t2797[1].clone();
        if ({ let t2779 = v_k.clone(); Py::Bool({ let t2780 = v_outputs.clone(); if !py::contains(&t2780, &t2779)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2816(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2818: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2817 = v_inputs.clone(); f_normal_shock(py::getitem(&t2817, &Py::str("M1"))?, py::method(&t2817, "get", &[Py::str("gamma"), Py::Float(1.4)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2818 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2819 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2838 = (t2819).items()?;
        if t2838.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2838[0].clone();
        v_v = t2838[1].clone();
        if ({ let t2820 = v_k.clone(); Py::Bool({ let t2821 = v_outputs.clone(); if !py::contains(&t2821, &t2820)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_283(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t285: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t284 = v_inputs.clone(); f_entry_velocity(py::getitem(&t284, &Py::str("m"))?, py::getitem(&t284, &Py::str("CD"))?, py::getitem(&t284, &Py::str("A"))?, py::getitem(&t284, &Py::str("VE"))?, py::getitem(&t284, &Py::str("gamma_e"))?, py::getitem(&t284, &Py::str("h"))?, py::method(&t284, "get", &[Py::str("H"), Py::Float(7200.0)])?, py::method(&t284, "get", &[Py::str("rho0"), Py::Float(1.225)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t285 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t286 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t305 = (t286).items()?;
        if t305.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t305[0].clone();
        v_v = t305[1].clone();
        if ({ let t287 = v_k.clone(); Py::Bool({ let t288 = v_outputs.clone(); if !py::contains(&t288, &t287)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2857(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2859: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2858 = v_inputs.clone(); f_normal_shock_from_pressure_ratio(py::getitem(&t2858, &Py::str("p21"))?, py::method(&t2858, "get", &[Py::str("gamma"), Py::Float(1.4)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2859 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2860 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2879 = (t2860).items()?;
        if t2879.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2879[0].clone();
        v_v = t2879[1].clone();
        if ({ let t2861 = v_k.clone(); Py::Bool({ let t2862 = v_outputs.clone(); if !py::contains(&t2862, &t2861)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2938(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2940: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2939 = v_inputs.clone(); f_nozzle_normal_shock(py::getitem(&t2939, &Py::str("pc"))?, py::getitem(&t2939, &Py::str("pb"))?, py::getitem(&t2939, &Py::str("eps"))?, py::method(&t2939, "get", &[Py::str("gamma"), Py::Float(1.4)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2940 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2941 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2960 = (t2941).items()?;
        if t2960.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2960[0].clone();
        v_v = t2960[1].clone();
        if ({ let t2942 = v_k.clone(); Py::Bool({ let t2943 = v_outputs.clone(); if !py::contains(&t2943, &t2942)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3014(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3016: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3015 = v_inputs.clone(); f_oblique_shock(py::getitem(&t3015, &Py::str("M1"))?, py::getitem(&t3015, &Py::str("theta"))?, py::method(&t3015, "get", &[Py::str("branch"), Py::str("weak")])?, py::method(&t3015, "get", &[Py::str("gamma"), Py::Float(1.4)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3016 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3017 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3036 = (t3017).items()?;
        if t3036.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3036[0].clone();
        v_v = t3036[1].clone();
        if ({ let t3018 = v_k.clone(); Py::Bool({ let t3019 = v_outputs.clone(); if !py::contains(&t3019, &t3018)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3061(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3063: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3062 = v_inputs.clone(); f_oblique_from_wave_angle(py::getitem(&t3062, &Py::str("M1"))?, py::getitem(&t3062, &Py::str("beta"))?, py::method(&t3062, "get", &[Py::str("gamma"), Py::Float(1.4)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3063 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3064 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3083 = (t3064).items()?;
        if t3083.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3083[0].clone();
        v_v = t3083[1].clone();
        if ({ let t3065 = v_k.clone(); Py::Bool({ let t3066 = v_outputs.clone(); if !py::contains(&t3066, &t3065)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3155(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3157: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3156 = v_inputs.clone(); f_optimal_staging(py::getitem(&t3156, &Py::str("Isp"))?, py::getitem(&t3156, &Py::str("eps"))?, py::getitem(&t3156, &Py::str("dv"))?, py::getitem(&t3156, &Py::str("m_pl"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3157 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3158 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3177 = (t3158).items()?;
        if t3177.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3177[0].clone();
        v_v = t3177[1].clone();
        if ({ let t3159 = v_k.clone(); Py::Bool({ let t3160 = v_outputs.clone(); if !py::contains(&t3160, &t3159)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3201(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3203: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3202 = v_inputs.clone(); f_optimum_area_ratio(py::getitem(&t3202, &Py::str("gamma"))?, py::getitem(&t3202, &Py::str("pc"))?, py::getitem(&t3202, &Py::str("pa"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3203 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3204 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3223 = (t3204).items()?;
        if t3223.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3223[0].clone();
        v_v = t3223[1].clone();
        if ({ let t3205 = v_k.clone(); Py::Bool({ let t3206 = v_outputs.clone(); if !py::contains(&t3206, &t3205)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_324(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t326: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t325 = v_inputs.clone(); f_bielliptic(py::getitem(&t325, &Py::str("h1"))?, py::getitem(&t325, &Py::str("h2"))?, py::getitem(&t325, &Py::str("hb"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t326 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t327 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t346 = (t327).items()?;
        if t346.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t346[0].clone();
        v_v = t346[1].clone();
        if ({ let t328 = v_k.clone(); Py::Bool({ let t329 = v_outputs.clone(); if !py::contains(&t329, &t328)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3241(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3243: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3242 = v_inputs.clone(); f_prandtl_meyer(py::getitem(&t3242, &Py::str("M"))?, py::method(&t3242, "get", &[Py::str("gamma"), Py::Float(1.4)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3243 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3244 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3263 = (t3244).items()?;
        if t3263.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3263[0].clone();
        v_v = t3263[1].clone();
        if ({ let t3245 = v_k.clone(); Py::Bool({ let t3246 = v_outputs.clone(); if !py::contains(&t3246, &t3245)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3296(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3298: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3297 = v_inputs.clone(); f_pressure_altitude(py::getitem(&t3297, &Py::str("p"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3298 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3299 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3318 = (t3299).items()?;
        if t3318.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3318[0].clone();
        v_v = t3318[1].clone();
        if ({ let t3300 = v_k.clone(); Py::Bool({ let t3301 = v_outputs.clone(); if !py::contains(&t3301, &t3300)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3367(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3369: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3368 = v_inputs.clone(); f_prop_endurance(py::getitem(&t3368, &Py::str("eta"))?, py::getitem(&t3368, &Py::str("c"))?, py::getitem(&t3368, &Py::str("CL"))?, py::getitem(&t3368, &Py::str("CD"))?, py::getitem(&t3368, &Py::str("S"))?, py::getitem(&t3368, &Py::str("h"))?, py::getitem(&t3368, &Py::str("W0"))?, py::getitem(&t3368, &Py::str("W1"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3369 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3370 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3389 = (t3370).items()?;
        if t3389.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3389[0].clone();
        v_v = t3389[1].clone();
        if ({ let t3371 = v_k.clone(); Py::Bool({ let t3372 = v_outputs.clone(); if !py::contains(&t3372, &t3371)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3410(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3412: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3411 = v_inputs.clone(); f_propellant_for_delta_v(py::getitem(&t3411, &Py::str("dv"))?, py::getitem(&t3411, &Py::str("Isp"))?, py::getitem(&t3411, &Py::str("mf"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3412 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3413 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3432 = (t3413).items()?;
        if t3432.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3432[0].clone();
        v_v = t3432[1].clone();
        if ({ let t3414 = v_k.clone(); Py::Bool({ let t3415 = v_outputs.clone(); if !py::contains(&t3415, &t3414)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3450(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3452: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3451 = v_inputs.clone(); f_rayleigh(py::getitem(&t3451, &Py::str("M"))?, py::method(&t3451, "get", &[Py::str("gamma"), Py::Float(1.4)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3452 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3453 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3472 = (t3453).items()?;
        if t3472.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3472[0].clone();
        v_v = t3472[1].clone();
        if ({ let t3454 = v_k.clone(); Py::Bool({ let t3455 = v_outputs.clone(); if !py::contains(&t3455, &t3454)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3493(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3495: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3494 = v_inputs.clone(); f_rocket_delta_v(py::getitem(&t3494, &Py::str("Isp"))?, py::getitem(&t3494, &Py::str("m0"))?, py::getitem(&t3494, &Py::str("mf"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3495 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3496 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3515 = (t3496).items()?;
        if t3515.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3515[0].clone();
        v_v = t3515[1].clone();
        if ({ let t3497 = v_k.clone(); Py::Bool({ let t3498 = v_outputs.clone(); if !py::contains(&t3498, &t3497)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3582(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3584: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3583 = v_inputs.clone(); f_rocket_performance(py::getitem(&t3583, &Py::str("pc"))?, py::getitem(&t3583, &Py::str("Tc"))?, py::getitem(&t3583, &Py::str("Mw"))?, py::getitem(&t3583, &Py::str("gamma"))?, py::getitem(&t3583, &Py::str("At"))?, py::getitem(&t3583, &Py::str("eps"))?, py::getitem(&t3583, &Py::str("pa"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3584 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3585 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3604 = (t3585).items()?;
        if t3604.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3604[0].clone();
        v_v = t3604[1].clone();
        if ({ let t3586 = v_k.clone(); Py::Bool({ let t3587 = v_outputs.clone(); if !py::contains(&t3587, &t3586)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3630(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3632: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3631 = v_inputs.clone(); f_rocket_thrust(py::getitem(&t3631, &Py::str("mdot"))?, py::getitem(&t3631, &Py::str("ve"))?, py::method(&t3631, "get", &[Py::str("pe"), Py::None])?, py::method(&t3631, "get", &[Py::str("pa"), Py::None])?, py::method(&t3631, "get", &[Py::str("Ae"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3632 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3633 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3652 = (t3633).items()?;
        if t3652.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3652[0].clone();
        v_v = t3652[1].clone();
        if ({ let t3634 = v_k.clone(); Py::Bool({ let t3635 = v_outputs.clone(); if !py::contains(&t3635, &t3634)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3670(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3672: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3671 = v_inputs.clone(); f_satellite_coverage(py::getitem(&t3671, &Py::str("h"))?, py::method(&t3671, "get", &[Py::str("eps_min"), Py::Float(0.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3672 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3673 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3692 = (t3673).items()?;
        if t3692.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3692[0].clone();
        v_v = t3692[1].clone();
        if ({ let t3674 = v_k.clone(); Py::Bool({ let t3675 = v_outputs.clone(); if !py::contains(&t3675, &t3674)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_371(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t373: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t372 = v_inputs.clone(); f_breguet_endurance_jet(py::getitem(&t372, &Py::str("c"))?, py::getitem(&t372, &Py::str("LD"))?, py::getitem(&t372, &Py::str("W0"))?, py::getitem(&t372, &Py::str("W1"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t373 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t374 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t393 = (t374).items()?;
        if t393.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t393[0].clone();
        v_v = t393[1].clone();
        if ({ let t375 = v_k.clone(); Py::Bool({ let t376 = v_outputs.clone(); if !py::contains(&t376, &t375)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3763(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3765: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3764 = v_inputs.clone(); f_service_ceiling_jet(py::getitem(&t3764, &Py::str("W"))?, py::getitem(&t3764, &Py::str("S"))?, py::getitem(&t3764, &Py::str("T0"))?, py::getitem(&t3764, &Py::str("CD0"))?, py::getitem(&t3764, &Py::str("AR"))?, py::getitem(&t3764, &Py::str("e"))?, py::method(&t3764, "get", &[Py::str("RCs"), Py::Float(0.508)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3765 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3766 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3785 = (t3766).items()?;
        if t3785.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3785[0].clone();
        v_v = t3785[1].clone();
        if ({ let t3767 = v_k.clone(); Py::Bool({ let t3768 = v_outputs.clone(); if !py::contains(&t3768, &t3767)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3818(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3820: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3819 = v_inputs.clone(); f_stall_speed(py::getitem(&t3819, &Py::str("W"))?, py::getitem(&t3819, &Py::str("S"))?, py::getitem(&t3819, &Py::str("CLmax"))?, py::getitem(&t3819, &Py::str("h"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3820 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3821 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3840 = (t3821).items()?;
        if t3840.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3840[0].clone();
        v_v = t3840[1].clone();
        if ({ let t3822 = v_k.clone(); Py::Bool({ let t3823 = v_outputs.clone(); if !py::contains(&t3823, &t3822)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3856(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3858: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3857 = v_inputs.clone(); f_sun_synchronous(py::getitem(&t3857, &Py::str("h"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3858 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3859 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3878 = (t3859).items()?;
        if t3878.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3878[0].clone();
        v_v = t3878[1].clone();
        if ({ let t3860 = v_k.clone(); Py::Bool({ let t3861 = v_outputs.clone(); if !py::contains(&t3861, &t3860)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_39(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t41: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t40 = v_inputs.clone(); f_absolute_ceiling_jet(py::getitem(&t40, &Py::str("W"))?, py::getitem(&t40, &Py::str("S"))?, py::getitem(&t40, &Py::str("T0"))?, py::getitem(&t40, &Py::str("CD0"))?, py::getitem(&t40, &Py::str("AR"))?, py::getitem(&t40, &Py::str("e"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t41 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t42 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t61 = (t42).items()?;
        if t61.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t61[0].clone();
        v_v = t61[1].clone();
        if ({ let t43 = v_k.clone(); Py::Bool({ let t44 = v_outputs.clone(); if !py::contains(&t44, &t43)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3929(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3931: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3930 = v_inputs.clone(); f_takeoff_roll(py::getitem(&t3930, &Py::str("W"))?, py::getitem(&t3930, &Py::str("S"))?, py::getitem(&t3930, &Py::str("h"))?, py::getitem(&t3930, &Py::str("T"))?, py::getitem(&t3930, &Py::str("mu"))?, py::getitem(&t3930, &Py::str("CL"))?, py::getitem(&t3930, &Py::str("CD"))?, py::getitem(&t3930, &Py::str("VLO"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3931 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3932 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3951 = (t3932).items()?;
        if t3951.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3951[0].clone();
        v_v = t3951[1].clone();
        if ({ let t3933 = v_k.clone(); Py::Bool({ let t3934 = v_outputs.clone(); if !py::contains(&t3934, &t3933)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4020(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4022: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4021 = v_inputs.clone(); f_throat_area_for_thrust(py::getitem(&t4021, &Py::str("F"))?, py::getitem(&t4021, &Py::str("pc"))?, py::getitem(&t4021, &Py::str("Tc"))?, py::getitem(&t4021, &Py::str("Mw"))?, py::getitem(&t4021, &Py::str("gamma"))?, py::getitem(&t4021, &Py::str("eps"))?, py::getitem(&t4021, &Py::str("pa"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4022 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4023 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4042 = (t4023).items()?;
        if t4042.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4042[0].clone();
        v_v = t4042[1].clone();
        if ({ let t4024 = v_k.clone(); Py::Bool({ let t4025 = v_outputs.clone(); if !py::contains(&t4025, &t4024)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4105(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4107: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4106 = v_inputs.clone(); f_thrust_coefficient(py::getitem(&t4106, &Py::str("gamma"))?, py::getitem(&t4106, &Py::str("eps"))?, py::getitem(&t4106, &Py::str("pc"))?, py::getitem(&t4106, &Py::str("pa"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4107 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4108 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4127 = (t4108).items()?;
        if t4127.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4127[0].clone();
        v_v = t4127[1].clone();
        if ({ let t4109 = v_k.clone(); Py::Bool({ let t4110 = v_outputs.clone(); if !py::contains(&t4110, &t4109)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4148(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4150: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4149 = v_inputs.clone(); f_time_to_climb(py::getitem(&t4149, &Py::str("RC0"))?, py::getitem(&t4149, &Py::str("Hc"))?, py::getitem(&t4149, &Py::str("h1"))?, py::getitem(&t4149, &Py::str("h2"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4150 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4151 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4170 = (t4151).items()?;
        if t4170.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4170[0].clone();
        v_v = t4170[1].clone();
        if ({ let t4152 = v_k.clone(); Py::Bool({ let t4153 = v_outputs.clone(); if !py::contains(&t4153, &t4152)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_418(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t420: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t419 = v_inputs.clone(); f_breguet_jet(py::getitem(&t419, &Py::str("V"))?, py::getitem(&t419, &Py::str("c"))?, py::getitem(&t419, &Py::str("LD"))?, py::getitem(&t419, &Py::str("W0"))?, py::getitem(&t419, &Py::str("W1"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t420 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t421 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t440 = (t421).items()?;
        if t440.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t440[0].clone();
        v_v = t440[1].clone();
        if ({ let t422 = v_k.clone(); Py::Bool({ let t423 = v_outputs.clone(); if !py::contains(&t423, &t422)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4190(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4192: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4191 = v_inputs.clone(); f_total_temperature(py::getitem(&t4191, &Py::str("V"))?, py::getitem(&t4191, &Py::str("T"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4192 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4193 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4212 = (t4193).items()?;
        if t4212.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4212[0].clone();
        v_v = t4212[1].clone();
        if ({ let t4194 = v_k.clone(); Py::Bool({ let t4195 = v_outputs.clone(); if !py::contains(&t4195, &t4194)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4251(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4253: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4252 = v_inputs.clone(); f_true_airspeed_from_cas(py::getitem(&t4252, &Py::str("CAS"))?, py::getitem(&t4252, &Py::str("hp"))?, py::method(&t4252, "get", &[Py::str("T"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4253 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4254 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4273 = (t4254).items()?;
        if t4273.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4273[0].clone();
        v_v = t4273[1].clone();
        if ({ let t4255 = v_k.clone(); Py::Bool({ let t4256 = v_outputs.clone(); if !py::contains(&t4256, &t4255)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4299(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4301: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4300 = v_inputs.clone(); f_vertical_rocket(py::getitem(&t4300, &Py::str("Isp"))?, py::getitem(&t4300, &Py::str("m0"))?, py::getitem(&t4300, &Py::str("mf"))?, py::getitem(&t4300, &Py::str("tb"))?, py::method(&t4300, "get", &[Py::str("g"), c_G0()?])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4301 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4302 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4321 = (t4302).items()?;
        if t4321.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4321[0].clone();
        v_v = t4321[1].clone();
        if ({ let t4303 = v_k.clone(); Py::Bool({ let t4304 = v_outputs.clone(); if !py::contains(&t4304, &t4303)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4349(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4351: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4350 = v_inputs.clone(); f_wind_triangle(py::getitem(&t4350, &Py::str("TAS"))?, py::getitem(&t4350, &Py::str("course"))?, py::getitem(&t4350, &Py::str("wind_from"))?, py::getitem(&t4350, &Py::str("wind_speed"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4351 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4352 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4371 = (t4352).items()?;
        if t4371.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4371[0].clone();
        v_v = t4371[1].clone();
        if ({ let t4353 = v_k.clone(); Py::Bool({ let t4354 = v_outputs.clone(); if !py::contains(&t4354, &t4353)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4386(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4388: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4387 = v_inputs.clone(); f_wing_loading(py::getitem(&t4387, &Py::str("W"))?, py::getitem(&t4387, &Py::str("S"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4388 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4389 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4408 = (t4389).items()?;
        if t4408.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4408[0].clone();
        v_v = t4408[1].clone();
        if ({ let t4390 = v_k.clone(); Py::Bool({ let t4391 = v_outputs.clone(); if !py::contains(&t4391, &t4390)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_469(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t471: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t470 = v_inputs.clone(); f_breguet_prop(py::getitem(&t470, &Py::str("eta"))?, py::getitem(&t470, &Py::str("c"))?, py::getitem(&t470, &Py::str("LD"))?, py::getitem(&t470, &Py::str("W0"))?, py::getitem(&t470, &Py::str("W1"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t471 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t472 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t491 = (t472).items()?;
        if t491.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t491[0].clone();
        v_v = t491[1].clone();
        if ({ let t473 = v_k.clone(); Py::Bool({ let t474 = v_outputs.clone(); if !py::contains(&t474, &t473)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_530(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t532: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t531 = v_inputs.clone(); f_calibrated_airspeed(py::getitem(&t531, &Py::str("V"))?, py::getitem(&t531, &Py::str("hp"))?, py::method(&t531, "get", &[Py::str("T"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t532 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t533 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t552 = (t533).items()?;
        if t552.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t552[0].clone();
        v_v = t552[1].clone();
        if ({ let t534 = v_k.clone(); Py::Bool({ let t535 = v_outputs.clone(); if !py::contains(&t535, &t534)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_575(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t577: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t576 = v_inputs.clone(); f_characteristic_velocity(py::getitem(&t576, &Py::str("Tc"))?, py::getitem(&t576, &Py::str("Mw"))?, py::getitem(&t576, &Py::str("gamma"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t577 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t578 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t597 = (t578).items()?;
        if t597.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t597[0].clone();
        v_v = t597[1].clone();
        if ({ let t579 = v_k.clone(); Py::Bool({ let t580 = v_outputs.clone(); if !py::contains(&t580, &t579)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_616(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t618: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t617 = v_inputs.clone(); f_choked_mass_flow(py::getitem(&t617, &Py::str("p0"))?, py::getitem(&t617, &Py::str("T0"))?, py::getitem(&t617, &Py::str("A"))?, py::method(&t617, "get", &[Py::str("gamma"), Py::Float(1.4)])?, py::method(&t617, "get", &[Py::str("R"), c_R_AIR()?])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t618 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t619 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t638 = (t619).items()?;
        if t638.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t638[0].clone();
        v_v = t638[1].clone();
        if ({ let t620 = v_k.clone(); Py::Bool({ let t621 = v_outputs.clone(); if !py::contains(&t621, &t620)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_652(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t654: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t653 = v_inputs.clone(); f_circular_orbit(py::getitem(&t653, &Py::str("h"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t654 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t655 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t674 = (t655).items()?;
        if t674.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t674[0].clone();
        v_v = t674[1].clone();
        if ({ let t656 = v_k.clone(); Py::Bool({ let t657 = v_outputs.clone(); if !py::contains(&t657, &t656)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_719(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t721: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t720 = v_inputs.clone(); f_critical_mach(py::getitem(&t720, &Py::str("Cp0"))?, py::getitem(&t720, &Py::str("rule"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t721 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t722 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t741 = (t722).items()?;
        if t741.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t741[0].clone();
        v_v = t741[1].clone();
        if ({ let t723 = v_k.clone(); Py::Bool({ let t724 = v_outputs.clone(); if !py::contains(&t724, &t723)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_76(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t78: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t77 = v_inputs.clone(); f_actuator_disk_forward(py::getitem(&t77, &Py::str("T"))?, py::getitem(&t77, &Py::str("V"))?, py::getitem(&t77, &Py::str("D"))?, py::getitem(&t77, &Py::str("rho"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t78 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t79 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t98 = (t79).items()?;
        if t98.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t98[0].clone();
        v_v = t98[1].clone();
        if ({ let t80 = v_k.clone(); Py::Bool({ let t81 = v_outputs.clone(); if !py::contains(&t81, &t80)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_765(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t767: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t766 = v_inputs.clone(); f_gravity_loss(py::getitem(&t766, &Py::str("Isp"))?, py::getitem(&t766, &Py::str("m0"))?, py::getitem(&t766, &Py::str("mf"))?, py::getitem(&t766, &Py::str("tb"))?, py::method(&t766, "get", &[Py::str("g"), c_G0()?])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t767 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t768 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t787 = (t768).items()?;
        if t787.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t787[0].clone();
        v_v = t787[1].clone();
        if ({ let t769 = v_k.clone(); Py::Bool({ let t770 = v_outputs.clone(); if !py::contains(&t770, &t769)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_830(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t832: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t831 = v_inputs.clone(); f_density_altitude(py::getitem(&t831, &Py::str("hp"))?, py::getitem(&t831, &Py::str("T"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t832 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t833 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t852 = (t833).items()?;
        if t852.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t852[0].clone();
        v_v = t852[1].clone();
        if ({ let t834 = v_k.clone(); Py::Bool({ let t835 = v_outputs.clone(); if !py::contains(&t835, &t834)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_875(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t877: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t876 = v_inputs.clone(); f_drag_polar_optimum(py::getitem(&t876, &Py::str("CD0"))?, py::getitem(&t876, &Py::str("AR"))?, py::getitem(&t876, &Py::str("e"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t877 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t878 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t897 = (t878).items()?;
        if t897.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t897[0].clone();
        v_v = t897[1].clone();
        if ({ let t879 = v_k.clone(); Py::Bool({ let t880 = v_outputs.clone(); if !py::contains(&t880, &t879)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_921(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t923: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t922 = v_inputs.clone(); f_elliptic_wing(py::getitem(&t922, &Py::str("AR"))?, py::getitem(&t922, &Py::str("alpha"))?, py::method(&t922, "get", &[Py::str("a0"), py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t923 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t924 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t943 = (t924).items()?;
        if t943.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t943[0].clone();
        v_v = t943[1].clone();
        if ({ let t925 = v_k.clone(); Py::Bool({ let t926 = v_outputs.clone(); if !py::contains(&t926, &t925)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_958(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t960: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t959 = v_inputs.clone(); f_escape_speed(py::getitem(&t959, &Py::str("h"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t960 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t961 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t980 = (t961).items()?;
        if t980.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t980[0].clone();
        v_v = t980[1].clone();
        if ({ let t962 = v_k.clone(); Py::Bool({ let t963 = v_outputs.clone(); if !py::contains(&t963, &t962)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}
