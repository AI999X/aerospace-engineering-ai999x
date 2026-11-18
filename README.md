# Aerospace Engineering (aerospace-engineering-ai999x)

[![tests](https://github.com/AI999X/aerospace-engineering-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/aerospace-engineering-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/aerospace-engineering-ai999x)](https://pypi.org/project/aerospace-engineering-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/aerospace-engineering-ai999x)

Aerospace Engineering covers the standard atmosphere, lift and drag, aircraft performance, stability, propulsion and rocket performance, nozzles and orbital transfers: 81 calculations in 13 topics, each with its formula, its steps and its units, and a plain reason instead of a number when an input is missing or out of range. One compiled engine, four ways to use it: the desktop app, the command line, the browser and Python. Everything runs on your own computer, offline.

[![The Aerospace Engineering desktop app: Standard atmosphere at an altitude, with its worked example and live scene](https://www.ai999x.com/?ax_shot=aerospace-engineering-ai999x/hero.webp)](https://www.ai999x.com/models/aerospace-engineering-ai999x/)

## Four ways to use it

| | |
|---|---|
| **Desktop app** (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-aerospace-engineering-1.6.5-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-aerospace-engineering-1.6.5-linux-x86_64.tar.gz) |
| **Command line** `aerospace-engineering` (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-cli-1.6.4-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-cli-1.6.4-linux-x86_64.tar.gz) |
| **Browser** (any device) | [Try it on the model page](https://www.ai999x.com/models/aerospace-engineering-ai999x/#capabilities) |
| **Python** 3.10+ | `pip install aerospace-engineering-ai999x`: on Windows and Linux (x86-64) `pip` installs the same compiled engine; on other systems (macOS, ARM) it installs a pure-Python build that gives the same answers |

## See it

From the Aerospace Engineering desktop app: every scene and chart is drawn from the computed answer.

[![Aerospace Engineering desktop app: a slider moves an input and the answer, the scene and the charts follow](https://www.ai999x.com/?ax_shot=aerospace-engineering-ai999x/clip-loop.webp)](https://www.ai999x.com/?ax_shot=aerospace-engineering-ai999x/clip.mp4)

*[Watch the full clip (MP4)](https://www.ai999x.com/?ax_shot=aerospace-engineering-ai999x/clip.mp4)*

| | |
|---|---|
| [![Standard atmosphere at an altitude: the live scene in the Aerospace Engineering desktop app](https://www.ai999x.com/?ax_shot=aerospace-engineering-ai999x/scene-1.webp)](https://www.ai999x.com/models/aerospace-engineering-ai999x/#cap-aero_isa_properties)<br>Standard atmosphere at an altitude | [![Normal shock relations: the live scene in the Aerospace Engineering desktop app](https://www.ai999x.com/?ax_shot=aerospace-engineering-ai999x/scene-2.webp)](https://www.ai999x.com/models/aerospace-engineering-ai999x/#cap-aero_normal_shock)<br>Normal shock relations |
| [![True airspeed from calibrated airspeed: the live scene in the Aerospace Engineering desktop app](https://www.ai999x.com/?ax_shot=aerospace-engineering-ai999x/scene-3.webp)](https://www.ai999x.com/models/aerospace-engineering-ai999x/#cap-aero_true_airspeed_from_calibrated)<br>True airspeed from calibrated airspeed | [![Breguet range of a jet aircraft: the live scene in the Aerospace Engineering desktop app](https://www.ai999x.com/?ax_shot=aerospace-engineering-ai999x/scene-4.webp)](https://www.ai999x.com/models/aerospace-engineering-ai999x/#cap-aero_breguet_range_jet)<br>Breguet range of a jet aircraft |
| [![Critical Mach number of an airfoil: the live scene in the Aerospace Engineering desktop app](https://www.ai999x.com/?ax_shot=aerospace-engineering-ai999x/scene-5.webp)](https://www.ai999x.com/models/aerospace-engineering-ai999x/#cap-aero_critical_mach)<br>Critical Mach number of an airfoil | [![Choked mass flow through a nozzle throat: the live scene in the Aerospace Engineering desktop app](https://www.ai999x.com/?ax_shot=aerospace-engineering-ai999x/scene-6.webp)](https://www.ai999x.com/models/aerospace-engineering-ai999x/#cap-aero_choked_mass_flow)<br>Choked mass flow through a nozzle throat |
| [![Circular-orbit altitude for an orbital period: the live scene in the Aerospace Engineering desktop app](https://www.ai999x.com/?ax_shot=aerospace-engineering-ai999x/scene-7.webp)](https://www.ai999x.com/models/aerospace-engineering-ai999x/#cap-aero_altitude_for_period)<br>Circular-orbit altitude for an orbital period | [![Ideal hover power of a rotor (momentum theory): the live scene in the Aerospace Engineering desktop app](https://www.ai999x.com/?ax_shot=aerospace-engineering-ai999x/scene-8.webp)](https://www.ai999x.com/models/aerospace-engineering-ai999x/#cap-aero_actuator_disk_hover)<br>Ideal hover power of a rotor (momentum theory) |

Charts:

|   |   |   |   |
|---|---|---|---|
| [![Standard atmosphere at an altitude: a chart in the Aerospace Engineering desktop app](https://www.ai999x.com/?ax_shot=aerospace-engineering-ai999x/chart-1.webp)](https://www.ai999x.com/models/aerospace-engineering-ai999x/#cap-aero_isa_properties)<br>Standard atmosphere at an altitude | [![Normal shock relations: a chart in the Aerospace Engineering desktop app](https://www.ai999x.com/?ax_shot=aerospace-engineering-ai999x/chart-2.webp)](https://www.ai999x.com/models/aerospace-engineering-ai999x/#cap-aero_normal_shock)<br>Normal shock relations | [![True airspeed from calibrated airspeed: a chart in the Aerospace Engineering desktop app](https://www.ai999x.com/?ax_shot=aerospace-engineering-ai999x/chart-3.webp)](https://www.ai999x.com/models/aerospace-engineering-ai999x/#cap-aero_true_airspeed_from_calibrated)<br>True airspeed from calibrated airspeed | [![Breguet range of a jet aircraft: a chart in the Aerospace Engineering desktop app](https://www.ai999x.com/?ax_shot=aerospace-engineering-ai999x/chart-4.webp)](https://www.ai999x.com/models/aerospace-engineering-ai999x/#cap-aero_breguet_range_jet)<br>Breguet range of a jet aircraft |

## Ask in words: 13 examples

Each question below was asked on this release; the comment under it is the line it printed.

```python
import aerospace_engineering_ai999x as aerospace_engineering

a = aerospace_engineering.ask(
    "Critical Mach: Cp0 = -0.43, rule = prandtl-glauert"
)
print(a.outcome, a.values)
# SOLVED {'M_cr': 0.737105914227, 'Cp_cr': -0.636304358584}

a = aerospace_engineering.ask(
    "Ballistic coefficient: m = 1000 kg, CD = 1.2, A = 1.5 m^2"
)
print(a.outcome, a.values)
# SOLVED {'beta': 555.555555556, 'beta_psf': 113.786746457}

a = aerospace_engineering.ask(
    "Characteristic velocity: Tc = 3000 K, Mw = 20 g/mol, gamma = 1.2"
)
print(a.outcome, a.values)
# SOLVED {'c_star': 1721.99492587, 'Gamma': 0.648531170706}

a = aerospace_engineering.ask("Geopotential height: h = 0 km")
print(a.outcome, a.values)
# SOLVED {'H': 0.0, 'g': 9.80665, 'dH': 0.0}

a = aerospace_engineering.ask(
    "Mach from area ratio: ar = 1.6875, regime = supersonic"
)
print(a.outcome, a.values)
# SOLVED {'M': 2.0, 'p_p0': 0.127804525463, 'T_T0': 0.555555555556}

a = aerospace_engineering.ask("Pressure altitude: p = 1013.25 hPa")
print(a.outcome, a.values)
# SOLVED {'h': 0.0, 'h_ft': 0.0, 'H': 0.0, 'T_std': 288.15}

a = aerospace_engineering.ask("Sun-synchronous orbit: h = 700 km")
print(a.outcome, a.values)
# SOLVED {'i_deg': 98.1879559879, 'period_min': 98.7729761461}

a = aerospace_engineering.ask(
    "Propellant for delta-v: dv = 2941.995 m/s, Isp = 300 s, mf = 1000 "
    "kg"
)
print(a.outcome, a.values)
# SOLVED {'m0': 2718.28182846, 'mp': 1718.28182846, 'R': 2.71828182846}

a = aerospace_engineering.ask(
    "Rocket delta-v: Isp = 450 s, m0 = 4000 kg, mf = 1000 kg"
)
print(a.outcome, a.values)
# SOLVED {'dv': 6117.70661841}

a = aerospace_engineering.ask(
    "Landing roll: W = 10 kN, S = 10 m^2, h = 0 m, VTD = 30 m/s, mu = "
    "0.4, CL = 0.25, CD = 0.1"
)
print(a.outcome, a.values)
# SOLVED {'s': 114.71807396, 't': 7.64787159733}

a = aerospace_engineering.ask("Prandtl-Meyer angle: M = 2")
print(a.outcome, a.values)
# SOLVED {'nu_deg': 26.3797608134}

a = aerospace_engineering.ask(
    "Lift coefficient in level flight: W = 10 kN, S = 16 m^2, V = 50 "
    "m/s, h = 0 m"
)
print(a.outcome, a.values)
# SOLVED {'CL': 0.40816354656, 'q': 1531.24894486, 'wing_loading': 625.0}

a = aerospace_engineering.ask(
    "Takeoff roll: W = 10 kN, S = 10 m^2, h = 0 m, T = 3 kN, mu = "
    "0.02, CL = 1, CD = 0.02, VLO = 40 m/s"
)
print(a.outcome, a.values)
# SOLVED {'s': 291.347489422, 't': 14.5673744711}
```

## Command line: 15 examples

Each command below was run with `aerospace-engineering` of this release; the lines under it are the answer it printed (the full answer goes on with the method, every input in SI units and the conditions it assumes).

```text
$ aerospace-engineering solve aero_critical_mach Cp0=-0.43 rule=prandtl-glauert
M_cr = 0.7371059142
Cp_cr = -0.6363043586

$ aerospace-engineering solve aero_absolute_ceiling_jet W=100000.0 S=30.0 T0=13486.71062689471 CD0=0.02 AR=7.0 e=0.8
h_abs = 6669.76749 m
h_abs_ft = 21882.43927
sigma = 0.5

$ aerospace-engineering solve aero_calibrated_airspeed "V=77.16666667 m/s" "hp=0 ft"
CAS = 77.16666667 m/s
CAS_kt = 150
qc = 3694.376623 Pa
pt = 105019.3766 Pa

$ aerospace-engineering solve aero_fanno_flow M=2
fLD = 0.3049965026
p_pstar = 0.4082482905
T_Tstar = 0.6666666667
rho_rhostar = 0.6123724357
p0_p0star = 1.6875

$ aerospace-engineering solve aero_choked_mass_flow "p0=100 kPa" "T0=300 K" "A=10 cm^2"
mdot = 0.233334283 kg/s
p_star = 52828.17877 Pa
T_star = 250 K
V_star = 316.9677826 m/s

$ aerospace-engineering solve aero_bielliptic_transfer h1=6378136.6 h2=19134409.799999997 hb=19134409.799999997
dv1 = 864.7664 m/s
dv2 = 725.330842 m/s
dv3 = 0 m/s
dv = 1590.097242 m/s
t = 33447.91358 s
t_days = 0.3871286294

$ aerospace-engineering solve aero_actuator_disk_hover "T=6.283185307 N" "D=2 m" "rho=1 kg/m^3"
v_i = 1 m/s
P_ideal = 6.283185307 W
w = 2 m/s

$ aerospace-engineering solve aero_ballistic_coefficient "m=1000 kg" CD=1.2 "A=1.5 m^2"
beta = 555.5555556 kg/m^2
beta_psf = 113.7867465

$ aerospace-engineering solve aero_vertical_rocket_trajectory "Isp=300 s" "m0=2718.281828 kg" "mf=1000 kg" "tb=100 s"
v_b = 1961.329999 m/s
h_b = 73948.99381 m
h_max = 270081.9937 m
t_apex = 299.9999999 s

$ aerospace-engineering solve aero_characteristic_velocity "Tc=3000 K" "Mw=20 g/mol" gamma=1.2
c_star = 1721.994926 m/s
Gamma = 0.6485311707

$ aerospace-engineering solve aero_rocket_thrust "mdot=250 kg/s" "ve=3000 m/s"
F = 750000 N
F_kN = 750
Isp = 305.9148639 s
c_eff = 3000 m/s
F_pressure = 0 N

$ aerospace-engineering solve aero_neutral_point h_ac=0.25 a_w=5 a_t=5 St_S=0.2 lt_c=4 deps=0.5 h_cg=0.3
h_n = 0.6136363636
SM = 0.3136363636
CL_alpha = 5.5
Cm_alpha = -1.725

$ aerospace-engineering solve aero_density_altitude "hp=0 ft" "T=15 °C"
rho = 1.224999156 kg/m^3
sigma = 1
h_rho = 0 m
h_rho_ft = 0
p = 101325 Pa

$ aerospace-engineering solve aero_elliptic_wing_lifting_line AR=2 "alpha=0.1 rad"
CL_alpha = 3.141592654
CL = 0.3141592654
CDi = 0.01570796327
alpha_i_deg = 2.864788976

$ aerospace-engineering solve aero_drag_polar_optimum CD0=0.02 AR=10 e=0.8
CL_md = 0.7089815404
LD_max = 17.72453851
CL_mp = 1.22799205
LD_mp = 15.34990062
CL_mr = 0.4093306832
LD_mr = 15.34990062
```

## Rust library: 15 examples

The crate `aerospace-engineering-ai999x` (in `rust/` of the GitHub repository) answers a JSON request with `aerospace_engineering_ai999x::solve_json`. Each call below was compiled and run on this release; the comment under it is what it printed. The whole program is `rust/aerospace-engineering/examples/readme.rs`: `cd rust && cargo run --release --example readme -p aerospace-engineering-ai999x`.

```rust
use serde_json::{json, Value};

fn show(request: Value) {
    let answer: Value = serde_json::from_str(&aerospace_engineering_ai999x::solve_json(&request.to_string())).unwrap();
    println!("{}", answer["outcome"].as_str().unwrap_or(""));
    for v in answer["values"].as_array().into_iter().flatten() {
        let unit = v[2].as_str().unwrap_or("");
        let unit = if unit.is_empty() || unit == "1" { String::new() } else { format!(" {unit}") };
        println!("{} = {}{}", v[0].as_str().unwrap_or(""), v[1], unit);
    }
}

show(json!({"key": "aero_critical_mach", "inputs": {"Cp0": "-0.43", "rule": "prandtl-glauert"}}));
// SOLVED
// M_cr = 0.737105914227
// Cp_cr = -0.636304358584

show(json!({"key": "aero_absolute_ceiling_jet", "inputs": {"W": "100000.0", "S": "30.0", "T0": "13486.71062689471", "CD0": "0.02", "AR": "7.0", "e": "0.8"}}));
// SOLVED
// h_abs = 6669.76749044 m
// h_abs_ft = 21882.4392731
// sigma = 0.5

show(json!({"key": "aero_calibrated_airspeed", "inputs": {"V": "77.16666667 m/s", "hp": "0 ft"}}));
// SOLVED
// CAS = 77.16666667 m/s
// CAS_kt = 150.000000006
// qc = 3694.37662273 Pa
// pt = 105019.376623 Pa

show(json!({"key": "aero_fanno_flow", "inputs": {"M": "2"}}));
// SOLVED
// fLD = 0.304996502581
// p_pstar = 0.408248290464
// T_Tstar = 0.666666666667
// rho_rhostar = 0.612372435696
// p0_p0star = 1.6875

show(json!({"key": "aero_choked_mass_flow", "inputs": {"p0": "100 kPa", "T0": "300 K", "A": "10 cm^2"}}));
// SOLVED
// mdot = 0.233334282967 kg/s
// p_star = 52828.1787717 Pa
// T_star = 250.0 K
// V_star = 316.967782616 m/s

show(json!({"key": "aero_bielliptic_transfer", "inputs": {"h1": "6378136.6", "h2": "19134409.799999997", "hb": "19134409.799999997"}}));
// SOLVED
// dv1 = 864.7664 m/s
// dv2 = 725.330841972 m/s
// dv3 = 0.0 m/s
// dv = 1590.09724197 m/s
// t = 33447.9135801 s
// t_days = 0.387128629399

show(json!({"key": "aero_actuator_disk_hover", "inputs": {"T": "6.283185307 N", "D": "2 m", "rho": "1 kg/m^3"}}));
// SOLVED
// v_i = 0.999999999986 m/s
// P_ideal = 6.28318530691 W
// w = 1.99999999997 m/s

show(json!({"key": "aero_ballistic_coefficient", "inputs": {"m": "1000 kg", "CD": "1.2", "A": "1.5 m^2"}}));
// SOLVED
// beta = 555.555555556 kg/m^2
// beta_psf = 113.786746457

show(json!({"key": "aero_vertical_rocket_trajectory", "inputs": {"Isp": "300 s", "m0": "2718.281828 kg", "mf": "1000 kg", "tb": "100 s"}}));
// SOLVED
// v_b = 1961.3299995 m/s
// h_b = 73948.9938106 m
// h_max = 270081.993711 m
// t_apex = 299.999999949 s

show(json!({"key": "aero_characteristic_velocity", "inputs": {"Tc": "3000 K", "Mw": "20 g/mol", "gamma": "1.2"}}));
// SOLVED
// c_star = 1721.99492587 m/s
// Gamma = 0.648531170706

show(json!({"key": "aero_rocket_thrust", "inputs": {"mdot": "250 kg/s", "ve": "3000 m/s"}}));
// SOLVED
// F = 750000.0 N
// F_kN = 750.0
// Isp = 305.914863893 s
// c_eff = 3000.0 m/s
// F_pressure = 0.0 N

show(json!({"key": "aero_neutral_point", "inputs": {"h_ac": "0.25", "a_w": "5", "a_t": "5", "St_S": "0.2", "lt_c": "4", "deps": "0.5", "h_cg": "0.3"}}));
// SOLVED
// h_n = 0.613636363636
// SM = 0.313636363636
// CL_alpha = 5.5
// Cm_alpha = -1.725

show(json!({"key": "aero_density_altitude", "inputs": {"hp": "0 ft", "T": "15 °C"}}));
// SOLVED
// rho = 1.22499915589 kg/m^3
// sigma = 1.0
// h_rho = 0.0 m
// h_rho_ft = 0.0
// p = 101325.0 Pa

show(json!({"key": "aero_elliptic_wing_lifting_line", "inputs": {"AR": "2", "alpha": "0.1 rad"}}));
// SOLVED
// CL_alpha = 3.14159265359
// CL = 0.314159265359
// CDi = 0.0157079632679
// alpha_i_deg = 2.86478897565

show(json!({"key": "aero_drag_polar_optimum", "inputs": {"CD0": "0.02", "AR": "10", "e": "0.8"}}));
// SOLVED
// CL_md = 0.708981540362
// LD_max = 17.7245385091
// CL_mp = 1.22799204954
// LD_mp = 15.3499006192
// CL_mr = 0.409330683179
// LD_mr = 15.3499006192
```

## Guides

- Desktop app guide: the window, every calculation, scenes and charts: [PDF](https://huggingface.co/AI999X/aerospace-engineering-ai999x/blob/main/docs/aerospace_engineering_ai999x_desktop_guide.pdf) · [HTML](https://huggingface.co/AI999X/aerospace-engineering-ai999x/blob/main/docs/aerospace_engineering_ai999x_desktop_guide.html) · [Markdown](https://huggingface.co/AI999X/aerospace-engineering-ai999x/blob/main/docs/aerospace_engineering_ai999x_desktop_guide.md)
- Python guide: every calculation with a Python example: [PDF](https://huggingface.co/AI999X/aerospace-engineering-ai999x/blob/main/docs/aerospace_engineering_ai999x_capabilities.pdf) · [HTML](https://huggingface.co/AI999X/aerospace-engineering-ai999x/blob/main/docs/aerospace_engineering_ai999x_capabilities.html) · [Markdown](https://huggingface.co/AI999X/aerospace-engineering-ai999x/blob/main/docs/aerospace_engineering_ai999x_capabilities.md)
- Command-line guide: every calculation with the command and what it printed: [PDF](https://huggingface.co/AI999X/aerospace-engineering-ai999x/blob/main/docs/aerospace_engineering_ai999x_rust_guide.pdf) · [HTML](https://huggingface.co/AI999X/aerospace-engineering-ai999x/blob/main/docs/aerospace_engineering_ai999x_rust_guide.html) · [Markdown](https://huggingface.co/AI999X/aerospace-engineering-ai999x/blob/main/docs/aerospace_engineering_ai999x_rust_guide.md)

Every calculation, with its formula, a worked example and its figures: **[Aerospace Engineering on ai999x.com](https://www.ai999x.com/models/aerospace-engineering-ai999x/)**. Install guides for the app and the command line: [https://www.ai999x.com/desktop-cli/](https://www.ai999x.com/desktop-cli/).

Version 1.6.4 · License: Apache-2.0 · Author: Ibnul Jaif Farabi · Organization: AI999X · Contact: support@ai999x.com · [Support the project](https://buymeacoffee.com/ibnuljaiffarabi)

*The Future Is Self-Improving.*

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/aerospace-engineering-ai999x/) (`pip install .` builds it). Releases are tagged `v1.6.4`; the [GitHub release](https://github.com/AI999X/aerospace-engineering-ai999x/releases/tag/v1.6.4) carries the wheel, the desktop app and the command-line program for Windows and Linux, and their checksums.

## Build the program from source

`rust/` holds the Rust source of the `aerospace-engineering` command-line program and browser module: the AI999X engine (`rust/ax-engine`), this model (`rust/aerospace-engineering`) and its data (`rust/models/aerospace_engineering/pack.json`). With Rust 1.80 or later:

```bash
cd rust
cargo build --release -p aerospace-engineering-ai999x
./target/release/aerospace-engineering --help
cargo test -p aerospace-engineering-ai999x
```
