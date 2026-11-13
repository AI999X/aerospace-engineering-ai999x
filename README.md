# aerospace-engineering-ai999x: offline, verified aerospace engineering solver with step-by-step answers

[![tests](https://github.com/AI999X/aerospace-engineering-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/aerospace-engineering-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/aerospace-engineering-ai999x)](https://pypi.org/project/aerospace-engineering-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/aerospace-engineering-ai999x)

**[Desktop app & command line](https://www.ai999x.com/desktop-cli/)** · **[Compute in your browser](https://www.ai999x.com/models/aerospace-engineering-ai999x/#try)** · **[Install (Python)](https://pypi.org/project/aerospace-engineering-ai999x/)** · **[Guides](https://huggingface.co/AI999X/aerospace-engineering-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/aerospace-engineering-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.6.0

Field: **Engineering** · Tasks: Question Answering

aerospace-engineering-ai999x is an offline, deterministic aerospace engineering solver written in Rust: a desktop app, a command-line program, a browser module and a Python package. It answers aerospace engineering questions with verified results from 81 certified capabilities, gives 375 documented answers identically on Windows and Linux, and refuses rather than guesses when an answer cannot be verified.

**aerospace-engineering-ai999x** reads a question in plain English (or a typed call), picks a certified aerospace engineering capability, computes the answer exactly, checks it, and returns the result with its steps, units, LaTeX and assumptions, or a refusal that says what is missing. It runs fully offline on any CPU, needs no GPU, no API key and no internet, and is built to give **zero wrong answers**: when it is not sure, it refuses instead of guessing.

[![Buy me a coffee](https://img.shields.io/badge/Buy%20me%20a%20coffee-support%20development-FFDD00?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ibnuljaiffarabi) ![License](https://img.shields.io/badge/license-Apache--2.0-blue) ![Python](https://img.shields.io/badge/python-3.10%20%7C%203.11%20%7C%203.12%20%7C%203.13%20%7C%203.14-blue) ![Offline](https://img.shields.io/badge/runs-offline%20on%20CPU-green) ![Rust](https://img.shields.io/badge/Rust-Windows%20%7C%20Linux%20%7C%20WebAssembly-orange)

**Run it your way.** aerospace-engineering-ai999x comes as the AI999X desktop app and as the command-line program `aerospace-engineering` for Windows and Linux, runs in your browser as WebAssembly, and installs as a Python package. The desktop app, the command line and the browser run the same compiled Rust program on your own device: offline, no Python needed.

## Get it

| Way to run it | For | Get it |
|---|---|---|
| **AI999X Aerospace Engineering desktop app** 1.6.0 | Windows, Linux. This model in its own app: search, ask in words, forms, live figures, explanations, offline | [Windows (x86-64)](https://downloads.ai999x.com/desktop/1.6.0/ai999x-aerospace-engineering-1.6.0-windows-x86_64.zip) · [Linux (x86-64)](https://downloads.ai999x.com/desktop/1.6.0/ai999x-aerospace-engineering-1.6.0-linux-x86_64.tar.gz) |
| **AI999X desktop app** 1.6.0 | Windows, Linux. Every AI999X model in one app: search, ask in words, forms, live figures, explanations, offline | [Windows (x86-64)](https://downloads.ai999x.com/desktop/1.6.0/ai999x-1.6.0-windows-x86_64.zip) · [Linux (x86-64)](https://downloads.ai999x.com/desktop/1.6.0/ai999x-1.6.0-linux-x86_64.tar.gz) |
| **Command line** (`aerospace-engineering`) | Windows, Linux. One program per model, offline, no Python | [Windows (x86-64)](https://downloads.ai999x.com/desktop/1.6.0/ai999x-cli-1.6.0-windows-x86_64.zip) · [Linux (x86-64)](https://downloads.ai999x.com/desktop/1.6.0/ai999x-cli-1.6.0-linux-x86_64.tar.gz) |
| **In your browser** | any device. Nothing to install | [Compute on the model page](https://www.ai999x.com/models/aerospace-engineering-ai999x/#try) |
| **Python** | Python 3.10+ | `pip install aerospace-engineering-ai999x` |

Checksums: [SHA256SUMS](https://downloads.ai999x.com/desktop/1.6.0/SHA256SUMS) · every version: [https://downloads.ai999x.com/desktop/latest/](https://downloads.ai999x.com/desktop/latest/) · install guide: [desktop & command line](https://www.ai999x.com/desktop-cli/)

## Quick start

### Desktop app

Download **AI999X Aerospace Engineering** for your system (above), unpack it and start it; type a question in words or pick a capability, fill in the form and press Compute. Figures and an explanation of the answer appear beside it. The AI999X desktop app has the same, with Aerospace Engineering among all AI999X models.

Linux, from a terminal:

```bash
curl -LO https://downloads.ai999x.com/desktop/1.6.0/ai999x-aerospace-engineering-1.6.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.6.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-aerospace-engineering-1.6.0-linux-x86_64.tar.gz
./ai999x-aerospace-engineering-1.6.0-linux-x86_64/ai999x-aerospace-engineering
```

### Command line

Linux:

```bash
curl -LO https://downloads.ai999x.com/desktop/1.6.0/ai999x-cli-1.6.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.6.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
mkdir -p ~/.local/share && tar xzf ai999x-cli-1.6.0-linux-x86_64.tar.gz -C ~/.local/share
echo 'export PATH="$HOME/.local/share/ai999x-cli-1.6.0-linux-x86_64:$PATH"' >> ~/.bashrc && source ~/.bashrc
aerospace-engineering --version
```

Windows (PowerShell):

```powershell
$F = "ai999x-cli-1.6.0-windows-x86_64.zip"
Invoke-WebRequest "https://downloads.ai999x.com/desktop/1.6.0/$F" -OutFile $F
Invoke-WebRequest "https://downloads.ai999x.com/desktop/1.6.0/SHA256SUMS" -OutFile SHA256SUMS
(Get-FileHash $F -Algorithm SHA256).Hash.ToLower() -eq ((Select-String -SimpleMatch $F SHA256SUMS).Line -split ' ')[0]
Expand-Archive $F -DestinationPath "$env:LOCALAPPDATA\AI999X" -Force
$D = "$env:LOCALAPPDATA\AI999X\ai999x-cli-1.6.0-windows-x86_64"; $env:Path += ";$D"
[Environment]::SetEnvironmentVariable("Path", [Environment]::GetEnvironmentVariable("Path", "User") + ";$D", "User")
aerospace-engineering --version
```

Every command below was run as shown; the text under it is what the program printed.

**What a capability needs**: The equation or method, every input and output with its SI unit. Read this before solve: it tells you the input names.

```console
$ aerospace-engineering info aero_oblique_shock
Oblique shock from the deflection angle
  theta-beta-M relation tan(theta) = 2 cot(beta) (M1^2 sin^2 beta - 1)/(M1^2 (g + cos 2 beta) + 2), solved by bisection on the chosen branch; normal-shock relations on Mn1 = M1 sin beta; M2 = Mn2 / sin(beta - theta)
  in   M1       upstream Mach number (> 1) []
  in   theta    flow deflection (wedge half-) angle (deg, rad) [rad]
  in   branch   weak (default) or strong shock solution (choice) []
  in   gamma    ratio of specific heats (default 1.4, air) []
  out  beta_deg shock wave angle in degrees []
  out  M2       downstream Mach number []
  out  p2_p1    pressure ratio p2/p1 []
  out  T2_T1    temperature ratio T2/T1 []
  out  rho2_rho1 density ratio rho2/rho1 []
  out  p02_p01  stagnation pressure ratio []
  out  Mn1      upstream Mach number normal to the shock []
```

**Solve**: Inputs as name=value, each with any unit of the right kind. The answer lists every output, the method used, each input converted to SI, and the conditions the answer assumes.

```console
$ aerospace-engineering solve aero_oblique_shock M1=2 "theta=10 deg"
beta_deg = 39.31393184
M2 = 1.640522229
p2_p1 = 1.706578604
T2_T1 = 1.170151284
rho2_rho1 = 1.458425613
p02_p01 = 0.9846440225
Mn1 = 1.267138036
  by: Oblique shock from the deflection angle  (theta-beta-M relation tan(theta) = 2 cot(beta) (M1^2 sin^2 beta - 1)/(M1^2 (g + cos 2 beta) + 2), solved by bisection on the chosen branch; normal-shock relations on Mn1 = M1 sin beta; M2 = Mn2 / sin(beta - theta))
  M1 = 2
  theta = 0.1745329252 rad
  assumes: 1 < M1 <= 20; 0 < theta < theta_max(M1) (attached shock); 1.05 <= gamma <= 1.7
```

**Many questions at once**: One JSON request per line in, one JSON answer per line out, in order: for scripts, notebooks and agents (the program starts once). In PowerShell: Get-Content cases.jsonl | aerospace-engineering batch

`cases.jsonl`:

```json
{"key": "aero_oblique_shock", "inputs": {"M1": 2, "theta": "10 deg"}}
{"key": "aero_isa_properties", "inputs": {"h": "35000 ft"}}
```

```console
$ aerospace-engineering batch < cases.jsonl
{"outcome":"SOLVED","message":"","value":39.3139318448,"unit":"","solved_for":"beta_deg","si":[["M1",2.0],["theta",0.17453292519943295]],"values":[["beta_deg",39.3139318448,""],["M2",1.640522229,""],["p2_p1",1.706578604,""],["T2_T1",1.1701512843,""],["rho2_rho1",1.45842561291,""],["p02_p01",0.984644022503,""],["Mn1",1.26713803645,""]],"target":"beta_deg","phrase":"shock wave angle in degrees","assumptions":["1 < M1 <= 20","0 < theta < theta_max(M1) (attached shock)","1.05 <= gamma <= 1.7"],"name":"Oblique shock from the deflection angle","equation":"theta-beta-M relation tan(theta) = 2 cot(beta) (M1^2 sin^2 beta - 1)/(M1^2 (g + cos 2 beta) + 2), solved by bisection on the chosen branch; normal-shock relations on Mn1 = M1 sin beta; M2 = Mn2 / sin(beta - theta)"}
{"outcome":"SOLVED","message":"","value":0.380455436006,"unit":"kg/m^3","solved_for":"rho","si":[["h",10668.0]],"values":[["T",218.924175599,"K"],["p",23908.9066172,"Pa"],["rho",0.380455436006,"kg/m^3"],["a",296.614227594,"m/s"],["mu",0.0000143408368522,"Pa*s"],["sigma",0.310576080137,""],["delta",0.235962562223,""],["theta",0.75975768037,""],["H",10650.126831,"m"]],"target":"rho","phrase":"temperature","assumptions":["-1000 m <= h <= 47000 m"],"name":"Standard atmosphere at an altitude","equation":"H = r0 h/(r0 + h); in each layer T = Tb + L (H - Hb); p = pb (Tb/T)^(g0/(R L)) (L != 0) or pb exp(-g0 (H - Hb)/(R Tb)); rho = p/(R T); a = sqrt(gamma R T)"}
```

### In your browser

Open [Aerospace Engineering on ai999x.com](https://www.ai999x.com/models/aerospace-engineering-ai999x/#try), choose a capability and press **Compute**: the model runs on your own device as WebAssembly; nothing is sent anywhere. Change any input (any unit of the right kind) and the answer, its explanation and the figures follow.

In your own page: `aerospace_engineering_ai999x.wasm` and the loader `ax-wasm.js` (https://www.ai999x.com/compute/rust/ax-wasm.js).

### Python

```bash
pip install aerospace-engineering-ai999x
```

## Worked examples in every form

The same worked example from the command line, from Python and in the browser. Each command-line output is what the program printed.

### Critical Mach number of an airfoil

The free-stream Mach number at which the flow first reaches sonic speed at the airfoil's minimum-pressure point, from its incompressible minimum pressure coefficient and a compressibility rule (Prandtl-Glauert or Karman-Tsien), air (gamma = 1.4).

Command line:

```console
$ aerospace-engineering solve aero_critical_mach Cp0=-0.43 rule=prandtl-glauert
M_cr = 0.7371059142
Cp_cr = -0.6363043586
  by: Critical Mach number of an airfoil  (Cp_cr(M) = 2/(g M^2) [((1 + (g-1)/2 M^2)/(1 + (g-1)/2))^(g/(g-1)) - 1]; equate to Cp0/sqrt(1 - M^2) (or the Karman-Tsien form) and solve)
  Cp0 = -0.43
  rule = prandtl-glauert
  assumes: -3 <= Cp0 < -0.01; subsonic, small-perturbation compressibility rules; air, gamma = 1.4
```

Python:

```python
import aerospace_engineering_ai999x as m

a = m.solve("aero_critical_mach", Cp0="-0.43", rule="prandtl-glauert")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/aerospace-engineering-ai999x/#cap-aero_critical_mach) and press **Compute in your browser**.

_What this means:_ This calculation determines the free-stream Mach number at which local flow first becomes sonic over an airfoil, based on its incompressible minimum pressure coefficient of -0.43 and the selected compressibility rule. The resulting critical Mach number is 0.737106, with a matching critical pressure coefficient of -0.636304, indicating the threshold for significant compressibility effects.

### Elliptic wing by lifting-line theory

The lift-curve slope, lift coefficient, induced drag coefficient and induced angle of attack of an untwisted elliptic wing of a given aspect ratio at an angle of attack measured from zero lift (Prandtl lifting-line theory, span efficiency 1).

Command line:

```console
$ aerospace-engineering solve aero_elliptic_wing_lifting_line AR=2 "alpha=0.1 rad"
CL_alpha = 3.141592654
CL = 0.3141592654
CDi = 0.01570796327
alpha_i_deg = 2.864788976
  by: Elliptic wing by lifting-line theory  (a = a0/(1 + a0/(pi AR)); CL = a alpha; CDi = CL^2/(pi AR); alpha_i = CL/(pi AR))
  AR = 2
  alpha = 0.1 rad
  assumes: 2 <= AR <= 40; 3 <= a0 <= 7.5 per rad; |alpha| <= 15 deg; untwisted elliptic planform, attached flow
```

Python:

```python
import aerospace_engineering_ai999x as m

a = m.solve("aero_elliptic_wing_lifting_line", AR="2", alpha="0.1 rad")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/aerospace-engineering-ai999x/#cap-aero_elliptic_wing_lifting_line) and press **Compute in your browser**.

_What this means:_ The finite-wing lift-curve slope is 3.14159 per radian, producing a lift coefficient of 0.314159 and an induced drag coefficient of 0.015708.

### Thin-airfoil theory for a NACA 4-digit airfoil

The zero-lift angle, lift coefficient and the pitching-moment coefficients about the quarter chord and the leading edge of a cambered NACA 4-digit airfoil at an angle of attack, from thin-airfoil theory.

Command line:

```console
$ aerospace-engineering solve aero_naca4_thin_airfoil m=0 p=0 "alpha=0.1 rad"
alpha_L0_deg = 0
CL = 0.6283185307
Cm_c4 = 0
Cm_le = -0.1570796327
  by: Thin-airfoil theory for a NACA 4-digit airfoil  (Glauert: A0 = alpha - (1/pi) int dz/dx dtheta, An = (2/pi) int dz/dx cos(n theta) dtheta; CL = pi(2A0 + A1); Cm_c/4 = pi/4 (A2 - A1); integrals of the NACA mean line in closed form)
  m = 0
  p = 0
  alpha = 0.1 rad
  assumes: 0 <= m <= 0.095; 0.05 <= p <= 0.95 when m > 0; |alpha| <= 15 deg; thin airfoil, inviscid, incompressible, unstalled
```

Python:

```python
import aerospace_engineering_ai999x as m

a = m.solve("aero_naca4_thin_airfoil", m="0", p="0", alpha="0.1 rad")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/aerospace-engineering-ai999x/#cap-aero_naca4_thin_airfoil) and press **Compute in your browser**.

_What this means:_ This calculation determines the aerodynamic characteristics of a NACA four-digit airfoil using thin-airfoil theory. Given the maximum camber fraction 0, the position of maximum camber 0, and the angle of attack 0.1 rad, it computes the section lift coefficient 0.628319, the zero-lift angle of attack 0, and the pitching moment coefficients about the quarter chord 0 and leading edge -0.15708. These values indicate how much lift the airfoil generates and where its aerodynamic center lies relative to the geometric chord.

### Absolute ceiling of a jet aircraft

The absolute ceiling of a jet aircraft (where the available thrust just equals the minimum drag) when the thrust falls in proportion to the air density, T = T0 rho/rho0.

Command line:

```console
$ aerospace-engineering solve aero_absolute_ceiling_jet W=100000.0 S=30.0 T0=13486.71062689471 CD0=0.02 AR=7.0 e=0.8
h_abs = 6669.76749 m
h_abs_ft = 21882.43927
sigma = 0.5
  by: Absolute ceiling of a jet aircraft  (T0 sigma = D_min = 2W sqrt(K CD0): sigma = 2W sqrt(K CD0)/T0; the height of that density in the standard atmosphere)
  W = 100000 N
  S = 30 m^2
  T0 = 13486.71063 N
  CD0 = 0.02
  AR = 7
  e = 0.8
  assumes: minimum drag below the sea-level thrust; ceiling below 47 km; thrust proportional to density (stated lapse); parabolic drag polar
```

Python:

```python
import aerospace_engineering_ai999x as m

a = m.solve("aero_absolute_ceiling_jet", W=100000.0, S=30.0, T0=13486.71062689471, CD0=0.02, AR=7.0, e=0.8)
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/aerospace-engineering-ai999x/#cap-aero_absolute_ceiling_jet) and press **Compute in your browser**.

_What this means:_ This calculation determines the highest altitude where level flight is possible by equating available thrust to minimum drag. Using the weight 100000 N, sea-level thrust 13486.7 N, and aerodynamic coefficients including aspect ratio 7 and zero-lift drag coefficient 0.02, the model computes the required density ratio 0.5. The resulting absolute ceiling is 6669.77 m meters, or 21882.4 feet. This value represents the limit where excess thrust becomes zero, assuming thrust decreases linearly with air density.

## Documentation

- **Rust user guide**: the command line, every capability with a command and what it printed: [PDF](https://huggingface.co/AI999X/aerospace-engineering-ai999x/blob/main/docs/aerospace_engineering_ai999x_rust_guide.pdf) · [Markdown](https://huggingface.co/AI999X/aerospace-engineering-ai999x/blob/main/docs/aerospace_engineering_ai999x_rust_guide.md)
- **Python guide**: every capability with a Python example: [PDF](https://huggingface.co/AI999X/aerospace-engineering-ai999x/blob/main/docs/aerospace_engineering_ai999x_capabilities.pdf) · [Markdown](https://huggingface.co/AI999X/aerospace-engineering-ai999x/blob/main/docs/aerospace_engineering_ai999x_capabilities.md)
- **Desktop app & command line**: downloads, install, ask and search, figures, offline use, checksums: https://www.ai999x.com/desktop-cli/
- **Model page**: compute in your browser, every capability explained: https://www.ai999x.com/models/aerospace-engineering-ai999x/

## Highlights

- **Verified, not generated.** Every answer is computed by a certified aerospace engineering capability and checked (substituted back, dimension-checked or compared with an independent computation) before it is returned.
- **Zero wrong answers by design.** Seven explicit outcomes (SOLVED, NEEDS_INPUT, AMBIGUOUS, INVALID_INPUT, OUT_OF_SCOPE, UNRECOGNIZED_FORMAT, INTERNAL_ERROR), never a silent guess. Every one of its 375 documented answers is reproduced exactly on every tested platform.
- **Plain-English questions and a guaranteed typed interface.** `ask()` reads documented question forms; `solve(capability_id, **inputs)` always solves a supported task with valid inputs.
- **Explains itself.** Steps, formulas in LaTeX, units, assumptions and the model used, ready for notebooks, tutoring apps and homework checkers.
- **Native and in the browser.** Every capability also computes natively in Rust: the desktop app and the `aerospace-engineering` command line for Windows and Linux, and WebAssembly on the model page, no Python, no network.
- **Small and portable.** Pure Python (sympy, mpmath); identical answers on Windows and Linux (Python 3.10, 3.11, 3.12, 3.13, 3.14).
- **Coverage:** the standard atmosphere, airspeeds, compressible flow and shocks, nozzles and rocket propulsion, rocket performance and staging, aircraft performance, static stability, propellers, wings and airfoils, orbits and re-entry.

## Model overview

| Property | Value |
|---|---|
| Package | `aerospace-engineering-ai999x` 1.6.0 (import `aerospace_engineering_ai999x`) |
| Type | Deterministic neuro-symbolic aerospace engineering engine (rule-based reading + certified symbolic capabilities) |
| Capabilities | 81 certified procedures |
| Knowledge pack | version 1.0.1 |
| Golden answers | 375 documented answers, identical on every tested platform |
| Platforms tested | Windows (Python 3.13), Linux x86-64 (Python 3.10, 3.11, 3.12, 3.13, 3.14) |
| Runtime | CPU only, offline; no GPU, no network, no API key |
| License | Apache-2.0 |
| Creator / engineer | Ibnul Jaif Farabi |
| Organization | AI999X |

## Python quick start

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

The Python package also installs a Python command, `aerospace-engineering-ai999x "<question>"`; for the native program use `aerospace-engineering` (above). The outputs above were produced by running this exact code on aerospace-engineering-ai999x 1.6.0.

## How every answer is verified

1. **Reading**: the question is turned into typed quantities (value, unit, kind, role) and one asked quantity; every stated number must be used or the question is refused.
2. **Planning**: only certified capabilities may answer; a capability applies only under the conditions the wording states (for example a pendulum, a liquid at rest, boiling).
3. **Computing**: exact symbolic or high-precision numeric evaluation.
4. **Checking**: dimensions, substitution back into the defining equation, and agreement between independent readings; any disagreement becomes a refusal, never a number.

Capabilities are certified before they ship: each must reproduce hand-worked reference cases, reject a deliberately wrong variant (a control), and leave every benchmark answer unchanged.

## Python worked examples, with their steps

Each question was put to `aerospace-engineering-ai999x`; the answer is the one it returned, with the steps it took.

**Standard atmosphere at an altitude** · _Standard atmosphere_

> ISA properties: h = 35000 ft

→ **T = 218.9241756 K, p = 23908.90662 Pa, rho = 0.380455436 kg/m^3, a = 296.6142276 m/s, mu = 1.434083685e-05 Pa*s, sigma = 0.3105760801, delta = 0.2359625622, theta = 0.7597576804, H = 10650.12683 m**

<details><summary>Steps</summary>

- Standard atmosphere at an altitude: h = 35000 ft → T = 218.9241756 K, p = 23908.90662 Pa, rho = 0.380455436 kg/m^3, a = 296.6142276 m/s, mu = 1.434083685e-05 Pa*s, sigma = 0.3105760801 kg/m^3, delta = 0.2359625622 kg/m^3, theta = 0.7597576804 kg/m^3, H = 10650.12683 m

</details>

**Normal shock relations** · _Compressible flow_

> Normal shock: M1 = 2

→ **M2 = 0.5773502692, p2_p1 = 4.5, T2_T1 = 1.6875, rho2_rho1 = 2.666666667, p02_p01 = 0.7208738615, p02_p1 = 5.640440813**

<details><summary>Steps</summary>

- Normal shock relations: M1 = 2 → M2 = 0.5773502692, p2_p1 = 4.5, T2_T1 = 1.6875, rho2_rho1 = 2.666666667, p02_p01 = 0.7208738615, p02_p1 = 5.640440813

</details>

**True airspeed from calibrated airspeed** · _Airspeeds_

> True airspeed: CAS = 250 kt, hp = 30000 ft

→ **TAS = 202.4112964 m/s, TAS_kt = 393.4560838, qc = 10498.2154 Pa, M = 0.6675168197**

<details><summary>Steps</summary>

- True airspeed from calibrated airspeed: CAS = 128.6111111 m/s, hp = 30000 ft → TAS = 202.4112964 m/s, TAS_kt = 393.4560838 m/s, qc = 10498.2154 Pa, M = 0.6675168197 m/s

</details>

**Breguet range of a jet aircraft** · _Aircraft performance_

> Breguet range: V = 230 m/s, c = 0.6 1/h, LD = 17, W0 = 2000 kN, W1 = 1500 kN

→ **R = 6749021.42 m, R_km = 6749.02142, t = 29343.57139 s, t_h = 8.150992053**

<details><summary>Steps</summary>

- Breguet range of a jet aircraft: V = 230 m/s, c = 1.699527022e-05 kg/(N*s), LD = 17, W0 = 2000 kN, W1 = 1500 kN → R = 6749021.42 m, R_km = 6749.02142 m, t = 29343.57139 s, t_h = 8.150992053 m

</details>

**Oblique shock from the deflection angle** · _Compressible flow_

`m.solve("aero_oblique_shock", M1=2, theta="10 deg")`

→ **beta_deg = 39.31393184, M2 = 1.640522229, p2_p1 = 1.706578604, T2_T1 = 1.170151284, rho2_rho1 = 1.458425613, p02_p01 = 0.9846440225, Mn1 = 1.267138036**

<details><summary>Steps</summary>

- Oblique shock from the deflection angle: M1 = 2, theta = 10 deg → beta_deg = 39.31393184, M2 = 1.640522229, p2_p1 = 1.706578604, T2_T1 = 1.170151284, rho2_rho1 = 1.458425613, p02_p01 = 0.9846440225, Mn1 = 1.267138036

</details>

**Critical Mach number of an airfoil** · _Aerodynamics_

> Critical Mach: Cp0 = -0.43, rule = prandtl-glauert

→ **M_cr = 0.7371059142, Cp_cr = -0.6363043586**

<details><summary>Steps</summary>

- Critical Mach number of an airfoil: Cp0 = -0.43, rule = prandtl-glauert → M_cr = 0.7371059142, Cp_cr = -0.6363043586

</details>

## Figures

Every point was computed by `aerospace-engineering-ai999x`.

**The standard atmosphere: temperature with altitude**

![The standard atmosphere: temperature with altitude, computed by aerospace-engineering-ai999x](https://huggingface.co/AI999X/aerospace-engineering-ai999x/resolve/main/figures/aero-isa-properties-profile.svg)

_Temperature of the International Standard Atmosphere from 0 to 47 km, 95 altitudes computed by the package (it covers the atmosphere to 47 km): 288.15 K at sea level, the tropospheric lapse to 216.774 K at 11 km (marked), the isothermal layer (coldest computed point 216.65 K at 11.5 km) and the warming stratosphere, 269.684 K at 47 km._

**Ground speed around the compass in a wind**

![Ground speed around the compass in a wind, computed by aerospace-engineering-ai999x](https://huggingface.co/AI999X/aerospace-engineering-ai999x/resolve/main/figures/aero-wind-triangle-polar.svg)

_Ground speed for every course (5° steps) of an aircraft flying 100 m/s through a 20 m/s wind from 300°, each solved by the package's wind triangle: slowest 80 m/s heading into the wind (course 300°), fastest 120 m/s with it (course 120°, marked); largest wind correction 11.537°._

**Drag polar and the best lift-to-drag ratio**

![Drag polar and the best lift-to-drag ratio, computed by aerospace-engineering-ai999x](https://huggingface.co/AI999X/aerospace-engineering-ai999x/resolve/main/figures/aero-drag-polar-optimum-scatter.svg)

_Drag polar C_D = C_D0 + C_Di for C_D0 = 0.02, aspect ratio 10 and Oswald factor 0.8: the induced drag at 41 speeds from the package (W = 10000 N, S = 16 m², sea level); the tangent from the origin touches at the best lift-to-drag point it computes, marked: C_D = 0.04, C_L = 0.708982, L/D = 17.7245._

**The normal-shock table**

![The normal-shock table, computed by aerospace-engineering-ai999x](https://huggingface.co/AI999X/aerospace-engineering-ai999x/resolve/main/figures/aero-normal-shock-heatmap.svg)

_The normal-shock table for air (γ = 1.4): for upstream Mach numbers M₁ = 1.5, 2, 3, 4, 5 (rows, top to bottom) the downstream Mach number and the pressure, density, temperature and stagnation-pressure ratios, every cell computed by the package and each column shaded by its own largest value; at M₁ = 2: M₂ = 0.57735, p₂/p₁ = 4.5, ρ₂/ρ₁ = 2.66667, T₂/T₁ = 1.6875, p₀₂/p₀₁ = 0.720874._

**Bi-elliptic burns against a Hohmann transfer**

![Bi-elliptic burns against a Hohmann transfer, computed by aerospace-engineering-ai999x](https://huggingface.co/AI999X/aerospace-engineering-ai999x/resolve/main/figures/aero-bielliptic-transfer-bars.svg)

_The three burns of bi-elliptic transfers from a circular orbit of r₁ = 12756.3 km to 15 r₁ around the Earth, stacked, for five intermediate apoapses, against the Hohmann transfer's 2.99743 km/s, all from the package; totals r_b = 17 r₁: 2.99791 km/s; r_b = 25 r₁: 2.9904 km/s; r_b = 35 r₁: 2.97848 km/s; r_b = 45 r₁: 2.96888 km/s; r_b = 60 r₁: 2.95846 km/s._

**Ballistic entry: speed with altitude**

![Ballistic entry: speed with altitude, computed by aerospace-engineering-ai999x](https://huggingface.co/AI999X/aerospace-engineering-ai999x/resolve/main/figures/aero-ballistic-entry-velocity-profile.svg)

_Speed against altitude of a ballistic entry (m = 1000 kg, C_D = 1, A = 1 m², entry speed 7 km/s, vertical), 91 altitudes computed by the package; marked: peak deceleration 127.649 g at h = 15.6746 km, V = 4.24571 km/s._

## Capabilities

81 certified procedures. A few of them:

- **Procedures:** Absolute ceiling of a jet aircraft; Ballistic coefficient; Best lift coefficients and lift-to-drag ratios of a drag polar; Bi-elliptic transfer between circular orbits; Breguet range of a jet aircraft; Breguet range of a propeller aircraft; Burnout height and apogee of a vertical rocket; Calibrated airspeed from true airspeed; Characteristic velocity c* of a rocket propellant; Choked mass flow through a nozzle throat; Circular orbit at an altitude above the Earth; Circular-orbit altitude for an orbital period; Coverage of a satellite at an altitude; Critical Mach number of an airfoil; Delta-v of a multistage rocket; Density altitude from pressure altitude and temperature; Elliptic wing by lifting-line theory; Endurance of a jet aircraft; Endurance of a propeller aircraft at constant altitude; Escape speed at an altitude above the Earth; Fanno flow (adiabatic duct flow with friction); Fuel a jet needs for a cruise range; Geopotential height from geometric altitude; Glide ratio, angle and range …

`aerospace_engineering_ai999x.templates()` lists every question form `ask()` reads; the capability guide (PDF, every example executed) is included in this repository.

## Out of scope

- Open-ended explanation, essay writing or tutoring dialogue: it is a solver, not a chatbot.
- Questions outside its certified aerospace engineering capabilities, and any question whose stated numbers it cannot all use: it refuses with the reason instead.
- High-stakes engineering, medical or legal decisions without independent review.

## Limitations

- It answers what its certified capabilities cover (the standard atmosphere, airspeeds, compressible flow and shocks, nozzles and rocket propulsion, rocket performance and staging, aircraft performance, static stability, propellers, wings and airfoils, orbits and re-entry); anything else is refused with a reason.
- `ask()` reads documented question forms and common phrasings; unusual wording may be refused; use `solve()` for guaranteed results.
- Results are as correct as the certified relations and their stated conditions; a certified relation is a checked model, not a claim about every physical situation.

## Citation

```bibtex
@software{aerospace_engineering_ai999x_1_6_0,
  title  = {aerospace-engineering-ai999x: an offline, verified aerospace engineering solver},
  author = {Farabi, Ibnul Jaif}, organization = {AI999X},
  year   = {2026}, version = {1.6.0},
  url    = {https://huggingface.co/AI999X/aerospace-engineering-ai999x}
}
```

## Contact

Questions, bug reports and requests: open a discussion on https://huggingface.co/AI999X/aerospace-engineering-ai999x/discussions or visit https://ai999x.com.

## Support the project

aerospace-engineering-ai999x is free and open source. If it helps you, you can support further development on [Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi).

Created and engineered by **Ibnul Jaif Farabi** · **[AI999X](https://huggingface.co/AI999X)** · [ai999x.com](https://ai999x.com)

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/aerospace-engineering-ai999x/) (`pip install .` builds it). Releases are tagged `v1.6.0`; the [GitHub release](https://github.com/AI999X/aerospace-engineering-ai999x/releases/tag/v1.6.0) carries the wheel, the desktop app and the command-line program for Windows and Linux, and their checksums.

## Build the program from source

`rust/` holds the Rust source of the `aerospace-engineering` command-line program and browser module: the AI999X engine (`rust/ax-engine`), this model (`rust/aerospace-engineering`) and its data (`rust/models/aerospace_engineering/pack.json`). With Rust 1.80 or later:

```bash
cd rust
cargo build --release -p aerospace-engineering-ai999x
./target/release/aerospace-engineering --help
cargo test -p aerospace-engineering-ai999x
```
