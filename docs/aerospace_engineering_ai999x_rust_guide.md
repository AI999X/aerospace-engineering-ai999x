# aerospace-engineering: the Aerospace Engineering program, user guide

**aerospace-engineering-ai999x 1.6.0**, Rust build: command line, desktop app and browser. **Creator / engineer:** Ibnul Jaif Farabi · **Company:** AI999X · support@ai999x.com

Aerospace Engineering covers the standard atmosphere, lift and drag, aircraft performance, stability, propulsion and rocket performance, nozzles and orbital transfers. This guide is about `aerospace-engineering`, the model as a self-contained program for Windows and Linux: 81 capabilities in 13 topics, answered on your own computer with no network, no Python and nothing else to install. The same compiled model runs in the AI999X desktop app and in the browser on the model's page (https://www.ai999x.com/models/aerospace-engineering-ai999x/); the Python package (`pip install aerospace-engineering-ai999x`) has its own guide. Every command output printed here is what the program printed.

## Contents

- 1. Get it: desktop app, command line, browser, Python
- 2. Quick start
- 3. Writing inputs
- 4. Outcomes and exit codes
- 5. Scripts and agents: batch and JSON
- 6. Figures
- 7. In a web page or Node.js: WebAssembly
- 8. Every capability

## 1. Get it

Version 1.5.0 of the AI999X desktop app and of the command-line bundle. Checksums for every file: https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS; the newest version is always under https://downloads.ai999x.com/desktop/latest/. Full instructions: https://www.ai999x.com/desktop-cli/

| what | system | download |
|---|---|---|
| Desktop app: Aerospace Engineering | Windows x86-64 | https://downloads.ai999x.com/desktop/1.5.0/ai999x-aerospace-engineering-1.5.0-windows-x86_64.zip |
| Desktop app: Aerospace Engineering | Linux x86-64 | https://downloads.ai999x.com/desktop/1.5.0/ai999x-aerospace-engineering-1.5.0-linux-x86_64.tar.gz |
| Desktop app: all models | Windows x86-64 | https://downloads.ai999x.com/desktop/1.5.0/ai999x-1.5.0-windows-x86_64.zip |
| Desktop app: all models | Linux x86-64 | https://downloads.ai999x.com/desktop/1.5.0/ai999x-1.5.0-linux-x86_64.tar.gz |
| Command line | Windows x86-64 | https://downloads.ai999x.com/desktop/1.5.0/ai999x-cli-1.5.0-windows-x86_64.zip |
| Command line | Linux x86-64 | https://downloads.ai999x.com/desktop/1.5.0/ai999x-cli-1.5.0-linux-x86_64.tar.gz |

### Desktop app

Aerospace Engineering in its own app: search, questions in words, input forms, live figures and explanations. Windows: Download ai999x-aerospace-engineering-1.5.0-windows-x86_64.zip, right-click it → Extract All, then open the folder and start ai999x-aerospace-engineering.exe. If Windows SmartScreen asks, choose More info → Run anyway. Linux:

```shell
curl -LO https://downloads.ai999x.com/desktop/1.5.0/ai999x-aerospace-engineering-1.5.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-aerospace-engineering-1.5.0-linux-x86_64.tar.gz
./ai999x-aerospace-engineering-1.5.0-linux-x86_64/ai999x-aerospace-engineering
```

It opens on this model: type a question or pick a capability, fill in the form and press Compute. The AI999X desktop app with every model works the same way (choose the model first). Windows: Download ai999x-1.5.0-windows-x86_64.zip, right-click it → Extract All, then open the folder and start ai999x.exe. If Windows SmartScreen asks, choose More info → Run anyway. Linux:

```shell
curl -LO https://downloads.ai999x.com/desktop/1.5.0/ai999x-1.5.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-1.5.0-linux-x86_64.tar.gz
./ai999x-1.5.0-linux-x86_64/ai999x
```

### Command line: Linux

The bundle holds one program per model; `aerospace-engineering` is this one. The checksum line should print OK.

```shell
curl -LO https://downloads.ai999x.com/desktop/1.5.0/ai999x-cli-1.5.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
mkdir -p ~/.local/share && tar xzf ai999x-cli-1.5.0-linux-x86_64.tar.gz -C ~/.local/share
echo 'export PATH="$HOME/.local/share/ai999x-cli-1.5.0-linux-x86_64:$PATH"' >> ~/.bashrc && source ~/.bashrc
aerospace-engineering --version
```

### Command line: Windows (PowerShell)

The fourth line should print True.

```powershell
$F = "ai999x-cli-1.5.0-windows-x86_64.zip"
Invoke-WebRequest "https://downloads.ai999x.com/desktop/1.5.0/$F" -OutFile $F
Invoke-WebRequest "https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS" -OutFile SHA256SUMS
(Get-FileHash $F -Algorithm SHA256).Hash.ToLower() -eq ((Select-String -SimpleMatch $F SHA256SUMS).Line -split ' ')[0]
Expand-Archive $F -DestinationPath "$env:LOCALAPPDATA\AI999X" -Force
$D = "$env:LOCALAPPDATA\AI999X\ai999x-cli-1.5.0-windows-x86_64"; $env:Path += ";$D"
[Environment]::SetEnvironmentVariable("Path", [Environment]::GetEnvironmentVariable("Path", "User") + ";$D", "User")
aerospace-engineering --version
```

### In your browser

Open https://www.ai999x.com/models/aerospace-engineering-ai999x/, choose a capability and press Compute in your browser: the same program runs on your device as WebAssembly; nothing is sent anywhere.

### Python

```shell
pip install aerospace-engineering-ai999x
```

## 2. Quick start

### 2.1 Check the install

Prints the model, its version and that this is the Rust build.

```shell
$ aerospace-engineering --version
aerospace-engineering-ai999x 1.6.0 (Rust build, ax-engine 1.6.0)
```

### 2.2 Find a capability

Search the capabilities by what you want to know; the first column is the id to pass to info and solve.

```shell
$ aerospace-engineering search "oblique shock from the deflection angle"
aero_oblique_shock                   Oblique shock from the deflection angle  (compressible flow / oblique shocks)
aero_max_deflection_angle            Maximum deflection angle of an attached oblique shock  (compressible flow / oblique shocks)
aero_oblique_shock_from_wave_angle   Oblique shock from the wave angle  (compressible flow / oblique shocks)
aero_normal_shock                    Normal shock relations  (compressible flow / normal shocks)
aero_normal_shock_from_pressure_ratio Normal shock from its pressure ratio  (compressible flow / normal shocks)
aero_nozzle_normal_shock             Normal shock position in a converging-diverging nozzle  (nozzles / off-design operation)
aero_glide_range                     Glide ratio, angle and range  (aircraft performance / gliding flight)
aero_max_climb_angle_jet             Steepest climb angle of a jet  (aircraft performance / climb)
… 12 more
```

### 2.3 List the capabilities

Every capability the program answers: its id (what you pass to solve) and its name.

```shell
$ aerospace-engineering list
aero_absolute_ceiling_jet                Absolute ceiling of a jet aircraft
aero_actuator_disk_forward               Ideal propeller in axial flight (momentum theory)
aero_actuator_disk_hover                 Ideal hover power of a rotor (momentum theory)
aero_altitude_for_period                 Circular-orbit altitude for an orbital period
aero_ballistic_coefficient               Ballistic coefficient
aero_ballistic_entry_peak_deceleration   Peak deceleration of a ballistic re-entry (Allen-Eggers)
aero_ballistic_entry_velocity            Speed and deceleration of a ballistic re-entry at an altitude
aero_bielliptic_transfer                 Bi-elliptic transfer between circular orbits
… 73 more
```

### 2.4 What a capability needs

The equation or method, every input and output with its SI unit. Read this before solve: it tells you the input names.

```shell
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

### 2.5 Solve

Inputs as name=value, each with any unit of the right kind. The answer lists every output, the method used, each input converted to SI, and the conditions the answer assumes.

```shell
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

### 2.6 Machine-readable answer

--json prints the same answer as one line of JSON (values in SI): for scripts and other programs.

```shell
$ aerospace-engineering solve aero_oblique_shock M1=2 "theta=10 deg" --json
{"outcome":"SOLVED","message":"","value":39.3139318448,"unit":"","solved_for":"beta_deg","si":[["M1",2.0],["theta",0.17453292519943295]],"values":[["beta_deg",39.3139318448,""],["M2",1.640522229,""],["p2_p1",1.706578604,""],["T2_T1",1.1701512843,""],["rho2_rho1",1.45842561291,""],["p02_p01",0.984644022503,""],["Mn1",1.26713803645,""]],"target":"beta_deg","phrase":"shock wave angle in degrees","assumptions":["1 < M1 <= 20","0 < theta < theta_max(M1) (attached shock)","1.05 <= gamma <= 1.7"],"name":"Oblique shock from the deflection angle","equation":"theta-beta-M relation tan(theta) = 2 cot(beta) (M1^2 sin^2 beta - 1)/(M1^2 (g + cos 2 beta) + 2), solved by bisection on the chosen branch; normal-shock relations on Mn1 = M1 sin beta; M2 = Mn2 / sin(beta - theta)"}
```

### 2.7 When an input is missing

No number is guessed: the answer names what is missing, shows a complete call, and the program exits with status 1.

```shell
$ aerospace-engineering solve aero_oblique_shock M1=2
NEEDS_INPUT: aero_oblique_shock needs theta (flow deflection (wedge half-) angle (deg, rad)). Example: solve('aero_oblique_shock', M1=2, theta='0.185405 rad').
```

### 2.8 Many questions at once

One JSON request per line in, one JSON answer per line out, in order: for scripts, notebooks and agents (the program starts once). In PowerShell: Get-Content cases.jsonl | aerospace-engineering batch

`cases.jsonl`:

```json
{"key": "aero_oblique_shock", "inputs": {"M1": 2, "theta": "10 deg"}}
{"key": "aero_isa_properties", "inputs": {"h": "35000 ft"}}
```

```shell
$ aerospace-engineering batch < cases.jsonl
{"outcome":"SOLVED","message":"","value":39.3139318448,"unit":"","solved_for":"beta_deg","si":[["M1",2.0],["theta",0.17453292519943295]],"values":[["beta_deg",39.3139318448,""],["M2",1.640522229,""],["p2_p1",1.706578604,""],["T2_T1",1.1701512843,""],["rho2_rho1",1.45842561291,""],["p02_p01",0.984644022503,""],["Mn1",1.26713803645,""]],"target":"beta_deg","phrase":"shock wave angle in degrees","assumptions":["1 < M1 <= 20","0 < theta < theta_max(M1) (attached shock)","1.05 <= gamma <= 1.7"],"name":"Oblique shock from the deflection angle","equation":"theta-beta-M relation tan(theta) = 2 cot(beta) (M1^2 sin^2 beta - 1)/(M1^2 (g + cos 2 beta) + 2), solved by bisection on the chosen branch; normal-shock relations on Mn1 = M1 sin beta; M2 = Mn2 / sin(beta - theta)"}
{"outcome":"SOLVED","message":"","value":0.380455436006,"unit":"kg/m^3","solved_for":"rho","si":[["h",10668.0]],"values":[["T",218.924175599,"K"],["p",23908.9066172,"Pa"],["rho",0.380455436006,"kg/m^3"],["a",296.614227594,"m/s"],["mu",0.0000143408368522,"Pa*s"],["sigma",0.310576080137,""],["delta",0.235962562223,""],["theta",0.75975768037,""],["H",10650.126831,"m"]],"target":"rho","phrase":"temperature","assumptions":["-1000 m <= h <= 47000 m"],"name":"Standard atmosphere at an altitude","equation":"H = r0 h/(r0 + h); in each layer T = Tb + L (H - Hb); p = pb (Tb/T)^(g0/(R L)) (L != 0) or pb exp(-g0 (H - Hb)/(R Tb)); rho = p/(R T); a = sqrt(gamma R T)"}
```

## 3. Writing inputs

Inputs are written `name=value`, one per argument; `aerospace-engineering info <capability>` lists the names. Quote an argument that holds a space or brackets (bash, zsh and PowerShell all accept double quotes). The answer always states each input as it was read, converted to SI, so a misread unit is visible at once.

| kind | example | how it is read |
|---|---|---|
| a number with a unit | `"R=0.128 nm"` | any unit of the same kind; converted to SI |
| a number in SI | `R=1.28e-10` | no unit means the SI unit shown by info |
| a choice | `structure=FCC` | one of the values info lists |
| a list or vector | `"v=[1, 2, 3]" or "v=1, 2, 3"` | brackets keep it a list |
| a matrix | `"A=[[1, 2], [3, 4]]"` | one list per row |
| true / false | `inclusive=true` | for yes/no options |

An equation can be solved for any of its quantities: give the others and add `--target <symbol>`. Without `--target`, the program solves for the quantity the capability is named after.

## 4. Outcomes and exit codes

The program never guesses a number: when it cannot answer, it says why and exits with status 1, so a script can tell an answer from a refusal without reading the text.

| outcome | meaning | exit status |
|---|---|---|
| SOLVED | the answer: every output with its unit, the method, the inputs in SI, the conditions assumed | 0 |
| NEEDS_INPUT | an input is missing; the message names it and shows a complete call | 1 |
| INVALID_INPUT | a value or unit of the wrong kind, an unknown input name, or a value outside the conditions | 1 |
| AMBIGUOUS | the inputs allow two different answers; say which one is meant | 1 |
| OUT_OF_SCOPE | the model has no capability for this, or the result would not be meaningful | 1 |
| (usage) | a command the program does not know: it prints its usage | 2 |

## 5. Scripts and agents: batch and JSON

`aerospace-engineering solve ... --json` prints one answer as one line of JSON. `aerospace-engineering batch` reads one request per line on standard input and writes one JSON answer per line, in order, starting the program once for any number of requests. A request is `{"key": "<capability>", "inputs": {"name": "value", ...}, "target": null}`. In PowerShell, pipe the file in: `Get-Content cases.jsonl | aerospace-engineering batch`. `aerospace-engineering catalog` prints every capability with its inputs as JSON, to build forms or tool definitions.

| answer field | meaning |
|---|---|
| outcome | one of the outcomes above |
| message | why there is no answer (empty when SOLVED) |
| value, unit | the main result in SI units |
| values | every output: [symbol, value, unit] (procedures) |
| target | the quantity solved for |
| si | each numeric input converted to SI: [symbol, value] |
| assumptions | the conditions the answer relies on |
| name, equation | the capability and its equation or method |

```python
import json, subprocess

reqs = [{"key": "<capability>", "inputs": {"name": "value"}}]
out = subprocess.run(["aerospace-engineering", "batch"], input="\n".join(map(json.dumps, reqs)),
                     capture_output=True, text=True, check=True).stdout
answers = [json.loads(line) for line in out.splitlines()]
```

## 6. Figures

`aerospace-engineering figures <capability> name=value ... --out DIR` draws the live figures for those inputs and writes each as an SVG file into DIR (an existing folder); without `--out` the SVG text comes back as JSON. The desktop app and the model page draw the same figures as soon as you press Compute.

## 7. In a web page or Node.js: WebAssembly

