# aerospace-engineering-ai999x — offline, verified aerospace engineering solver with step-by-step answers

[![tests](https://github.com/AI999X/aerospace-engineering-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/aerospace-engineering-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/aerospace-engineering-ai999x)](https://pypi.org/project/aerospace-engineering-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/aerospace-engineering-ai999x)

**[Install](https://pypi.org/project/aerospace-engineering-ai999x/)** · **[Capability guide](https://huggingface.co/AI999X/aerospace-engineering-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/aerospace-engineering-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.0.0

Field: **Engineering** · Tasks: Question Answering, Token Classification, Text Classification, Sentence Similarity, Feature Extraction

aerospace-engineering-ai999x is an offline, deterministic Python solver that answers aerospace engineering questions with verified results from 80 certified capabilities, gives 369 documented answers identically on Windows and Linux, and refuses rather than guesses when an answer cannot be verified.

**aerospace-engineering-ai999x** is a compact, deterministic aerospace engineering engine for Python. It reads a question in plain English (or a typed call), picks a certified aerospace engineering capability, computes the answer exactly, checks it, and returns the result with its steps, units, LaTeX and assumptions — or an honest refusal that says what is missing. It runs fully offline on any CPU, needs no GPU, no API key and no internet, and is built to give **zero wrong answers**: when it is not sure, it refuses instead of guessing.

[![Buy me a coffee](https://img.shields.io/badge/Buy%20me%20a%20coffee-support%20development-FFDD00?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ibnuljaiffarabi) ![License](https://img.shields.io/badge/license-Apache--2.0-blue) ![Python](https://img.shields.io/badge/python-3.10%20%7C%203.11%20%7C%203.12%20%7C%203.13%20%7C%203.14-blue) ![Offline](https://img.shields.io/badge/runs-offline%20on%20CPU-green)

## Highlights

- **Verified, not generated.** Every answer is computed by a certified aerospace engineering capability and checked (substituted back, dimension-checked or compared with an independent computation) before it is returned.
- **Zero wrong answers by design.** Seven explicit outcomes — SOLVED, NEEDS_INPUT, AMBIGUOUS, INVALID_INPUT, OUT_OF_SCOPE, UNRECOGNIZED_FORMAT, INTERNAL_ERROR — never a silent guess. Every one of its 369 documented answers is reproduced exactly on every tested platform.
- **Plain-English questions and a guaranteed typed interface.** `ask()` reads documented question forms; `solve(capability_id, **inputs)` always solves a supported task with valid inputs.
- **Explains itself.** Steps, formulas in LaTeX, units, assumptions and the model used, ready for notebooks, tutoring apps and homework checkers.
- **Small and portable.** Pure Python (sympy, mpmath); identical answers on Windows and Linux (Python 3.10, 3.11, 3.12, 3.13, 3.14).
- **Coverage:** the standard atmosphere, airspeeds, compressible flow and shocks, nozzles and rocket propulsion, rocket performance and staging, aircraft performance, static stability, propellers, wings and airfoils, orbits and re-entry.

## Model overview

| Property | Value |
|---|---|
| Package | `aerospace-engineering-ai999x` 1.0.0 (import `aerospace_engineering_ai999x`) |
| Type | Deterministic neuro-symbolic aerospace engineering engine (rule-based reading + certified symbolic capabilities) |
| Capabilities | 80 certified procedures |
| Knowledge pack | version 1.0.1 |
| Golden answers | 369 documented answers, identical on every tested platform |
| Platforms tested | Windows (Python 3.13), Linux x86-64 (Python 3.10, 3.11, 3.12, 3.13, 3.14) |
| Runtime | CPU only, offline; no GPU, no network, no API key |
| License | Apache-2.0 |
| Creator / engineer | Ibnul Jaif Farabi |
| Organization | AI999X |

## Quickstart

```bash
pip install aerospace-engineering-ai999x
```

```python
import aerospace_engineering_ai999x as aerospace_engineering

aerospace_engineering.ask("ISA properties: h = 35000 ft").show()
# -> SOLVED — T = 218.924, p = 23908.9, rho = 0.380455, a = 296.614  (aero_isa_properties)
aerospace_engineering.ask("Normal shock: M1 = 2").show()
# -> SOLVED — M2 = 0.57735, p2_p1 = 4.5, T2_T1 = 1.6875, rho2_rho1 = 2.66667  (aero_normal_shock)
aerospace_engineering.ask("True airspeed: CAS = 250 kt, hp = 30000 ft").show()
# -> SOLVED — TAS = 202.411, TAS_kt = 393.456, qc = 10498.2, M = 0.667517  (aero_true_airspeed_from_calibrated)
aerospace_engineering.ask("Breguet range: V = 230 m/s, c = 0.6 1/h, LD = 17, W0 = 2000 kN, W1 = 1500 kN").show()
# -> SOLVED — R = 6.74902e+06, R_km = 6749.02, t = 29343.6, t_h = 8.15099  (aero_breguet_range_jet)
aerospace_engineering.solve("aero_oblique_shock", M1=2, theta="10 deg").show()
# -> SOLVED — beta_deg = 39.3139, M2 = 1.64052, p2_p1 = 1.70658, T2_T1 = 1.17015  (aero_oblique_shock)
aerospace_engineering.ask("Who created you?").message
# -> aerospace-engineering-ai999x was created and engineered by Ibnul Jaif Farabi at AI999X.
```

Command line: `aerospace-engineering-ai999x "<question>"`, `aerospace-engineering-ai999x --capabilities`. The outputs above were produced by running this exact code on aerospace-engineering-ai999x 1.0.0.

## How every answer is verified

1. **Reading** — the question is turned into typed quantities (value, unit, kind, role) and one asked quantity; every stated number must be used or the question is refused.
2. **Planning** — only certified capabilities may answer; a capability applies only under the conditions the wording states (for example a pendulum, a liquid at rest, boiling).
3. **Computing** — exact symbolic or high-precision numeric evaluation.
4. **Checking** — dimensions, substitution back into the defining equation, and agreement between independent readings; any disagreement becomes a refusal, never a number.

Capabilities are certified before they ship: each must reproduce hand-worked reference cases, reject a deliberately wrong variant (a control), and leave every benchmark answer unchanged.

## Capabilities

80 certified procedures. A few of them:

- **Procedures:** Absolute ceiling of a jet aircraft; Ballistic coefficient; Best lift coefficients and lift-to-drag ratios of a drag polar; Bi-elliptic transfer between circular orbits; Breguet range of a jet aircraft; Breguet range of a propeller aircraft; Burnout height and apogee of a vertical rocket; Calibrated airspeed from true airspeed; Characteristic velocity c* of a rocket propellant; Choked mass flow through a nozzle throat; Circular orbit at an altitude above the Earth; Circular-orbit altitude for an orbital period; Coverage of a satellite at an altitude; Critical Mach number of an airfoil; Delta-v of a multistage rocket; Density altitude from pressure altitude and temperature; Elliptic wing by lifting-line theory; Endurance of a jet aircraft; Endurance of a propeller aircraft at constant altitude; Escape speed at an altitude above the Earth; Fanno flow (adiabatic duct flow with friction); Fuel a jet needs for a cruise range; Geopotential height from geometric altitude; Glide ratio, angle and range …

`aerospace_engineering_ai999x.templates()` lists every question form `ask()` reads; the capability guide (PDF, every example executed) is included in this repository.

## Out of scope

- Open-ended explanation, essay writing or tutoring dialogue: it is a solver, not a chatbot.
- Questions outside its certified aerospace engineering capabilities, and any question whose stated numbers it cannot all use — it refuses with the reason instead.
- High-stakes engineering, medical or legal decisions without independent review.

## Limitations

- It answers what its certified capabilities cover (the standard atmosphere, airspeeds, compressible flow and shocks, nozzles and rocket propulsion, rocket performance and staging, aircraft performance, static stability, propellers, wings and airfoils, orbits and re-entry); anything else is refused with a reason.
- `ask()` reads documented question forms and common phrasings; unusual wording may be refused — use `solve()` for guaranteed results.
- Results are as correct as the certified relations and their stated conditions; a certified relation is a checked model, not a claim about every physical situation.

## Citation

```bibtex
@software{aerospace_engineering_ai999x_1_0_0,
  title  = {aerospace-engineering-ai999x: an offline, verified aerospace engineering solver},
  author = {Farabi, Ibnul Jaif}, organization = {AI999X},
  year   = {2026}, version = {1.0.0},
  url    = {https://huggingface.co/AI999X/aerospace-engineering-ai999x}
}
```

## Contact

Questions, bug reports and requests: open a discussion on https://huggingface.co/AI999X/aerospace-engineering-ai999x/discussions or visit https://ai999x.com.

## Support the project

aerospace-engineering-ai999x is free and open source. If it helps you, you can support further development on [Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi).

Created and engineered by **Ibnul Jaif Farabi** · **[AI999X](https://huggingface.co/AI999X)** · [ai999x.com](https://ai999x.com)

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/aerospace-engineering-ai999x/) (`pip install .` builds it). Releases are tagged `v<version>`; the wheel is attached to each [GitHub release](https://github.com/AI999X/aerospace-engineering-ai999x/releases). The optional ONNX models (formula graphs and, where measured, a decision model) are on [Hugging Face](https://huggingface.co/AI999X/aerospace-engineering-ai999x/tree/main/onnx).
