# Aerospace Engineering (aerospace-engineering-ai999x)

[![tests](https://github.com/AI999X/aerospace-engineering-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/aerospace-engineering-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/aerospace-engineering-ai999x)](https://pypi.org/project/aerospace-engineering-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/aerospace-engineering-ai999x)

Aerospace Engineering covers the standard atmosphere, lift and drag, aircraft performance, stability, propulsion and rocket performance, nozzles and orbital transfers: 81 calculations in 13 topics, each with its formula, its steps and its units, and a plain reason instead of a number when an input is missing or out of range. One compiled engine, four ways to use it: the desktop app, the command line, the browser and Python. Everything runs on your own computer, offline.

[![The Aerospace Engineering desktop app: Standard atmosphere at an altitude, with its worked example and live scene](https://www.ai999x.com/?ax_shot=aerospace-engineering-ai999x/hero.webp)](https://www.ai999x.com/models/aerospace-engineering-ai999x/)

## Four ways to use it

| | |
|---|---|
| **Desktop app** (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.7/ai999x-aerospace-engineering-1.6.7-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.7/ai999x-aerospace-engineering-1.6.7-linux-x86_64.tar.gz) |
| **Command line** `aerospace-engineering` (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.7/ai999x-cli-1.6.7-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.7/ai999x-cli-1.6.7-linux-x86_64.tar.gz) |
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

## Guides

- Desktop app guide: the window, every calculation, scenes and charts: [PDF](https://huggingface.co/AI999X/aerospace-engineering-ai999x/blob/main/docs/aerospace_engineering_ai999x_desktop_guide.pdf) · [HTML](https://huggingface.co/AI999X/aerospace-engineering-ai999x/blob/main/docs/aerospace_engineering_ai999x_desktop_guide.html) · [Markdown](https://huggingface.co/AI999X/aerospace-engineering-ai999x/blob/main/docs/aerospace_engineering_ai999x_desktop_guide.md)
- Python guide: every calculation with a Python example: [PDF](https://huggingface.co/AI999X/aerospace-engineering-ai999x/blob/main/docs/aerospace_engineering_ai999x_capabilities.pdf) · [HTML](https://huggingface.co/AI999X/aerospace-engineering-ai999x/blob/main/docs/aerospace_engineering_ai999x_capabilities.html) · [Markdown](https://huggingface.co/AI999X/aerospace-engineering-ai999x/blob/main/docs/aerospace_engineering_ai999x_capabilities.md)
- Command-line guide: every calculation with the command and what it printed: [PDF](https://huggingface.co/AI999X/aerospace-engineering-ai999x/blob/main/docs/aerospace_engineering_ai999x_rust_guide.pdf) · [HTML](https://huggingface.co/AI999X/aerospace-engineering-ai999x/blob/main/docs/aerospace_engineering_ai999x_rust_guide.html) · [Markdown](https://huggingface.co/AI999X/aerospace-engineering-ai999x/blob/main/docs/aerospace_engineering_ai999x_rust_guide.md)

Every calculation, with its formula, a worked example and its figures: **[Aerospace Engineering on ai999x.com](https://www.ai999x.com/models/aerospace-engineering-ai999x/)**. Install guides for the app and the command line: [https://www.ai999x.com/desktop-cli/](https://www.ai999x.com/desktop-cli/).

Version 1.6.7 · License: Apache-2.0 · Author: Ibnul Jaif Farabi · Organization: AI999X · Contact: support@ai999x.com · [Support the project](https://buymeacoffee.com/ibnuljaiffarabi)

*The Future Is Self-Improving.*

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/aerospace-engineering-ai999x/) (`pip install .` builds it). Releases are tagged `v1.6.7`; the [GitHub release](https://github.com/AI999X/aerospace-engineering-ai999x/releases/tag/v1.6.7) carries the wheel, the desktop app and the command-line program for Windows and Linux, and their checksums.

## Build the program from source

`rust/` holds the Rust source of the `aerospace-engineering` command-line program and browser module: the AI999X engine (`rust/ax-engine`), this model (`rust/aerospace-engineering`) and its data (`rust/models/aerospace_engineering/pack.json`). With Rust 1.80 or later:

```bash
cd rust
cargo build --release -p aerospace-engineering-ai999x
./target/release/aerospace-engineering --help
cargo test -p aerospace-engineering-ai999x
```
