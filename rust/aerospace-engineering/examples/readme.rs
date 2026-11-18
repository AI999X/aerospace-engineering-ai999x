//! Aerospace Engineering: 15 calculations through the Rust library (the same engine as the desktop app, the
//! command line and the browser). Run from the rust/ folder:
//!     cargo run --release --example readme -p aerospace-engineering-ai999x
//! Inputs are text, read like a form field ("2 m", "0.5", "x^3 - 3x"); a JSON list stays a list.

use serde_json::{json, Value};

/// One calculation: the request in, its outcome and every value with its unit out.
fn show(request: Value) {
    let answer: Value = serde_json::from_str(&aerospace_engineering_ai999x::solve_json(&request.to_string())).unwrap();
    println!("{}", answer["outcome"].as_str().unwrap_or(""));
    for v in answer["values"].as_array().into_iter().flatten() {
        let unit = v[2].as_str().unwrap_or("");
        let unit = if unit.is_empty() || unit == "1" { String::new() } else { format!(" {unit}") };
        println!("{} = {}{}", v[0].as_str().unwrap_or(""), v[1], unit);
    }
}

fn main() {
    show(json!({"key": "aero_critical_mach", "inputs": {"Cp0": "-0.43", "rule": "prandtl-glauert"}}));
    println!();
    show(json!({"key": "aero_absolute_ceiling_jet", "inputs": {"W": "100000.0", "S": "30.0", "T0": "13486.71062689471", "CD0": "0.02", "AR": "7.0", "e": "0.8"}}));
    println!();
    show(json!({"key": "aero_calibrated_airspeed", "inputs": {"V": "77.16666667 m/s", "hp": "0 ft"}}));
    println!();
    show(json!({"key": "aero_fanno_flow", "inputs": {"M": "2"}}));
    println!();
    show(json!({"key": "aero_choked_mass_flow", "inputs": {"p0": "100 kPa", "T0": "300 K", "A": "10 cm^2"}}));
    println!();
    show(json!({"key": "aero_bielliptic_transfer", "inputs": {"h1": "6378136.6", "h2": "19134409.799999997", "hb": "19134409.799999997"}}));
    println!();
    show(json!({"key": "aero_actuator_disk_hover", "inputs": {"T": "6.283185307 N", "D": "2 m", "rho": "1 kg/m^3"}}));
    println!();
    show(json!({"key": "aero_ballistic_coefficient", "inputs": {"m": "1000 kg", "CD": "1.2", "A": "1.5 m^2"}}));
    println!();
    show(json!({"key": "aero_vertical_rocket_trajectory", "inputs": {"Isp": "300 s", "m0": "2718.281828 kg", "mf": "1000 kg", "tb": "100 s"}}));
    println!();
    show(json!({"key": "aero_characteristic_velocity", "inputs": {"Tc": "3000 K", "Mw": "20 g/mol", "gamma": "1.2"}}));
    println!();
    show(json!({"key": "aero_rocket_thrust", "inputs": {"mdot": "250 kg/s", "ve": "3000 m/s"}}));
    println!();
    show(json!({"key": "aero_neutral_point", "inputs": {"h_ac": "0.25", "a_w": "5", "a_t": "5", "St_S": "0.2", "lt_c": "4", "deps": "0.5", "h_cg": "0.3"}}));
    println!();
    show(json!({"key": "aero_density_altitude", "inputs": {"hp": "0 ft", "T": "15 °C"}}));
    println!();
    show(json!({"key": "aero_elliptic_wing_lifting_line", "inputs": {"AR": "2", "alpha": "0.1 rad"}}));
    println!();
    show(json!({"key": "aero_drag_polar_optimum", "inputs": {"CD0": "0.02", "AR": "10", "e": "0.8"}}));
}
