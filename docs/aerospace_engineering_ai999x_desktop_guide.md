# Aerospace Engineering: the desktop app, user guide

**Aerospace Engineering**, desktop app 1.6.5 for Windows and Linux. **Creator / engineer:** Ibnul Jaif Farabi · **Company:** AI999X · support@ai999x.com

Aerospace Engineering covers the standard atmosphere, lift and drag, aircraft performance, stability, propulsion and rocket performance, nozzles and orbital transfers. Its desktop app holds all 81 calculations in 13 topics: pick one, change its inputs and the answer, a live scene and charts follow at once. It runs on your own computer with no network connection and nothing else to install. The pictures in this guide were taken in this app.

## Contents

- 1. Get it and start it
- 2. The window
- 3. Compute a calculation
- 4. Ask in words
- 5. The live scene and the charts
- 6. Light and dark
- 7. Every calculation
- 8. Other ways to use it

## 1. Get it and start it

| system | download |
|---|---|
| Windows x86-64 | https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-aerospace-engineering-1.6.5-windows-x86_64.zip |
| Linux x86-64 | https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-aerospace-engineering-1.6.5-linux-x86_64.tar.gz |

Windows: Download ai999x-aerospace-engineering-1.6.5-windows-x86_64.zip, right-click it → Extract All, then open the folder and start ai999x-aerospace-engineering.exe. If Windows SmartScreen asks, choose More info → Run anyway.

Linux:

```shell
curl -LO https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-aerospace-engineering-1.6.5-linux-x86_64.tar.gz
curl -LO https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-aerospace-engineering-1.6.5-linux-x86_64.tar.gz
./ai999x-aerospace-engineering-1.6.5-linux-x86_64/ai999x-aerospace-engineering
```

Checksums of every download: https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/SHA256SUMS. Install steps for every app: https://www.ai999x.com/desktop-cli/

## 2. The window