The same model is a WebAssembly module, `aerospace_engineering_ai999x.wasm` (https://www.ai999x.com/compute/rust/aerospace_engineering_ai999x.wasm), with a small loader, `ax-wasm.js` (https://www.ai999x.com/compute/rust/ax-wasm.js). It runs in a browser, a Web Worker or Node.js, with no server.

```html
<script src="ax-wasm.js"></script>
<script>
  AXWasm.load("aerospace_engineering_ai999x.wasm").then(function (m) {
    var a = m.solve({key: "<capability>", inputs: {name: "value"}});
    console.log(a.outcome, a.values || a.value);
    console.log(m.catalog().capabilities.length + " capabilities");
  });
</script>
```

## 8. Every capability

By topic. For each: what it does, what it needs and gives, when it applies, and a command with what the program printed for it. `aerospace-engineering info <id>` shows the same inputs and outputs in the terminal.

### Aerodynamics (3)

<a id="aero_critical_mach"></a>

#### Critical Mach number of an airfoil: `aero_critical_mach`

The free-stream Mach number at which the flow first reaches sonic speed at the airfoil's minimum-pressure point, from its incompressible minimum pressure coefficient and a compressibility rule (Prandtl-Glauert or Karman-Tsien), air (gamma = 1.4).

$$
C_{p,cr} = \frac{2}{\gamma M^2}\left[\left(\frac{2 + (\gamma-1)M^2}{\gamma+1}\right)^{\frac{\gamma}{\gamma-1}} - 1\right] = \frac{C_{p0}}{\sqrt{1 - M^2}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Cp0` | in | incompressible minimum pressure coefficient (negative) | - |
| `rule` | in | compressibility correction: prandtl-glauert or karman-tsien | - |
| `M_cr` | out | critical Mach number | - |
| `Cp_cr` | out | critical (sonic) pressure coefficient at M_cr | - |

**Applies when:** -3 <= Cp0 < -0.01; subsonic, small-perturbation compressibility rules; air, gamma = 1.4.

```shell
$ aerospace-engineering solve aero_critical_mach Cp0=-0.43 rule=prandtl-glauert
M_cr = 0.7371059142
Cp_cr = -0.6363043586
  by: Critical Mach number of an airfoil  (Cp_cr(M) = 2/(g M^2) [((1 + (g-1)/2 M^2)/(1 + (g-1)/2))^(g/(g-1)) - 1]; equate to Cp0/sqrt(1 - M^2) (or the Karman-Tsien form) and solve)
  Cp0 = -0.43
  rule = prandtl-glauert
  assumes: -3 <= Cp0 < -0.01; subsonic, small-perturbation compressibility rules; air, gamma = 1.4
```

**What the worked example means:** This calculation determines the free-stream Mach number at which local flow first becomes sonic over an airfoil, based on its incompressible minimum pressure coefficient of -0.43 and the selected compressibility rule. The resulting critical Mach number is 0.737106, with a matching critical pressure coefficient of -0.636304, indicating the threshold for significant compressibility effects.

<a id="aero_elliptic_wing_lifting_line"></a>

#### Elliptic wing by lifting-line theory: `aero_elliptic_wing_lifting_line`

The lift-curve slope, lift coefficient, induced drag coefficient and induced angle of attack of an untwisted elliptic wing of a given aspect ratio at an angle of attack measured from zero lift (Prandtl lifting-line theory, span efficiency 1).

$$
a = \frac{a_0}{1 + \frac{a_0}{\pi A\!R}},\quad C_{D_i} = \frac{C_L^2}{\pi A\!R},\quad \alpha_i = \frac{C_L}{\pi A\!R}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `AR` | in | aspect ratio | - |
| `alpha` | in | angle of attack from zero lift (deg) | rad |
| `a0` | in | section lift-curve slope per radian (default 2 pi) (optional) | - |
| `CL_alpha` | out | wing lift-curve slope per radian | - |
| `CL` | out | wing lift coefficient | - |
| `CDi` | out | induced drag coefficient | - |
| `alpha_i_deg` | out | induced angle of attack in degrees | - |

**Applies when:** 2 <= AR <= 40; 3 <= a0 <= 7.5 per rad; |alpha| <= 15 deg; untwisted elliptic planform, attached flow.

```shell
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

**What the worked example means:** The finite-wing lift-curve slope is 3.14159 per radian, producing a lift coefficient of 0.314159 and an induced drag coefficient of 0.015708.

<a id="aero_naca4_thin_airfoil"></a>

#### Thin-airfoil theory for a NACA 4-digit airfoil, `aero_naca4_thin_airfoil`

The zero-lift angle, lift coefficient and the pitching-moment coefficients about the quarter chord and the leading edge of a cambered NACA 4-digit airfoil at an angle of attack, from thin-airfoil theory.

$$
C_L = 2\pi(\alpha - \alpha_{L0}),\quad \alpha_{L0} = -\frac{1}{\pi}\int_0^\pi \frac{dz}{dx}(\cos\theta - 1)\,d\theta,\quad C_{m,c/4} = \frac{\pi}{4}(A_2 - A_1)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `m` | in | maximum camber as a fraction of the chord (first digit / 100) | - |
| `p` | in | chordwise position of maximum camber (second digit / 10; 0 for a symmetric section) | - |
| `alpha` | in | angle of attack (deg) | rad |
| `alpha_L0_deg` | out | zero-lift angle of attack in degrees | - |
| `CL` | out | section lift coefficient | - |
| `Cm_c4` | out | moment coefficient about the quarter chord | - |
| `Cm_le` | out | moment coefficient about the leading edge | - |

**Applies when:** 0 <= m <= 0.095; 0.05 <= p <= 0.95 when m > 0; |alpha| <= 15 deg; thin airfoil, inviscid, incompressible, unstalled.

```shell
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

**What the worked example means:** This calculation determines the aerodynamic characteristics of a NACA four-digit airfoil using thin-airfoil theory. Given the maximum camber fraction 0, the position of maximum camber 0, and the angle of attack 0.1 rad, it computes the section lift coefficient 0.628319, the zero-lift angle of attack 0, and the pitching moment coefficients about the quarter chord 0 and leading edge -0.15708. These values indicate how much lift the airfoil generates and where its aerodynamic center lies relative to the geometric chord.

### Aircraft performance (25)

<a id="aero_absolute_ceiling_jet"></a>

#### Absolute ceiling of a jet aircraft: `aero_absolute_ceiling_jet`

The absolute ceiling of a jet aircraft (where the available thrust just equals the minimum drag) when the thrust falls in proportion to the air density, T = T0 rho/rho0.

$$
\sigma_{abs} = \frac{2W\sqrt{K C_{D0}}}{T_0} = \frac{W}{T_0 (L/D)_{max}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `W` | in | weight | N |
| `S` | in | wing area | m^2 |
| `T0` | in | sea-level thrust | N |
| `CD0` | in | zero-lift drag coefficient | - |
| `AR` | in | aspect ratio | - |
| `e` | in | Oswald efficiency factor | - |
| `h_abs` | out | absolute ceiling | m |
| `h_abs_ft` | out | absolute ceiling in feet | - |
| `sigma` | out | density ratio at the ceiling | - |

**Applies when:** minimum drag below the sea-level thrust; ceiling below 47 km; thrust proportional to density (stated lapse); parabolic drag polar.

```shell
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

**What the worked example means:** This calculation determines the highest altitude where level flight is possible by equating available thrust to minimum drag. Using the weight 100000 N, sea-level thrust 13486.7 N, and aerodynamic coefficients including aspect ratio 7 and zero-lift drag coefficient 0.02, the model computes the required density ratio 0.5. The resulting absolute ceiling is 6669.77 m meters, or 21882.4 feet. This value represents the limit where excess thrust becomes zero, assuming thrust decreases linearly with air density.

<a id="aero_drag_polar_optimum"></a>

#### Best lift coefficients and lift-to-drag ratios of a drag polar, `aero_drag_polar_optimum`

For a parabolic drag polar CD = CD0 + CL^2/(pi e AR): the lift coefficient and L/D for minimum drag (maximum L/D), for minimum power (best propeller endurance) and for the jet's best range (maximum sqrt(CL)/CD).

$$
C_{L,md} = \sqrt{\frac{C_{D0}}{K}},\quad \left(\frac{L}{D}\right)_{max} = \frac{1}{2\sqrt{C_{D0}K}},\quad K = \frac{1}{\pi e A\!R}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `CD0` | in | zero-lift (parasite) drag coefficient | - |
| `AR` | in | wing aspect ratio b^2/S | - |
| `e` | in | Oswald span efficiency factor | - |
| `CL_md` | out | lift coefficient for minimum drag | - |
| `LD_max` | out | maximum lift-to-drag ratio | - |
| `CL_mp` | out | lift coefficient for minimum power required | - |
| `LD_mp` | out | L/D at minimum power | - |
| `CL_mr` | out | lift coefficient for best jet range (max sqrt(CL)/CD) | - |
| `LD_mr` | out | L/D at best jet range | - |

**Applies when:** 0.002 <= CD0 <= 0.2; 1 <= AR <= 50; 0.3 <= e <= 1; parabolic drag polar.

```shell
$ aerospace-engineering solve aero_drag_polar_optimum CD0=0.02 AR=10 e=0.8
CL_md = 0.7089815404
LD_max = 17.72453851
CL_mp = 1.22799205
LD_mp = 15.34990062
CL_mr = 0.4093306832
LD_mr = 15.34990062
  by: Best lift coefficients and lift-to-drag ratios of a drag polar  (K = 1/(pi e AR); CL_md = sqrt(CD0/K), (L/D)max = 1/(2 sqrt(CD0 K)); CL_mp = sqrt(3 CD0/K), CD = 4 CD0; CL_mr = sqrt(CD0/(3K)), CD = 4/3 CD0)
  CD0 = 0.02
  AR = 10
  e = 0.8
  assumes: 0.002 <= CD0 <= 0.2; 1 <= AR <= 50; 0.3 <= e <= 1; parabolic drag polar
```

In the worked example, CD0 = 0.02, AR = 10, e = 0.8 go through the equation above and give CL_md = 0.7089815404, LD_max = 17.72453851, CL_mp = 1.22799205, LD_mp = 15.34990062. It applies only when 0.002 <= CD0 <= 0.2; 1 <= AR <= 50; 0.3 <= e <= 1; parabolic drag polar: outside that it says what is wrong instead of returning a number.

<a id="aero_breguet_range_jet"></a>

#### Breguet range of a jet aircraft: `aero_breguet_range_jet`

The cruise range and flight time of a jet aircraft at constant speed and lift-to-drag ratio (cruise-climb) from its thrust-specific fuel consumption and the weights at the start and end of cruise.

$$
R = \frac{V}{g_0 c}\,\frac{L}{D}\,\ln\frac{W_0}{W_1}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `V` | in | cruise true airspeed | m/s |
| `c` | in | thrust-specific fuel consumption (kg/(N*s), 1/h, lb/(lbf*h)) | kg/(N*s) |
| `LD` | in | lift-to-drag ratio | - |
| `W0` | in | weight at the start of cruise | N |
| `W1` | in | weight at the end of cruise | N |
| `R` | out | range | m |
| `R_km` | out | range in km | - |
| `t` | out | flight time | s |
| `t_h` | out | flight time in hours | - |

**Applies when:** V > 0; 0 < W1 < W0; 1 <= L/D <= 60; constant V, L/D and TSFC (cruise-climb).

```shell
$ aerospace-engineering solve aero_breguet_range_jet "V=200 m/s" "c=2.832545036e-05 kg/(N*s)" LD=15 "W0=271.8281828 kN" "W1=100 kN"
R = 10800000 m
R_km = 10800
t = 53999.99999 s
t_h = 15
  by: Breguet range of a jet aircraft  (R = (V/(g0 c)) (L/D) ln(W0/W1), c the mass-flow TSFC)
  V = 200 m/s
  c = 2.832545036e-05 kg/(N*s)
  LD = 15
  W0 = 271828.1828 N
  W1 = 100000 N
  assumes: V > 0; 0 < W1 < W0; 1 <= L/D <= 60; constant V, L/D and TSFC (cruise-climb)
```

**What the worked example means:** This calculation determines the Breguet range for a jet aircraft using the formula R equals V divided by g0 times c, multiplied by L over D, and then multiplied by the natural log of W0 over W1. The result, 1.08e+07 m, represents the distance covered in meters, while 10800 shows this distance in kilometers. Additionally, the duration of the cruise phase is calculated as 54000 s seconds or 15 hours. These values rely on the input parameters: true airspeed 200 m/s, thrust-specific fuel consumption 2.83255e-05 kg/(N*s), lift-to-drag ratio 15, initial weight 271.828 kN, and final weight 100 kN. A higher lift-to-drag ratio or greater speed increases the range, whereas higher fuel consumption reduces it.

<a id="aero_breguet_range_prop"></a>

#### Breguet range of a propeller aircraft: `aero_breguet_range_prop`

The cruise range of a propeller-driven aircraft at constant lift-to-drag ratio and propeller efficiency from its power-specific fuel consumption and initial and final weights.

$$
R = \frac{\eta_p}{g_0 c_P}\,\frac{L}{D}\,\ln\frac{W_0}{W_1}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `eta` | in | propeller efficiency | - |
| `c` | in | power-specific fuel consumption (kg/J, g/(kW*h), lb/(hp*h)) | kg/J |
| `LD` | in | lift-to-drag ratio | - |
| `W0` | in | initial weight | N |
| `W1` | in | final weight | N |
| `R` | out | range | m |
| `R_km` | out | range in km | - |

**Applies when:** 0.1 <= eta <= 1; 0 < W1 < W0; 1 <= L/D <= 60; constant L/D, efficiency and BSFC.

```shell
$ aerospace-engineering solve aero_breguet_range_prop eta=0.8 "c=1e-07 kg/J" LD=15 "W0=27.18281828 kN" "W1=10 kN"
R = 12236594.55 m
R_km = 12236.59455
  by: Breguet range of a propeller aircraft  (R = (eta/(g0 c)) (L/D) ln(W0/W1), c the mass flow per shaft power)
  eta = 0.8
  c = 1e-07 kg/J
  LD = 15
  W0 = 27182.81828 N
  W1 = 10000 N
  assumes: 0.1 <= eta <= 1; 0 < W1 < W0; 1 <= L/D <= 60; constant L/D, efficiency and BSFC
```

**What the worked example means:** This calculation determines the maximum distance a propeller-driven aircraft can travel during cruise. It uses the initial weight of 27.1828 kN and the final weight of 10 kN, along with an aerodynamic lift-to-drag ratio of 15. The result depends on the propeller efficiency of 0.8 and the power-specific fuel consumption of 1e-07 kg/J. The computed range is 1.22366e+07 m, or 12236.6 kilometers, representing the distance covered while maintaining steady flight conditions.

<a id="aero_breguet_endurance_jet"></a>

#### Endurance of a jet aircraft: `aero_breguet_endurance_jet`

The time a jet aircraft can stay airborne at constant lift-to-drag ratio from its thrust-specific fuel consumption and initial and final weights (independent of speed).

$$
E = \frac{1}{g_0 c}\,\frac{L}{D}\,\ln\frac{W_0}{W_1}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `c` | in | thrust-specific fuel consumption (kg/(N*s), 1/h, lb/(lbf*h)) | kg/(N*s) |
| `LD` | in | lift-to-drag ratio | - |
| `W0` | in | initial weight | N |
| `W1` | in | final weight | N |
| `E` | out | endurance | s |
| `E_h` | out | endurance in hours | - |

**Applies when:** 0 < W1 < W0; 1 <= L/D <= 60; constant L/D and TSFC.

```shell
$ aerospace-engineering solve aero_breguet_endurance_jet "c=2.832545036e-05 kg/(N*s)" LD=15 "W0=271.8281828 kN" "W1=100 kN"
E = 53999.99999 s
E_h = 15
  by: Endurance of a jet aircraft  (E = (1/(g0 c)) (L/D) ln(W0/W1))
  c = 2.832545036e-05 kg/(N*s)
  LD = 15
  W0 = 271828.1828 N
  W1 = 100000 N
  assumes: 0 < W1 < W0; 1 <= L/D <= 60; constant L/D and TSFC
```

**What the worked example means:** This calculation determines the total time a jet aircraft can remain airborne during steady cruise. It uses the thrust specific fuel consumption, lift to drag ratio, and the natural logarithm of the ratio between initial weight and final weight. The resulting endurance of 54000 s seconds indicates how long the aircraft can fly before its weight drops to the specified final value, assuming constant aerodynamic efficiency and engine performance.

<a id="aero_prop_endurance_constant_altitude"></a>

#### Endurance of a propeller aircraft at constant altitude, `aero_prop_endurance_constant_altitude`

How long a propeller-driven aircraft can loiter at a constant altitude and lift coefficient, from its propeller efficiency, power-specific fuel consumption, drag polar point, wing area and weights.

$$
E = \frac{\eta_p}{g_0 c_P}\,\frac{C_L^{3/2}}{C_D}\sqrt{2\rho S}\left(W_1^{-1/2} - W_0^{-1/2}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `eta` | in | propeller efficiency | - |
| `c` | in | power-specific fuel consumption | kg/J |
| `CL` | in | lift coefficient | - |
| `CD` | in | drag coefficient | - |
| `S` | in | wing area | m^2 |
| `h` | in | altitude | m |
| `W0` | in | initial weight | N |
| `W1` | in | final weight | N |
| `E` | out | endurance | s |
| `E_h` | out | endurance in hours | - |

**Applies when:** 0.1 <= eta <= 1; 0 < W1 < W0; 0.05 <= CL <= 3; constant altitude and CL.

```shell
$ aerospace-engineering solve aero_prop_endurance_constant_altitude eta=0.8 "c=1e-07 kg/J" CL=1 CD=0.05 "S=20 m^2" "h=0 m" "W0=40 kN" "W1=10 kN"
E = 571040.8825 s
E_h = 158.6224674
  by: Endurance of a propeller aircraft at constant altitude  (E = (eta/(g0 c)) (CL^1.5/CD) sqrt(2 rho S) (W1^(-1/2) - W0^(-1/2)))
  eta = 0.8
  c = 1e-07 kg/J
  CL = 1
  CD = 0.05
  S = 20 m^2
  h = 0 m
  W0 = 40000 N
  W1 = 10000 N
  assumes: 0.1 <= eta <= 1; 0 < W1 < W0; 0.05 <= CL <= 3; constant altitude and CL
```

**What the worked example means:** This calculation determines the total time a propeller-driven aircraft can remain airborne at a constant altitude and lift coefficient. It uses inputs such as propeller efficiency 0.8, power-specific fuel consumption 1e-07 kg/J, aerodynamic coefficients 1 and 0.05, wing area 20 m^2, and weight limits 40 kN and 10 kN. The resulting endurance 571041 s represents the maximum duration available for mission tasks like surveillance before fuel reserves are exhausted.

<a id="aero_jet_fuel_for_range"></a>

#### Fuel a jet needs for a cruise range: `aero_jet_fuel_for_range`

The fuel weight (and mass) a jet aircraft burns cruising a distance at constant speed and L/D, and its weight at the end, from the Breguet relation.

$$
W_1 = W_0\exp\left(-\frac{R\,g_0 c}{V\,(L/D)}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `R` | in | cruise distance (km, nmi) | m |
| `V` | in | cruise speed | m/s |
| `c` | in | thrust-specific fuel consumption | kg/(N*s) |
| `LD` | in | lift-to-drag ratio | - |
| `W0` | in | weight at the start of cruise | N |
| `W1` | out | weight at the end of cruise | N |
| `Wf` | out | fuel weight burned | N |
| `mf` | out | fuel mass burned | kg |

**Applies when:** R, V > 0; fuel at most 80% of W0; constant V, L/D and TSFC.

```shell
$ aerospace-engineering solve aero_jet_fuel_for_range "R=10800 km" "V=200 m/s" "c=2.832545036e-05 kg/(N*s)" LD=15 "W0=271.8281828 kN"
W1 = 99999.99998 N
Wf = 171828.1828 N
mf = 17521.59839 kg
  by: Fuel a jet needs for a cruise range  (W1 = W0 exp(-R g0 c/(V L/D)); Wf = W0 - W1)
  R = 10800000 m
  V = 200 m/s
  c = 2.832545036e-05 kg/(N*s)
  LD = 15
  W0 = 271828.1828 N
  assumes: R, V > 0; fuel at most 80% of W0; constant V, L/D and TSFC
```

**What the worked example means:** This calculation determines the fuel consumed and remaining aircraft weight during a constant-speed cruise. Using the Breguet range equation, it processes inputs like cruise distance, speed, lift-to-drag ratio, and thrust-specific fuel consumption against the initial takeoff weight. The result shows that for a journey covering 10800 km, the aircraft expends 171828 N in fuel, resulting in an end-of-cruise weight of 100000 N and a burned mass of 17521.6 kg. This helps verify if the planned route is feasible within current fuel reserves.

<a id="aero_glide_range"></a>

#### Glide ratio, angle and range: `aero_glide_range`

The still-air glide range R = h (L/D) from a height h, the glide angle atan(D/L) and, with the airspeed, the sink rate and time aloft.

$$
R = h\,\frac{L}{D},\quad \tan\gamma = \frac{D}{L}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `dh` | in | height to be lost | m |
| `LD` | in | lift-to-drag (glide) ratio | - |
| `V` | in | airspeed along the glide path (optional) (optional) | m/s |
| `range` | out | glide range over the ground | m |
| `range_km` | out | glide range, km | - |
| `glide_angle_deg` | out | glide angle below the horizon, degrees | - |
| `sink_rate` | out | sink rate V sin(gamma) | m/s |
| `time` | out | time to lose the height | s |

**Applies when:** h > 0; 0 < L/D <= 100; still air, steady glide.

```shell
$ aerospace-engineering solve aero_glide_range "dh=2000 m" LD=15
range = 30000 m
range_km = 30
glide_angle_deg = 3.814074834
  by: Glide ratio, angle and range  (tan(gamma) = D/L; R = h L/D; sink rate = V sin(gamma))
  dh = 2000 m
  LD = 15
  assumes: h > 0; 0 < L/D <= 100; still air, steady glide
```

**What the worked example means:** From a height of 2000 m, an aircraft with a lift-to-drag ratio of 15 glides a range of 30000 m at a glide angle of 3.81407.

<a id="aero_induced_drag"></a>

#### Induced drag in level flight: `aero_induced_drag`

The lift coefficient, induced drag coefficient CL^2/(pi e AR), induced drag and induced power of an aircraft in level flight at a true airspeed and altitude.

$$
C_{D_i} = \frac{C_L^2}{\pi e A\!R},\quad D_i = \frac{W^2}{q S\,\pi e A\!R}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `W` | in | aircraft weight | N |
| `S` | in | wing area | m^2 |
| `V` | in | true airspeed | m/s |
| `h` | in | altitude (standard atmosphere) | m |
| `AR` | in | wing aspect ratio b^2/S | - |
| `e` | in | Oswald span efficiency factor | - |
| `CL` | out | lift coefficient | - |
| `CDi` | out | induced drag coefficient | - |
| `Di` | out | induced drag | N |
| `P_induced` | out | induced power Di V | W |

**Applies when:** W, S, V > 0; 1 <= AR <= 50; 0.3 <= e <= 1; level flight, L = W.

```shell
$ aerospace-engineering solve aero_induced_drag "W=10 kN" "S=16 m^2" "V=50 m/s" "h=0 m" AR=8 e=0.8
CL = 0.4081635466
CDi = 0.008285878927
Di = 203.0038938 N
P_induced = 10150.19469 W
  by: Induced drag in level flight  (CL = W/(q S), CDi = CL^2/(pi e AR), Di = q S CDi)
  W = 10000 N
  S = 16 m^2
  V = 50 m/s
  h = 0 m
  AR = 8
  e = 0.8
  assumes: W, S, V > 0; 1 <= AR <= 50; 0.3 <= e <= 1; level flight, L = W
```

**What the worked example means:** This calculation determines the lift coefficient, induced drag coefficient, total induced drag, and induced power for an aircraft in level flight. It uses the weight, wing area, true airspeed, altitude, aspect ratio, and Oswald efficiency factor. The resulting induced drag of 203.004 N represents the aerodynamic penalty incurred to generate the necessary lift, while the induced power of 10150.2 W indicates the energy required per unit time to overcome this specific component of drag.

<a id="aero_jet_range_constant_altitude"></a>

#### Jet range at constant altitude and lift coefficient, `aero_jet_range_constant_altitude`

The range and flight time of a jet cruising at a constant altitude and constant lift coefficient, so that its speed falls as fuel burns (the second Breguet form).

$$
R = \frac{2}{g_0 c}\sqrt{\frac{2}{\rho S}}\,\frac{C_L^{1/2}}{C_D}\left(\sqrt{W_0} - \sqrt{W_1}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `W0` | in | initial weight | N |
| `W1` | in | final weight | N |
| `S` | in | wing area | m^2 |
| `h` | in | cruise altitude | m |
| `CL` | in | cruise lift coefficient | - |
| `CD` | in | cruise drag coefficient | - |
| `c` | in | thrust-specific fuel consumption | kg/(N*s) |
| `R` | out | range | m |
| `R_km` | out | range in km | - |
| `t` | out | flight time | s |
| `t_h` | out | flight time in hours | - |

**Applies when:** 0 < W1 < W0; 0.05 <= CL <= 3; CD < CL; constant altitude, CL and TSFC.

```shell
$ aerospace-engineering solve aero_jet_range_constant_altitude "W0=400 kN" "W1=100 kN" "S=100 m^2" "h=0 m" CL=0.5 CD=0.05 "c=1e-05 kg/(N*s)"
R = 11653903.59 m
R_km = 11653.90359
t = 141362.6836 s
t_h = 39.26741211
  by: Jet range at constant altitude and lift coefficient  (R = (2/(g0 c)) sqrt(2/(rho S)) (sqrt(CL)/CD) (sqrt(W0) - sqrt(W1)); t = (CL/CD)/(g0 c) ln(W0/W1))
  W0 = 400000 N
  W1 = 100000 N
  S = 100 m^2
  h = 0 m
  CL = 0.5
  CD = 0.05
  c = 1e-05 kg/(N*s)
  assumes: 0 < W1 < W0; 0.05 <= CL <= 3; CD < CL; constant altitude, CL and TSFC
```

**What the worked example means:** This calculation determines the distance a jet can travel while maintaining a constant altitude and lift coefficient. Using an initial weight of 400 kN and a final weight of 100 kN, along with a wing area of 100 m^2 and a cruise altitude of 0 m, the model computes a range of 1.16539e+07 m and a duration of 141363 s. The result reflects how fuel consumption and aerodynamic efficiency interact when the aircraft slows down as it becomes lighter.

<a id="aero_landing_ground_roll"></a>

#### Landing ground roll: `aero_landing_ground_roll`

The ground-roll distance and time from touchdown to a stop of an aircraft braking on a level runway, with aerodynamic drag, lift (which unloads the wheels) and optional reverse thrust.

$$
s = \frac{1}{2B}\ln\left(1 + \frac{B V_{TD}^2}{A}\right),\quad A = g\left(\frac{T_r}{W} + \mu\right),\quad B = \frac{g\rho S}{2W}(C_D - \mu C_L)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `W` | in | weight at landing | N |
| `S` | in | wing area | m^2 |
| `h` | in | airfield altitude | m |
| `VTD` | in | touchdown speed | m/s |
| `mu` | in | braking friction coefficient | - |
| `CL` | in | lift coefficient during the roll | - |
| `CD` | in | drag coefficient during the roll | - |
| `Tr` | in | reverse thrust (default 0) (optional) | N |
| `s` | out | ground-roll distance | m |
| `t` | out | ground-roll time | s |

**Applies when:** 0 <= mu <= 0.8; the net deceleration positive throughout; level runway, no wind, constant reverse thrust.

```shell
$ aerospace-engineering solve aero_landing_ground_roll "W=10 kN" "S=10 m^2" "h=0 m" "VTD=30 m/s" mu=0.4 CL=0.25 CD=0.1
s = 114.718074 m
t = 7.647871597 s
  by: Landing ground roll  (-dV/dt = A + B V^2, A = g (Tr/W + mu), B = g rho S (CD - mu CL)/(2W); s = ln(1 + B VTD^2/A)/(2B); t = atan(VTD sqrt(B/A))/sqrt(A B))
  W = 10000 N
  S = 10 m^2
  h = 0 m
  VTD = 30 m/s
  mu = 0.4
  CL = 0.25
  CD = 0.1
  assumes: 0 <= mu <= 0.8; the net deceleration positive throughout; level runway, no wind, constant reverse thrust
```

**What the worked example means:** The calculated ground-roll distance is 114.718 m and the time to stop is 7.64787 s.

<a id="aero_level_turn_bank"></a>

#### Level coordinated turn at a bank angle: `aero_level_turn_bank`

The load factor, turn radius, turn rate and the time for a full circle of an aircraft in a level, coordinated turn at a true airspeed and bank angle.

$$
n = \frac{1}{\cos\phi},\quad R = \frac{V^2}{g\tan\phi},\quad \omega = \frac{g\tan\phi}{V}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `V` | in | true airspeed | m/s |
| `phi` | in | bank angle (deg) | rad |
| `g` | in | gravity (default 9.80665 m/s^2) (optional) | m/s^2 |
| `n` | out | load factor L/W | - |
| `phi_deg` | out | bank angle in degrees | - |
| `R` | out | turn radius | m |
| `omega` | out | turn rate | rad/s |
| `omega_deg` | out | turn rate in degrees per second | - |
| `t_360` | out | time for a 360-degree turn | s |

**Applies when:** 1 deg <= phi <= 85 deg; V > 0; level, coordinated turn at constant speed.

```shell
$ aerospace-engineering solve aero_level_turn_bank "V=100 m/s" "phi=45 deg"
n = 1.414213562
phi_deg = 45
R = 1019.716213 m
omega = 0.0980665 rad/s
omega_deg = 5.618796562
t_360 = 64.07065927 s
  by: Level coordinated turn at a bank angle  (n = 1/cos(phi); R = V^2/(g tan(phi)); omega = g tan(phi)/V)
  V = 100 m/s
  phi = 0.7853981634 rad
  assumes: 1 deg <= phi <= 85 deg; V > 0; level, coordinated turn at constant speed
```

**What the worked example means:** This calculation determines the kinematic and dynamic parameters for an aircraft executing a level, coordinated turn. Using the true airspeed of 100 m/s m/s and a bank angle of 45 deg radians, it computes the necessary load factor as 1.41421, which represents the ratio of lift to weight. The resulting turn radius is 1019.72 m meters, with a turn rate of 0.0980665 rad/s radians per second. These values indicate how tightly the aircraft can turn without slipping or skidding while maintaining constant altitude.

<a id="aero_level_turn_load_factor"></a>

#### Level turn at a load factor: `aero_level_turn_load_factor`

The bank angle, turn radius, turn rate and time for a full circle of a level coordinated turn flown at a true airspeed and load factor (g).

$$
R = \frac{V^2}{g\sqrt{n^2 - 1}},\quad \omega = \frac{g\sqrt{n^2-1}}{V}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `V` | in | true airspeed | m/s |
| `n` | in | load factor (g) | - |
| `g` | in | gravity (default 9.80665 m/s^2) (optional) | m/s^2 |
| `n` | out | load factor | - |
| `phi_deg` | out | bank angle in degrees | - |
| `R` | out | turn radius | m |
| `omega` | out | turn rate | rad/s |
| `omega_deg` | out | turn rate in degrees per second | - |
| `t_360` | out | time for a 360-degree turn | s |

**Applies when:** 1.0002 <= n <= 11.5; V > 0; level, coordinated turn at constant speed.

```shell
$ aerospace-engineering solve aero_level_turn_load_factor "V=100 m/s" n=2
n = 2
phi_deg = 60
R = 588.7334301 m
omega = 0.1698561605 rad/s
omega_deg = 9.732041122
t_360 = 36.99121238 s
  by: Level turn at a load factor  (cos(phi) = 1/n; R = V^2/(g sqrt(n^2 - 1)); omega = g sqrt(n^2 - 1)/V)
  V = 100 m/s
  n = 2
  assumes: 1.0002 <= n <= 11.5; V > 0; level, coordinated turn at constant speed
```

**What the worked example means:** This calculation determines the flight parameters for a coordinated level turn. Given a true airspeed of 100 m/s and a load factor of 2, it computes the required bank angle, which results in 60. The turn radius is calculated as 588.733 m, and the angular velocity of the turn is 0.169856 rad/s. These values describe the geometric path and dynamic performance of the aircraft during the maneuver.

<a id="aero_level_flight_lift_coefficient"></a>

#### Lift coefficient in level flight: `aero_level_flight_lift_coefficient`

The lift coefficient CL = 2W/(rho V^2 S) needed for steady level flight at a true airspeed and altitude, with the dynamic pressure.

$$
C_L = \frac{2W}{\rho V^2 S}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `W` | in | aircraft weight | N |
| `S` | in | wing area | m^2 |
| `V` | in | true airspeed (m/s, kt, km/h) | m/s |
| `h` | in | altitude (standard atmosphere) | m |
| `CL` | out | lift coefficient | - |
| `q` | out | dynamic pressure | Pa |
| `wing_loading` | out | wing loading W/S | N/m^2 |

**Applies when:** W, S, V > 0; -1 km <= h <= 47 km; level flight, L = W.

```shell
$ aerospace-engineering solve aero_level_flight_lift_coefficient "W=10 kN" "S=16 m^2" "V=50 m/s" "h=0 m"
CL = 0.4081635466
q = 1531.248945 Pa
wing_loading = 625 N/m^2
  by: Lift coefficient in level flight  (CL = W/(q S), q = rho V^2/2)
  W = 10000 N
  S = 16 m^2
  V = 50 m/s
  h = 0 m
  assumes: W, S, V > 0; -1 km <= h <= 47 km; level flight, L = W
```

**What the worked example means:** This calculation determines the lift coefficient necessary to maintain steady level flight, given an aircraft weight of 10 kN, wing area of 16 m^2, true airspeed of 50 m/s, and altitude of 0 m. The result, 0.408164, represents the aerodynamic efficiency required from the wings. The associated dynamic pressure is 1531.25 Pa, and the wing loading is 625 N/m^2. These values help assess whether the current flight conditions are within the operational envelope of the aircraft.

<a id="aero_lift_drag_forces"></a>

#### Lift, drag and lift-to-drag ratio from a drag polar, `aero_lift_drag_forces`

The dynamic pressure, lift q S CL, the drag coefficient CD0 + CL^2/(pi e AR) of the parabolic drag polar, the parasite, induced and total drag and the lift-to-drag ratio of a wing at a lift coefficient, airspeed and air density (or altitude of the standard atmosphere).

$$
L = \tfrac12\rho V^2 S C_L,\quad D = \tfrac12\rho V^2 S\left(C_{D_0} + \frac{C_L^2}{\pi e A\!R}\right),\quad \frac{L}{D} = \frac{C_L}{C_D}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `V` | in | true airspeed | m/s |
| `S` | in | wing area | m^2 |
| `CL` | in | lift coefficient | - |
| `CD0` | in | zero-lift (parasite) drag coefficient | - |
| `AR` | in | wing aspect ratio b^2/S | - |
| `e` | in | Oswald span efficiency factor | - |
| `rho` | in | air density (give the density or the altitude) (optional) | kg/m^3 |
| `h` | in | altitude of the standard atmosphere (give the density or the altitude) (optional) | m |
| `q` | out | dynamic pressure | Pa |
| `CD` | out | drag coefficient CD0 + K CL^2 | - |
| `L` | out | lift | N |
| `D` | out | total drag | N |
| `D0` | out | parasite (zero-lift) drag | N |
| `Di` | out | induced drag | N |
| `LD` | out | lift-to-drag ratio | - |

**Applies when:** V, S > 0; 0 < CL <= 3; 0.002 <= CD0 <= 0.2; 1 <= AR <= 50; 0.3 <= e <= 1; the air density (at most 2 kg/m^3) or an altitude from -1 km to 47 km, one of them.

```shell
$ aerospace-engineering solve aero_lift_drag_forces "rho=1.225 kg/m^3" "V=60 m/s" "S=16 m^2" CL=0.5 CD0=0.025 AR=8 e=0.8
q = 2205 Pa
CD = 0.03743397993
L = 17640 N
D = 1320.670812 N
D0 = 882 N
Di = 438.6708119 N
LD = 13.35684854
  by: Lift, drag and lift-to-drag ratio from a drag polar  (q = rho V^2/2; L = q S CL; CD = CD0 + CL^2/(pi e AR); D0 = q S CD0; Di = q S CL^2/(pi e AR); L/D = CL/CD)
  rho = 1.225 kg/m^3
  V = 60 m/s
  S = 16 m^2
  CL = 0.5
  CD0 = 0.025
  AR = 8
  e = 0.8
  assumes: V, S > 0; 0 < CL <= 3; 0.002 <= CD0 <= 0.2; 1 <= AR <= 50; 0.3 <= e <= 1; the air density (at most 2 kg/m^3) or an altitude from -1 km to 47 km, one of them
```

In the worked example, rho = 1.225 kg/m^3, V = 60 m/s, S = 16 m^2, CL = 0.5, CD0 = 0.025, AR = 8, e = 0.8 go through the equation above and give q = 2205 Pa, CD = 0.03743397993, L = 17640 N, D = 1320.670812 N. It applies only when V, S > 0; 0 < CL <= 3; 0.002 <= CD0 <= 0.2; 1 <= AR <= 50; 0.3 <= e <= 1; the air density (at most 2 kg/m^3) or an altitude from -1 km to 47 km, one of them: outside that it says what is wrong instead of returning a number.

<a id="aero_max_rate_of_climb_jet"></a>

#### Maximum rate of climb of a jet: `aero_max_rate_of_climb_jet`

The maximum rate of climb of a jet aircraft (constant thrust) at an altitude, the speed at which it occurs and the climb angle there, for a parabolic drag polar.

$$
V^2 = \frac{T + \sqrt{T^2 + 12ab}}{6a},\quad RC_{max} = \frac{V(T - aV^2 - b/V^2)}{W}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `W` | in | weight | N |
| `S` | in | wing area | m^2 |
| `h` | in | altitude | m |
| `T` | in | available thrust | N |
| `CD0` | in | zero-lift drag coefficient | - |
| `AR` | in | aspect ratio | - |
| `e` | in | Oswald efficiency factor | - |
| `RC_max` | out | maximum rate of climb | m/s |
| `V` | out | speed for the maximum rate of climb | m/s |
| `gamma_deg` | out | climb angle at that speed, degrees | - |

**Applies when:** minimum drag < T < 0.8 W; small climb angle (lift = weight); parabolic drag polar.

```shell
$ aerospace-engineering solve aero_max_rate_of_climb_jet W=100000.0 S=30.0 h=0.0 T=30000.0 CD0=0.02 AR=7.0 e=0.8
RC_max = 31.13321496 m/s
V = 167.945431 m/s
gamma_deg = 10.6831113
  by: Maximum rate of climb of a jet  (RC = V (T - D)/W with D = a V^2 + b/V^2, a = rho S CD0/2, b = 2 K W^2/(rho S); dRC/dV = 0 gives V^2 = (T + sqrt(T^2 + 12 a b))/(6a))
  W = 100000 N
  S = 30 m^2
  h = 0 m
  T = 30000 N
  CD0 = 0.02
  AR = 7
  e = 0.8
  assumes: minimum drag < T < 0.8 W; small climb angle (lift = weight); parabolic drag polar
```

**What the worked example means:** This calculation determines the optimal vertical ascent performance for a jet aircraft under steady thrust conditions. Using the provided weight, wing area, altitude, and aerodynamic coefficients, it identifies the specific airspeed that yields the highest possible climb rate. The result shows a maximum rate of climb of 31.1332 m/s m/s achieved at an indicated airspeed of 167.945 m/s m/s, corresponding to a climb angle of 10.6831 degrees. This information helps pilots select the best speed for gaining altitude efficiently and allows engineers to assess the aircraft's performance margins against mission requirements.

<a id="aero_max_rate_of_climb_prop"></a>

#### Maximum rate of climb of a propeller aircraft, `aero_max_rate_of_climb_prop`

The maximum rate of climb of a propeller aircraft (constant shaft power and propeller efficiency) at an altitude, the speed for it (minimum power required) and the climb angle there.

$$
RC_{max} = \frac{\eta_p P - P_{R,min}}{W},\quad V_{mp} = \sqrt{\frac{2W}{\rho S}}\left(\frac{K}{3C_{D0}}\right)^{1/4}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `W` | in | weight | N |
| `S` | in | wing area | m^2 |
| `h` | in | altitude | m |
| `P` | in | shaft power (W, kW, hp) | W |
| `eta` | in | propeller efficiency | - |
| `CD0` | in | zero-lift drag coefficient | - |
| `AR` | in | aspect ratio | - |
| `e` | in | Oswald efficiency factor | - |
| `RC_max` | out | maximum rate of climb | m/s |
| `V` | out | speed for the maximum rate of climb | m/s |
| `gamma_deg` | out | climb angle in degrees | - |

**Applies when:** available power above the minimum power required; climb angle below 30 degrees (lift = weight); parabolic drag polar.

```shell
$ aerospace-engineering solve aero_max_rate_of_climb_prop W=11000.0 S=16.0 h=0.0 P=120000.0 eta=0.8 CD0=0.03 AR=7.5 e=0.8
RC_max = 6.022648238 m/s
V = 29.3560451 m/s
gamma_deg = 11.83878973
  by: Maximum rate of climb of a propeller aircraft  (RC = (eta P - D V)/W is largest at the minimum-power speed V = sqrt(2W/(rho S)) (K/(3 CD0))^(1/4))
  W = 11000 N
  S = 16 m^2
  h = 0 m
  P = 120000 W
  eta = 0.8
  CD0 = 0.03
  AR = 7.5
  e = 0.8
  assumes: available power above the minimum power required; climb angle below 30 degrees (lift = weight); parabolic drag polar
```

**What the worked example means:** This calculation determines the best climb performance for a propeller-driven aircraft. It finds the maximum vertical speed, which is 6.02265 m/s, and the specific airspeed required to achieve it, 29.356 m/s, at the given altitude. The result also includes the climb angle, 11.8388, which is critical for takeoff safety and operational planning.

<a id="aero_service_ceiling_jet"></a>

#### Service ceiling of a jet aircraft: `aero_service_ceiling_jet`

The service ceiling of a jet aircraft: the altitude where its maximum rate of climb has fallen to 100 ft/min (0.508 m/s, or a stated value), with thrust proportional to density.

$$
RC_{max}(h_s) = RC_s,\quad T(h) = T_0\frac{\rho(h)}{\rho_0}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `W` | in | weight | N |
| `S` | in | wing area | m^2 |
| `T0` | in | sea-level thrust | N |
| `CD0` | in | zero-lift drag coefficient | - |
| `AR` | in | aspect ratio | - |
| `e` | in | Oswald efficiency factor | - |
| `RCs` | in | rate of climb that defines the ceiling (default 100 ft/min = 0.508 m/s) (optional) | m/s |
| `h_s` | out | service ceiling | m |
| `h_s_ft` | out | service ceiling in feet | - |

**Applies when:** T0 < 0.8 W; climb possible at sea level; ceiling below 47 km; thrust proportional to density; small climb angle.

```shell
$ aerospace-engineering solve aero_service_ceiling_jet W=100000.0 S=30.0 T0=30000.0 CD0=0.02 AR=7.0 e=0.8
h_s = 12557.92017 m
h_s_ft = 41200.52548
  by: Service ceiling of a jet aircraft  (solve RC_max(h) = RCs by bisection, with RC_max from the optimum climb speed V^2 = (T + sqrt(T^2 + 12ab))/(6a) and T = T0 rho/rho0)
  W = 100000 N
  S = 30 m^2
  T0 = 30000 N
  CD0 = 0.02
  AR = 7
  e = 0.8
  assumes: T0 < 0.8 W; climb possible at sea level; ceiling below 47 km; thrust proportional to density; small climb angle
```

**What the worked example means:** The service ceiling is calculated as 12557.9 m, corresponding to 41200.5 feet.

<a id="aero_min_drag_power_speeds"></a>

#### Speeds for minimum drag, minimum power and best jet range, `aero_min_drag_power_speeds`

In steady level flight at an altitude: the speed for minimum drag and the minimum drag, the speed for minimum power required and that power, and the jet's best-range speed, for a parabolic drag polar.

$$
V_{md} = \sqrt{\frac{2W}{\rho S}\sqrt{\frac{K}{C_{D0}}}},\quad D_{min} = 2W\sqrt{K C_{D0}},\quad V_{mp} = \frac{V_{md}}{3^{1/4}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `W` | in | aircraft weight (N, kN, lbf) | N |
| `S` | in | wing area (m^2, ft^2) | m^2 |
| `h` | in | altitude (standard atmosphere) | m |
| `CD0` | in | zero-lift drag coefficient | - |
| `AR` | in | aspect ratio | - |
| `e` | in | Oswald efficiency factor | - |
| `V_md` | out | speed for minimum drag (max L/D) | m/s |
| `D_min` | out | minimum drag | N |
| `V_mp` | out | speed for minimum power required | m/s |
| `P_min` | out | minimum power required | W |
| `V_mr` | out | best-range speed of a jet (max V/D) | m/s |

**Applies when:** W, S > 0; -1 km <= h <= 47 km; parabolic drag polar; steady level flight, L = W.

```shell
$ aerospace-engineering solve aero_min_drag_power_speeds "W=10 kN" "S=20 m^2" "h=0 m" CD0=0.02 AR=10 e=0.8
V_md = 33.93240497 m/s
D_min = 564.1895835 N
V_mp = 25.78305219 m/s
P_min = 16796.88542 W
V_mr = 44.65755637 m/s
  by: Speeds for minimum drag, minimum power and best jet range  (V_md = sqrt(2W/(rho S) sqrt(K/CD0)); D_min = 2W sqrt(K CD0); V_mp = V_md/3^(1/4); V_mr = 3^(1/4) V_md)
  W = 10000 N
  S = 20 m^2
  h = 0 m
  CD0 = 0.02
  AR = 10
  e = 0.8
  assumes: W, S > 0; -1 km <= h <= 47 km; parabolic drag polar; steady level flight, L = W
```

**What the worked example means:** This calculation determines key performance speeds for an aircraft in steady level flight using a parabolic drag polar. Based on the weight of 10 kN and wing area of 20 m^2 at altitude 0 m, it computes the speed for maximum lift-to-drag ratio as 33.9324 m/s. The corresponding minimum drag is 564.19 N. Additionally, it finds the speed for minimum power required, which is lower than the minimum drag speed, calculated as 25.7831 m/s, and the speed for maximum range for a jet engine, which is higher, given by 44.6576 m/s. These values help pilots and engineers understand optimal operating points for efficiency and endurance.

<a id="aero_stall_speed"></a>

#### Stall speed in level flight: `aero_stall_speed`

The stall speed V_s = sqrt(2W/(rho S CL_max)) of an aircraft in level flight at an altitude of the standard atmosphere, as true and equivalent airspeed.

$$
V_s = \sqrt{\frac{2W}{\rho S C_{L,max}}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `W` | in | aircraft weight (N, kN, lbf) | N |
| `S` | in | wing area (m^2, ft^2) | m^2 |
| `CLmax` | in | maximum lift coefficient | - |
| `h` | in | altitude (standard atmosphere) | m |
| `V_stall` | out | stall speed (true airspeed) | m/s |
| `V_stall_kt` | out | stall speed in knots (TAS) | - |
| `EAS_stall` | out | stall speed as equivalent airspeed | m/s |
| `wing_loading` | out | wing loading W/S | N/m^2 |

**Applies when:** W, S > 0; 0.5 <= CL_max <= 5; -1 km <= h <= 47 km; level flight, load factor 1.

```shell
$ aerospace-engineering solve aero_stall_speed "W=10 kN" "S=16 m^2" CLmax=1.6 "h=0 m"
V_stall = 25.25382231 m/s
V_stall_kt = 49.08950342
EAS_stall = 25.25382231 m/s
wing_loading = 625 N/m^2
  by: Stall speed in level flight  (V_s = sqrt(2 W/(rho S CL_max)); EAS = V sqrt(rho/rho0))
  W = 10000 N
  S = 16 m^2
  CLmax = 1.6
  h = 0 m
  assumes: W, S > 0; 0.5 <= CL_max <= 5; -1 km <= h <= 47 km; level flight, load factor 1
```

**What the worked example means:** This calculation determines the minimum speed required for an aircraft to maintain level flight without stalling, based on its weight of 10 kN, wing area of 16 m^2, maximum lift coefficient of 1.6, and altitude of 0 m. The resulting true airspeed of 25.2538 m/s represents the actual speed through the air mass, while the equivalent airspeed of 25.2538 m/s reflects the dynamic pressure at sea level that would produce the same aerodynamic effect. The wing loading of 625 N/m^2 indicates how much weight each unit of wing area must support, directly influencing these speed requirements.

<a id="aero_max_climb_angle_jet"></a>

#### Steepest climb angle of a jet: `aero_max_climb_angle_jet`

The maximum climb angle of a jet aircraft (constant thrust, small-angle approximation lift = weight), the speed for it and the rate of climb at that speed.

$$
\sin\gamma_{max} = \frac{T}{W} - \frac{1}{(L/D)_{max}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `W` | in | weight | N |
| `S` | in | wing area | m^2 |
| `h` | in | altitude | m |
| `T` | in | available thrust | N |
| `CD0` | in | zero-lift drag coefficient | - |
| `AR` | in | aspect ratio | - |
| `e` | in | Oswald efficiency factor | - |
| `gamma_max_deg` | out | maximum climb angle in degrees | - |
| `V` | out | speed for the steepest climb | m/s |
| `RC` | out | rate of climb at that speed | m/s |

**Applies when:** minimum drag < T < 0.8 W; lift = weight (small angle); parabolic drag polar.

```shell
$ aerospace-engineering solve aero_max_climb_angle_jet "W=100 kN" "S=30 m^2" "h=0 m" "T=30 kN" CD0=0.02 AR=7 e=0.8
gamma_max_deg = 13.44821644
V = 95.78433798 m/s
RC = 22.27622315 m/s
  by: Steepest climb angle of a jet  (sin(gamma_max) = T/W - 2 sqrt(K CD0) at the minimum-drag speed)
  W = 100000 N
  S = 30 m^2
  h = 0 m
  T = 30000 N
  CD0 = 0.02
  AR = 7
  e = 0.8
  assumes: minimum drag < T < 0.8 W; lift = weight (small angle); parabolic drag polar
```

In the worked example, W = 100 kN, S = 30 m^2, h = 0 m, T = 30 kN, CD0 = 0.02, AR = 7, e = 0.8 go through the equation above and give gamma_max_deg = 13.44821644, V = 95.78433798 m/s, RC = 22.27622315 m/s. It applies only when minimum drag < T < 0.8 W; lift = weight (small angle); parabolic drag polar: outside that it says what is wrong instead of returning a number.

<a id="aero_takeoff_ground_roll"></a>

#### Takeoff ground roll: `aero_takeoff_ground_roll`

The ground-roll distance and time to lift-off speed of an aircraft accelerating under constant thrust against aerodynamic drag and rolling friction on a level runway.

$$
s = \frac{1}{2B}\ln\frac{A}{A - BV_{LO}^2},\quad A = g\left(\frac{T}{W} - \mu\right),\quad B = \frac{g\rho S}{2W}(C_D - \mu C_L)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `W` | in | weight | N |
| `S` | in | wing area | m^2 |
| `h` | in | airfield altitude (standard atmosphere) | m |
| `T` | in | thrust (constant during the roll) | N |
| `mu` | in | rolling friction coefficient | - |
| `CL` | in | lift coefficient during the ground run | - |
| `CD` | in | drag coefficient during the ground run | - |
| `VLO` | in | lift-off speed | m/s |
| `s` | out | ground-roll distance | m |
| `t` | out | ground-roll time | s |

**Applies when:** thrust large enough to keep accelerating up to VLO; 0 <= mu <= 0.8; constant thrust, level runway, no wind.

```shell
$ aerospace-engineering solve aero_takeoff_ground_roll "W=10 kN" "S=10 m^2" "h=0 m" "T=3 kN" mu=0.02 CL=1 CD=0.02 "VLO=40 m/s"
s = 291.3474894 m
t = 14.56737447 s
  by: Takeoff ground roll  (dV/dt = A - B V^2, A = g (T/W - mu), B = g rho S (CD - mu CL)/(2W); s = ln(A/(A - B VLO^2))/(2B); t = artanh(VLO sqrt(B/A))/sqrt(A B))
  W = 10000 N
  S = 10 m^2
  h = 0 m
  T = 3000 N
  mu = 0.02
  CL = 1
  CD = 0.02
  VLO = 40 m/s
  assumes: thrust large enough to keep accelerating up to VLO; 0 <= mu <= 0.8; constant thrust, level runway, no wind
```

**What the worked example means:** This calculation determines the runway length required for takeoff by integrating the aircraft's acceleration from rest to its lift-off speed. It uses the constant thrust of 3 kN, weight of 10 kN, wing area of 10 m^2, and friction coefficient of 0.02 to compute a net acceleration profile against aerodynamic drag. The resulting ground-roll distance of 291.347 m and duration of 14.5674 s indicate how much runway space and time are needed under these specific atmospheric and operational conditions.

<a id="aero_time_to_climb"></a>

#### Time to climb with a linearly decreasing rate of climb, `aero_time_to_climb`

The time to climb between two altitudes when the maximum rate of climb falls linearly from its sea-level value to zero at the absolute ceiling (the standard textbook model).

$$
t = \int_{h_1}^{h_2}\frac{dh}{RC_0(1 - h/H_c)} = \frac{H_c}{RC_0}\ln\frac{H_c - h_1}{H_c - h_2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `RC0` | in | maximum rate of climb at sea level (m/s, ft/min) | m/s |
| `Hc` | in | absolute ceiling | m |
| `h1` | in | starting altitude | m |
| `h2` | in | final altitude | m |
| `t` | out | time to climb | s |
| `t_min` | out | time to climb in minutes | - |

**Applies when:** 0 <= h1 < h2 < Hc; RC(h) = RC0 (1 - h/Hc) (stated linear model).

```shell
$ aerospace-engineering solve aero_time_to_climb "RC0=10 m/s" "Hc=10 km" "h1=0 m" "h2=6321.205588 m"
t = 999.9999999 s
t_min = 16.66666667
  by: Time to climb with a linearly decreasing rate of climb  (t = integral of dh/RC = (Hc/RC0) ln((Hc - h1)/(Hc - h2)))
  RC0 = 10 m/s
  Hc = 10000 m
  h1 = 0 m
  h2 = 6321.205588 m
  assumes: 0 <= h1 < h2 < Hc; RC(h) = RC0 (1 - h/Hc) (stated linear model)
```

**What the worked example means:** This calculation determines the duration required for an aircraft to ascend between two specific altitudes using a linear rate of climb model. Given a sea-level maximum rate of climb of 10 m/s and an absolute ceiling of 10 km, the result shows that climbing from 0 m to 6321.21 m takes 1000 s. This value represents the elapsed time in seconds, which converts to 16.6667 for easier operational planning.

<a id="aero_wind_triangle"></a>

#### Wind triangle: heading and ground speed, `aero_wind_triangle`

The heading to fly, the wind correction angle and the ground speed for an aircraft of a given true airspeed to make good a true course in a wind (direction the wind blows from, and its speed).

$$
\sin\delta = \frac{V_w\sin(W - C)}{V},\quad V_g = V\cos\delta - V_w\cos(W - C)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `TAS` | in | true airspeed (kt, km/h, m/s) | m/s |
| `course` | in | true course to make good (deg) | rad |
| `wind_from` | in | direction the wind blows from (deg true) | rad |
| `wind_speed` | in | wind speed | m/s |
| `heading_deg` | out | true heading to fly, degrees | - |
| `wca_deg` | out | wind correction angle, degrees (positive = right of course) | - |
| `GS` | out | ground speed | m/s |
| `GS_kt` | out | ground speed in knots | - |

**Applies when:** crosswind component below 0.99 TAS; positive ground speed; angles in [0, 360] degrees.

```shell
$ aerospace-engineering solve aero_wind_triangle "TAS=51.44444444 m/s" "course=90 deg" "wind_from=90 deg" "wind_speed=10.28888889 m/s"
heading_deg = 90
wca_deg = 0
GS = 41.15555555 m/s
GS_kt = 79.99999999
  by: Wind triangle: heading and ground speed  (WCA = asin(Vw sin(W - C)/TAS); heading = C + WCA; GS = TAS cos(WCA) - Vw cos(W - C))
  TAS = 51.44444444 m/s
  course = 1.570796327 rad
  wind_from = 1.570796327 rad
  wind_speed = 10.28888889 m/s
  assumes: crosswind component below 0.99 TAS; positive ground speed; angles in [0, 360] degrees
```

**What the worked example means:** This calculation determines the necessary flight parameters to counteract wind drift. Given a true airspeed of 51.4444 m/s and a desired true course of 90 deg, the system computes the required true heading as 90. This adjustment involves applying a wind correction angle of 0 to account for the wind blowing from 90 deg at a speed of 10.2889 m/s. Consequently, the aircraft's effective ground speed is calculated to be 41.1556 m/s, ensuring accurate track maintenance over the ground.

<a id="aero_wing_loading"></a>

#### Wing loading: `aero_wing_loading`

The wing loading W/S of an aircraft in N/m^2, as a mass per area (kg/m^2) and in lbf/ft^2.

$$
\frac{W}{S}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `W` | in | aircraft weight (N, kN, lbf) | N |
| `S` | in | wing area (m^2, ft^2) | m^2 |
| `wing_loading` | out | wing loading W/S | N/m^2 |
| `wing_loading_kg_m2` | out | wing loading as mass per area, kg/m^2 | - |
| `wing_loading_psf` | out | wing loading in lbf/ft^2 | - |

**Applies when:** W, S > 0.

```shell
$ aerospace-engineering solve aero_wing_loading "W=600 kN" "S=120 m^2"
wing_loading = 5000 N/m^2
wing_loading_kg_m2 = 509.8581065
wing_loading_psf = 104.4271712
  by: Wing loading  (W/S; W/(g0 S); 1 lbf/ft^2 = 47.880 N/m^2)
  W = 600000 N
  S = 120 m^2
  assumes: W, S > 0
```

**What the worked example means:** This calculation determines the wing loading, which represents the aircraft weight supported by each unit of wing area. Dividing the total weight 600 kN by the projected planform area 120 m^2 gives a primary performance metric of 5000 N/m^2. This value is also expressed as an effective mass per unit area of 509.858 and converted to imperial units as 104.427. Wing loading is a fundamental parameter that influences stall speed, maneuverability, and ride comfort during flight operations.

### Airspeeds (4)

<a id="aero_calibrated_airspeed"></a>

#### Calibrated airspeed from true airspeed: `aero_calibrated_airspeed`

The calibrated airspeed, impact pressure and pitot total pressure of an aircraft flying at a true airspeed at a pressure altitude (subsonic, compressible pitot formula).

$$
q_c = p\left[\left(1 + 0.2M^2\right)^{3.5} - 1\right],\quad V_{c} = a_0\sqrt{5\left[\left(\frac{q_c}{p_0} + 1\right)^{2/7} - 1\right]}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `V` | in | true airspeed (m/s, km/h, kt) | m/s |
| `hp` | in | pressure altitude (m, ft) | m |
| `T` | in | outside air temperature (default: standard temperature) (optional) | K |
| `CAS` | out | calibrated airspeed | m/s |
| `CAS_kt` | out | calibrated airspeed in knots | - |
| `qc` | out | impact pressure pt - p | Pa |
| `pt` | out | pitot total pressure | Pa |

**Applies when:** V > 0; flight Mach number below 1; -1 km <= hp <= 47 km; 150 K <= T <= 350 K.

```shell
$ aerospace-engineering solve aero_calibrated_airspeed "V=77.16666667 m/s" "hp=0 ft"
CAS = 77.16666667 m/s
CAS_kt = 150
qc = 3694.376623 Pa
pt = 105019.3766 Pa
  by: Calibrated airspeed from true airspeed  (M = V/sqrt(gamma R T); qc = p[(1 + 0.2 M^2)^3.5 - 1]; CAS = a0 sqrt(5[(qc/p0 + 1)^(2/7) - 1]))
  V = 77.16666667 m/s
  hp = 0 m
  assumes: V > 0; flight Mach number below 1; -1 km <= hp <= 47 km; 150 K <= T <= 350 K
```

**What the worked example means:** The calibrated airspeed is 77.1667 m/s, with an impact pressure of 3694.38 Pa and pitot total pressure of 105019 Pa.

<a id="aero_mach_from_pitot"></a>

#### Mach number from pitot and static pressures, `aero_mach_from_pitot`

The flight Mach number from the static pressure and the pitot (total) pressure: the isentropic relation when subsonic, the Rayleigh pitot formula (normal shock ahead of the probe) when supersonic.

$$
\frac{p_{t2}}{p_1} = \left[\frac{(\gamma+1)^2 M^2}{4\gamma M^2 - 2(\gamma-1)}\right]^{\frac{\gamma}{\gamma-1}}\frac{1 - \gamma + 2\gamma M^2}{\gamma + 1}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `p` | in | static pressure | Pa |
| `pt` | in | pitot (total) pressure measured by the probe | Pa |
| `gamma` | in | ratio of specific heats (default 1.4, air) (optional) | - |
| `M` | out | flight Mach number | - |

**Applies when:** pt > p > 0; M <= 20; 1.05 <= gamma <= 1.7.

```shell
$ aerospace-engineering solve aero_mach_from_pitot "p=10 kPa" "pt=18.92929159 kPa"
M = 1
  by: Mach number from pitot and static pressures  (subsonic: pt/p = (1 + (g-1)/2 M^2)^(g/(g-1)); supersonic: Rayleigh pitot formula pt2/p1 = [(g+1)^2 M^2/(4 g M^2 - 2(g-1))]^(g/(g-1)) (1 - g + 2 g M^2)/(g+1), solved by bisection)
  p = 10000 Pa
  pt = 18929.29159 Pa
  assumes: pt > p > 0; M <= 20; 1.05 <= gamma <= 1.7
```

In the worked example, p = 10 kPa, pt = 18.92929159 kPa go through the equation above and give M = 1. It applies only when pt > p > 0; M <= 20; 1.05 <= gamma <= 1.7: outside that it says what is wrong instead of returning a number.

<a id="aero_total_temperature"></a>

#### Total (stagnation) air temperature in flight, `aero_total_temperature`

The total (stagnation) air temperature, the ram temperature rise and the Mach number of an aircraft flying at a true airspeed through air of a static temperature (full recovery, perfect gas, air gamma = 1.4).

$$
T_0 = T + \frac{V^2}{2c_p} = T\left(1 + \frac{\gamma-1}{2}M^2\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `V` | in | true airspeed | m/s |
| `T` | in | static (outside) air temperature | K |
| `T0` | out | total (stagnation) temperature | K |
| `dT` | out | ram temperature rise | K |
| `M` | out | flight Mach number | - |

**Applies when:** 150 K <= T <= 350 K; M < 5; full recovery (recovery factor 1).

```shell
$ aerospace-engineering solve aero_total_temperature "V=316.9677826 m/s" "T=250 K"
T0 = 300 K
dT = 50 K
M = 0.9999999999
  by: Total (stagnation) air temperature in flight  (T0 = T + V^2/(2 cp), cp = gamma R/(gamma - 1) = 1004.69 J/(kg K); M = V/sqrt(gamma R T))
  V = 316.9677826 m/s
  T = 250 K
  assumes: 150 K <= T <= 350 K; M < 5; full recovery (recovery factor 1)
```

**What the worked example means:** This calculation determines the stagnation temperature, ram temperature rise, and flight Mach number for an aircraft moving through air. Using the true airspeed of 316.968 m/s and static temperature of 250 K, it computes that the total temperature reaches 300 K Kelvin due to adiabatic compression at the stagnation point. The resulting ram rise of 50 K Kelvin represents the conversion of kinetic energy into thermal energy, while the Mach number of 1 indicates the ratio of flight speed to the local speed of sound.

<a id="aero_true_airspeed_from_calibrated"></a>

#### True airspeed from calibrated airspeed: `aero_true_airspeed_from_calibrated`

The true airspeed and flight Mach number from the calibrated airspeed, the pressure altitude and the outside air temperature (subsonic).

$$
q_c = p_0\left[\left(1 + 0.2\,\frac{V_c^2}{a_0^2}\right)^{3.5} - 1\right],\quad M = \sqrt{5\left[\left(\frac{q_c}{p} + 1\right)^{2/7} - 1\right]},\quad V = M a
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `CAS` | in | calibrated airspeed (m/s, kt, km/h) | m/s |
| `hp` | in | pressure altitude (m, ft) | m |
| `T` | in | outside air temperature (default: standard temperature) (optional) | K |
| `TAS` | out | true airspeed | m/s |
| `TAS_kt` | out | true airspeed in knots | - |
| `qc` | out | impact pressure | Pa |
| `M` | out | flight Mach number | - |

**Applies when:** 0 < CAS < a0; subsonic flight; -1 km <= hp <= 47 km; 150 K <= T <= 350 K.

```shell
$ aerospace-engineering solve aero_true_airspeed_from_calibrated "CAS=61.73333333 m/s" "hp=0 ft"
TAS = 61.73333333 m/s
TAS_kt = 120
qc = 2353.506938 Pa
M = 0.1814117022
  by: True airspeed from calibrated airspeed  (qc = p0[(1 + 0.2 (CAS/a0)^2)^3.5 - 1]; M = sqrt(5[(qc/p + 1)^(2/7) - 1]); TAS = M sqrt(gamma R T))
  CAS = 61.73333333 m/s
  hp = 0 m
  assumes: 0 < CAS < a0; subsonic flight; -1 km <= hp <= 47 km; 150 K <= T <= 350 K
```

**What the worked example means:** This calculation determines the true airspeed and flight Mach number based on the calibrated airspeed, pressure altitude, and outside air temperature. Starting with a calibrated airspeed of 61.7333 m/s, the process first computes the impact pressure as 2353.51 Pa. Using this pressure value along with the static pressure derived from the altitude of 0 ft, the flight Mach number is found to be 0.181412. Finally, multiplying this Mach number by the local speed of sound yields a true airspeed of 61.7333 m/s. This result represents the actual velocity of the aircraft relative to the surrounding air mass, accounting for compressibility and density effects that differ from standard sea level conditions.

### Compressible flow (13)

<a id="aero_fanno_flow"></a>

#### Fanno flow (adiabatic duct flow with friction), `aero_fanno_flow`

For adiabatic flow with wall friction in a constant-area duct: the friction length f L*/D needed to reach Mach 1 from Mach M, and the pressure, temperature, density and stagnation-pressure ratios to the sonic (choked) state.

$$
\frac{\bar f L^*}{D} = \frac{1 - M^2}{\gamma M^2} + \frac{\gamma+1}{2\gamma}\ln\frac{(\gamma+1)M^2}{2 + (\gamma-1)M^2},\quad \frac{p}{p^*} = \frac{1}{M}\sqrt{\frac{\gamma+1}{2 + (\gamma-1)M^2}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `M` | in | Mach number at the duct station (0.05 to 10, not 1) | - |
| `gamma` | in | ratio of specific heats (default 1.4, air) (optional) | - |
| `fLD` | out | f L*/D with the Darcy friction factor f (= 4 times the Fanning factor) | - |
| `p_pstar` | out | p/p* | - |
| `T_Tstar` | out | T/T* | - |
| `rho_rhostar` | out | rho/rho* (= V*/V) | - |
| `p0_p0star` | out | p0/p0* | - |

**Applies when:** 0.05 <= M <= 10; adiabatic, constant area, constant friction factor; 1.05 <= gamma <= 1.7.

```shell
$ aerospace-engineering solve aero_fanno_flow M=2
fLD = 0.3049965026
p_pstar = 0.4082482905
T_Tstar = 0.6666666667
rho_rhostar = 0.6123724357
p0_p0star = 1.6875
  by: Fanno flow (adiabatic duct flow with friction)  (f L*/D = (1 - M^2)/(g M^2) + (g+1)/(2g) ln[(g+1)M^2/(2 + (g-1)M^2)]; T/T* = (g+1)/(2 + (g-1)M^2); p/p* = sqrt(T/T*)/M; p0/p0* = (T*/T)^((g+1)/(2(g-1)))/M)
  M = 2
  assumes: 0.05 <= M <= 10; adiabatic, constant area, constant friction factor; 1.05 <= gamma <= 1.7
```

In the worked example, M = 2 go through the equation above and give fLD = 0.3049965026, p_pstar = 0.4082482905, T_Tstar = 0.6666666667, rho_rhostar = 0.6123724357. It applies only when 0.05 <= M <= 10; adiabatic, constant area, constant friction factor; 1.05 <= gamma <= 1.7: outside that it says what is wrong instead of returning a number.

<a id="aero_isentropic_flow"></a>

#### Isentropic flow ratios at a Mach number: `aero_isentropic_flow`

Stagnation-to-static temperature, pressure and density ratios and the area ratio A/A* of isentropic flow of a perfect gas at a Mach number.

$$
\frac{T_0}{T} = 1 + \frac{\gamma-1}{2}M^2,\quad \frac{p_0}{p} = \left(\frac{T_0}{T}\right)^{\frac{\gamma}{\gamma-1}},\quad \frac{A}{A^*} = \frac{1}{M}\left[\frac{2}{\gamma+1}\frac{T_0}{T}\right]^{\frac{\gamma+1}{2(\gamma-1)}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `M` | in | Mach number (0.01 to 10) | - |
| `gamma` | in | ratio of specific heats (default 1.4, air) (optional) | - |
| `T0_T` | out | stagnation-to-static temperature ratio T0/T | - |
| `p0_p` | out | stagnation-to-static pressure ratio p0/p | - |
| `rho0_rho` | out | stagnation-to-static density ratio rho0/rho | - |
| `A_Astar` | out | area ratio A/A* (A* = sonic throat area) | - |

**Applies when:** 0.01 <= M <= 10; 1.05 <= gamma <= 1.7; isentropic flow of a calorically perfect gas.

```shell
$ aerospace-engineering solve aero_isentropic_flow M=2
T0_T = 1.8
p0_p = 7.824449067
rho0_rho = 4.346916148
A_Astar = 1.6875
  by: Isentropic flow ratios at a Mach number  (T0/T = 1 + (g-1)/2 M^2; p0/p = (T0/T)^(g/(g-1)); rho0/rho = (T0/T)^(1/(g-1)); A/A* = (1/M)[(2/(g+1))(T0/T)]^((g+1)/(2(g-1))))
  M = 2
  assumes: 0.01 <= M <= 10; 1.05 <= gamma <= 1.7; isentropic flow of a calorically perfect gas
```

**What the worked example means:** The stagnation-to-static temperature ratio is 1.8, the pressure ratio is 7.82445, and the area ratio A/A* is 1.6875.

<a id="aero_mach_from_area_ratio"></a>

#### Mach number from the area ratio A/A*, `aero_mach_from_area_ratio`

The subsonic or supersonic Mach number at which isentropic flow has a given area ratio A/A*, with the static-to-stagnation pressure and temperature ratios there.

$$
\frac{A}{A^*} = \frac{1}{M}\left[\frac{2}{\gamma+1}\left(1 + \frac{\gamma-1}{2}M^2\right)\right]^{\frac{\gamma+1}{2(\gamma-1)}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `ar` | in | area ratio A/A* (>= 1) | - |
| `regime` | in | which branch: subsonic or supersonic | - |
| `gamma` | in | ratio of specific heats (default 1.4, air) (optional) | - |
| `M` | out | Mach number | - |
| `p_p0` | out | static-to-stagnation pressure ratio p/p0 | - |
| `T_T0` | out | static-to-stagnation temperature ratio T/T0 | - |

**Applies when:** 1 <= A/A* <= 10000; 1.05 <= gamma <= 1.7.

```shell
$ aerospace-engineering solve aero_mach_from_area_ratio ar=1.6875 regime=supersonic
M = 2
p_p0 = 0.1278045255
T_T0 = 0.5555555556
  by: Mach number from the area ratio A/A*  (solve A/A* = (1/M)[(2/(g+1))(1 + (g-1)/2 M^2)]^((g+1)/(2(g-1))) on the chosen branch by bisection)
  ar = 1.6875
  regime = supersonic
  assumes: 1 <= A/A* <= 10000; 1.05 <= gamma <= 1.7
```

**What the worked example means:** This calculation determines the Mach number corresponding to an area ratio of 1.6875 for the selected supersonic branch. The result shows that at this specific geometry, the flow velocity corresponds to a Mach number of 2. Accompanying this velocity are the thermodynamic states derived from isentropic relations, yielding a static-to-stagnation pressure ratio of 0.127805 and a static-to-stagnation temperature ratio of 0.555556. These values indicate how much the local static properties differ from the total conditions due to the conversion between pressure energy and kinetic energy.

<a id="aero_mach_from_prandtl_meyer"></a>

#### Mach number from the Prandtl-Meyer angle: `aero_mach_from_prandtl_meyer`

The supersonic Mach number whose Prandtl-Meyer angle is nu, with the maximum turning angle of the gas (expansion to vacuum).

$$
\nu(M) = \nu,\quad \nu_{max} = \frac{\pi}{2}\left(\sqrt{\frac{\gamma+1}{\gamma-1}} - 1\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `nu` | in | Prandtl-Meyer angle (deg, rad) | rad |
| `gamma` | in | ratio of specific heats (default 1.4, air) (optional) | - |
| `M` | out | Mach number | - |
| `nu_max_deg` | out | maximum Prandtl-Meyer angle (M -> infinity) in degrees | - |

**Applies when:** 0 <= nu < nu(M = 50); 1.05 <= gamma <= 1.7.

```shell
$ aerospace-engineering solve aero_mach_from_prandtl_meyer "nu=26.37976081 deg"
M = 2
nu_max_deg = 130.4540769
  by: Mach number from the Prandtl-Meyer angle  (invert nu(M) by bisection; nu_max = (sqrt((g+1)/(g-1)) - 1) pi/2)
  nu = 0.460413682 rad
  assumes: 0 <= nu < nu(M = 50); 1.05 <= gamma <= 1.7
```

**What the worked example means:** This calculation determines the supersonic Mach number that corresponds to a specific Prandtl-Meyer expansion angle. Inverting the relationship between flow turning and Mach number shows that an input angle of 26.3798 deg yields a Mach number of 2. The result represents the speed of the gas relative to the local speed of sound after undergoing this isentropic expansion. Additionally, the theoretical maximum turning angle for this gas, occurring as the Mach number approaches infinity, is calculated as 130.454.

<a id="aero_mach_from_pressure_ratio"></a>

#### Mach number from the stagnation-to-static pressure ratio, `aero_mach_from_pressure_ratio`

The Mach number of isentropic flow from the ratio of stagnation to static pressure p0/p, with T0/T and A/A*.

$$
M = \sqrt{\frac{2}{\gamma-1}\left[\left(\frac{p_0}{p}\right)^{\frac{\gamma-1}{\gamma}} - 1\right]}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `pr` | in | stagnation-to-static pressure ratio p0/p (> 1) | - |
| `gamma` | in | ratio of specific heats (default 1.4, air) (optional) | - |
| `M` | out | Mach number | - |
| `T0_T` | out | stagnation-to-static temperature ratio | - |
| `A_Astar` | out | area ratio A/A* | - |

**Applies when:** 1 < p0/p <= 1e6; M <= 20; 1.05 <= gamma <= 1.7.

```shell
$ aerospace-engineering solve aero_mach_from_pressure_ratio pr=7.824449067
M = 2
T0_T = 1.8
A_Astar = 1.6875
  by: Mach number from the stagnation-to-static pressure ratio  (M = sqrt(2/(g-1) [(p0/p)^((g-1)/g) - 1]))
  pr = 7.824449067
  assumes: 1 < p0/p <= 1e6; M <= 20; 1.05 <= gamma <= 1.7
```

**What the worked example means:** The calculated Mach number is 2, corresponding to a stagnation-to-static pressure ratio of 7.82445 and an area ratio of 1.6875.

<a id="aero_max_deflection_angle"></a>

#### Maximum deflection angle of an attached oblique shock, `aero_max_deflection_angle`

The largest wedge (deflection) angle for which an oblique shock stays attached at a given upstream Mach number, with the wave angle, downstream Mach number and pressure ratio at that limit; beyond it the shock detaches.

$$
\sin^2\beta^* = \frac{\frac{\gamma+1}{4}M_1^2 - 1 + \sqrt{(\gamma+1)\left(1 + \frac{\gamma-1}{2}M_1^2 + \frac{\gamma+1}{16}M_1^4\right)}}{\gamma M_1^2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `M1` | in | upstream Mach number (>= 1.01) | - |
| `gamma` | in | ratio of specific heats (default 1.4, air) (optional) | - |
| `theta_max_deg` | out | maximum deflection angle in degrees | - |
| `beta_deg` | out | wave angle at the maximum, degrees | - |
| `M2` | out | downstream Mach number at the maximum | - |
| `p2_p1` | out | pressure ratio at the maximum | - |

**Applies when:** 1.01 <= M1 <= 20; 1.05 <= gamma <= 1.7.

```shell
$ aerospace-engineering solve aero_max_deflection_angle M1=2
theta_max_deg = 22.97353176
beta_deg = 64.66897983
M2 = 0.9242557487
p2_p1 = 3.645751311
  by: Maximum deflection angle of an attached oblique shock  (sin^2 beta* = [(g+1)M^2/4 - 1 + sqrt((g+1)(1 + (g-1)M^2/2 + (g+1)M^4/16))]/(g M^2); theta_max = theta(beta*))
  M1 = 2
  assumes: 1.01 <= M1 <= 20; 1.05 <= gamma <= 1.7
```

**What the worked example means:** This calculation determines the largest wedge angle that can be turned by an attached oblique shock for an upstream Mach number of 2. The result shows a maximum deflection angle of 22.9735 degrees, occurring at a shock wave angle of 64.669 degrees. At this limit, the downstream flow has a Mach number of 0.924256 and a pressure ratio across the shock of 3.64575. If the actual deflection angle exceeds 22.9735, the shock detaches from the body.

<a id="aero_normal_shock_from_pressure_ratio"></a>

#### Normal shock from its pressure ratio: `aero_normal_shock_from_pressure_ratio`

The upstream and downstream Mach numbers and the temperature, density and stagnation-pressure ratios of a normal shock of known static pressure ratio p2/p1 (shock strength).

$$
M_1 = \sqrt{\frac{\gamma+1}{2\gamma}\left(\frac{p_2}{p_1} - 1\right) + 1}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `p21` | in | static pressure ratio across the shock p2/p1 (> 1) | - |
| `gamma` | in | ratio of specific heats (default 1.4, air) (optional) | - |
| `M1` | out | upstream Mach number | - |
| `M2` | out | downstream Mach number | - |
| `T2_T1` | out | temperature ratio | - |
| `rho2_rho1` | out | density ratio | - |
| `p02_p01` | out | stagnation pressure ratio | - |

**Applies when:** 1 < p2/p1 <= 500; 1.05 <= gamma <= 1.7.

```shell
$ aerospace-engineering solve aero_normal_shock_from_pressure_ratio p21=4.5
M1 = 2
M2 = 0.5773502692
T2_T1 = 1.6875
rho2_rho1 = 2.666666667
p02_p01 = 0.7208738615
  by: Normal shock from its pressure ratio  (M1 = sqrt((g+1)/(2g) (p2/p1 - 1) + 1), then the normal-shock relations)
  p21 = 4.5
  assumes: 1 < p2/p1 <= 500; 1.05 <= gamma <= 1.7
```

**What the worked example means:** This calculation determines the flow properties across a normal shock wave given a static pressure ratio of 4.5. The upstream Mach number is found to be 2, while the downstream Mach number is 0.57735. Additionally, the temperature ratio is 1.6875, the density ratio is 2.66667, and the stagnation pressure ratio is 0.720874. These values describe how the gas accelerates or decelerates and changes thermodynamic state as it passes through the shock.

<a id="aero_normal_shock"></a>

#### Normal shock relations: `aero_normal_shock`

Downstream Mach number and the pressure, temperature, density and stagnation-pressure ratios across a normal shock, with the pitot ratio p02/p1.

$$
\frac{p_2}{p_1} = 1 + \frac{2\gamma}{\gamma+1}(M_1^2 - 1),\quad \frac{\rho_2}{\rho_1} = \frac{(\gamma+1)M_1^2}{(\gamma-1)M_1^2 + 2},\quad M_2^2 = \frac{1 + \frac{\gamma-1}{2}M_1^2}{\gamma M_1^2 - \frac{\gamma-1}{2}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `M1` | in | upstream Mach number (> 1) | - |
| `gamma` | in | ratio of specific heats (default 1.4, air) (optional) | - |
| `M2` | out | downstream Mach number | - |
| `p2_p1` | out | static pressure ratio p2/p1 | - |
| `T2_T1` | out | temperature ratio T2/T1 | - |
| `rho2_rho1` | out | density ratio rho2/rho1 | - |
| `p02_p01` | out | stagnation pressure ratio p02/p01 | - |
| `p02_p1` | out | pitot ratio p02/p1 | - |

**Applies when:** 1 < M1 <= 20; 1.05 <= gamma <= 1.7.

```shell
$ aerospace-engineering solve aero_normal_shock M1=2
M2 = 0.5773502692
p2_p1 = 4.5
T2_T1 = 1.6875
rho2_rho1 = 2.666666667
p02_p01 = 0.7208738615
p02_p1 = 5.640440813
  by: Normal shock relations  (p2/p1 = 1 + 2g/(g+1)(M1^2 - 1); rho2/rho1 = (g+1)M1^2/((g-1)M1^2 + 2); M2^2 = (1 + (g-1)/2 M1^2)/(g M1^2 - (g-1)/2))
  M1 = 2
  assumes: 1 < M1 <= 20; 1.05 <= gamma <= 1.7
```

**What the worked example means:** Across the normal shock, the upstream Mach number 2 yields a downstream Mach number 0.57735, a static pressure ratio p2 over p1 of 4.5, and a stagnation pressure ratio p02 over p01 of 0.720874.

<a id="aero_oblique_shock"></a>

#### Oblique shock from the deflection angle: `aero_oblique_shock`

The wave angle, downstream Mach number and the pressure, temperature, density and stagnation-pressure ratios of an attached oblique shock turning a supersonic flow by a deflection angle (weak or strong solution).

$$
\tan\theta = 2\cot\beta\,\frac{M_1^2\sin^2\beta - 1}{M_1^2(\gamma + \cos 2\beta) + 2},\quad M_{n1} = M_1\sin\beta,\quad M_2 = \frac{M_{n2}}{\sin(\beta - \theta)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `M1` | in | upstream Mach number (> 1) | - |
| `theta` | in | flow deflection (wedge half-) angle (deg, rad) | rad |
| `branch` | in | weak (default) or strong shock solution (optional) | - |
| `gamma` | in | ratio of specific heats (default 1.4, air) (optional) | - |
| `beta_deg` | out | shock wave angle in degrees | - |
| `M2` | out | downstream Mach number | - |
| `p2_p1` | out | pressure ratio p2/p1 | - |
| `T2_T1` | out | temperature ratio T2/T1 | - |
| `rho2_rho1` | out | density ratio rho2/rho1 | - |
| `p02_p01` | out | stagnation pressure ratio | - |
| `Mn1` | out | upstream Mach number normal to the shock | - |

**Applies when:** 1 < M1 <= 20; 0 < theta < theta_max(M1) (attached shock); 1.05 <= gamma <= 1.7.

```shell
$ aerospace-engineering solve aero_oblique_shock M1=2 "theta=10.62290962 deg"
beta_deg = 39.99999999
M2 = 1.617318834
p2_p1 = 1.761487585
T2_T1 = 1.181766104
rho2_rho1 = 1.490555178
p02_p01 = 0.981791426
Mn1 = 1.285575219
  by: Oblique shock from the deflection angle  (theta-beta-M relation tan(theta) = 2 cot(beta) (M1^2 sin^2 beta - 1)/(M1^2 (g + cos 2 beta) + 2), solved by bisection on the chosen branch; normal-shock relations on Mn1 = M1 sin beta; M2 = Mn2 / sin(beta - theta))
  M1 = 2
  theta = 0.185404749 rad
  assumes: 1 < M1 <= 20; 0 < theta < theta_max(M1) (attached shock); 1.05 <= gamma <= 1.7
```

**What the worked example means:** This calculation determines the properties of an attached oblique shock formed when supersonic air flows over a wedge. Using the theta-beta-Mach relation, it finds the shock wave angle 40 for an upstream Mach number of 2 and deflection angle of 10.6229 deg. The normal component of the upstream velocity is 1.28558, which drives the thermodynamic changes across the discontinuity. Downstream, the flow remains supersonic with a Mach number of 1.61732, while static pressure rises to 1.76149 times the initial value and temperature increases to 1.18177 times the initial value.

<a id="aero_oblique_shock_from_wave_angle"></a>

#### Oblique shock from the wave angle: `aero_oblique_shock_from_wave_angle`

The flow deflection, downstream Mach number and the property ratios of an oblique shock of known wave angle (for example measured on a schlieren photograph).

$$
\tan\theta = 2\cot\beta\,\frac{M_1^2\sin^2\beta - 1}{M_1^2(\gamma + \cos 2\beta) + 2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `M1` | in | upstream Mach number (> 1) | - |
| `beta` | in | shock wave angle (deg, rad) | rad |
| `gamma` | in | ratio of specific heats (default 1.4, air) (optional) | - |
| `theta_deg` | out | flow deflection angle in degrees | - |
| `M2` | out | downstream Mach number | - |
| `p2_p1` | out | pressure ratio | - |
| `T2_T1` | out | temperature ratio | - |
| `rho2_rho1` | out | density ratio | - |
| `p02_p01` | out | stagnation pressure ratio | - |
| `Mn1` | out | upstream normal Mach number | - |

**Applies when:** 1 < M1 <= 20; asin(1/M1) < beta <= 90 deg; 1.05 <= gamma <= 1.7.

```shell
$ aerospace-engineering solve aero_oblique_shock_from_wave_angle M1=2 "beta=90 deg"
theta_deg = 0
M2 = 0.5773502692
p2_p1 = 4.5
T2_T1 = 1.6875
rho2_rho1 = 2.666666667
p02_p01 = 0.7208738615
Mn1 = 2
  by: Oblique shock from the wave angle  (theta from the theta-beta-M relation; normal-shock relations on Mn1 = M1 sin beta; M2 = Mn2/sin(beta - theta))
  M1 = 2
  beta = 1.570796327 rad
  assumes: 1 < M1 <= 20; asin(1/M1) < beta <= 90 deg; 1.05 <= gamma <= 1.7
```

**What the worked example means:** This calculation determines the post-shock flow conditions for an oblique shock defined by a known wave angle. Given the upstream Mach number of 2 and a shock wave angle of 90 deg, the method computes the normal component of the upstream velocity as 2. Using standard normal shock relations, it derives the static property ratios, including a pressure ratio of 4.5 and a temperature ratio of 1.6875. The final output provides the flow deflection angle of 0 and the downstream Mach number of 0.57735, which indicates whether the flow remains supersonic or becomes subsonic after passing through the shock.

<a id="aero_prandtl_meyer"></a>

#### Prandtl-Meyer angle of a Mach number: `aero_prandtl_meyer`

The Prandtl-Meyer function nu(M): the angle through which a sonic flow must turn isentropically to reach Mach M.

$$
\nu(M) = \sqrt{\frac{\gamma+1}{\gamma-1}}\arctan\sqrt{\frac{\gamma-1}{\gamma+1}(M^2-1)} - \arctan\sqrt{M^2-1}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `M` | in | Mach number (>= 1) | - |
| `gamma` | in | ratio of specific heats (default 1.4, air) (optional) | - |
| `nu_deg` | out | Prandtl-Meyer angle in degrees | - |

**Applies when:** 1 <= M <= 50; 1.05 <= gamma <= 1.7.

```shell
$ aerospace-engineering solve aero_prandtl_meyer M=2
nu_deg = 26.37976081
  by: Prandtl-Meyer angle of a Mach number  (nu = sqrt((g+1)/(g-1)) atan(sqrt((g-1)/(g+1)(M^2 - 1))) - atan(sqrt(M^2 - 1)))
  M = 2
  assumes: 1 <= M <= 50; 1.05 <= gamma <= 1.7
```

In the worked example, M = 2 go through the equation above and give nu_deg = 26.37976081. It applies only when 1 <= M <= 50; 1.05 <= gamma <= 1.7: outside that it says what is wrong instead of returning a number.

<a id="aero_expansion_fan"></a>

#### Prandtl-Meyer expansion fan: `aero_expansion_fan`

The downstream Mach number and the pressure, temperature and density ratios when a supersonic flow turns away from itself by an angle through a centred expansion fan.

$$
\nu(M_2) = \nu(M_1) + \theta,\quad \frac{p_2}{p_1} = \left[\frac{1 + \frac{\gamma-1}{2}M_1^2}{1 + \frac{\gamma-1}{2}M_2^2}\right]^{\frac{\gamma}{\gamma-1}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `M1` | in | upstream Mach number (>= 1) | - |
| `theta` | in | turning angle of the flow (deg, rad) | rad |
| `gamma` | in | ratio of specific heats (default 1.4, air) (optional) | - |
| `M2` | out | downstream Mach number | - |
| `nu1_deg` | out | upstream Prandtl-Meyer angle (deg) | - |
| `nu2_deg` | out | downstream Prandtl-Meyer angle (deg) | - |
| `p2_p1` | out | pressure ratio p2/p1 | - |
| `T2_T1` | out | temperature ratio T2/T1 | - |
| `rho2_rho1` | out | density ratio rho2/rho1 | - |

**Applies when:** 1 <= M1 <= 20; 0 < theta < 90 deg; downstream Mach below 50; 1.05 <= gamma <= 1.7.

```shell
$ aerospace-engineering solve aero_expansion_fan M1=1 "theta=26.37976081 deg"
M2 = 2
nu1_deg = 0
nu2_deg = 26.37976081
p2_p1 = 0.2419249129
T2_T1 = 0.6666666667
rho2_rho1 = 0.3628873694
  by: Prandtl-Meyer expansion fan  (nu2 = nu(M1) + theta; M2 from nu(M2) = nu2; p2/p1 = [(1 + (g-1)/2 M1^2)/(1 + (g-1)/2 M2^2)]^(g/(g-1)) (isentropic))
  M1 = 1
  theta = 0.460413682 rad
  assumes: 1 <= M1 <= 20; 0 < theta < 90 deg; downstream Mach below 50; 1.05 <= gamma <= 1.7
```

**What the worked example means:** For an upstream Mach number of 1 turning by 26.3798 deg, the downstream Mach number is 2.

<a id="aero_rayleigh_flow"></a>

#### Rayleigh flow (duct flow with heat addition), `aero_rayleigh_flow`

For frictionless constant-area flow with heat addition: the stagnation temperature, temperature, pressure, density and stagnation-pressure ratios to the sonic (thermally choked) state at a Mach number.

$$
\frac{p}{p^*} = \frac{1+\gamma}{1 + \gamma M^2},\quad \frac{T_0}{T_0^*} = \frac{(\gamma+1)M^2\left[2 + (\gamma-1)M^2\right]}{(1 + \gamma M^2)^2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `M` | in | Mach number (0.05 to 10) | - |
| `gamma` | in | ratio of specific heats (default 1.4, air) (optional) | - |
| `T0_T0star` | out | T0/T0* | - |
| `T_Tstar` | out | T/T* | - |
| `p_pstar` | out | p/p* | - |
| `rho_rhostar` | out | rho/rho* | - |
| `p0_p0star` | out | p0/p0* | - |

**Applies when:** 0.05 <= M <= 10; frictionless, constant area; 1.05 <= gamma <= 1.7.

```shell
$ aerospace-engineering solve aero_rayleigh_flow M=2
T0_T0star = 0.7933884298
T_Tstar = 0.5289256198
p_pstar = 0.3636363636
rho_rhostar = 0.6875
p0_p0star = 1.503095979
  by: Rayleigh flow (duct flow with heat addition)  (p/p* = (1+g)/(1 + g M^2); T/T* = M^2 (p/p*)^2; T0/T0* = (g+1)M^2(2 + (g-1)M^2)/(1 + g M^2)^2; p0/p0* = (p/p*)[(2 + (g-1)M^2)/(g+1)]^(g/(g-1)))
  M = 2
  assumes: 0.05 <= M <= 10; frictionless, constant area; 1.05 <= gamma <= 1.7
```

**What the worked example means:** The stagnation temperature ratio is 0.793388 and the pressure ratio is 0.363636.

### Nozzles (2)

<a id="aero_choked_mass_flow"></a>

#### Choked mass flow through a nozzle throat: `aero_choked_mass_flow`

The maximum (choked) mass flow of a perfect gas through a sonic throat from the stagnation pressure and temperature, with the throat (critical) pressure, temperature and speed.

$$
\dot m = \frac{p_0 A^*}{\sqrt{R T_0}}\sqrt{\gamma}\left(\frac{2}{\gamma+1}\right)^{\frac{\gamma+1}{2(\gamma-1)}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `p0` | in | stagnation (reservoir) pressure | Pa |
| `T0` | in | stagnation temperature | K |
| `A` | in | throat area (m^2, cm^2, mm^2) | m^2 |
| `gamma` | in | ratio of specific heats (default 1.4, air) (optional) | - |
| `R` | in | specific gas constant (default air, 287.053 J/(kg*K)) (optional) | J/(kg*K) |
| `mdot` | out | choked mass flow rate | kg/s |
| `p_star` | out | throat pressure | Pa |
| `T_star` | out | throat temperature | K |
| `V_star` | out | throat (sonic) speed | m/s |

**Applies when:** p0, T0, A, R > 0; isentropic flow to a sonic throat; 1.05 <= gamma <= 1.7.

```shell
$ aerospace-engineering solve aero_choked_mass_flow "p0=100 kPa" "T0=300 K" "A=10 cm^2"
mdot = 0.233334283 kg/s
p_star = 52828.17877 Pa
T_star = 250 K
V_star = 316.9677826 m/s
  by: Choked mass flow through a nozzle throat  (mdot = Gamma p0 A/sqrt(R T0), Gamma = sqrt(g)(2/(g+1))^((g+1)/(2(g-1))); p*/p0 = (2/(g+1))^(g/(g-1)); T*/T0 = 2/(g+1))
  p0 = 100000 Pa
  T0 = 300 K
  A = 0.001 m^2
  assumes: p0, T0, A, R > 0; isentropic flow to a sonic throat; 1.05 <= gamma <= 1.7
```

**What the worked example means:** This calculation finds the maximum mass flow rate of a perfect gas through a nozzle throat when the flow becomes sonic. Using the stagnation pressure, temperature, and throat area, it computes the critical mass flow rate, which is 0.233334 kg/s. It also determines the static pressure at the throat, 52828.2 Pa, and the static temperature, 250 K, based on isentropic relations. These values represent the limiting case where further reduction in downstream pressure does not increase the mass flow.

<a id="aero_nozzle_normal_shock"></a>

#### Normal shock position in a converging-diverging nozzle, `aero_nozzle_normal_shock`

Where a normal shock stands in the diverging part of a converging-diverging nozzle for a given back pressure: the area ratio at the shock, the Mach numbers before and after it, the exit Mach number and the stagnation-pressure loss.

$$
\frac{p_e A_e}{p_{01} A_t} = \frac{p_e}{p_{02}}\frac{A_e}{A_2^*}\Big|_{M_e},\quad \frac{p_{02}}{p_{01}} = \frac{A_t}{A_2^*}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `pc` | in | reservoir (stagnation) pressure | Pa |
| `pb` | in | back pressure at the exit | Pa |
| `eps` | in | exit-to-throat area ratio Ae/At | - |
| `gamma` | in | ratio of specific heats (default 1.4, air) (optional) | - |
| `As_At` | out | area ratio at the shock As/At | - |
| `M1` | out | Mach number just before the shock | - |
| `M2` | out | Mach number just after the shock | - |
| `Me` | out | exit Mach number (subsonic) | - |
| `p02_p01` | out | stagnation pressure ratio across the shock | - |

**Applies when:** 1.05 <= eps <= 100; back pressure between the exit-shock and the critical subsonic exit pressures; 1.05 <= gamma <= 1.7.

```shell
$ aerospace-engineering solve aero_nozzle_normal_shock "pc=1 MPa" "pb=682.5585882 kPa" eps=3
As_At = 1.6875
M1 = 2
M2 = 0.5773502692
Me = 0.2804184562
p02_p01 = 0.7208738614
  by: Normal shock position in a converging-diverging nozzle  (mass flow: pe Ae/(p01 At) = (p/p0)(A/A*) at Me (subsonic) gives Me; p02/p01 = pe/(p01 (p/p0)(Me)); M1 from the normal-shock p02/p01; As/At = A/A*(M1))
  pc = 1000000 Pa
  pb = 682558.5882 Pa
  eps = 3
  assumes: 1.05 <= eps <= 100; back pressure between the exit-shock and the critical subsonic exit pressures; 1.05 <= gamma <= 1.7
```

**What the worked example means:** This calculation determines the position of a normal shock wave within a converging-diverging nozzle for a given back pressure. Using the reservoir pressure 1 MPa and back pressure 682.559 kPa, it finds that the shock stands where the cross-sectional area is 1.6875 times the throat area. The flow accelerates to supersonic speed before the shock, reaching Mach 2, then decelerates abruptly to subsonic speed Mach 0.57735 immediately after. This results in a stagnation pressure loss across the shock, indicated by the ratio 0.720874, and establishes the final exit conditions.

### Orbits (9)

<a id="aero_bielliptic_transfer"></a>

#### Bi-elliptic transfer between circular orbits, `aero_bielliptic_transfer`

The three burns, total delta-v and transfer time of a bi-elliptic transfer from a lower to a higher circular orbit around the Earth through an intermediate apoapsis, with the Hohmann delta-v for comparison.

$$
v = \sqrt{\mu\left(\frac{2}{r} - \frac{1}{a}\right)},\quad t = \pi\left(\sqrt{\frac{a_1^3}{\mu}} + \sqrt{\frac{a_2^3}{\mu}}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `h1` | in | altitude of the initial circular orbit | m |
| `h2` | in | altitude of the final circular orbit | m |
| `hb` | in | altitude of the intermediate apoapsis (>= h2) | m |
| `dv1` | out | first burn | m/s |
| `dv2` | out | second burn (at the intermediate apoapsis) | m/s |
| `dv3` | out | third burn (circularization) | m/s |
| `dv` | out | total delta-v | m/s |
| `t` | out | total transfer time | s |
| `t_days` | out | transfer time in days | - |
| `dv_hohmann` | out | total delta-v of the Hohmann transfer between the same orbits | m/s |

**Applies when:** 100 km <= h1 < h2 <= hb; coplanar circular orbits, impulsive burns, two-body Earth.

```shell
$ aerospace-engineering solve aero_bielliptic_transfer h1=6378136.6 h2=19134409.799999997 hb=19134409.799999997
dv1 = 864.7664 m/s
dv2 = 725.330842 m/s
dv3 = 0 m/s
dv = 1590.097242 m/s
t = 33447.91358 s
t_days = 0.3871286294
dv_hohmann = 1590.097242 m/s
  by: Bi-elliptic transfer between circular orbits  (two transfer ellipses (r1, rb) and (r2, rb): burns from vis-viva at r1, rb and r2; t = pi (sqrt(a1^3/mu) + sqrt(a2^3/mu)))
  h1 = 6378136.6 m
  h2 = 19134409.8 m
  hb = 19134409.8 m
  assumes: 100 km <= h1 < h2 <= hb; coplanar circular orbits, impulsive burns, two-body Earth
```

**What the worked example means:** This calculation determines the velocity changes needed for a three-burn maneuver moving from an initial circular orbit at altitude 6.37814e+06 m to a final one at 1.91344e+07 m, passing through an intermediate apoapsis at 1.91344e+07 m. The first burn accelerates the spacecraft to 864.766 m/s, the second adjusts the trajectory at the high point by 725.331 m/s, and the third circularizes the path with 0 m/s. The sum of these impulses is 1590.1 m/s, while the Hohmann comparison value is 1590.1 m/s. The total duration of this journey is 33447.9 s seconds, or 0.387129 days. This model helps assess whether the extra time spent in transit justifies any potential fuel savings over the simpler two-burn approach.

<a id="aero_circular_orbit"></a>

#### Circular orbit at an altitude above the Earth, `aero_circular_orbit`

Orbital speed, period and revolutions per day of a circular orbit at an altitude above the Earth's equatorial radius.

$$
v = \sqrt{\frac{\mu}{R + h}},\quad T = 2\pi\sqrt{\frac{(R + h)^3}{\mu}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `h` | in | orbit altitude above the Earth (km) | m |
| `v` | out | circular orbital speed | m/s |
| `T` | out | orbital period | s |
| `T_min` | out | period in minutes | - |
| `revs_per_day` | out | revolutions per day (86400 s) | - |

**Applies when:** 100 km <= h <= 1e6 km; two-body Earth, mu = 3.986004418e14 m^3/s^2, R = 6378.1366 km.

```shell
$ aerospace-engineering solve aero_circular_orbit "h=19134.4098 km"
v = 3952.682983 m/s
T = 40554.74658 s
T_min = 675.9124429
revs_per_day = 2.130453456
  by: Circular orbit at an altitude above the Earth  (r = R + h; v = sqrt(mu/r); T = 2 pi sqrt(r^3/mu))
  h = 19134409.8 m
  assumes: 100 km <= h <= 1e6 km; two-body Earth, mu = 3.986004418e14 m^3/s^2, R = 6378.1366 km
```

**What the worked example means:** This calculation determines the velocity and time required for a satellite to maintain a stable circular path at a specific height above Earth. Given an input altitude of 19134.4 km, the model computes a circular orbital speed of 3952.68 m/s and an orbital period of 40554.7 s. The period expressed in minutes is 675.912, which corresponds to 2.13045 complete revolutions around the planet each day. These values are essential for understanding the basic dynamics of low to medium Earth orbit missions.

<a id="aero_altitude_for_period"></a>

#### Circular-orbit altitude for an orbital period, `aero_altitude_for_period`

The altitude above the Earth of the circular orbit with a given period (for example a geostationary orbit, one sidereal day), and its orbital speed.

$$
R + h = \left(\mu\,\frac{T^2}{4\pi^2}\right)^{1/3}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `T` | in | orbital period (s, min, h, day) | s |
| `h` | out | orbit altitude | m |
| `h_km` | out | altitude in km | - |
| `v` | out | orbital speed | m/s |

**Applies when:** the altitude between 100 km and 1e6 km; two-body Earth.

```shell
$ aerospace-engineering solve aero_altitude_for_period "T=40554.74658 s"
h = 19134409.8 m
h_km = 19134.4098
v = 3952.682983 m/s
  by: Circular-orbit altitude for an orbital period  (r = (mu (T/(2 pi))^2)^(1/3); h = r - R)
  T = 40554.74658 s
  assumes: the altitude between 100 km and 1e6 km; two-body Earth
```

**What the worked example means:** This calculation determines the altitude of a circular orbit required to achieve a specific period. By using the given period 40554.7 s, the algorithm computes the orbital radius and subtracts the Earth's mean radius to find the height above the surface. The resulting altitude is 19134.4, which corresponds to an orbital velocity of 3952.68 m/s. This relationship is fundamental for mission planning, such as placing satellites in geostationary or other synchronized orbits.

<a id="aero_satellite_coverage"></a>

#### Coverage of a satellite at an altitude: `aero_satellite_coverage`

The Earth central angle out to the edge of a satellite's coverage, the off-nadir angle and slant range to that edge, and the fraction of the Earth's surface in view, for a minimum elevation angle at the ground (0 = the geometric horizon).

$$
\sin\eta = \frac{R\cos\varepsilon}{R + h},\quad \lambda = 90^\circ - \varepsilon - \eta,\quad d = \frac{R\sin\lambda}{\sin\eta},\quad f = \frac{1 - \cos\lambda}{2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `h` | in | satellite altitude | m |
| `eps_min` | in | minimum elevation angle at the ground station (default 0 deg) (optional) | rad |
| `lambda_deg` | out | Earth central angle to the coverage edge, degrees | - |
| `nadir_deg` | out | off-nadir angle to the edge, degrees | - |
| `slant_range` | out | slant range to the coverage edge | m |
| `coverage` | out | fraction of the Earth's surface covered | - |

**Applies when:** 100 km <= h <= 1e6 km; 0 <= eps_min <= 80 deg; spherical Earth of radius 6378.1366 km.

```shell
$ aerospace-engineering solve aero_satellite_coverage "h=6378.1366 km"
lambda_deg = 60
nadir_deg = 30
slant_range = 11047256.65 m
coverage = 0.25
  by: Coverage of a satellite at an altitude  (sin(eta) = R cos(eps)/(R + h); lambda = 90 deg - eps - eta; d = R sin(lambda)/sin(eta); coverage = (1 - cos(lambda))/2)
  h = 6378136.6 m
  assumes: 100 km <= h <= 1e6 km; 0 <= eps_min <= 80 deg; spherical Earth of radius 6378.1366 km
```

**What the worked example means:** The satellite at altitude 6378.14 km sees a central angle of 60, covering 0.25 of the Earth.

<a id="aero_escape_speed"></a>

#### Escape speed at an altitude above the Earth, `aero_escape_speed`

The speed needed to escape the Earth's gravity from an altitude, the circular orbital speed there and the burn needed to escape from that circular orbit.

$$
v_{esc} = \sqrt{\frac{2\mu}{R + h}} = \sqrt{2}\,v_{circ}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `h` | in | altitude above the Earth (km); 0 = the surface | m |
| `v_esc` | out | escape speed | m/s |
| `v_circ` | out | circular orbital speed at that altitude | m/s |
| `dv` | out | burn from the circular orbit to escape | m/s |

**Applies when:** 0 <= h <= 1e6 km; two-body Earth, no atmosphere.

```shell
$ aerospace-engineering solve aero_escape_speed "h=0 km"
v_esc = 11179.87577 m/s
v_circ = 7905.365967 m/s
dv = 3274.509799 m/s
  by: Escape speed at an altitude above the Earth  (v_esc = sqrt(2 mu/(R + h)); v_circ = sqrt(mu/(R + h)))
  h = 0 m
  assumes: 0 <= h <= 1e6 km; two-body Earth, no atmosphere
```

**What the worked example means:** This calculation determines the velocity required to break free from Earth's gravitational pull starting from a specific altitude. Given an altitude of 0 km, the resulting escape speed is 11179.9 m/s. For comparison, the speed needed to maintain a stable circular orbit at this same height is 7905.37 m/s. If a spacecraft were already traveling in that circular orbit, it would need to increase its velocity by 3274.51 m/s to achieve escape trajectory.

<a id="aero_sun_synchronous_inclination"></a>

#### Inclination of a sun-synchronous orbit: `aero_sun_synchronous_inclination`

The inclination at which J2 makes a circular orbit's plane turn eastward at one revolution per year (sun-synchronous), at an altitude, with the orbital period.

$$
\cos i = -\frac{\dot\Omega_\odot}{\frac{3}{2}\,n\,J_2\,(R/r)^2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `h` | in | altitude of the circular orbit (km) | m |
| `i_deg` | out | sun-synchronous inclination in degrees (retrograde, > 90) | - |
| `period_min` | out | orbital period in minutes | - |

**Applies when:** 100 km <= h <= 6000 km (none exists above about 5970 km); circular orbit, first-order J2, node rate 360 deg per 365.2421897 days.

```shell
$ aerospace-engineering solve aero_sun_synchronous_inclination "h=700 km"
i_deg = 98.18795599
period_min = 98.77297615
  by: Inclination of a sun-synchronous orbit  (cos i = -(dOmega/dt)_sun / ((3/2) n J2 (R/r)^2))
  h = 700000 m
  assumes: 100 km <= h <= 6000 km (none exists above about 5970 km); circular orbit, first-order J2, node rate 360 deg per 365.2421897 days
```

**What the worked example means:** This calculation determines the specific inclination needed for a satellite's orbital plane to precess once per year, matching Earth's revolution around the Sun. Given an altitude of 700 km, the resulting sun-synchronous inclination is 98.188. This retrograde angle ensures consistent lighting conditions for imaging, while the corresponding orbital period is 98.773.

<a id="aero_lambert_problem"></a>

#### Lambert's problem (orbit through two positions in a given time), `aero_lambert_problem`

The velocities at departure and arrival of the zero-revolution Earth orbit that goes from one position to another in a given time of flight (prograde or retrograde), with the transfer angle.

$$
\sqrt{\mu}\,t = \left(\frac{y}{C(z)}\right)^{3/2}S(z) + A\sqrt{y},\quad \mathbf v_1 = \frac{\mathbf r_2 - f\mathbf r_1}{g},\quad \mathbf v_2 = \frac{\dot g\,\mathbf r_2 - \mathbf r_1}{g}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `r1` | in | departure position vector in km, e.g. '[7000, 0, 0]' | - |
| `r2` | in | arrival position vector in km | - |
| `tof` | in | time of flight | s |
| `direction` | in | prograde (default) or retrograde transfer (optional) | - |
| `v1` | out | departure velocity vector in km/s | - |
| `v2` | out | arrival velocity vector in km/s | - |
| `dtheta_deg` | out | transfer angle in degrees | - |

**Applies when:** 2 deg <= transfer angle <= 358 deg, not within 2 deg of 180; tof > 0; zero revolutions, two-body Earth (mu = 398600.4418 km^3/s^2).

```shell
$ aerospace-engineering solve aero_lambert_problem "r1=[7000, 0, 0]" "r2=[0, 7000, 0]" "tof=1457.129159 s"
v1 = [-2.08589762905e-9,7.54605329115,0.0]
v2 = [-7.54605329115,2.08589762905e-9,0.0]
dtheta_deg = 90
  by: Lambert's problem (orbit through two positions in a given time)  (universal variables: solve sqrt(mu) t = (y/C)^1.5 S + A sqrt(y) for z by bisection (Stumpff functions C, S); f = 1 - y/r1, g = A sqrt(y/mu), g' = 1 - y/r2; v1 = (r2 - f r1)/g, v2 = (g' r2 - r1)/g)
  tof = 1457.129159 s
  assumes: 2 deg <= transfer angle <= 358 deg, not within 2 deg of 180; tof > 0; zero revolutions, two-body Earth (mu = 398600.4418 km^3/s^2)
```

**What the worked example means:** This calculation determines the precise velocities required at the start and end of an orbital transfer. By solving Lambert's problem using universal variables, it finds the unique trajectory that connects the departure position [7000, 0, 0] to the arrival position [0, 7000, 0] in exactly 1457.13 s seconds. The resulting departure vector [0, 7.54605, 0] and arrival vector [-7.54605, 0, 0] define the maneuver needed for this zero-revolution path, which spans a central angle of 90.

<a id="aero_j2_nodal_regression"></a>

#### Nodal regression from the Earth's oblateness (J2), `aero_j2_nodal_regression`

The secular drift of the right ascension of the ascending node caused by the Earth's equatorial bulge (J2), from the perigee and apogee altitudes and the inclination.

$$
\dot\Omega = -\frac{3}{2}\,n\,J_2\left(\frac{R}{p}\right)^2\cos i
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `hp` | in | perigee altitude (or the altitude of a circular orbit) | m |
| `i` | in | inclination (deg) | rad |
| `ha` | in | apogee altitude (default: circular orbit) (optional) | m |
| `raan_rate_deg_day` | out | node drift rate in degrees per day (negative = westward regression) | - |
| `raan_rate` | out | node drift rate | rad/s |

**Applies when:** 100 km <= hp <= ha <= 1e5 km; 1 deg <= i <= 179 deg; first-order secular J2 theory, J2 = 1.08263e-3.

```shell
$ aerospace-engineering solve aero_j2_nodal_regression "hp=19134.4098 km" "i=60 deg"
raan_rate_deg_day = -0.03892206642
raan_rate = -7.862479291e-09 rad/s
  by: Nodal regression from the Earth's oblateness (J2)  (dOmega/dt = -(3/2) n J2 (R/p)^2 cos i, n = sqrt(mu/a^3), p = a(1 - e^2))
  hp = 19134409.8 m
  i = 1.047197551 rad
  assumes: 100 km <= hp <= ha <= 1e5 km; 1 deg <= i <= 179 deg; first-order secular J2 theory, J2 = 1.08263e-3
```

**What the worked example means:** The node drift rate is -0.0389221 degrees per day, with the raw angular rate being -7.86248e-09 rad/s radians per second.

<a id="aero_j2_apsidal_rotation"></a>

#### Rotation of the line of apsides from J2: `aero_j2_apsidal_rotation`

The secular rotation of the argument of perigee of an eccentric Earth orbit caused by J2 (zero at the critical inclination 63.4 degrees, as used by Molniya orbits).

$$
\dot\omega = \frac{3}{4}\,n\,J_2\left(\frac{R}{p}\right)^2\left(5\cos^2 i - 1\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `hp` | in | perigee altitude | m |
| `ha` | in | apogee altitude | m |
| `i` | in | inclination (deg) | rad |
| `argp_rate_deg_day` | out | perigee rotation rate in degrees per day | - |
| `argp_rate` | out | perigee rotation rate | rad/s |

**Applies when:** 100 km <= hp < ha <= 1e5 km; eccentricity >= 1e-4; 1 deg <= i <= 179 deg; first-order secular J2 theory.

```shell
$ aerospace-engineering solve aero_j2_apsidal_rotation hp=12756273.2 ha=25512546.4 i=1.5707963267948966
argp_rate_deg_day = -0.04428466224
argp_rate = -8.945754216e-09 rad/s
  by: Rotation of the line of apsides from J2  (domega/dt = (3/4) n J2 (R/p)^2 (5 cos^2 i - 1))
  hp = 12756273.2 m
  ha = 25512546.4 m
  i = 1.570796327 rad
  assumes: 100 km <= hp < ha <= 1e5 km; eccentricity >= 1e-4; 1 deg <= i <= 179 deg; first-order secular J2 theory
```

**What the worked example means:** This calculation determines the secular drift of the argument of perigee caused by Earth's oblateness. Using the input values for perigee altitude 1.27563e+07 m, apogee altitude 2.55125e+07 m, and inclination 1.5708 rad, the model computes a rotation rate of -0.0442847 degrees per day. This result indicates how quickly the ellipse of the orbit rotates within its orbital plane, which is critical for maintaining specific ground tracks or apogee positions in eccentric orbits.

### Propellers and rotors (2)

<a id="aero_actuator_disk_hover"></a>

#### Ideal hover power of a rotor (momentum theory), `aero_actuator_disk_hover`

The induced velocity at the disk, the ideal (induced) power and the far-wake velocity of a hovering rotor or static propeller from its thrust, diameter and the air density (actuator-disk momentum theory).

$$
v_i = \sqrt{\frac{T}{2\rho A}},\quad P_i = T v_i = \frac{T^{3/2}}{\sqrt{2\rho A}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `T` | in | thrust | N |
| `D` | in | rotor (disk) diameter | m |
| `rho` | in | air density | kg/m^3 |
| `v_i` | out | induced velocity at the disk | m/s |
| `P_ideal` | out | ideal induced power | W |
| `w` | out | far-wake velocity | m/s |

**Applies when:** T, D, rho > 0; ideal actuator disk: uniform, inviscid, no swirl.

```shell
$ aerospace-engineering solve aero_actuator_disk_hover "T=6.283185307 N" "D=2 m" "rho=1 kg/m^3"
v_i = 1 m/s
P_ideal = 6.283185307 W
w = 2 m/s
  by: Ideal hover power of a rotor (momentum theory)  (v_i = sqrt(T/(2 rho A)), A = pi D^2/4; P = T v_i; w = 2 v_i)
  T = 6.283185307 N
  D = 2 m
  rho = 1 kg/m^3
  assumes: T, D, rho > 0; ideal actuator disk: uniform, inviscid, no swirl
```

**What the worked example means:** This calculation determines the theoretical power required for a hovering rotor or static propeller using momentum theory. Based on the input thrust of 6.28319 N, diameter of 2 m, and air density of 1 kg/m^3, it computes the induced velocity at the disk as 1 m/s. The resulting ideal induced power is 6.28319 W, which represents the minimum energy needed to generate that thrust in an ideal fluid, while the far-wake velocity is found to be 2 m/s. These values provide a baseline for efficiency analysis before accounting for real-world losses.

<a id="aero_actuator_disk_forward"></a>

#### Ideal propeller in axial flight (momentum theory), `aero_actuator_disk_forward`

The induced velocity, ideal power, ideal (Froude) propulsive efficiency and far-wake velocity increment of a propeller producing a thrust at a flight speed (actuator-disk momentum theory).

$$
v_i = -\frac{V}{2} + \sqrt{\frac{V^2}{4} + \frac{T}{2\rho A}},\quad \eta_i = \frac{V}{V + v_i}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `T` | in | thrust | N |
| `V` | in | flight (axial) speed | m/s |
| `D` | in | propeller diameter | m |
| `rho` | in | air density | kg/m^3 |
| `v_i` | out | induced velocity at the disk | m/s |
| `P_ideal` | out | ideal power absorbed | W |
| `eta_ideal` | out | ideal propulsive efficiency T V/P | - |
| `w` | out | far-wake velocity increment | m/s |

**Applies when:** T, V, D, rho > 0; ideal actuator disk.

```shell
$ aerospace-engineering solve aero_actuator_disk_forward "T=25.13274123 N" "V=3 m/s" "D=2 m" "rho=1 kg/m^3"
v_i = 1 m/s
P_ideal = 100.5309649 W
eta_ideal = 0.75
w = 2 m/s
  by: Ideal propeller in axial flight (momentum theory)  (v = -V/2 + sqrt(V^2/4 + T/(2 rho A)); P = T (V + v); eta = V/(V + v); w = 2v)
  T = 25.13274123 N
  V = 3 m/s
  D = 2 m
  rho = 1 kg/m^3
  assumes: T, V, D, rho > 0; ideal actuator disk
```

In the worked example, T = 25.13274123 N, V = 3 m/s, D = 2 m, rho = 1 kg/m^3 go through the equation above and give v_i = 1 m/s, P_ideal = 100.5309649 W, eta_ideal = 0.75, w = 2 m/s. It applies only when T, V, D, rho > 0; ideal actuator disk: outside that it says what is wrong instead of returning a number.

### Re-entry (3)

<a id="aero_ballistic_coefficient"></a>

#### Ballistic coefficient: `aero_ballistic_coefficient`

The ballistic coefficient beta = m/(CD A) of a body (mass per unit drag area), in kg/m^2 and as a weight per area in lbf/ft^2.

$$
\beta = \frac{m}{C_D A}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `m` | in | mass | kg |
| `CD` | in | drag coefficient | - |
| `A` | in | reference area | m^2 |
| `beta` | out | ballistic coefficient m/(CD A) | kg/m^2 |
| `beta_psf` | out | ballistic coefficient as weight per drag area, lbf/ft^2 | - |

**Applies when:** m, CD, A > 0; CD <= 5.

```shell
$ aerospace-engineering solve aero_ballistic_coefficient "m=1000 kg" CD=1.2 "A=1.5 m^2"
beta = 555.5555556 kg/m^2
beta_psf = 113.7867465
  by: Ballistic coefficient  (beta = m/(CD A))
  m = 1000 kg
  CD = 1.2
  A = 1.5 m^2
  assumes: m, CD, A > 0; CD <= 5
```

**What the worked example means:** This calculation determines the ballistic coefficient, defined as mass divided by the product of drag coefficient and reference area. Using an input mass of 1000 kg, a drag coefficient of 1.2, and a reference area of 1.5 m^2, the resulting ballistic coefficient is 555.556 kg/m^2 in kilograms per square meter. This value represents the body's resistance to deceleration due to aerodynamic drag; a higher coefficient indicates that the object retains its velocity more effectively during flight.

<a id="aero_ballistic_entry_peak_deceleration"></a>

#### Peak deceleration of a ballistic re-entry (Allen-Eggers), `aero_ballistic_entry_peak_deceleration`

The peak deceleration (in g), the altitude where it occurs and the speed there for a body entering an exponential atmosphere on a straight ballistic path, from its mass, drag coefficient and reference area (ballistic coefficient m/(CD A)), entry speed and flight-path angle.

$$
a_{max} = \frac{V_E^2\sin\gamma_E}{2eH},\quad h^* = H\ln\frac{\rho_0 H}{\beta\sin\gamma_E},\quad V^* = V_E\,e^{-1/2},\quad \beta = \frac{m}{C_D A}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `m` | in | mass | kg |
| `CD` | in | drag coefficient | - |
| `A` | in | drag reference area | m^2 |
| `VE` | in | entry speed | m/s |
| `gamma_e` | in | entry flight-path angle below the horizontal (deg) | rad |
| `H` | in | atmospheric scale height (default 7.2 km) (optional) | m |
| `rho0` | in | surface density of the exponential atmosphere (default 1.225 kg/m^3) (optional) | kg/m^3 |
| `n_max` | out | peak deceleration in g (9.80665 m/s^2) | - |
| `a_max` | out | peak deceleration | m/s^2 |
| `h_peak` | out | altitude of peak deceleration | m |
| `V_peak` | out | speed at peak deceleration | m/s |

**Applies when:** 2 deg <= gamma_e <= 90 deg; peak above sea level; straight-line path, gravity and lift neglected, exponential atmosphere rho0 exp(-h/H).

```shell
$ aerospace-engineering solve aero_ballistic_entry_peak_deceleration "m=1000 kg" CD=1 "A=1 m^2" "VE=7 km/s" "gamma_e=90 deg"
n_max = 127.6492979
a_max = 1251.811987 m/s^2
h_peak = 15674.55746 m
V_peak = 4245.714618 m/s
  by: Peak deceleration of a ballistic re-entry (Allen-Eggers)  (beta = m/(CD A); a_max = VE^2 sin(gamma)/(2 e H); h* = H ln(rho0 H/(beta sin gamma)); V* = VE e^(-1/2))
  m = 1000 kg
  CD = 1
  A = 1 m^2
  VE = 7000 m/s
  gamma_e = 1.570796327 rad
  assumes: 2 deg <= gamma_e <= 90 deg; peak above sea level; straight-line path, gravity and lift neglected, exponential atmosphere rho0 exp(-h/H)
```

**What the worked example means:** This calculation determines the maximum deceleration force experienced by a spacecraft during ballistic re-entry, expressed as 1251.81 m/s^2. This peak load occurs when the vehicle reaches an altitude of 15674.6 m and is traveling at 4245.71 m/s. These values are derived from the entry speed 7 km/s, flight-path angle 90 deg, and the ballistic coefficient defined by mass 1000 kg, drag coefficient 1, and reference area 1 m^2, assuming an exponential atmosphere.

<a id="aero_ballistic_entry_velocity"></a>

#### Speed and deceleration of a ballistic re-entry at an altitude, `aero_ballistic_entry_velocity`

The speed and the deceleration of a body on a straight ballistic entry into an exponential atmosphere when it reaches a given altitude (Allen-Eggers).

$$
V = V_E\exp\left(-\frac{\rho_0 H e^{-h/H}}{2\beta\sin\gamma_E}\right),\quad a = \frac{\rho V^2}{2\beta}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `m` | in | mass | kg |
| `CD` | in | drag coefficient | - |
| `A` | in | drag reference area | m^2 |
| `VE` | in | entry speed | m/s |
| `gamma_e` | in | entry flight-path angle below the horizontal (deg) | rad |
| `h` | in | altitude | m |
| `H` | in | atmospheric scale height (default 7.2 km) (optional) | m |
| `rho0` | in | surface density (default 1.225 kg/m^3) (optional) | kg/m^3 |
| `V` | out | speed at the altitude | m/s |
| `n` | out | deceleration in g | - |
| `a` | out | deceleration | m/s^2 |

**Applies when:** 2 deg <= gamma_e <= 90 deg; 0 <= h <= 40 H; straight-line path, gravity and lift neglected, exponential atmosphere.

```shell
$ aerospace-engineering solve aero_ballistic_entry_velocity m=1000.0 CD=1.0 A=1.0 VE=7000.0 gamma_e=1.5707963267948966 h=15674.557464134641
V = 4245.714618 m/s
n = 127.6492979
a = 1251.811987 m/s^2
  by: Speed and deceleration of a ballistic re-entry at an altitude  (V = VE exp(-rho0 H e^(-h/H)/(2 beta sin gamma)); a = rho V^2/(2 beta), beta = m/(CD A))
  m = 1000 kg
  CD = 1
  A = 1 m^2
  VE = 7000 m/s
  gamma_e = 1.570796327 rad
  h = 15674.55746 m
  assumes: 2 deg <= gamma_e <= 90 deg; 0 <= h <= 40 H; straight-line path, gravity and lift neglected, exponential atmosphere
```

**What the worked example means:** This calculation determines the speed and deceleration of a body during a straight ballistic entry into an exponential atmosphere. Using the Allen-Eggers method, it computes the velocity based on the entry speed, flight path angle, and atmospheric properties. The result shows that at the specified altitude, the vehicle maintains a speed of 4245.71 m/s, while aerodynamic drag produces a deceleration of 1251.81 m/s^2, equivalent to 127.649 g's.

### Rocket performance (7)

<a id="aero_vertical_rocket_trajectory"></a>

#### Burnout height and apogee of a vertical rocket, `aero_vertical_rocket_trajectory`

Burnout speed and height, apogee height and time to apogee of a rocket launched vertically under constant thrust and mass flow and constant gravity, neglecting drag.

$$
h_b = c\,t_b\left(1 - \frac{\ln R}{R - 1}\right) - \frac{1}{2}g t_b^2,\quad h_{max} = h_b + \frac{v_b^2}{2g}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Isp` | in | specific impulse | s |
| `m0` | in | initial mass | kg |
| `mf` | in | burnout mass | kg |
| `tb` | in | burn time | s |
| `g` | in | gravitational acceleration (default 9.80665 m/s^2) (optional) | m/s^2 |
| `v_b` | out | burnout speed | m/s |
| `h_b` | out | burnout height | m |
| `h_max` | out | apogee height | m |
| `t_apex` | out | time from lift-off to apogee | s |

**Applies when:** 0 < mf < m0; thrust-to-weight above 1; constant thrust, constant g, no drag; vertical flight.

```shell
$ aerospace-engineering solve aero_vertical_rocket_trajectory "Isp=300 s" "m0=2718.281828 kg" "mf=1000 kg" "tb=100 s"
v_b = 1961.329999 m/s
h_b = 73948.99381 m
h_max = 270081.9937 m
t_apex = 299.9999999 s
  by: Burnout height and apogee of a vertical rocket  (v_b = c ln R - g tb; h_b = c tb (1 - ln R/(R - 1)) - g tb^2/2; h_max = h_b + v_b^2/(2g); t_apex = tb + v_b/g (c = Isp g0, R = m0/mf))
  Isp = 300 s
  m0 = 2718.281828 kg
  mf = 1000 kg
  tb = 100 s
  assumes: 0 < mf < m0; thrust-to-weight above 1; constant thrust, constant g, no drag; vertical flight
```

**What the worked example means:** This calculation determines the vertical rocket's performance at engine cutoff and peak altitude. Using inputs for specific impulse, initial mass, burnout mass, and burn time, it computes the burnout speed as 1961.33 m/s, the burnout height as 73949 m, the maximum apogee height as 270082 m, and the total time to reach that apex as 300 s. These values assume constant thrust and gravity while neglecting air resistance.

<a id="aero_multistage_delta_v"></a>

#### Delta-v of a multistage rocket: `aero_multistage_delta_v`

The total ideal velocity change of a multistage rocket and the contribution of each stage, from each stage's specific impulse and the vehicle mass at its ignition and at its burnout.

$$
\Delta v = \sum_i I_{sp,i}\,g_0\ln\frac{m_{0,i}}{m_{f,i}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Isp` | in | specific impulse of each stage in seconds, e.g. '[290, 340]' | - |
| `m0` | in | vehicle mass at each stage's ignition in kilograms | - |
| `mf` | in | vehicle mass at each stage's burnout in kilograms | - |
| `dv` | out | total delta-v | m/s |
| `dv_stages` | out | delta-v of each stage in m/s | - |

**Applies when:** equal numbers of stages; 0 < mf < m0 for each stage; each ignition mass below the previous burnout mass; no gravity or drag.

```shell
$ aerospace-engineering solve aero_multistage_delta_v "Isp=[300, 300]" "m0=[2718.28, 500]" "mf=[1000, 183.94]"
dv = 5883.983552 m/s
dv_stages = [2941.99302106,2941.99053096]
  by: Delta-v of a multistage rocket  (dv = sum of Isp_i g0 ln(m0_i/mf_i))
  assumes: equal numbers of stages; 0 < mf < m0 for each stage; each ignition mass below the previous burnout mass; no gravity or drag
```

**What the worked example means:** This calculation determines the ideal velocity change for a multistage rocket by summing the contributions from each stage. Using the specific impulse and mass ratios provided as inputs, the method applies the Tsiolkovsky equation to each stage sequentially. The result, 5883.99 m/s, represents the maximum theoretical speed increment achievable in a vacuum without gravity or drag losses, while [2941.99, 2941.99] details how much each individual stage adds to this total.

<a id="aero_delta_v_gravity_loss"></a>

#### Gravity loss of a vertical rocket burn: `aero_delta_v_gravity_loss`

The burnout speed of a rocket climbing vertically under constant thrust and constant gravity (no drag), its ideal delta-v and the gravity loss g tb.

$$
v_b = I_{sp}g_0\ln\frac{m_0}{m_f} - g\,t_b
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Isp` | in | specific impulse | s |
| `m0` | in | initial mass | kg |
| `mf` | in | burnout mass | kg |
| `tb` | in | burn time | s |
| `g` | in | gravitational acceleration (default 9.80665 m/s^2) (optional) | m/s^2 |
| `v_b` | out | vertical speed at burnout | m/s |
| `dv_ideal` | out | ideal delta-v (no gravity) | m/s |
| `loss` | out | gravity loss | m/s |

**Applies when:** 0 < mf < m0; thrust-to-weight above 1 at lift-off; constant thrust and mass flow, constant g, no drag, vertical flight.

```shell
$ aerospace-engineering solve aero_delta_v_gravity_loss "Isp=300 s" "m0=4 t" "mf=1 t" "tb=100 s"
v_b = 3097.806079 m/s
dv_ideal = 4078.471079 m/s
loss = 980.665 m/s
  by: Gravity loss of a vertical rocket burn  (v_b = Isp g0 ln(m0/mf) - g tb)
  Isp = 300 s
  m0 = 4000 kg
  mf = 1000 kg
  tb = 100 s
  assumes: 0 < mf < m0; thrust-to-weight above 1 at lift-off; constant thrust and mass flow, constant g, no drag, vertical flight
```

**What the worked example means:** The vertical burnout speed is 3097.81 m/s, with an ideal delta-v of 4078.47 m/s and a gravity loss of 980.665 m/s.

<a id="aero_optimal_staging"></a>

#### Optimal staging of a multistage rocket: `aero_optimal_staging`

The lightest rocket that gives a payload a required delta-v with stages of given specific impulses and structural coefficients: the optimal mass ratio and mass of each stage, the lift-off mass and the payload fraction (Lagrange-multiplier solution).

$$
R_i = \frac{\lambda c_i - 1}{\lambda c_i \varepsilon_i},\quad \sum_i c_i\ln R_i = \Delta v,\quad m_i = m_{above}\frac{R_i - 1}{1 - R_i\varepsilon_i}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Isp` | in | specific impulse of each stage in seconds, bottom stage first, e.g. '[300, 350, 450]' | - |
| `eps` | in | structural coefficient ms/(ms + mp) of each stage | - |
| `dv` | in | required total delta-v | m/s |
| `m_pl` | in | payload mass | kg |
| `m0` | out | minimum lift-off mass | kg |
| `payload_fraction` | out | payload fraction m_pl/m0 | - |
| `R` | out | optimal mass ratio of each stage (list) | - |
| `stage_mass` | out | mass of each stage, structure plus propellant, in kg (list) | - |

**Applies when:** 2 to 5 stages; 0.01 <= eps <= 0.5; delta-v below what the stages could reach with zero payload; impulsive staging, no gravity or drag losses.

```shell
$ aerospace-engineering solve aero_optimal_staging "Isp=[300, 300]" "eps=[0.1, 0.1]" "dv=8156.942158 m/s" "m_pl=1000 kg"
m0 = 36000 kg
payload_fraction = 0.02777777778
R = [4.00000000008,4.00000000008]
stage_mass = [30000.0000021,5000.00000019]
  by: Optimal staging of a multistage rocket  (R_i = (lambda c_i - 1)/(lambda c_i eps_i) with the multiplier lambda from sum c_i ln R_i = dv (bisection); stage masses built from the payload up, m_i = m_above (R_i - 1)/(1 - R_i eps_i))
  dv = 8156.942158 m/s
  m_pl = 1000 kg
  assumes: 2 to 5 stages; 0.01 <= eps <= 0.5; delta-v below what the stages could reach with zero payload; impulsive staging, no gravity or drag losses
```

**What the worked example means:** This calculation determines the optimal staging configuration for a multistage rocket to achieve the required delta-v of 8156.94 m/s while carrying a payload of 1000 kg. Using Lagrange multipliers, it computes the ideal mass ratios for each stage based on their specific impulses and structural coefficients. The resulting minimum lift-off mass is 36000 kg, which corresponds to a payload fraction of 0.0277778. This represents the lightest possible vehicle that meets the mission requirements under impulsive burn assumptions.

<a id="aero_identical_stages_payload"></a>

#### Payload fraction of a rocket with identical stages, `aero_identical_stages_payload`

For a rocket of n identical stages (same specific impulse and structural coefficient, equal delta-v split): the payload ratio of each stage, the overall payload fraction, the stage mass ratio and the gross lift-off mass per kilogram of payload.

$$
R = e^{\Delta v/(n I_{sp} g_0)},\quad \lambda = \frac{1 - \varepsilon R}{R - 1},\quad \frac{m_{pl}}{m_0} = \left(\frac{\lambda}{1+\lambda}\right)^n
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `n` | in | number of stages (1 to 6) | - |
| `dv` | in | total delta-v | m/s |
| `Isp` | in | specific impulse of every stage | s |
| `eps` | in | structural coefficient ms/(ms + mp) of every stage | - |
| `lam` | out | payload ratio of each stage (payload above / stage mass) | - |
| `payload_fraction` | out | overall payload fraction m_payload/m0 | - |
| `R` | out | mass ratio of each stage | - |
| `gross_per_payload` | out | lift-off mass per unit payload mass | - |

**Applies when:** 1 <= n <= 6; 0.01 <= eps <= 0.5; eps R < 1; identical stages, equal delta-v split.

```shell
$ aerospace-engineering solve aero_identical_stages_payload n=1 "dv=4078.471079 m/s" "Isp=300 s" eps=0.1
lam = 0.2
payload_fraction = 0.1666666667
R = 4
gross_per_payload = 6
  by: Payload fraction of a rocket with identical stages  (R = exp(dv/(n Isp g0)); lambda = (1 - eps R)/(R - 1); payload fraction = (lambda/(1 + lambda))^n)
  dv = 4078.471079 m/s
  Isp = 300 s
  eps = 0.1
  assumes: 1 <= n <= 6; 0.01 <= eps <= 0.5; eps R < 1; identical stages, equal delta-v split
```

**What the worked example means:** This calculation determines the efficiency of a multi-stage rocket where all stages are identical. Given 1 stages, each providing an equal share of the total delta-v of 4078.47 m/s m/s, and assuming a specific impulse of 300 s s and a structural coefficient of 0.1, the model computes the stage mass ratio R as 4. The resulting payload ratio for each stage is 0.2, which leads to an overall payload fraction of 0.166667. This means that for every kilogram of payload delivered, the initial gross lift-off mass required is 6 kilograms.

<a id="aero_propellant_for_delta_v"></a>

#### Propellant mass for a required delta-v: `aero_propellant_for_delta_v`

The propellant mass, initial mass and mass ratio a rocket stage needs to give a velocity change, from its specific impulse and burnout mass (structure plus payload).

$$
\frac{m_0}{m_f} = e^{\Delta v/(I_{sp} g_0)},\quad m_p = m_f\left(e^{\Delta v/(I_{sp} g_0)} - 1\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `dv` | in | required velocity change (m/s, km/s) | m/s |
| `Isp` | in | specific impulse (s) | s |
| `mf` | in | burnout mass (dry mass plus payload) | kg |
| `m0` | out | initial mass | kg |
| `mp` | out | propellant mass | kg |
| `R` | out | mass ratio m0/mf | - |

**Applies when:** dv > 0; 1 s <= Isp <= 10000 s; mf > 0; mass ratio <= 10000.

```shell
$ aerospace-engineering solve aero_propellant_for_delta_v "dv=2941.995 m/s" "Isp=300 s" "mf=1000 kg"
m0 = 2718.281828 kg
mp = 1718.281828 kg
R = 2.718281828
  by: Propellant mass for a required delta-v  (R = exp(dv/(Isp g0)); m0 = R mf; mp = (R - 1) mf)
  dv = 2941.995 m/s
  Isp = 300 s
  mf = 1000 kg
  assumes: dv > 0; 1 s <= Isp <= 10000 s; mf > 0; mass ratio <= 10000
```

**What the worked example means:** This calculation determines the propellant mass needed to achieve a specific velocity change, based on the burnout mass and engine efficiency. Using the ideal rocket equation, the mass ratio is computed as 2.71828, which indicates how many times heavier the full stage is compared to its empty structure and payload. Multiplying this ratio by the burnout mass yields the total initial mass of 2718.28 kg. The difference between these two values gives the necessary propellant load of 1718.28 kg. This result helps engineers size fuel tanks and estimate the overall launch vehicle requirements for the mission.

<a id="aero_rocket_delta_v"></a>

#### Rocket delta-v from specific impulse (Tsiolkovsky), `aero_rocket_delta_v`

The ideal velocity change of a rocket (no gravity or drag) from its specific impulse and its initial and burnout masses.

$$
\Delta v = I_{sp}\,g_0\ln\frac{m_0}{m_f}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Isp` | in | specific impulse (s) | s |
| `m0` | in | initial (lift-off or ignition) mass | kg |
| `mf` | in | final (burnout) mass | kg |
| `dv` | out | ideal velocity change delta-v | m/s |

**Applies when:** 1 s <= Isp <= 10000 s; 0 < mf < m0; no gravity, no drag.

```shell
$ aerospace-engineering solve aero_rocket_delta_v "Isp=450 s" "m0=4000 kg" "mf=1000 kg"
dv = 6117.706618 m/s
  by: Rocket delta-v from specific impulse (Tsiolkovsky)  (dv = Isp g0 ln(m0/mf))
  Isp = 450 s
  m0 = 4000 kg
  mf = 1000 kg
  assumes: 1 s <= Isp <= 10000 s; 0 < mf < m0; no gravity, no drag
```

**What the worked example means:** This calculation determines the rocket's ideal velocity change, denoted as delta-v, using the Tsiolkovsky rocket equation. It relies on the specific impulse of 450 s and the mass ratio derived from an initial mass of 4000 kg and a final burnout mass of 1000 kg. The resulting velocity change of 6117.71 m/s represents the maximum speed gain achievable in a vacuum without gravitational or drag losses.

### Rocket propulsion (6)

<a id="aero_characteristic_velocity"></a>

#### Characteristic velocity c* of a rocket propellant, `aero_characteristic_velocity`

The characteristic exhaust velocity c* = pc At / mdot of ideal rocket combustion products from the chamber temperature, molar mass and ratio of specific heats, with the Vandenkerckhove function.

$$
c^* = \frac{\sqrt{R T_c}}{\Gamma},\quad \Gamma = \sqrt{\gamma}\left(\frac{2}{\gamma+1}\right)^{\frac{\gamma+1}{2(\gamma-1)}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Tc` | in | chamber (combustion) temperature | K |
| `Mw` | in | molar mass of the combustion products (g/mol, kg/kmol) | kg/mol |
| `gamma` | in | ratio of specific heats of the products | - |
| `c_star` | out | characteristic velocity c* | m/s |
| `Gamma` | out | Vandenkerckhove function Gamma(gamma) | - |

**Applies when:** Tc > 0; 1 <= Mw <= 300 g/mol; 1.05 <= gamma <= 1.7; ideal, frozen, isentropic flow.

```shell
$ aerospace-engineering solve aero_characteristic_velocity "Tc=3000 K" "Mw=20 g/mol" gamma=1.2
c_star = 1721.994926 m/s
Gamma = 0.6485311707
  by: Characteristic velocity c* of a rocket propellant  (c* = sqrt(R Tc)/Gamma, R = Ru/Mw, Gamma = sqrt(g)(2/(g+1))^((g+1)/(2(g-1))))
  Tc = 3000 K
  Mw = 0.02 kg/mol
  gamma = 1.2
  assumes: Tc > 0; 1 <= Mw <= 300 g/mol; 1.05 <= gamma <= 1.7; ideal, frozen, isentropic flow
```

**What the worked example means:** This calculation determines the characteristic velocity c*, which measures the efficiency of the rocket combustion chamber independent of nozzle design. Using an ideal gas assumption with frozen composition, the result relies on the chamber temperature 3000 K, the product molar mass 20 g/mol, and the specific heat ratio 1.2. The computed value is 1721.99 m/s m/s, indicating the potential energy available for conversion into thrust based on these thermodynamic properties.

<a id="aero_rocket_engine_performance"></a>

#### Ideal rocket engine performance: `aero_rocket_engine_performance`

Thrust, mass flow, specific impulse, thrust coefficient, characteristic velocity and the exit pressure, velocity and Mach number of an ideal rocket engine from its chamber conditions, propellant products, throat area, expansion ratio and the ambient pressure.

$$
F = \dot m v_e + (p_e - p_a)A_e,\quad \dot m = \frac{p_c A_t}{c^*},\quad I_{sp} = \frac{F}{\dot m g_0}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `pc` | in | chamber pressure | Pa |
| `Tc` | in | chamber temperature | K |
| `Mw` | in | molar mass of the products (g/mol) | kg/mol |
| `gamma` | in | ratio of specific heats | - |
| `At` | in | throat area | m^2 |
| `eps` | in | expansion area ratio Ae/At | - |
| `pa` | in | ambient pressure (0 for vacuum) | Pa |
| `F` | out | thrust | N |
| `mdot` | out | propellant mass flow | kg/s |
| `Isp` | out | specific impulse | s |
| `CF` | out | thrust coefficient | - |
| `c_star` | out | characteristic velocity | m/s |
| `pe` | out | exit pressure | Pa |
| `ve` | out | exit (exhaust) velocity | m/s |
| `Me` | out | exit Mach number | - |

**Applies when:** 1 < eps <= 500; 0 <= pa < pc; pe >= 0.4 pa (no flow separation); 1 <= Mw <= 300 g/mol; 1.05 <= gamma <= 1.7; ideal, frozen, isentropic, one-dimensional nozzle flow.

```shell
$ aerospace-engineering solve aero_rocket_engine_performance "pc=1 MPa" "Tc=3000 K" "Mw=20 g/mol" gamma=1.4 "At=0.01 m^2" eps=1.6875 "pa=0 Pa"
F = 14234.22902 N
mdot = 6.131370467 kg/s
Isp = 236.731318 s
CF = 1.423422902
c_star = 1630.956742 m/s
pe = 127804.5255 Pa
ve = 1969.792516 m/s
Me = 2
  by: Ideal rocket engine performance  (mdot = pc At/c*; Me from eps; pe = pc (1 + (g-1)/2 Me^2)^(-g/(g-1)); ve from the isentropic expansion; F = mdot ve + (pe - pa) Ae; Isp = F/(mdot g0))
  pc = 1000000 Pa
  Tc = 3000 K
  Mw = 0.02 kg/mol
  gamma = 1.4
  At = 0.01 m^2
  eps = 1.6875
  pa = 0 Pa
  assumes: 1 < eps <= 500; 0 <= pa < pc; pe >= 0.4 pa (no flow separation); 1 <= Mw <= 300 g/mol; 1.05 <= gamma <= 1.7; ideal, frozen, isentropic, one-dimensional nozzle flow
```

**What the worked example means:** This calculation determines the performance of an ideal rocket engine based on its chamber conditions and nozzle geometry. Given a chamber pressure of 1 MPa and throat area of 0.01 m^2, the propellant mass flow is 6.13137 kg/s. The expansion ratio of 1.6875 yields an exit Mach number of 2 and exit pressure of 127805 Pa. Consequently, the total thrust is 14234.2 N, resulting in a specific impulse of 236.731 s seconds.

<a id="aero_ideal_exhaust_velocity"></a>

#### Ideal rocket exhaust velocity: `aero_ideal_exhaust_velocity`

The exhaust velocity of an ideal rocket nozzle expanding combustion products from the chamber to the exit pressure, with the specific impulse at that exit pressure (optimum expansion) and the exit Mach number.

$$
v_e = \sqrt{\frac{2\gamma}{\gamma-1}\frac{R_u T_c}{\mathcal{M}}\left[1 - \left(\frac{p_e}{p_c}\right)^{\frac{\gamma-1}{\gamma}}\right]}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Tc` | in | chamber temperature | K |
| `Mw` | in | molar mass of the products (g/mol) | kg/mol |
| `gamma` | in | ratio of specific heats | - |
| `pc` | in | chamber pressure | Pa |
| `pe` | in | nozzle exit pressure (> 0) | Pa |
| `ve` | out | exhaust velocity | m/s |
| `Isp` | out | specific impulse ve/g0 (exit pressure = ambient) | s |
| `Me` | out | exit Mach number | - |

**Applies when:** 0 < pe < pc; 1 <= Mw <= 300 g/mol; 1.05 <= gamma <= 1.7; ideal, frozen, isentropic expansion.

```shell
$ aerospace-engineering solve aero_ideal_exhaust_velocity "Tc=3000 K" "Mw=20 g/mol" gamma=1.4 "pc=1 MPa" "pe=127.8045255 kPa"
ve = 1969.792516 m/s
Isp = 200.8629364 s
Me = 2
  by: Ideal rocket exhaust velocity  (ve = sqrt(2g/(g-1) (Ru/Mw) Tc [1 - (pe/pc)^((g-1)/g)]))
  Tc = 3000 K
  Mw = 0.02 kg/mol
  gamma = 1.4
  pc = 1000000 Pa
  pe = 127804.5255 Pa
  assumes: 0 < pe < pc; 1 <= Mw <= 300 g/mol; 1.05 <= gamma <= 1.7; ideal, frozen, isentropic expansion
```

**What the worked example means:** This calculation determines the speed at which combustion gases leave the nozzle under ideal conditions. It uses the chamber temperature, molar mass, pressure ratio, and specific heat ratio to compute the exhaust velocity, which comes out to 1969.79 m/s. This velocity directly translates to a specific impulse of 200.863 s, representing the engine's efficiency in seconds. The result also yields an exit Mach number of 2, indicating how supersonic the flow is relative to the local speed of sound at the nozzle exit.

<a id="aero_thrust_coefficient"></a>

#### Nozzle thrust coefficient: `aero_thrust_coefficient`

The thrust coefficient CF = F/(pc At) of an ideal rocket nozzle of given expansion area ratio at an ambient pressure, with the exit pressure ratio and Mach number and the vacuum thrust coefficient.

$$
C_F = \sqrt{\frac{2\gamma^2}{\gamma-1}\left(\frac{2}{\gamma+1}\right)^{\frac{\gamma+1}{\gamma-1}}\left[1 - \left(\frac{p_e}{p_c}\right)^{\frac{\gamma-1}{\gamma}}\right]} + \frac{p_e - p_a}{p_c}\frac{A_e}{A_t}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `gamma` | in | ratio of specific heats of the exhaust | - |
| `eps` | in | nozzle expansion area ratio Ae/At (> 1) | - |
| `pc` | in | chamber pressure | Pa |
| `pa` | in | ambient pressure (0 for vacuum) | Pa |
| `CF` | out | thrust coefficient | - |
| `pe_pc` | out | exit-to-chamber pressure ratio | - |
| `Me` | out | exit Mach number | - |
| `CF_vac` | out | vacuum thrust coefficient | - |

**Applies when:** 1 < eps <= 500; 0 <= pa < pc; pe >= 0.4 pa (no flow separation); 1.05 <= gamma <= 1.7.

```shell
$ aerospace-engineering solve aero_thrust_coefficient gamma=1.4 eps=1.6875 "pc=1 MPa" "pa=0 Pa"
CF = 1.423422902
pe_pc = 0.1278045255
Me = 2
CF_vac = 1.423422902
  by: Nozzle thrust coefficient  (Me from the area ratio (supersonic); pe/pc isentropic; CF = Gamma sqrt(2g/(g-1)(1 - (pe/pc)^((g-1)/g))) + (pe - pa)/pc eps)
  gamma = 1.4
  eps = 1.6875
  pc = 1000000 Pa
  pa = 0 Pa
  assumes: 1 < eps <= 500; 0 <= pa < pc; pe >= 0.4 pa (no flow separation); 1.05 <= gamma <= 1.7
```

**What the worked example means:** This calculation determines the ideal rocket nozzle's thrust coefficient, which quantifies how effectively chamber pressure converts to total thrust. Using the specified expansion area ratio 1.6875 and exhaust properties defined by the specific heat ratio 1.4, the model computes the exit Mach number 2 and the exit-to-chamber pressure ratio 0.127805. The resulting thrust coefficient 1.42342 incorporates both momentum and pressure contributions relative to ambient pressure 0 Pa, while the vacuum thrust coefficient 1.42342 represents performance in zero-pressure environments.

<a id="aero_optimum_area_ratio"></a>

#### Optimum nozzle area ratio: `aero_optimum_area_ratio`

The nozzle expansion area ratio that expands the exhaust exactly to the ambient pressure (optimum expansion, maximum thrust at that pressure), with the exit Mach number and the thrust coefficient.

$$
M_e = \sqrt{\frac{2}{\gamma-1}\left[\left(\frac{p_c}{p_a}\right)^{\frac{\gamma-1}{\gamma}} - 1\right]},\quad \varepsilon = \frac{A}{A^*}(M_e)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `gamma` | in | ratio of specific heats | - |
| `pc` | in | chamber pressure | Pa |
| `pa` | in | ambient pressure | Pa |
| `eps` | out | optimum area ratio Ae/At | - |
| `Me` | out | exit Mach number | - |
| `CF` | out | thrust coefficient at optimum expansion | - |

**Applies when:** 2 <= pc/pa <= 1e6; 1.05 <= gamma <= 1.7.

```shell
$ aerospace-engineering solve aero_optimum_area_ratio gamma=1.4 "pc=782.4449067 kPa" "pa=100 kPa"
eps = 1.6875
Me = 2
CF = 1.207752766
  by: Optimum nozzle area ratio  (pe = pa: Me = sqrt(2/(g-1)[(pc/pa)^((g-1)/g) - 1]); eps = A/A*(Me); CF = Gamma sqrt(2g/(g-1)(1 - (pa/pc)^((g-1)/g))))
  gamma = 1.4
  pc = 782444.9067 Pa
  pa = 100000 Pa
  assumes: 2 <= pc/pa <= 1e6; 1.05 <= gamma <= 1.7
```

**What the worked example means:** This calculation determines the nozzle expansion area ratio that matches exhaust pressure to ambient pressure for maximum thrust. Given a specific heat ratio of 1.4, a chamber pressure of 782.445 kPa, and an ambient pressure of 100 kPa, the model computes an optimum area ratio of 1.6875. This corresponds to an exit Mach number of 2 and yields a thrust coefficient of 1.20775, representing the most efficient expansion under these conditions.

<a id="aero_throat_area_for_thrust"></a>

#### Throat area of a rocket engine for a required thrust, `aero_throat_area_for_thrust`

The throat and exit areas, mass flow and specific impulse of an ideal rocket engine sized to give a required thrust at an ambient pressure.

$$
A_t = \frac{F}{C_F\,p_c},\quad \dot m = \frac{p_c A_t}{c^*},\quad I_{sp} = \frac{c^* C_F}{g_0}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `F` | in | required thrust (N, kN, lbf) | N |
| `pc` | in | chamber pressure | Pa |
| `Tc` | in | chamber temperature | K |
| `Mw` | in | molar mass of the products | kg/mol |
| `gamma` | in | ratio of specific heats | - |
| `eps` | in | expansion area ratio Ae/At | - |
| `pa` | in | ambient pressure (0 for vacuum) | Pa |
| `At` | out | throat area | m^2 |
| `Ae` | out | exit area | m^2 |
| `mdot` | out | propellant mass flow | kg/s |
| `Isp` | out | specific impulse | s |

**Applies when:** F > 0; 1 < eps <= 500; 0 <= pa < pc; pe >= 0.4 pa; 1.05 <= gamma <= 1.7.

```shell
$ aerospace-engineering solve aero_throat_area_for_thrust "F=1 kN" "pc=1 MPa" "Tc=3000 K" "Mw=20 g/mol" gamma=1.4 eps=1.6875 "pa=127.8045255 kPa"
At = 0.0008279840283 m^2
Ae = 0.001397223048 m^2
mdot = 0.5076676818 kg/s
Isp = 200.8629364 s
  by: Throat area of a rocket engine for a required thrust  (At = F/(CF pc); Ae = eps At; mdot = pc At/c*; Isp = c* CF/g0)
  F = 1000 N
  pc = 1000000 Pa
  Tc = 3000 K
  Mw = 0.02 kg/mol
  gamma = 1.4
  eps = 1.6875
  pa = 127804.5255 Pa
  assumes: F > 0; 1 < eps <= 500; 0 <= pa < pc; pe >= 0.4 pa; 1.05 <= gamma <= 1.7
```

**What the worked example means:** This calculation determines the nozzle geometry needed to produce a specific thrust. Using the desired force of 1 kN, chamber pressure of 1 MPa, and expansion ratio of 1.6875, it computes the throat area as 0.000827984 m^2 and the exit area as 0.00139722 m^2. The resulting propellant mass flow rate is 0.507668 kg/s and the specific impulse is 200.863 s. These values define the physical size of the engine components and the rate at which fuel must be consumed to meet performance targets.

### Rockets (1)

<a id="aero_rocket_thrust"></a>

#### Rocket thrust and specific impulse from mass flow and exhaust velocity, `aero_rocket_thrust`

The thrust F = mdot ve + (pe - pa) Ae of a rocket engine, its specific impulse F/(mdot g0) and effective exhaust velocity, from the propellant mass flow and the exhaust velocity (with the pressure thrust when the exit conditions are given).

$$
F = \dot m v_e + (p_e - p_a)A_e,\quad I_{sp} = \frac{F}{\dot m g_0}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `mdot` | in | propellant mass flow rate (kg/s) | kg/s |
| `ve` | in | exhaust velocity at the nozzle exit | m/s |
| `pe` | in | exit pressure (optional, with pa and Ae) (optional) | Pa |
| `pa` | in | ambient pressure (optional) (optional) | Pa |
| `Ae` | in | nozzle exit area (optional) (optional) | m^2 |
| `F` | out | thrust | N |
| `F_kN` | out | thrust, kN | - |
| `Isp` | out | specific impulse F/(mdot g0) | s |
| `c_eff` | out | effective exhaust velocity F/mdot | m/s |
| `F_pressure` | out | pressure thrust (pe - pa) Ae | N |

**Applies when:** mdot, ve > 0; pe, pa, Ae given together or not at all; steady, one-dimensional exit flow.

```shell
$ aerospace-engineering solve aero_rocket_thrust "mdot=250 kg/s" "ve=3000 m/s"
F = 750000 N
F_kN = 750
Isp = 305.9148639 s
c_eff = 3000 m/s
F_pressure = 0 N
  by: Rocket thrust and specific impulse from mass flow and exhaust velocity  (F = mdot ve + (pe - pa) Ae; Isp = F/(mdot g0); c = F/mdot)
  mdot = 250 kg/s
  ve = 3000 m/s
  assumes: mdot, ve > 0; pe, pa, Ae given together or not at all; steady, one-dimensional exit flow
```

**What the worked example means:** This calculation determines the rocket engine's performance by computing total thrust, specific impulse, and effective exhaust velocity from the propellant mass flow rate and exhaust velocity. The resulting thrust of 750000 N represents the force generated, while the specific impulse of 305.915 s indicates efficiency in converting propellant into momentum. If exit pressure data was provided, the pressure thrust component is included to give a complete picture of the engine's output under the given conditions.

### Stability (1)

<a id="aero_neutral_point"></a>

#### Neutral point and static margin (wing and tail), `aero_neutral_point`

The stick-fixed neutral point, the static margin, the airplane lift-curve slope and the pitching-moment slope Cm_alpha of a wing-tail aircraft (positions as fractions of the mean aerodynamic chord, aft of its leading edge).

$$
h_n = h_{ac} + \eta\,\bar V_H\,\frac{a_t}{a}\left(1 - \frac{d\varepsilon}{d\alpha}\right),\quad \bar V_H = \frac{S_t l_t}{S\bar c},\quad C_{m\alpha} = -a\,(h_n - h_{cg})
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `h_ac` | in | wing-body aerodynamic centre / mean chord | - |
| `a_w` | in | wing lift-curve slope (per rad) | - |
| `a_t` | in | tail lift-curve slope (per rad) | - |
| `St_S` | in | tail area / wing area | - |
| `lt_c` | in | distance from the wing aerodynamic centre to the tail aerodynamic centre / mean chord | - |
| `deps` | in | downwash gradient d(epsilon)/d(alpha) | - |
| `h_cg` | in | centre of gravity / mean chord | - |
| `eta_t` | in | tail efficiency (dynamic-pressure ratio, default 1) (optional) | - |
| `h_n` | out | neutral point / mean chord | - |
| `SM` | out | static margin h_n - h_cg (positive = stable) | - |
| `CL_alpha` | out | airplane lift-curve slope (per rad) | - |
| `Cm_alpha` | out | pitching-moment slope about the cg (per rad) | - |

**Applies when:** linear aerodynamics (small angles); thrust and fuselage moment effects neglected beyond h_ac; stated ranges of each input.

```shell
$ aerospace-engineering solve aero_neutral_point h_ac=0.25 a_w=5 a_t=5 St_S=0.2 lt_c=4 deps=0.5 h_cg=0.3
h_n = 0.6136363636
SM = 0.3136363636
CL_alpha = 5.5
Cm_alpha = -1.725
  by: Neutral point and static margin (wing and tail)  (a = a_w + eta (St/S) a_t (1 - deps); h_n = h_ac + eta (St/S)(lt/c) a_t (1 - deps)/a; SM = h_n - h_cg; Cm_alpha = -a SM)
  h_ac = 0.25
  a_w = 5
  a_t = 5
  St_S = 0.2
  lt_c = 4
  deps = 0.5
  h_cg = 0.3
  assumes: linear aerodynamics (small angles); thrust and fuselage moment effects neglected beyond h_ac; stated ranges of each input
```

**What the worked example means:** This calculation determines the longitudinal static stability characteristics of a wing-tail aircraft configuration. Using the provided aerodynamic inputs, it computes the stick-fixed neutral point position, which represents the center of pressure for the entire airplane when the elevator is fixed. The result shows that the neutral point lies at 0.613636, resulting in a positive static margin of 0.313636, indicating that the aircraft is statically stable in pitch.

### Standard atmosphere (5)

<a id="aero_density_altitude"></a>

#### Density altitude from pressure altitude and temperature, `aero_density_altitude`

Air density and density altitude (the standard-atmosphere height with the same density) from the pressure altitude and the outside air temperature.

$$
\rho = \frac{p_{std}(h_p)}{R\,T},\quad \rho_{std}(h_\rho) = \rho
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `hp` | in | pressure altitude (m, ft) | m |
| `T` | in | outside air temperature (K, °C, °F) | K |
| `rho` | out | air density | kg/m^3 |
| `sigma` | out | density ratio rho/rho0 | - |
| `h_rho` | out | density altitude (geometric) | m |
| `h_rho_ft` | out | density altitude in feet | - |
| `p` | out | static pressure at the pressure altitude | Pa |

**Applies when:** 150 K <= T <= 350 K; -1 km <= hp <= 47 km; the density within the standard atmosphere.

```shell
$ aerospace-engineering solve aero_density_altitude "hp=0 ft" "T=15 °C"
rho = 1.224999156 kg/m^3
sigma = 1
h_rho = 0 m
h_rho_ft = 0
p = 101325 Pa
  by: Density altitude from pressure altitude and temperature  (p = p_std(hp); rho = p/(R T); density altitude = the standard height where rho_std(h) = rho (inverted layer by layer))
  hp = 0 m
  T = 288.15 K
  assumes: 150 K <= T <= 350 K; -1 km <= hp <= 47 km; the density within the standard atmosphere
```

**What the worked example means:** This calculation determines the density altitude, which represents the height in the standard atmosphere that has the same air density as the current conditions. Using a pressure altitude of 0 ft and an outside air temperature of 15 °C, the resulting static pressure is 101325 Pa and the air density is 1.225 kg/m^3. The calculated density altitude is 0 m, indicating how thin or hot the air is relative to standard sea level conditions, which directly impacts lift generation and engine performance.

<a id="aero_geopotential_altitude"></a>

#### Geopotential height from geometric altitude, `aero_geopotential_altitude`

The geopotential height (the height in a uniform gravity field g0 with the same potential energy) and the local gravity at a geometric altitude.

$$
H = \frac{1}{g_0}\int_0^h g\,dz = \frac{r_0 h}{r_0 + h},\quad g = g_0\left(\frac{r_0}{r_0 + h}\right)^2
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `h` | in | geometric altitude (m, km, ft), 0 to 1000 km | m |
| `H` | out | geopotential height | m |
| `g` | out | gravitational acceleration at the altitude | m/s^2 |
| `dH` | out | geometric minus geopotential height | m |

**Applies when:** 0 <= h <= 1000 km; inverse-square gravity with r0 = 6356766 m.

```shell
$ aerospace-engineering solve aero_geopotential_altitude "h=0 km"
H = 0 m
g = 9.80665 m/s^2
dH = 0 m
  by: Geopotential height from geometric altitude  (H = (1/g0) integral of g dz = r0 h/(r0 + h); g = g0 (r0/(r0 + h))^2)
  h = 0 m
  assumes: 0 <= h <= 1000 km; inverse-square gravity with r0 = 6356766 m
```

**What the worked example means:** This calculation determines the geopotential height and local gravitational acceleration for a given geometric altitude. Using an inverse-square gravity model, the result shows that at 0 km, the equivalent potential height in a uniform field is 0 m, while the actual gravity drops to 9.80665 m/s^2. The difference between these heights, 0 m, represents the correction needed when converting from physical distance to energy-based coordinates.

<a id="aero_flight_condition"></a>

#### Mach number, dynamic pressure, equivalent airspeed and Reynolds number at an altitude, `aero_flight_condition`

For a true airspeed at an altitude of the standard atmosphere: the Mach number, dynamic pressure, equivalent airspeed and (with a reference length) the Reynolds number.

$$
M = \frac{V}{\sqrt{\gamma R T}},\quad q = \tfrac12\rho V^2,\quad V_E = V\sqrt{\rho/\rho_0},\quad Re = \frac{\rho V L}{\mu}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `V` | in | true airspeed (m/s, kt, km/h) | m/s |
| `h` | in | geometric altitude (standard atmosphere) | m |
| `L` | in | reference length for the Reynolds number, e.g. the chord (optional) (optional) | m |
| `M` | out | Mach number | - |
| `q` | out | dynamic pressure | Pa |
| `EAS` | out | equivalent airspeed | m/s |
| `EAS_kt` | out | equivalent airspeed, knots | - |
| `a` | out | speed of sound | m/s |
| `rho` | out | air density | kg/m^3 |
| `Re` | out | Reynolds number rho V L/mu (with L) | - |

**Applies when:** V > 0; -1 km <= h <= 47 km.

```shell
$ aerospace-engineering solve aero_flight_condition "V=100 m/s" "h=0 m"
M = 0.2938634484
q = 6124.995779 Pa
EAS = 100 m/s
EAS_kt = 194.3844492
a = 340.2941078 m/s
rho = 1.224999156 kg/m^3
  by: Mach number, dynamic pressure, equivalent airspeed and Reynolds number at an altitude  (M = V/a, a = sqrt(gamma R T); q = rho V^2/2; EAS = V sqrt(rho/rho0); Re = rho V L/mu (Sutherland))
  V = 100 m/s
  h = 0 m
  assumes: V > 0; -1 km <= h <= 47 km
```

**What the worked example means:** At 0 m, the Mach number is 0.293863 and dynamic pressure is 6125 Pa.

<a id="aero_pressure_altitude"></a>

#### Pressure altitude from static pressure: `aero_pressure_altitude`

The pressure altitude: the height in the standard atmosphere at which the standard pressure equals the measured static pressure (what an altimeter set to 1013.25 hPa reads).

$$
H = H_b + \frac{T_b}{L}\left[\left(\frac{p}{p_b}\right)^{-RL/g_0} - 1\right],\quad h = \frac{r_0 H}{r_0 - H}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `p` | in | static pressure (Pa, hPa, inHg, psi) | Pa |
| `h` | out | pressure altitude (geometric) | m |
| `h_ft` | out | pressure altitude in feet | - |
| `H` | out | geopotential pressure altitude | m |
| `T_std` | out | standard temperature at that altitude | K |

**Applies when:** p within the standard atmosphere between -1 km and 47 km.

```shell
$ aerospace-engineering solve aero_pressure_altitude "p=101325 Pa"
h = 0 m
h_ft = 0
H = 0 m
T_std = 288.15 K
  by: Pressure altitude from static pressure  (invert p(H) layer by layer: H = Hb + (T - Tb)/L with T = Tb (p/pb)^(-R L/g0), or H = Hb + (R Tb/g0) ln(pb/p) in an isothermal layer; h = r0 H/(r0 - H))
  p = 101325 Pa
  assumes: p within the standard atmosphere between -1 km and 47 km
```

**What the worked example means:** This calculation determines the pressure altitude, which is the height in the International Standard Atmosphere where the static pressure equals the measured value of 101325 Pa. The result is 0 m, or 0 when converted to feet. This corresponds to a geopotential altitude of 0 m and a standard temperature of 288.15 K. Pressure altitude is critical for flight operations as it represents the reading on an altimeter set to the standard sea level pressure.

<a id="aero_isa_properties"></a>

#### Standard atmosphere at an altitude: `aero_isa_properties`

Temperature, pressure, density, speed of sound and viscosity of the International Standard Atmosphere (US 1976) at a geometric altitude, with the density, pressure and temperature ratios and the geopotential height.

$$
T = T_b + L(H - H_b),\quad p = p_b\left(\frac{T_b}{T}\right)^{g_0/(RL)},\quad \rho = \frac{p}{RT},\quad a = \sqrt{\gamma R T}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `h` | in | geometric altitude above mean sea level (m, km, ft), -1 km to 47 km | m |
| `T` | out | temperature | K |
| `p` | out | pressure | Pa |
| `rho` | out | density | kg/m^3 |
| `a` | out | speed of sound | m/s |
| `mu` | out | dynamic viscosity (Sutherland) | Pa*s |
| `sigma` | out | density ratio rho/rho0 | - |
| `delta` | out | pressure ratio p/p0 | - |
| `theta` | out | temperature ratio T/T0 | - |
| `H` | out | geopotential height | m |

**Applies when:** -1000 m <= h <= 47000 m.

```shell
$ aerospace-engineering solve aero_isa_properties "h=0 m"
T = 288.15 K
p = 101325 Pa
rho = 1.224999156 kg/m^3
a = 340.2941078 m/s
mu = 1.789380278e-05 Pa*s
sigma = 1
delta = 1
theta = 1
H = 0 m
  by: Standard atmosphere at an altitude  (H = r0 h/(r0 + h); in each layer T = Tb + L (H - Hb); p = pb (Tb/T)^(g0/(R L)) (L != 0) or pb exp(-g0 (H - Hb)/(R Tb)); rho = p/(R T); a = sqrt(gamma R T))
  h = 0 m
  assumes: -1000 m <= h <= 47000 m
```

**What the worked example means:** This calculation determines the properties of the International Standard Atmosphere at a specific geometric altitude. Using the input altitude of 0 m, it first computes the geopotential height 0 m to account for Earth's curvature. The method then applies layer-specific lapse rates to find the temperature 288.15 K, which drives the calculation of pressure 101325 Pa via hydrostatic equilibrium. From these, density 1.225 kg/m^3 and speed of sound 340.294 m/s are derived using ideal gas laws. The results include ratios like delta 1, sigma 1, and theta 1 relative to sea level values, providing a complete snapshot of atmospheric conditions for aerospace analysis.

Questions and reports: support@ai999x.com. AI999X, The Future Is Self-Improving.