![The Aerospace Engineering app, opened on Standard atmosphere at an altitude with its worked example.](https://www.ai999x.com/?ax_shot=aerospace-engineering-ai999x/hero.webp)

*The Aerospace Engineering app, opened on Standard atmosphere at an altitude with its worked example.*

On the left, a search box and the topics with their calculations. In the middle, the chosen calculation: its answer at the top, then the live scene (or its charts), what the answer means and the formula. On the right, its inputs, each with a slider where that makes sense, and the Compute and Reset buttons. At the bottom, a box for a question in words.

## 3. Compute a calculation

Open Standard atmosphere › Standard atmosphere at an altitude (or type part of its name in the search box). It opens with a worked example filled in: “ISA properties: h = 0 m” The answer: T = 288.15 K, p = 101325 Pa, rho = 1.224999156 kg/m^3, a = 340.2941078 m/s.

Change any input (type a number with its unit, or drag its slider) and the answer, the scene and the charts follow; Compute recomputes after typing, Reset brings back the worked example. An input outside the range the formula holds for is refused with the reason, never answered with a guessed number.

![Standard atmosphere at an altitude: the live scene of the worked example.](https://www.ai999x.com/?ax_shot=aerospace-engineering-ai999x/scene-1.webp)

*Standard atmosphere at an altitude: the live scene of the worked example.*

## 4. Ask in words

Type a question as you would say it: the app reads the quantities and their units, picks the calculation and fills in its form.

When a question names a quantity the app cannot place, it says what is missing instead of guessing.

## 5. The live scene and the charts

Every calculation draws a scene from its answer: it moves as an input changes. The Charts button shows charts that vary one input around your values; Enlarge opens one in a large view. Scenes from other topics:

![Normal shock relations](https://www.ai999x.com/?ax_shot=aerospace-engineering-ai999x/scene-2.webp)

*Normal shock relations*

![True airspeed from calibrated airspeed](https://www.ai999x.com/?ax_shot=aerospace-engineering-ai999x/scene-3.webp)

*True airspeed from calibrated airspeed*

![Breguet range of a jet aircraft](https://www.ai999x.com/?ax_shot=aerospace-engineering-ai999x/scene-4.webp)

*Breguet range of a jet aircraft*

![Critical Mach number of an airfoil](https://www.ai999x.com/?ax_shot=aerospace-engineering-ai999x/scene-5.webp)

*Critical Mach number of an airfoil*

![Choked mass flow through a nozzle throat](https://www.ai999x.com/?ax_shot=aerospace-engineering-ai999x/scene-6.webp)

*Choked mass flow through a nozzle throat*

![Circular-orbit altitude for an orbital period](https://www.ai999x.com/?ax_shot=aerospace-engineering-ai999x/scene-7.webp)

*Circular-orbit altitude for an orbital period*

![Ideal hover power of a rotor (momentum theory)](https://www.ai999x.com/?ax_shot=aerospace-engineering-ai999x/scene-8.webp)

*Ideal hover power of a rotor (momentum theory)*

![Standard atmosphere at an altitude: a chart, enlarged.](https://www.ai999x.com/?ax_shot=aerospace-engineering-ai999x/chart-1.webp)

*Standard atmosphere at an altitude: a chart, enlarged.*

![Normal shock relations: a chart, enlarged.](https://www.ai999x.com/?ax_shot=aerospace-engineering-ai999x/chart-2.webp)

*Normal shock relations: a chart, enlarged.*

## 6. Light and dark

The app follows a light or a dark theme (the switch is in its top bar).

![The Aerospace Engineering app in the dark theme.](https://www.ai999x.com/?ax_shot=aerospace-engineering-ai999x/hero-dark.webp)

*The Aerospace Engineering app in the dark theme.*

## 7. Every calculation

81 calculations in 13 topics; each has its own entry on the model's page (https://www.ai999x.com/models/aerospace-engineering-ai999x/) with its formula, worked example and charts.

| topic | calculations |
|---|---|
| Aerodynamics | Critical Mach number of an airfoil, Elliptic wing by lifting-line theory, Thin-airfoil theory for a NACA 4-digit airfoil |
| Aircraft performance | Absolute ceiling of a jet aircraft, Best lift coefficients and lift-to-drag ratios of a drag polar, Breguet range of a jet aircraft, Breguet range of a propeller aircraft, Endurance of a jet aircraft, Endurance of a propeller aircraft at constant altitude, Fuel a jet needs for a cruise range, Glide ratio, angle and range, Induced drag in level flight, Jet range at constant altitude and lift coefficient, Landing ground roll, Level coordinated turn at a bank angle, Level turn at a load factor, Lift coefficient in level flight, Lift, drag and lift-to-drag ratio from a drag polar, Maximum rate of climb of a jet, Maximum rate of climb of a propeller aircraft, Service ceiling of a jet aircraft, Speeds for minimum drag, minimum power and best jet range, Stall speed in level flight, Steepest climb angle of a jet, Takeoff ground roll, Time to climb with a linearly decreasing rate of climb, Wind triangle: heading and ground speed, Wing loading |
| Airspeeds | Calibrated airspeed from true airspeed, Mach number from pitot and static pressures, Total (stagnation) air temperature in flight, True airspeed from calibrated airspeed |
| Compressible flow | Fanno flow (adiabatic duct flow with friction), Isentropic flow ratios at a Mach number, Mach number from the area ratio A/A*, Mach number from the Prandtl-Meyer angle, Mach number from the stagnation-to-static pressure ratio, Maximum deflection angle of an attached oblique shock, Normal shock from its pressure ratio, Normal shock relations, Oblique shock from the deflection angle, Oblique shock from the wave angle, Prandtl-Meyer angle of a Mach number, Prandtl-Meyer expansion fan, Rayleigh flow (duct flow with heat addition) |
| Nozzles | Choked mass flow through a nozzle throat, Normal shock position in a converging-diverging nozzle |
| Orbits | Bi-elliptic transfer between circular orbits, Circular orbit at an altitude above the Earth, Circular-orbit altitude for an orbital period, Coverage of a satellite at an altitude, Escape speed at an altitude above the Earth, Inclination of a sun-synchronous orbit, Lambert's problem (orbit through two positions in a given time), Nodal regression from the Earth's oblateness (J2), Rotation of the line of apsides from J2 |
| Propellers and rotors | Ideal hover power of a rotor (momentum theory), Ideal propeller in axial flight (momentum theory) |
| Re-entry | Ballistic coefficient, Peak deceleration of a ballistic re-entry (Allen-Eggers), Speed and deceleration of a ballistic re-entry at an altitude |
| Rocket performance | Burnout height and apogee of a vertical rocket, Delta-v of a multistage rocket, Gravity loss of a vertical rocket burn, Optimal staging of a multistage rocket, Payload fraction of a rocket with identical stages, Propellant mass for a required delta-v, Rocket delta-v from specific impulse (Tsiolkovsky) |
| Rocket propulsion | Characteristic velocity c* of a rocket propellant, Ideal rocket engine performance, Ideal rocket exhaust velocity, Nozzle thrust coefficient, Optimum nozzle area ratio, Throat area of a rocket engine for a required thrust |
| Rockets | Rocket thrust and specific impulse from mass flow and exhaust velocity |
| Stability | Neutral point and static margin (wing and tail) |
| Standard atmosphere | Density altitude from pressure altitude and temperature, Geopotential height from geometric altitude, Mach number, dynamic pressure, equivalent airspeed and Reynolds number at an altitude, Pressure altitude from static pressure, Standard atmosphere at an altitude |

## 8. Other ways to use it

The same engine runs on the command line (`aerospace-engineering`, its own guide), in Python (`pip install aerospace-engineering-ai999x`, `import aerospace_engineering_ai999x`, its own guide) and in the browser on https://www.ai999x.com/models/aerospace-engineering-ai999x/.

Licence: Apache-2.0 · AI999X · https://www.ai999x.com/models/aerospace-engineering-ai999x/
