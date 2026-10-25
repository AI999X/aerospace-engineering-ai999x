"""Aerospace engineering procedures: deterministic capabilities with typed inputs and several outputs (domain code,
loaded by the pack; enabled only once tools/certify_procedures.py has certified them).

Topics: the standard atmosphere (aircraft view), airspeeds and pitot measurements, compressible flow (isentropic flow,
normal and oblique shocks, Prandtl-Meyer expansions, Fanno and Rayleigh flow), nozzles and rocket propulsion, rocket
performance and staging, aircraft performance (drag polar, range and endurance, climb, ceilings, takeoff and landing,
turns), static stability, propellers and rotors (momentum theory), wings and airfoils (thin-airfoil and lifting-line
theory), orbits (circular orbits, escape, bi-elliptic transfers, Lambert's problem, J2 drift) and ballistic re-entry.

Each procedure states its inputs, outputs, conditions, method, LaTeX, the documented question form it parses, worked
examples, an INDEPENDENT reference computation (used only by certification; the checking libraries are imported inside
the reference functions) and a deliberately wrong variant that the checks must reject. The shipped code (run, verify,
parse) is plain Python (math, re).

How the references differ from the shipped closed forms:
- standard atmosphere: the fluids library's own US Standard Atmosphere 1976; inverse problems by bisection on it;
- isentropic flow: the momentum (Euler) equation dp/rho = -V dV integrated numerically along the isentrope and the
  area-velocity relation dA/A = (M^2 - 1) dV/V integrated numerically, instead of the closed-form ratios;
- shocks: the mass, momentum and energy balances across the shock solved numerically (Rankine-Hugoniot), with the
  oblique shock built from its normal and tangential velocity components;
- Prandtl-Meyer, Fanno and Rayleigh flow: numerical integration of their differential relations;
- nozzles: the throat found as the maximum of the mass flux along the isentrope (numerical), the exit state from mass
  conservation, the exhaust velocity from the momentum integral;
- rockets and flight performance: numerical integration of the equations of motion (scipy DOP853 or quadrature),
  numerical optimization (complex-step derivatives and root finding) for best speeds and lift coefficients;
- orbits: hapsira (the maintained successor of poliastro) for circular orbits, bi-elliptic and Hohmann manoeuvres and
  Lambert's problem (Izzo's algorithm); orbit averaging of the J2 perturbation (Gauss equations) by quadrature.

Conventions (fixed, so run and reference agree exactly):
- Standard atmosphere: US Standard Atmosphere 1976 = ISA up to 47 km (as fluids): R* = 8314.32 J/(kmol K),
  M0 = 28.9644 kg/kmol, R = R*/M0 = 287.0531 J/(kg K), gamma = 1.4, g0 = 9.80665 m/s^2, r0 = 6356766 m, sea level
  288.15 K and 101325 Pa, lapse rates -6.5, 0, +1.0, +2.8 K/km from geopotential 0, 11, 20, 32 km, Sutherland
  viscosity 1.458e-6 T^1.5/(T + 110.4). ALTITUDES ARE GEOMETRIC heights above mean sea level (h); the geopotential
  height H = r0 h/(r0 + h) is an output. Pressure altitude and density altitude are the geometric heights in the
  standard atmosphere with the same pressure or density.
- Airspeeds: calibrated airspeed (CAS) by the subsonic compressible pitot formula referred to sea-level standard
  conditions (St Venant); true airspeed (TAS) from the pitot formula at the static pressure of the pressure altitude
  and the outside air temperature (ISA temperature when none is stated).
- Compressible flow: calorically perfect gas of constant gamma; ratios are written stagnation-to-static (p0/p) or
  downstream-to-upstream (p2/p1); angles are given in degrees in the outputs (*_deg).
- Rockets: g0 = 9.80665 m/s^2 for specific impulse; molar gas constant 8.314462618 J/(mol K); frozen, ideal,
  isentropic nozzle flow of constant gamma; impulsive staging.
- Aircraft: parabolic drag polar CD = CD0 + K CL^2 with K = 1/(pi e AR); thrust-specific fuel consumption (TSFC) is
  given as a mass flow per thrust (kg/(N s)); "1/h" or "lb/(lbf h)" is the weight-flow form and is converted (divided
  by g0); power-specific fuel consumption (BSFC) as a mass flow per shaft power (kg/J; g/(kW h) or lb/(hp h)
  converted); weights are forces (N, kN, lbf).
- Earth for orbits: mu = 3.986004418e14 m^3/s^2, equatorial radius R = 6378136.6 m, J2 = 1.08263e-3 (hapsira's
  values); orbit altitudes are above R; the sun-synchronous node rate is 360 degrees per 365.2421897 days.
- Results built from cos, sin, exp or log are rounded to 12 significant digits (identical on every platform); values
  that are zero by geometry are written as exactly 0.

Question form (one per procedure): the capability phrase, a colon, then `name = value` pairs separated by commas
(`ISA properties: h = 11 km`); physical values carry a unit. Aviation units the shared unit reader lacks are converted
here: kt/knots, nmi, fpm, lbf, lb, slug, hPa, mb, inHg, psi, psf, °F, 1/h and lb/(lbf h) for TSFC, lb/(hp h) for BSFC.
Lists (vectors) are written in brackets: `r1 = [7000, 0, 0]` (km). Any other number or an unknown name is refused.
"""
from __future__ import annotations

import math
import re

# ================================================================================================ constants
G0 = 9.80665
R0_GEO = 6356766.0                      # US 1976 effective Earth radius for geopotential height
RU76 = 8314.32                          # US 1976 gas constant, J/(kmol K)
M0_AIR = 28.9644                        # US 1976 molar mass of air, kg/kmol
R_AIR = RU76 / M0_AIR                   # 287.0531 J/(kg K)
GAMMA_AIR = 1.4
T_SL = 288.15
P_SL = 101325.0
RHO_SL = P_SL / (R_AIR * T_SL)
A_SL = math.sqrt(GAMMA_AIR * R_AIR * T_SL)
RU = 8.314462618                        # molar gas constant (CODATA), J/(mol K)
MU_E = 3.986004418e14                   # Earth's gravitational parameter (hapsira)
R_E = 6378136.6                         # Earth's equatorial radius (hapsira)
J2_E = 1.08263e-3                       # Earth's J2 (hapsira)
DAY = 86400.0
SSO_RATE = 2 * math.pi / (365.2421897 * DAY)
DEG = math.pi / 180
KT = 1852.0 / 3600.0
FT = 0.3048
LBF = 4.4482216152605
LB = 0.45359237

# US Standard Atmosphere 1976 layers to 47 km geopotential: base height, base temperature, lapse rate
_HB = [0.0, 11000.0, 20000.0, 32000.0, 47000.0]
_TB = [288.15, 216.65, 216.65, 228.65, 270.65]
_LB = [-0.0065, 0.0, 0.001, 0.0028, 0.0]


def _layer_p(pb, tb, lr, dh):
    if lr == 0.0:
        return pb * math.exp(-G0 * dh / (R_AIR * tb))
    return pb * (tb / (tb + lr * dh)) ** (G0 / (R_AIR * lr))


_PB = [P_SL]
for _i in range(1, len(_HB)):
    _PB.append(_layer_p(_PB[-1], _TB[_i - 1], _LB[_i - 1], _HB[_i] - _HB[_i - 1]))

H_MIN, H_MAX = -1000.0, 47000.0         # geometric altitude range of the atmosphere capabilities


# ================================================================================================ shared helpers
def _r(x):
    """12 significant digits, no negative zero (identical results on every platform and Python version)."""
    if isinstance(x, bool) or isinstance(x, str):
        return x
    if isinstance(x, int):
        return x
    if isinstance(x, (list, tuple)):
        return [_r(v) for v in x]
    if x == 0 or not math.isfinite(x):
        return 0.0 if x == 0 else x
    return float(f"{x:.12g}") + 0.0


def _out(**kw):
    return {k: _r(v) for k, v in kw.items()}


def _rel(a, b):
    if isinstance(a, (list, tuple)) or isinstance(b, (list, tuple)):
        if not (isinstance(a, (list, tuple)) and isinstance(b, (list, tuple))) or len(a) != len(b):
            return math.inf
        return max([_rel(x, y) for x, y in zip(a, b)] + [0.0])
    if isinstance(a, (str, bool)) or isinstance(b, (str, bool)):
        return 0.0 if a == b else math.inf
    try:
        a, b = float(a), float(b)
    except (TypeError, ValueError):
        return math.inf
    d = abs(a - b)
    if d == 0:
        return 0.0
    s = max(abs(a), abs(b))
    return d / s if s > 0 else math.inf


def _recheck(fn):
    """A verifier: every output the procedure gives is recomputed and compared (relative); a missing output fails."""
    def verify(inputs, outputs):
        try:
            want = fn(**inputs)
        except Exception:
            return math.inf
        worst = 0.0
        for k, v in want.items():
            if k not in outputs:
                return math.inf
            worst = max(worst, _rel(outputs[k], v))
        return worst
    return verify


def _safe(fn):
    """Guard a wrong variant against exceptions (a variant that raises cannot be compared)."""
    def g(**kw):
        try:
            return fn(**kw)
        except Exception:
            return {"_failed": True}
    return g


def _lu(rng, a, b):
    return math.exp(rng.uniform(math.log(a), math.log(b)))


def _pos(name, *vals):
    for v in vals:
        if not (isinstance(v, (int, float)) and math.isfinite(v) and v > 0):
            raise ValueError(f"{name} must be a positive number")


def _need(ok, msg):
    if not ok:
        raise ValueError(msg)


def _gam(gamma):
    _need(isinstance(gamma, (int, float)) and 1.05 <= gamma <= 1.7, "the ratio of specific heats gamma must lie between 1.05 and 1.7")
    return float(gamma)


def _bisect(f, a, b, n=200):
    """Root of f on [a, b] by bisection (f(a), f(b) of opposite signs), to full double precision."""
    fa = f(a)
    fb = f(b)
    if fa == 0:
        return a
    if fb == 0:
        return b
    if (fa > 0) == (fb > 0):
        raise ValueError("no root in the interval")
    for _ in range(n):
        m = 0.5 * (a + b)
        if m == a or m == b:
            break
        fm = f(m)
        if fm == 0:
            return m
        if (fm > 0) == (fa > 0):
            a, fa = m, fm
        else:
            b = m
    return 0.5 * (a + b)


# units the shared unit reader does not know: converted here to SI (factor, SI unit) or by a function
_TSFC_SI = "kg/(N*s)"
_BSFC_SI = "kg/J"
_CONV = {
    "kt": (KT, "m/s"), "kts": (KT, "m/s"), "knot": (KT, "m/s"), "knots": (KT, "m/s"), "KTAS": (KT, "m/s"),
    "KCAS": (KT, "m/s"), "KEAS": (KT, "m/s"), "fpm": (FT / 60, "m/s"), "ft/min": (FT / 60, "m/s"),
    "nmi": (1852.0, "m"), "NM": (1852.0, "m"), "nm_naut": (1852.0, "m"),
    "lbf": (LBF, "N"), "klbf": (1000 * LBF, "N"), "kgf": (G0, "N"),
    "lb": (LB, "kg"), "lbm": (LB, "kg"), "lbs": (LB, "kg"), "slug": (LBF / FT, "kg"), "slugs": (LBF / FT, "kg"),
    "hPa": (100.0, "Pa"), "mb": (100.0, "Pa"), "inHg": (3386.389, "Pa"), "psi": (LBF / 0.0254 ** 2, "Pa"),
    "psia": (LBF / 0.0254 ** 2, "Pa"), "psf": (LBF / FT ** 2, "Pa"), "lbf/ft^2": (LBF / FT ** 2, "Pa"),
    "lb/ft^2": (LBF / FT ** 2, "Pa"), "ft^2": (FT ** 2, "m^2"), "sq ft": (FT ** 2, "m^2"),
    # TSFC: the weight-flow forms (1/h, lb/(lbf h)) are the mass-flow form times g0
    "1/h": (1 / (3600 * G0), _TSFC_SI), "/h": (1 / (3600 * G0), _TSFC_SI), "1/hr": (1 / (3600 * G0), _TSFC_SI),
    "h^-1": (1 / (3600 * G0), _TSFC_SI), "1/s": (1 / G0, _TSFC_SI), "/s": (1 / G0, _TSFC_SI),
    "lb/(lbf*h)": (1 / (3600 * G0), _TSFC_SI), "lb/lbf/h": (1 / (3600 * G0), _TSFC_SI),
    "lbm/(lbf*h)": (1 / (3600 * G0), _TSFC_SI), "lb/(lbf*hr)": (1 / (3600 * G0), _TSFC_SI),
    # BSFC
    "lb/(hp*h)": (LB / (745.69987158227022 * 3600), _BSFC_SI), "lb/hp/h": (LB / (745.69987158227022 * 3600), _BSFC_SI),
    "lb/(hp*hr)": (LB / (745.69987158227022 * 3600), _BSFC_SI),
}
_TEMP = {"°F": lambda v: (v - 32) * 5 / 9 + 273.15, "degF": lambda v: (v - 32) * 5 / 9 + 273.15,
         "F": None}
_NUM = r"[-+]?(?:\d+(?:\.\d*)?|\.\d+)(?:[eE][-+]?\d+)?"
_SEG = re.compile(r"^\s*(?:and\s+)?([A-Za-z][A-Za-z_0-9/*]*)\s*=\s*(" + _NUM + r")\s*(.*?)\s*$")
_WORD = re.compile(r"^\s*(?:and\s+)?([A-Za-z][A-Za-z_0-9/*]*)\s*=\s*([A-Za-z][A-Za-z_ \-]*?)\s*$")
_VEC = re.compile(r"^\s*(?:and\s+)?([A-Za-z][A-Za-z_0-9/*]*)\s*=\s*\[([^\]]*)\]\s*(.*?)\s*$")


def _name(names, written):
    """The symbol a written name stands for: the exact spelling first (M and m may differ), then lower case."""
    return names.get(written, names.get(written.lower()))


def _split(rest):
    """Split on commas and semicolons outside brackets."""
    parts, depth, cur = [], 0, ""
    for ch in rest:
        if ch == "[":
            depth += 1
        elif ch == "]":
            depth -= 1
        if ch in ",;" and depth == 0:
            parts.append(cur)
            cur = ""
        else:
            cur += ch
    parts.append(cur)
    return parts


def _pairs(question, phrase, names, phys=(), words=(), vectors=(), num_units=None):
    """{symbol: value} from '<phrase>: name = value, ...'. None unless the phrase opens the question and every part is
    a 'name = value' pair of a known name. `phys`: symbols that need a unit (returned as 'value unit' strings, in SI
    when the unit is one converted here); `words`: symbols whose value is a word (a choice); `vectors`: symbols whose
    value is a bracketed list; `num_units`: {symbol: [allowed unit words]} for plain numbers."""
    if not isinstance(question, str):
        return None
    m = re.match(r"\s*(?:" + phrase + r")\s*:\s*", question, re.I)
    if not m:
        return None
    rest = re.sub(r"[.?!]\s*$", "", question[m.end():].strip()).strip()
    if not rest:
        return {}
    out = {}
    num_units = num_units or {}
    for seg in _split(rest):
        if not seg.strip():
            continue
        mv = _VEC.match(seg)
        if mv:
            key = _name(names, mv.group(1))
            if key is None or key in out or key not in set(vectors) or mv.group(3):
                return None
            out[key] = mv.group(2).strip()
            continue
        mw = _WORD.match(seg)
        if mw and _name(names, mw.group(1)) in set(words):
            key = _name(names, mw.group(1))
            if key in out:
                return None
            out[key] = mw.group(2).strip().lower()
            continue
        ms = _SEG.match(seg)
        if not ms:
            return None
        key = _name(names, ms.group(1))
        if key is None or key in out or key in words or key in vectors:
            return None
        num, unit = ms.group(2), ms.group(3).strip()
        if key in phys:
            if not unit:
                return None
            unit_c = unit.replace(" ", "") if unit not in _CONV else unit
            if unit in _TEMP or unit_c in _TEMP:
                f = _TEMP.get(unit, _TEMP.get(unit_c))
                if f is None:
                    return None
                out[key] = f"{f(float(num))!r} K"
            elif unit in _CONV or unit_c in _CONV:
                fac, si = _CONV.get(unit, _CONV.get(unit_c))
                out[key] = f"{float(num) * fac!r} {si}"
            else:
                out[key] = f"{num} {unit}"
        else:
            if unit and unit not in num_units.get(key, ()):
                return None
            out[key] = float(num)
    return out


def _phys(sym, kind, meaning, required=True):
    return {"symbol": sym, "kind": kind, "meaning": meaning, "required": required}


def _numin(sym, meaning, required=True, **extra):
    return {"symbol": sym, "kind": "number", "meaning": meaning, "required": required, **extra}


def _intin(sym, meaning, lo=None, hi=None, required=True):
    d = {"symbol": sym, "type": "integer", "meaning": meaning, "required": required}
    if lo is not None:
        d["min"] = lo
    if hi is not None:
        d["max"] = hi
    return d


def _vecin(sym, meaning, lo=1, hi=10, required=True):
    return {"symbol": sym, "type": "vector", "kind": "number", "meaning": meaning, "required": required,
            "min_length": lo, "max_length": hi}


def _choice(sym, choices, meaning, required=True):
    return {"symbol": sym, "type": "choice", "choices": list(choices), "meaning": meaning, "required": required}


def _o(sym, kind, meaning):
    return {"symbol": sym, "kind": kind, "meaning": meaning}


PROCEDURES: dict = {}


def _add(key, **spec):
    spec.setdefault("conditions", [])
    spec.setdefault("verify", _recheck(spec["run"]))
    PROCEDURES[key] = spec


_GAMMA_IN = _numin("gamma", "ratio of specific heats (default 1.4, air)", required=False)
_GNAMES = {"gamma": "gamma", "k": "gamma"}


# ================================================================================================ standard atmosphere
def _geopot(h):
    return R0_GEO * h / (R0_GEO + h)


def _geom(H):
    return R0_GEO * H / (R0_GEO - H)


def _layer(H):
    if H <= 0.0:
        return 0
    for i, hb in enumerate(_HB):
        if hb >= H:
            return i - 1
    return len(_HB) - 1


def _isa_H(H):
    """(T, p) at geopotential height H."""
    i = _layer(H)
    T = _TB[i] + _LB[i] * (H - _HB[i])
    return T, _layer_p(_PB[i], _TB[i], _LB[i], H - _HB[i])


def _isa(h):
    """(T, p, rho) at geometric altitude h."""
    _need(isinstance(h, (int, float)) and H_MIN <= h <= H_MAX, "the altitude must lie between -1 km and 47 km (standard atmosphere layers to 47 km)")
    T, p = _isa_H(_geopot(h))
    return T, p, p / (R_AIR * T)


def _sutherland(T):
    return 1.458e-6 * T * math.sqrt(T) / (T + 110.4)


def _H_from_p(p):
    """Geopotential height at which the standard pressure is p."""
    i = 0
    while i + 1 < len(_HB) and p < _PB[i + 1]:
        i += 1
    if _LB[i] == 0.0:
        return _HB[i] + R_AIR * _TB[i] / G0 * math.log(_PB[i] / p)
    T = _TB[i] * (p / _PB[i]) ** (-R_AIR * _LB[i] / G0)
    return _HB[i] + (T - _TB[i]) / _LB[i]


def _H_from_rho(rho):
    """Geopotential height at which the standard density is rho."""
    rb = [_PB[i] / (R_AIR * _TB[i]) for i in range(len(_HB))]
    i = 0
    while i + 1 < len(_HB) and rho < rb[i + 1]:
        i += 1
    if _LB[i] == 0.0:
        return _HB[i] + R_AIR * _TB[i] / G0 * math.log(rb[i] / rho)
    n = G0 / (R_AIR * _LB[i])
    T = _TB[i] * (rho / rb[i]) ** (-1.0 / (n + 1.0))
    return _HB[i] + (T - _TB[i]) / _LB[i]


P_MIN = _isa(H_MAX)[1]
P_MAX = _isa(H_MIN)[1]
RHO_MIN = _isa(H_MAX)[2]
RHO_MAX = _isa(H_MIN)[2]


def _fluids_atm(h, dT=0.0):
    from fluids.atmosphere import ATMOSPHERE_1976
    return ATMOSPHERE_1976(h, dT)


def _ref_altitude_where(fn, target, lo=H_MIN - 500.0, hi=H_MAX + 1000.0):
    """Geometric altitude where fn(fluids atmosphere) equals target (decreasing with height), by brentq."""
    from scipy.optimize import brentq
    return brentq(lambda z: fn(_fluids_atm(z)) - target, lo, hi, xtol=1e-13, rtol=8.9e-16, maxiter=500)


def isa_properties(h):
    T, p, rho = _isa(h)
    return _out(T=T, p=p, rho=rho, a=math.sqrt(GAMMA_AIR * R_AIR * T), mu=_sutherland(T), sigma=rho / RHO_SL,
                delta=p / P_SL, theta=T / T_SL, H=_geopot(h))


def ref_isa_properties(h):
    a, a0 = _fluids_atm(h), _fluids_atm(0.0)
    return {"T": a.T, "p": a.P, "rho": a.rho, "a": a.v_sonic, "mu": a.mu, "sigma": a.rho / a0.rho,
            "delta": a.P / a0.P, "theta": a.T / a0.T, "H": a.H}


def _isa_wrong(h):
    T, p = _isa_H(h)                                   # geometric altitude used as if it were geopotential
    rho = p / (287.0 * T)
    return _out(T=T, p=p, rho=rho, a=math.sqrt(1.4 * 287.0 * T), mu=_sutherland(T), sigma=rho / RHO_SL, delta=p / P_SL,
                theta=T / T_SL, H=h)


_ALT_NAMES = {"h": "h", "altitude": "h", "z": "h"}

_add("aero_isa_properties",
     name="Standard atmosphere at an altitude", topic="standard atmosphere / ISA properties",
     purpose="Temperature, pressure, density, speed of sound and viscosity of the International Standard Atmosphere (US 1976) at a geometric altitude, with the density, pressure and temperature ratios and the geopotential height.",
     inputs=[_phys("h", "length", "geometric altitude above mean sea level (m, km, ft), -1 km to 47 km")],
     outputs=[_o("T", "temperature", "temperature"), _o("p", "pressure", "pressure"), _o("rho", "density", "density"),
              _o("a", "speed", "speed of sound"), _o("mu", "dynamic_viscosity", "dynamic viscosity (Sutherland)"),
              _o("sigma", "number", "density ratio rho/rho0"), _o("delta", "number", "pressure ratio p/p0"),
              _o("theta", "number", "temperature ratio T/T0"), _o("H", "length", "geopotential height")],
     primary="rho", conditions=["-1000 m <= h <= 47000 m"],
     method="H = r0 h/(r0 + h); in each layer T = Tb + L (H - Hb); p = pb (Tb/T)^(g0/(R L)) (L != 0) or pb exp(-g0 (H - Hb)/(R Tb)); rho = p/(R T); a = sqrt(gamma R T)",
     latex=r"T = T_b + L(H - H_b),\quad p = p_b\left(\frac{T_b}{T}\right)^{g_0/(RL)},\quad \rho = \frac{p}{RT},\quad a = \sqrt{\gamma R T}",
     run=isa_properties,
     parse=lambda q: _pairs(q, r"isa properties|standard atmosphere(?: at (?:an )?altitude)?|isa", _ALT_NAMES, phys={"h"}),
     reference=ref_isa_properties,
     wrong=_safe(_isa_wrong),
     random_inputs=lambda rng: {"h": rng.uniform(H_MIN, H_MAX)},
     templates=["ISA properties: h = 11 km", "Standard atmosphere at altitude: h = 35000 ft", "ISA: h = 5000 m"],
     examples=[{"inputs": {"h": 0.0}, "outputs": {"T": 288.15, "p": 101325.0, "rho": 101325.0 / (R_AIR * 288.15), "sigma": 1.0,
                                                  "a": math.sqrt(1.4 * R_AIR * 288.15), "mu": 1.458e-6 * 288.15 ** 1.5 / 398.55}},
               {"inputs": {"h": _geom(11000.0)}, "outputs": {"T": 216.65, "p": P_SL * (216.65 / 288.15) ** (G0 / (R_AIR * 0.0065)), "H": 11000.0}},
               {"inputs": {"h": _geom(5000.0)}, "outputs": {"T": 255.65, "theta": 255.65 / 288.15}}],
     questions=[{"q": "ISA properties: h = 0 m", "outputs": {"T": 288.15, "p": 101325.0}},
                {"q": "Standard atmosphere at altitude: h = 15 km", "outputs": {"T": 216.65}},
                {"q": "ISA properties: h = 60 km", "outcome": "INVALID_INPUT"},
                {"q": "ISA properties: h = 3000", "outcome": "UNRECOGNIZED_FORMAT"},
                {"q": "ISA properties:", "outcome": "NEEDS_INPUT"}])


def pressure_altitude(p):
    _pos("the static pressure", p)
    _need(P_MIN <= p <= P_MAX, "the pressure must lie within the standard atmosphere from -1 km to 47 km")
    H = _H_from_p(p)
    h = _geom(H)
    return _out(h=h, h_ft=h / FT, H=H, T_std=_isa_H(H)[0])


def ref_pressure_altitude(p):
    h = _ref_altitude_where(lambda a: a.P, p)
    a = _fluids_atm(h)
    return {"h": h, "h_ft": h / FT, "H": a.H, "T_std": a.T}


def _pa_wrong(p):
    T = T_SL * (p / P_SL) ** (R_AIR * 0.0065 / G0)    # the tropospheric lapse rate used at every height
    H = (T_SL - T) / 0.0065
    h = _geom(H)
    return _out(h=h, h_ft=h / FT, H=H, T_std=T)


_add("aero_pressure_altitude",
     name="Pressure altitude from static pressure", topic="standard atmosphere / pressure altitude",
     purpose="The pressure altitude: the height in the standard atmosphere at which the standard pressure equals the measured static pressure (what an altimeter set to 1013.25 hPa reads).",
     inputs=[_phys("p", "pressure", "static pressure (Pa, hPa, inHg, psi)")],
     outputs=[_o("h", "length", "pressure altitude (geometric)"), _o("h_ft", "number", "pressure altitude in feet"),
              _o("H", "length", "geopotential pressure altitude"), _o("T_std", "temperature", "standard temperature at that altitude")],
     primary="h", conditions=["p within the standard atmosphere between -1 km and 47 km"],
     method="invert p(H) layer by layer: H = Hb + (T - Tb)/L with T = Tb (p/pb)^(-R L/g0), or H = Hb + (R Tb/g0) ln(pb/p) in an isothermal layer; h = r0 H/(r0 - H)",
     latex=r"H = H_b + \frac{T_b}{L}\left[\left(\frac{p}{p_b}\right)^{-RL/g_0} - 1\right],\quad h = \frac{r_0 H}{r_0 - H}",
     run=pressure_altitude,
     parse=lambda q: _pairs(q, r"pressure altitude(?: from (?:static )?pressure)?", {"p": "p", "pressure": "p", "ps": "p"}, phys={"p"}),
     reference=ref_pressure_altitude,
     wrong=_safe(_pa_wrong),
     random_inputs=lambda rng: {"p": _lu(rng, P_MIN * 1.0001, P_MAX * 0.9999)},
     templates=["Pressure altitude: p = 500 hPa", "Pressure altitude from static pressure: p = 29.92 inHg", "Pressure altitude: p = 23.84 kPa"],
     examples=[{"inputs": {"p": 101325.0}, "outputs": {"h": 0.0, "T_std": 288.15}},
               {"inputs": {"p": _PB[1]}, "outputs": {"H": 11000.0, "h": _geom(11000.0), "T_std": 216.65}},
               {"inputs": {"p": _PB[2]}, "outputs": {"H": 20000.0}}],
     questions=[{"q": "Pressure altitude: p = 1013.25 hPa", "outputs": {"h": 0.0}},
                {"q": "Pressure altitude: p = 101325 Pa", "outputs": {"h_ft": 0.0}},
                {"q": "Pressure altitude: p = 0.5 hPa", "outcome": "INVALID_INPUT"},
                {"q": "Pressure altitude: p = 500", "outcome": "UNRECOGNIZED_FORMAT"},
                {"q": "Pressure altitude:", "outcome": "NEEDS_INPUT"}])


def density_altitude(hp, T):
    _pos("the outside air temperature", T)
    _need(150.0 <= T <= 350.0, "the outside air temperature must lie between 150 K and 350 K")
    _Tstd, p, _rho = _isa(hp)
    rho = p / (R_AIR * T)
    _need(RHO_MIN <= rho <= RHO_MAX, "the density lies outside the standard atmosphere from -1 km to 47 km")
    H = _H_from_rho(rho)
    h = _geom(H)
    return _out(rho=rho, sigma=rho / RHO_SL, h_rho=h, h_rho_ft=h / FT, p=p)


def ref_density_altitude(hp, T):
    from fluids.atmosphere import ATMOSPHERE_1976
    p = _fluids_atm(hp).P
    rho = ATMOSPHERE_1976.density(T, p)
    h = _ref_altitude_where(lambda a: a.rho, rho)
    return {"rho": rho, "sigma": rho / _fluids_atm(0.0).rho, "h_rho": h, "h_rho_ft": h / FT, "p": p}


def _da_wrong(hp, T):
    Tstd, p, rho = _isa(hp)                            # the stated temperature ignored (standard day assumed)
    H = _H_from_rho(rho)
    return _out(rho=rho, sigma=rho / RHO_SL, h_rho=_geom(H), h_rho_ft=_geom(H) / FT, p=p)


def _da_random(rng):
    hp = rng.uniform(0.0, 12000.0)
    return {"hp": hp, "T": _isa(hp)[0] + rng.choice([-1, 1]) * rng.uniform(2.0, 30.0)}


_add("aero_density_altitude",
     name="Density altitude from pressure altitude and temperature", topic="standard atmosphere / density altitude",
     purpose="Air density and density altitude (the standard-atmosphere height with the same density) from the pressure altitude and the outside air temperature.",
     inputs=[_phys("hp", "length", "pressure altitude (m, ft)"), _phys("T", "temperature", "outside air temperature (K, °C, °F)")],
     outputs=[_o("rho", "density", "air density"), _o("sigma", "number", "density ratio rho/rho0"),
              _o("h_rho", "length", "density altitude (geometric)"), _o("h_rho_ft", "number", "density altitude in feet"),
              _o("p", "pressure", "static pressure at the pressure altitude")],
     primary="h_rho", conditions=["150 K <= T <= 350 K", "-1 km <= hp <= 47 km", "the density within the standard atmosphere"],
     method="p = p_std(hp); rho = p/(R T); density altitude = the standard height where rho_std(h) = rho (inverted layer by layer)",
     latex=r"\rho = \frac{p_{std}(h_p)}{R\,T},\quad \rho_{std}(h_\rho) = \rho",
     run=density_altitude,
     parse=lambda q: _pairs(q, r"density altitude(?: from pressure altitude and temperature)?",
                            {"hp": "hp", "pressure_altitude": "hp", "h": "hp", "t": "T", "oat": "T", "temperature": "T"}, phys={"hp", "T"}),
     reference=ref_density_altitude,
     wrong=_safe(_da_wrong),
     random_inputs=_da_random,
     templates=["Density altitude: hp = 5000 ft, T = 30 °C", "Density altitude from pressure altitude and temperature: hp = 1500 m, T = 25 degC"],
     examples=[{"inputs": {"hp": 0.0, "T": 288.15}, "outputs": {"h_rho": 0.0, "sigma": 1.0}},
               {"inputs": {"hp": _geom(11000.0), "T": 216.65}, "outputs": {"h_rho": _geom(11000.0)}},
               {"inputs": {"hp": 0.0, "T": 300.0}, "outputs": {"rho": P_SL / (R_AIR * 300.0)}}],
     questions=[{"q": "Density altitude: hp = 0 ft, T = 15 °C", "outputs": {"h_rho": 0.0}},
                {"q": "Density altitude: hp = 0 m, T = 300 K", "outputs": {"rho": P_SL / (R_AIR * 300.0)}},
                {"q": "Density altitude: hp = 2000 m, T = 500 K", "outcome": "INVALID_INPUT"},
                {"q": "Density altitude: hp = 2000 m", "outcome": "NEEDS_INPUT"}])


def geopotential_altitude(h):
    _need(isinstance(h, (int, float)) and 0 <= h <= 1.0e6, "the altitude must lie between 0 and 1000 km")
    x = R0_GEO / (R0_GEO + h)
    return _out(H=R0_GEO * h / (R0_GEO + h), g=G0 * x * x, dH=h - R0_GEO * h / (R0_GEO + h))


def ref_geopotential_altitude(h):
    from scipy.integrate import quad
    from fluids.atmosphere import ATMOSPHERE_1976
    H, _ = quad(lambda z: (R0_GEO / (R0_GEO + z)) ** 2, 0.0, h, epsabs=0, epsrel=2e-14, limit=200)   # H = (1/g0) int g dz
    return {"H": H, "g": ATMOSPHERE_1976.gravity(h), "dH": h - H}


_add("aero_geopotential_altitude",
     name="Geopotential height from geometric altitude", topic="standard atmosphere / geopotential height",
     purpose="The geopotential height (the height in a uniform gravity field g0 with the same potential energy) and the local gravity at a geometric altitude.",
     inputs=[_phys("h", "length", "geometric altitude (m, km, ft), 0 to 1000 km")],
     outputs=[_o("H", "length", "geopotential height"), _o("g", "acceleration", "gravitational acceleration at the altitude"),
              _o("dH", "length", "geometric minus geopotential height")],
     primary="H", conditions=["0 <= h <= 1000 km", "inverse-square gravity with r0 = 6356766 m"],
     method="H = (1/g0) integral of g dz = r0 h/(r0 + h); g = g0 (r0/(r0 + h))^2",
     latex=r"H = \frac{1}{g_0}\int_0^h g\,dz = \frac{r_0 h}{r_0 + h},\quad g = g_0\left(\frac{r_0}{r_0 + h}\right)^2",
     run=geopotential_altitude,
     parse=lambda q: _pairs(q, r"geopotential (?:height|altitude)(?: from geometric altitude)?", _ALT_NAMES, phys={"h"}),
     reference=ref_geopotential_altitude,
     wrong=_safe(lambda h: _out(H=R0_GEO * h / (R0_GEO - h), g=G0 * (R0_GEO / (R0_GEO + h)) ** 2, dH=h - R0_GEO * h / (R0_GEO - h))),
     random_inputs=lambda rng: {"h": _lu(rng, 100.0, 1.0e6)},
     templates=["Geopotential height: h = 20 km", "Geopotential altitude from geometric altitude: h = 100000 ft"],
     examples=[{"inputs": {"h": 0.0}, "outputs": {"H": 0.0, "g": G0}},
               {"inputs": {"h": R0_GEO / 9}, "outputs": {"H": R0_GEO / 10, "g": G0 * 0.81}}],
     questions=[{"q": "Geopotential height: h = 0 km", "outputs": {"H": 0.0}},
                {"q": "Geopotential height: h = " + repr(R0_GEO / 9) + " m", "outputs": {"H": R0_GEO / 10}},
                {"q": "Geopotential height: h = -5 km", "outcome": "INVALID_INPUT"},
                {"q": "Geopotential height:", "outcome": "NEEDS_INPUT"}])


# ================================================================================================ isentropic-flow references
_LEG = {}


def _gl(f, a, b, n=96):
    """Gauss-Legendre quadrature of a vectorized f on [a, b] (reference side only)."""
    import numpy as np
    if n not in _LEG:
        _LEG[n] = np.polynomial.legendre.leggauss(n)
    x, w = _LEG[n]
    t = 0.5 * (b - a) * x + 0.5 * (b + a)
    return 0.5 * (b - a) * float(np.dot(w, f(t)))


def _ref_I(x, g):
    """Euler's equation along an isentrope: (rho0/p0) * integral of dp/rho from p = x p0 to p0, i.e. the integral of
    s^(-1/g) ds over (x, 1), computed numerically in t = ln s (V^2 / 2 = (p0/rho0) I)."""
    import numpy as np
    c = 1.0 - 1.0 / g
    return _gl(lambda t: np.exp(c * t), math.log(x), 0.0)


def _ref_J(y, g):
    """(rho/p) * integral of dp'/rho' from a static pressure p to y p along an isentrope (y >= 1): the integral of
    s^(-1/g) ds over (1, y) in t = ln s."""
    import numpy as np
    c = 1.0 - 1.0 / g
    return _gl(lambda t: np.exp(c * t), 0.0, math.log(y))


def _ref_M2_of_x(x, g):
    """Mach number squared at static/stagnation pressure ratio x: V^2 from the Euler integral, a^2 = g p/rho."""
    return 2.0 * _ref_I(x, g) / (g * x ** (1.0 - 1.0 / g))


def _ref_x_of_M(M, g):
    """Static-to-stagnation pressure ratio p/p0 at Mach M (root of the Euler-integral Mach number)."""
    from scipy.optimize import brentq
    if M == 0:
        return 1.0
    return math.exp(brentq(lambda lx: _ref_M2_of_x(math.exp(lx), g) - M * M, -120.0, -1e-300, xtol=1e-15, rtol=8.9e-16, maxiter=500))


def _ref_lnA(M, g):
    """ln(A/A*) from the area-velocity relation dA/A = (M^2 - 1)/(1 + (g-1)/2 M^2) dM/M integrated in u = ln M."""
    import numpy as np
    k = 0.5 * (g - 1.0)
    return _gl(lambda u: (np.exp(2 * u) - 1.0) / (1.0 + k * np.exp(2 * u)), 0.0, math.log(M))


def _ref_M_of_area(ar, g, supersonic):
    from scipy.optimize import brentq
    la = math.log(ar)
    if supersonic:
        return math.exp(brentq(lambda u: _ref_lnA(math.exp(u), g) - la, 0.0, math.log(200.0), xtol=1e-15, rtol=8.9e-16, maxiter=500))
    return math.exp(brentq(lambda u: _ref_lnA(math.exp(u), g) - la, math.log(1e-8), 0.0, xtol=1e-15, rtol=8.9e-16, maxiter=500))


def _ref_shock(M1, g):
    """Normal shock from the mass, momentum and energy balances (upstream rho = p = 1): returns rho2, V2, p2, V1."""
    from scipy.optimize import brentq
    V1 = M1 * math.sqrt(g)
    c = g / (g - 1.0)
    # energy balance with V2 = V1/rho2 (mass) and p2 = 1 + V1^2 - V1^2/rho2 (momentum), multiplied by rho2^2
    hfun = lambda r: c * ((1.0 + V1 * V1) * r - V1 * V1) + 0.5 * V1 * V1 - (c + 0.5 * V1 * V1) * r * r   # noqa: E731
    r2 = brentq(hfun, 1.0 + 1e-3 * (M1 * M1 - 1.0) / (M1 * M1), (g + 1.0) / (g - 1.0), xtol=1e-15, rtol=8.9e-16, maxiter=500)
    V2 = V1 / r2
    p2 = 1.0 + V1 * V1 - V1 * V2
    return r2, V2, p2, V1


def _ref_shock_all(M1, g):
    r2, V2, p2, _V1 = _ref_shock(M1, g)
    M2 = V2 / math.sqrt(g * p2 / r2)
    x1, x2 = _ref_x_of_M(M1, g), _ref_x_of_M(M2, g)
    return {"M2": M2, "p2_p1": p2, "T2_T1": p2 / r2, "rho2_rho1": r2, "p02_p01": p2 * x1 / x2, "p02_p1": p2 / x2}


# ================================================================================================ closed forms (shipped)
def _t0t(M, g):
    return 1.0 + 0.5 * (g - 1.0) * M * M


def _p0p(M, g):
    return _t0t(M, g) ** (g / (g - 1.0))


def _area(M, g):
    return (2.0 / (g + 1.0) * _t0t(M, g)) ** ((g + 1.0) / (2.0 * (g - 1.0))) / M


def _M_of_area(ar, g, supersonic):
    if ar == 1.0:
        return 1.0
    if supersonic:
        hi = 2.0
        while _area(hi, g) < ar:
            hi *= 2.0
        return _bisect(lambda M: _area(M, g) - ar, 1.0, hi)
    lo = 0.5
    while _area(lo, g) < ar:
        lo *= 0.5
    return _bisect(lambda M: _area(M, g) - ar, lo, 1.0)


def _shock(M1, g):
    m2 = M1 * M1
    p21 = 1.0 + 2.0 * g / (g + 1.0) * (m2 - 1.0)
    r21 = (g + 1.0) * m2 / ((g - 1.0) * m2 + 2.0)
    M2 = math.sqrt((1.0 + 0.5 * (g - 1.0) * m2) / (g * m2 - 0.5 * (g - 1.0)))
    p0r = p21 * _p0p(M2, g) / _p0p(M1, g)
    return M2, p21, r21, p0r


# ================================================================================================ airspeeds
def _static(hp, T):
    Tstd, p, _rho = _isa(hp)
    T = Tstd if T is None else T
    _need(150.0 <= T <= 350.0, "the outside air temperature must lie between 150 K and 350 K")
    return p, T


def calibrated_airspeed(V, hp, T=None):
    _pos("the true airspeed", V)
    p, T = _static(hp, T)
    M = V / math.sqrt(GAMMA_AIR * R_AIR * T)
    _need(M < 1.0, "the flight Mach number must be subsonic (below 1) for the subsonic pitot formula")
    qc = p * (_p0p(M, GAMMA_AIR) - 1.0)
    cas = A_SL * math.sqrt(5.0 * ((qc / P_SL + 1.0) ** (2.0 / 7.0) - 1.0))
    _need(cas < A_SL, "the calibrated airspeed would exceed the sea-level speed of sound")
    return _out(CAS=cas, CAS_kt=cas / KT, qc=qc, pt=p + qc)


def _ref_y_from_V(V, p, rho, g=GAMMA_AIR):
    """Total-to-static pressure ratio from V^2/2 = integral of dp/rho along the isentrope (numerical)."""
    from scipy.optimize import brentq
    target = 0.5 * V * V * rho / p
    return math.exp(brentq(lambda ly: _ref_J(math.exp(ly), g) - target, 0.0, 20.0, xtol=1e-15, rtol=8.9e-16, maxiter=500))


def ref_calibrated_airspeed(V, hp, T=None):
    from fluids.atmosphere import ATMOSPHERE_1976
    a = _fluids_atm(hp)
    T = a.T if T is None else T
    p = a.P
    rho = ATMOSPHERE_1976.density(T, p)
    qc = p * (_ref_y_from_V(V, p, rho) - 1.0)
    a0 = _fluids_atm(0.0)
    cas = math.sqrt(2.0 * a0.P / a0.rho * _ref_J(1.0 + qc / a0.P, GAMMA_AIR))
    return {"CAS": cas, "CAS_kt": cas / KT, "qc": qc, "pt": p + qc}


def _cas_wrong(V, hp, T=None):
    p, T = _static(hp, T)
    rho = p / (R_AIR * T)
    eas = V * math.sqrt(rho / RHO_SL)                  # the equivalent airspeed taken for the calibrated airspeed
    q = 0.5 * rho * V * V
    return _out(CAS=eas, CAS_kt=eas / KT, qc=q, pt=p + q)


def _cas_random(rng):
    hp = rng.uniform(0.0, 13000.0)
    T = None if rng.random() < 0.5 else _isa(hp)[0] + rng.uniform(-20.0, 20.0)
    a = math.sqrt(1.4 * R_AIR * (_isa(hp)[0] if T is None else T))
    d = {"hp": hp, "V": rng.uniform(0.15, 0.85) * a}
    if T is not None:
        d["T"] = T
    return d


_SPEED_NAMES = {"v": "V", "tas": "V", "hp": "hp", "h": "hp", "pressure_altitude": "hp", "t": "T", "oat": "T"}

_add("aero_calibrated_airspeed",
     name="Calibrated airspeed from true airspeed", topic="airspeeds / calibrated airspeed",
     purpose="The calibrated airspeed, impact pressure and pitot total pressure of an aircraft flying at a true airspeed at a pressure altitude (subsonic, compressible pitot formula).",
     inputs=[_phys("V", "speed", "true airspeed (m/s, km/h, kt)"), _phys("hp", "length", "pressure altitude (m, ft)"),
             _phys("T", "temperature", "outside air temperature (default: standard temperature)", required=False)],
     outputs=[_o("CAS", "speed", "calibrated airspeed"), _o("CAS_kt", "number", "calibrated airspeed in knots"),
              _o("qc", "pressure", "impact pressure pt - p"), _o("pt", "pressure", "pitot total pressure")],
     primary="CAS", conditions=["V > 0", "flight Mach number below 1", "-1 km <= hp <= 47 km", "150 K <= T <= 350 K"],
     method="M = V/sqrt(gamma R T); qc = p[(1 + 0.2 M^2)^3.5 - 1]; CAS = a0 sqrt(5[(qc/p0 + 1)^(2/7) - 1])",
     latex=r"q_c = p\left[\left(1 + 0.2M^2\right)^{3.5} - 1\right],\quad V_{c} = a_0\sqrt{5\left[\left(\frac{q_c}{p_0} + 1\right)^{2/7} - 1\right]}",
     run=calibrated_airspeed,
     parse=lambda q: _pairs(q, r"calibrated airspeed(?: from true airspeed)?|cas from tas", _SPEED_NAMES, phys={"V", "hp", "T"}),
     reference=ref_calibrated_airspeed,
     wrong=_safe(_cas_wrong),
     random_inputs=_cas_random,
     templates=["Calibrated airspeed: V = 450 kt, hp = 35000 ft", "Calibrated airspeed from true airspeed: V = 200 m/s, hp = 6000 m, T = -20 °C"],
     examples=[{"inputs": {"V": 100.0, "hp": 0.0}, "outputs": {"CAS": 100.0}},
               {"inputs": {"V": 200.0, "hp": 0.0}, "outputs": {"CAS": 200.0, "pt": P_SL * _p0p(200.0 / A_SL, 1.4)}}],
     questions=[{"q": "Calibrated airspeed: V = 150 kt, hp = 0 ft", "outputs": {"CAS_kt": 150.0}},
                {"q": "Calibrated airspeed: V = 250 m/s, hp = 0 m", "outputs": {"CAS": 250.0}},
                {"q": "Calibrated airspeed: V = 400 m/s, hp = 0 m", "outcome": "INVALID_INPUT"},
                {"q": "Calibrated airspeed: V = 250 m/s", "outcome": "NEEDS_INPUT"}])


def true_airspeed_from_cas(CAS, hp, T=None):
    _pos("the calibrated airspeed", CAS)
    _need(CAS < A_SL, "the calibrated airspeed must be below the sea-level speed of sound (subsonic pitot formula)")
    p, T = _static(hp, T)
    qc = P_SL * (_p0p(CAS / A_SL, GAMMA_AIR) - 1.0)
    M = math.sqrt(5.0 * ((qc / p + 1.0) ** (2.0 / 7.0) - 1.0))
    _need(M < 1.0, "the flight Mach number would be supersonic; the subsonic pitot formula does not apply")
    V = M * math.sqrt(GAMMA_AIR * R_AIR * T)
    return _out(TAS=V, TAS_kt=V / KT, qc=qc, M=M)


def ref_true_airspeed_from_cas(CAS, hp, T=None):
    from fluids.atmosphere import ATMOSPHERE_1976
    a0 = _fluids_atm(0.0)
    qc = a0.P * (_ref_y_from_V(CAS, a0.P, a0.rho) - 1.0)
    a = _fluids_atm(hp)
    T = a.T if T is None else T
    rho = ATMOSPHERE_1976.density(T, a.P)
    y = 1.0 + qc / a.P
    V = math.sqrt(2.0 * a.P / rho * _ref_J(y, GAMMA_AIR))
    return {"TAS": V, "TAS_kt": V / KT, "qc": qc, "M": math.sqrt(_ref_M2_of_x(1.0 / y, GAMMA_AIR))}


def _tas_wrong(CAS, hp, T=None):
    p, T = _static(hp, T)
    rho = p / (R_AIR * T)
    V = CAS / math.sqrt(rho / RHO_SL)                  # compressibility ignored (CAS treated as EAS)
    return _out(TAS=V, TAS_kt=V / KT, qc=0.5 * RHO_SL * CAS * CAS, M=V / math.sqrt(1.4 * R_AIR * T))


def _tas_random(rng):
    hp = rng.uniform(0.0, 12000.0)
    d = {"hp": hp, "CAS": rng.uniform(40.0, 160.0)}
    if rng.random() < 0.5:
        d["T"] = _isa(hp)[0] + rng.uniform(-20.0, 20.0)
    return d


_add("aero_true_airspeed_from_calibrated",
     name="True airspeed from calibrated airspeed", topic="airspeeds / true airspeed",
     purpose="The true airspeed and flight Mach number from the calibrated airspeed, the pressure altitude and the outside air temperature (subsonic).",
     inputs=[_phys("CAS", "speed", "calibrated airspeed (m/s, kt, km/h)"), _phys("hp", "length", "pressure altitude (m, ft)"),
             _phys("T", "temperature", "outside air temperature (default: standard temperature)", required=False)],
     outputs=[_o("TAS", "speed", "true airspeed"), _o("TAS_kt", "number", "true airspeed in knots"),
              _o("qc", "pressure", "impact pressure"), _o("M", "number", "flight Mach number")],
     primary="TAS", conditions=["0 < CAS < a0", "subsonic flight", "-1 km <= hp <= 47 km", "150 K <= T <= 350 K"],
     method="qc = p0[(1 + 0.2 (CAS/a0)^2)^3.5 - 1]; M = sqrt(5[(qc/p + 1)^(2/7) - 1]); TAS = M sqrt(gamma R T)",
     latex=r"q_c = p_0\left[\left(1 + 0.2\,\frac{V_c^2}{a_0^2}\right)^{3.5} - 1\right],\quad M = \sqrt{5\left[\left(\frac{q_c}{p} + 1\right)^{2/7} - 1\right]},\quad V = M a",
     run=true_airspeed_from_cas,
     parse=lambda q: _pairs(q, r"true airspeed(?: from calibrated airspeed)?|tas from cas",
                            {"cas": "CAS", "vc": "CAS", "hp": "hp", "h": "hp", "pressure_altitude": "hp", "t": "T", "oat": "T"}, phys={"CAS", "hp", "T"}),
     reference=ref_true_airspeed_from_cas,
     wrong=_safe(_tas_wrong),
     random_inputs=_tas_random,
     templates=["True airspeed: CAS = 250 kt, hp = 30000 ft", "True airspeed from calibrated airspeed: CAS = 120 m/s, hp = 5000 m, T = -15 °C"],
     examples=[{"inputs": {"CAS": 100.0, "hp": 0.0}, "outputs": {"TAS": 100.0, "M": 100.0 / A_SL}},
               {"inputs": {"CAS": 150.0, "hp": 0.0}, "outputs": {"TAS": 150.0}}],
     questions=[{"q": "True airspeed: CAS = 120 kt, hp = 0 ft", "outputs": {"TAS_kt": 120.0}},
                {"q": "True airspeed from calibrated airspeed: CAS = 80 m/s, hp = 0 m", "outputs": {"TAS": 80.0}},
                {"q": "True airspeed: CAS = 400 m/s, hp = 0 m", "outcome": "INVALID_INPUT"},
                {"q": "True airspeed: CAS = 250 kt", "outcome": "NEEDS_INPUT"}])


def _pitot_ratio_super(M, g):
    m2 = M * M
    return ((g + 1.0) ** 2 * m2 / (4.0 * g * m2 - 2.0 * (g - 1.0))) ** (g / (g - 1.0)) * (1.0 - g + 2.0 * g * m2) / (g + 1.0)


def mach_from_pitot(p, pt, gamma=1.4):
    g = _gam(gamma)
    _pos("the static and pitot pressures", p, pt)
    r = pt / p
    _need(r > 1.0, "the pitot (total) pressure must exceed the static pressure")
    r_sonic = _p0p(1.0, g)
    if r <= r_sonic:
        M = math.sqrt(2.0 / (g - 1.0) * (r ** ((g - 1.0) / g) - 1.0))
    else:
        _need(r <= _pitot_ratio_super(20.0, g), "the pressure ratio corresponds to a Mach number above 20")
        M = _bisect(lambda m: _pitot_ratio_super(m, g) - r, 1.0, 20.0)
    return _out(M=M)


def ref_mach_from_pitot(p, pt, gamma=1.4):
    from scipy.optimize import brentq
    g, r = gamma, pt / p
    if r <= 1.0 / _ref_x_of_M(1.0, g):
        return {"M": math.sqrt(_ref_M2_of_x(1.0 / r, g))}
    f = lambda M: _ref_shock_all(M, g)["p02_p1"] - r       # noqa: E731  normal shock ahead of the probe, then isentropic
    return {"M": brentq(f, 1.001, 20.0, xtol=1e-14, rtol=8.9e-16, maxiter=500)}


def _pitot_random(rng):
    g = rng.choice([1.4, 1.4, rng.uniform(1.1, 1.67)])
    M = rng.uniform(0.1, 0.95) if rng.random() < 0.5 else rng.uniform(1.05, 6.0)
    p = _lu(rng, 1e3, 1e5)
    r = _p0p(M, g) if M < 1 else _pitot_ratio_super(M, g)
    return {"p": p, "pt": p * r, **({} if g == 1.4 else {"gamma": g})}


_add("aero_mach_from_pitot",
     name="Mach number from pitot and static pressures", topic="airspeeds / pitot-static Mach meter",
     purpose="The flight Mach number from the static pressure and the pitot (total) pressure: the isentropic relation when subsonic, the Rayleigh pitot formula (normal shock ahead of the probe) when supersonic.",
     inputs=[_phys("p", "pressure", "static pressure"), _phys("pt", "pressure", "pitot (total) pressure measured by the probe"), _GAMMA_IN],
     outputs=[_o("M", "number", "flight Mach number")],
     primary="M", conditions=["pt > p > 0", "M <= 20", "1.05 <= gamma <= 1.7"],
     method="subsonic: pt/p = (1 + (g-1)/2 M^2)^(g/(g-1)); supersonic: Rayleigh pitot formula pt2/p1 = [(g+1)^2 M^2/(4 g M^2 - 2(g-1))]^(g/(g-1)) (1 - g + 2 g M^2)/(g+1), solved by bisection",
     latex=r"\frac{p_{t2}}{p_1} = \left[\frac{(\gamma+1)^2 M^2}{4\gamma M^2 - 2(\gamma-1)}\right]^{\frac{\gamma}{\gamma-1}}\frac{1 - \gamma + 2\gamma M^2}{\gamma + 1}",
     run=mach_from_pitot,
     parse=lambda q: _pairs(q, r"mach (?:number )?from pitot(?: and static pressures?)?|pitot mach",
                            {"p": "p", "ps": "p", "p_static": "p", "pt": "pt", "p0": "pt", "p_pitot": "pt", **_GNAMES}, phys={"p", "pt"}),
     reference=ref_mach_from_pitot,
     wrong=_safe(lambda p, pt, gamma=1.4: _out(M=math.sqrt(2.0 * (pt - p) / (gamma * p)))),
     random_inputs=_pitot_random,
     templates=["Mach from pitot: p = 26.5 kPa, pt = 40 kPa", "Mach number from pitot and static pressures: p = 10 kPa, pt = 60 kPa, gamma = 1.4"],
     examples=[{"inputs": {"p": 1.0e4, "pt": 1.0e4 * 1.2 ** 3.5}, "outputs": {"M": 1.0}},
               {"inputs": {"p": 1.0e4, "pt": 1.0e4 * 1.1 ** 3.5}, "outputs": {"M": 0.5 ** 0.5}},
               {"inputs": {"p": 1.0e4, "pt": 1.0e4 * _pitot_ratio_super(2.0, 1.4)}, "outputs": {"M": 2.0}}],
     questions=[{"q": "Mach from pitot: p = 10 kPa, pt = " + repr(10 * 1.2 ** 3.5) + " kPa", "outputs": {"M": 1.0}},
                {"q": "Mach from pitot: p = 1 bar, pt = " + repr(_pitot_ratio_super(3.0, 1.4)) + " bar", "outputs": {"M": 3.0}},
                {"q": "Mach from pitot: p = 50 kPa, pt = 40 kPa", "outcome": "INVALID_INPUT"},
                {"q": "Mach from pitot: p = 50 kPa", "outcome": "NEEDS_INPUT"}])


# ================================================================================================ isentropic flow
def _mach_in(M, lo=1e-6, hi=50.0):
    _need(isinstance(M, (int, float)) and lo <= M <= hi, f"the Mach number must lie between {lo:g} and {hi:g}")


def isentropic_flow(M, gamma=1.4):
    g = _gam(gamma)
    _mach_in(M, 0.01, 10.0)
    t = _t0t(M, g)
    return _out(T0_T=t, p0_p=t ** (g / (g - 1.0)), rho0_rho=t ** (1.0 / (g - 1.0)), A_Astar=_area(M, g))


def ref_isentropic_flow(M, gamma=1.4):
    g = gamma
    x = _ref_x_of_M(M, g)
    return {"T0_T": x ** (-(g - 1.0) / g), "p0_p": 1.0 / x, "rho0_rho": x ** (-1.0 / g), "A_Astar": math.exp(_ref_lnA(M, g))}


def _iso_random(rng):
    g = rng.choice([1.4, rng.uniform(1.1, 1.67)])
    return {"M": rng.uniform(0.05, 6.0), **({} if g == 1.4 else {"gamma": g})}


_MG_NAMES = {"m": "M", "mach": "M", **_GNAMES}

_add("aero_isentropic_flow",
     name="Isentropic flow ratios at a Mach number", topic="compressible flow / isentropic flow",
     purpose="Stagnation-to-static temperature, pressure and density ratios and the area ratio A/A* of isentropic flow of a perfect gas at a Mach number.",
     inputs=[_numin("M", "Mach number (0.01 to 10)"), _GAMMA_IN],
     outputs=[_o("T0_T", "number", "stagnation-to-static temperature ratio T0/T"), _o("p0_p", "number", "stagnation-to-static pressure ratio p0/p"),
              _o("rho0_rho", "number", "stagnation-to-static density ratio rho0/rho"), _o("A_Astar", "number", "area ratio A/A* (A* = sonic throat area)")],
     primary="p0_p", conditions=["0.01 <= M <= 10", "1.05 <= gamma <= 1.7", "isentropic flow of a calorically perfect gas"],
     method="T0/T = 1 + (g-1)/2 M^2; p0/p = (T0/T)^(g/(g-1)); rho0/rho = (T0/T)^(1/(g-1)); A/A* = (1/M)[(2/(g+1))(T0/T)]^((g+1)/(2(g-1)))",
     latex=r"\frac{T_0}{T} = 1 + \frac{\gamma-1}{2}M^2,\quad \frac{p_0}{p} = \left(\frac{T_0}{T}\right)^{\frac{\gamma}{\gamma-1}},\quad \frac{A}{A^*} = \frac{1}{M}\left[\frac{2}{\gamma+1}\frac{T_0}{T}\right]^{\frac{\gamma+1}{2(\gamma-1)}}",
     run=isentropic_flow,
     parse=lambda q: _pairs(q, r"isentropic flow(?: ratios)?(?: at (?:a )?mach(?: number)?)?|isentropic ratios", _MG_NAMES),
     reference=ref_isentropic_flow,
     wrong=_safe(lambda M, gamma=1.4: (lambda t: _out(T0_T=t, p0_p=t ** (1.0 / (gamma - 1.0)), rho0_rho=t ** (gamma / (gamma - 1.0)),
                                                       A_Astar=_area(M, gamma)))(_t0t(M, gamma))),
     random_inputs=_iso_random,
     templates=["Isentropic flow: M = 2", "Isentropic flow ratios: M = 0.6, gamma = 1.3"],
     examples=[{"inputs": {"M": 2.0}, "outputs": {"T0_T": 1.8, "p0_p": 1.8 ** 3.5, "rho0_rho": 1.8 ** 2.5, "A_Astar": 1.6875}},
               {"inputs": {"M": 1.0, "gamma": 1.3}, "outputs": {"T0_T": 1.15, "A_Astar": 1.0}}],
     questions=[{"q": "Isentropic flow: M = 2", "outputs": {"A_Astar": 1.6875, "T0_T": 1.8}},
                {"q": "Isentropic flow: M = 1, gamma = 1.4", "outputs": {"T0_T": 1.2, "A_Astar": 1.0}},
                {"q": "Isentropic flow: M = 2, gamma = 3", "outcome": "INVALID_INPUT"},
                {"q": "Isentropic flow: gamma = 1.4", "outcome": "NEEDS_INPUT"}])


def mach_from_area_ratio(ar, regime, gamma=1.4):
    g = _gam(gamma)
    _need(isinstance(ar, (int, float)) and 1.0 <= ar <= 1.0e4, "the area ratio A/A* must lie between 1 and 10000")
    _need(regime in ("subsonic", "supersonic"), "the regime must be subsonic or supersonic")
    M = _M_of_area(ar, g, regime == "supersonic")
    _need(1e-4 <= M <= 100.0, "the Mach number lies outside 1e-4 to 100")
    t = _t0t(M, g)
    return _out(M=M, p_p0=t ** (-g / (g - 1.0)), T_T0=1.0 / t)


def ref_mach_from_area_ratio(ar, regime, gamma=1.4):
    g = gamma
    M = _ref_M_of_area(ar, g, regime == "supersonic")
    x = _ref_x_of_M(M, g)
    return {"M": M, "p_p0": x, "T_T0": x ** ((g - 1.0) / g)}


def _area_random(rng):
    g = rng.choice([1.4, rng.uniform(1.1, 1.67)])
    return {"ar": _lu(rng, 1.02, 50.0), "regime": rng.choice(["subsonic", "supersonic"]), **({} if g == 1.4 else {"gamma": g})}


_add("aero_mach_from_area_ratio",
     name="Mach number from the area ratio A/A*", topic="compressible flow / nozzle area-Mach relation",
     purpose="The subsonic or supersonic Mach number at which isentropic flow has a given area ratio A/A*, with the static-to-stagnation pressure and temperature ratios there.",
     inputs=[_numin("ar", "area ratio A/A* (>= 1)"), _choice("regime", ["subsonic", "supersonic"], "which branch: subsonic or supersonic"), _GAMMA_IN],
     outputs=[_o("M", "number", "Mach number"), _o("p_p0", "number", "static-to-stagnation pressure ratio p/p0"),
              _o("T_T0", "number", "static-to-stagnation temperature ratio T/T0")],
     primary="M", conditions=["1 <= A/A* <= 10000", "1.05 <= gamma <= 1.7"],
     method="solve A/A* = (1/M)[(2/(g+1))(1 + (g-1)/2 M^2)]^((g+1)/(2(g-1))) on the chosen branch by bisection",
     latex=r"\frac{A}{A^*} = \frac{1}{M}\left[\frac{2}{\gamma+1}\left(1 + \frac{\gamma-1}{2}M^2\right)\right]^{\frac{\gamma+1}{2(\gamma-1)}}",
     run=mach_from_area_ratio,
     parse=lambda q: _pairs(q, r"mach (?:number )?from (?:the )?area ratio(?: a/a\*)?",
                            {"ar": "ar", "a/a*": "ar", "a/astar": "ar", "area_ratio": "ar", "eps": "ar", "regime": "regime", "branch": "regime", **_GNAMES},
                            words={"regime"}),
     reference=ref_mach_from_area_ratio,
     wrong=_safe(lambda ar, regime, gamma=1.4: mach_from_area_ratio(ar, "subsonic" if regime == "supersonic" else "supersonic", gamma)),
     random_inputs=_area_random,
     templates=["Mach from area ratio: ar = 4, regime = supersonic", "Mach number from the area ratio: A/A* = 2.5, regime = subsonic, gamma = 1.3"],
     examples=[{"inputs": {"ar": 1.6875, "regime": "supersonic"}, "outputs": {"M": 2.0, "T_T0": 1 / 1.8}},
               {"inputs": {"ar": 1.0, "regime": "subsonic"}, "outputs": {"M": 1.0}}],
     questions=[{"q": "Mach from area ratio: ar = 1.6875, regime = supersonic", "outputs": {"M": 2.0}},
                {"q": "Mach from area ratio: A/A* = 1, regime = subsonic", "outputs": {"M": 1.0}},
                {"q": "Mach from area ratio: ar = 0.5, regime = subsonic", "outcome": "INVALID_INPUT"},
                {"q": "Mach from area ratio: ar = 2", "outcome": "NEEDS_INPUT"}])


def mach_from_pressure_ratio(pr, gamma=1.4):
    g = _gam(gamma)
    _need(isinstance(pr, (int, float)) and 1.0 < pr <= 1.0e6, "the pressure ratio p0/p must lie between 1 and 1e6")
    t = pr ** ((g - 1.0) / g)
    M = math.sqrt(2.0 / (g - 1.0) * (t - 1.0))
    _need(M <= 20.0, "the Mach number would exceed 20")
    return _out(M=M, T0_T=t, A_Astar=_area(M, g))


def ref_mach_from_pressure_ratio(pr, gamma=1.4):
    g = gamma
    M = math.sqrt(_ref_M2_of_x(1.0 / pr, g))
    return {"M": M, "T0_T": pr ** ((g - 1.0) / g), "A_Astar": math.exp(_ref_lnA(M, g))}


_add("aero_mach_from_pressure_ratio",
     name="Mach number from the stagnation-to-static pressure ratio", topic="compressible flow / isentropic flow",
     purpose="The Mach number of isentropic flow from the ratio of stagnation to static pressure p0/p, with T0/T and A/A*.",
     inputs=[_numin("pr", "stagnation-to-static pressure ratio p0/p (> 1)"), _GAMMA_IN],
     outputs=[_o("M", "number", "Mach number"), _o("T0_T", "number", "stagnation-to-static temperature ratio"),
              _o("A_Astar", "number", "area ratio A/A*")],
     primary="M", conditions=["1 < p0/p <= 1e6", "M <= 20", "1.05 <= gamma <= 1.7"],
     method="M = sqrt(2/(g-1) [(p0/p)^((g-1)/g) - 1])",
     latex=r"M = \sqrt{\frac{2}{\gamma-1}\left[\left(\frac{p_0}{p}\right)^{\frac{\gamma-1}{\gamma}} - 1\right]}",
     run=mach_from_pressure_ratio,
     parse=lambda q: _pairs(q, r"mach (?:number )?from (?:the )?(?:stagnation-to-static )?pressure ratio",
                            {"pr": "pr", "p0/p": "pr", "ratio": "pr", **_GNAMES}),
     reference=ref_mach_from_pressure_ratio,
     wrong=_safe(lambda pr, gamma=1.4: (lambda t: _out(M=math.sqrt(2.0 / (gamma - 1.0) * (t - 1.0)), T0_T=t,
                                                         A_Astar=_area(math.sqrt(2.0 / (gamma - 1.0) * (t - 1.0)), gamma)))(pr ** (1.0 / gamma))),
     random_inputs=lambda rng: {"pr": _lu(rng, 1.01, 500.0), **({"gamma": rng.uniform(1.1, 1.67)} if rng.random() < 0.5 else {})},
     templates=["Mach from pressure ratio: p0/p = 7.824", "Mach number from the stagnation-to-static pressure ratio: pr = 1.5, gamma = 1.4"],
     examples=[{"inputs": {"pr": 1.8 ** 3.5}, "outputs": {"M": 2.0, "T0_T": 1.8}},
               {"inputs": {"pr": 1.2 ** 3.5}, "outputs": {"M": 1.0, "A_Astar": 1.0}}],
     questions=[{"q": "Mach from pressure ratio: p0/p = " + repr(1.8 ** 3.5), "outputs": {"M": 2.0}},
                {"q": "Mach from pressure ratio: pr = " + repr(1.2 ** 3.5) + ", gamma = 1.4", "outputs": {"M": 1.0}},
                {"q": "Mach from pressure ratio: p0/p = 0.5", "outcome": "INVALID_INPUT"},
                {"q": "Mach from pressure ratio: gamma = 1.4", "outcome": "NEEDS_INPUT"}])


# ================================================================================================ shocks
def normal_shock(M1, gamma=1.4):
    g = _gam(gamma)
    _need(isinstance(M1, (int, float)) and 1.0 < M1 <= 20.0, "the upstream Mach number must lie between 1 and 20")
    M2, p21, r21, p0r = _shock(M1, g)
    return _out(M2=M2, p2_p1=p21, T2_T1=p21 / r21, rho2_rho1=r21, p02_p01=p0r, p02_p1=p21 * _p0p(M2, g))


def _ns_wrong(M1, gamma=1.4):
    g = gamma
    m2 = M1 * M1
    p21 = 1.0 + 2.0 * g / (g + 1.0) * (m2 - 1.0)
    r21 = (g + 1.0) * m2 / ((g - 1.0) * m2 + 2.0)
    M2 = math.sqrt((1.0 + 0.5 * (g - 1.0) * m2) / (g * m2 - (g - 1.0)))      # (g-1)/2 written as (g-1)
    return _out(M2=M2, p2_p1=p21, T2_T1=p21 / r21, rho2_rho1=r21, p02_p01=p21 * _p0p(M2, g) / _p0p(M1, g), p02_p1=p21 * _p0p(M2, g))


_add("aero_normal_shock",
     name="Normal shock relations", topic="compressible flow / normal shocks",
     purpose="Downstream Mach number and the pressure, temperature, density and stagnation-pressure ratios across a normal shock, with the pitot ratio p02/p1.",
     inputs=[_numin("M1", "upstream Mach number (> 1)"), _GAMMA_IN],
     outputs=[_o("M2", "number", "downstream Mach number"), _o("p2_p1", "number", "static pressure ratio p2/p1"),
              _o("T2_T1", "number", "temperature ratio T2/T1"), _o("rho2_rho1", "number", "density ratio rho2/rho1"),
              _o("p02_p01", "number", "stagnation pressure ratio p02/p01"), _o("p02_p1", "number", "pitot ratio p02/p1")],
     primary="p2_p1", conditions=["1 < M1 <= 20", "1.05 <= gamma <= 1.7"],
     method="p2/p1 = 1 + 2g/(g+1)(M1^2 - 1); rho2/rho1 = (g+1)M1^2/((g-1)M1^2 + 2); M2^2 = (1 + (g-1)/2 M1^2)/(g M1^2 - (g-1)/2)",
     latex=r"\frac{p_2}{p_1} = 1 + \frac{2\gamma}{\gamma+1}(M_1^2 - 1),\quad \frac{\rho_2}{\rho_1} = \frac{(\gamma+1)M_1^2}{(\gamma-1)M_1^2 + 2},\quad M_2^2 = \frac{1 + \frac{\gamma-1}{2}M_1^2}{\gamma M_1^2 - \frac{\gamma-1}{2}}",
     run=normal_shock,
     parse=lambda q: _pairs(q, r"normal shock(?: relations)?", {"m1": "M1", "m": "M1", "mach": "M1", **_GNAMES}),
     reference=lambda M1, gamma=1.4: _ref_shock_all(M1, gamma),
     wrong=_safe(_ns_wrong),
     random_inputs=lambda rng: {"M1": rng.uniform(1.05, 8.0), **({"gamma": rng.uniform(1.1, 1.67)} if rng.random() < 0.5 else {})},
     templates=["Normal shock: M1 = 2", "Normal shock relations: M1 = 3.5, gamma = 1.3"],
     examples=[{"inputs": {"M1": 2.0}, "outputs": {"p2_p1": 4.5, "rho2_rho1": 8 / 3, "T2_T1": 1.6875, "M2": 1 / 3 ** 0.5}},
               {"inputs": {"M1": 3.0}, "outputs": {"p2_p1": 1 + 2.8 / 2.4 * 8, "rho2_rho1": 2.4 * 9 / (0.4 * 9 + 2)}}],
     questions=[{"q": "Normal shock: M1 = 2", "outputs": {"p2_p1": 4.5, "rho2_rho1": 8 / 3}},
                {"q": "Normal shock: M1 = 2, gamma = 1.4", "outputs": {"T2_T1": 1.6875}},
                {"q": "Normal shock: M1 = 0.8", "outcome": "INVALID_INPUT"},
                {"q": "Normal shock: gamma = 1.4", "outcome": "NEEDS_INPUT"}])


def _theta_of_beta(M1, b, g):
    m2s = (M1 * math.sin(b)) ** 2
    return math.atan2(2.0 * (m2s - 1.0) / math.tan(b), M1 * M1 * (g + math.cos(2.0 * b)) + 2.0)


def _beta_max(M1, g):
    m2 = M1 * M1
    s2 = ((g + 1.0) * m2 / 4.0 - 1.0 + math.sqrt((g + 1.0) * (1.0 + 0.5 * (g - 1.0) * m2 + (g + 1.0) * m2 * m2 / 16.0))) / (g * m2)
    return math.asin(math.sqrt(s2))


def _oblique_from_beta(M1, b, g):
    Mn1 = M1 * math.sin(b)
    th = _theta_of_beta(M1, b, g)
    Mn2, p21, r21, p0r = _shock(Mn1, g)
    M2 = Mn2 / math.sin(b - th)
    return th, Mn1, M2, p21, r21, p0r


def oblique_shock(M1, theta, branch="weak", gamma=1.4):
    g = _gam(gamma)
    _need(isinstance(M1, (int, float)) and 1.0 < M1 <= 20.0, "the upstream Mach number must lie between 1 and 20")
    _need(branch in ("weak", "strong"), "the branch must be weak or strong")
    _need(0 < theta < math.pi / 2, "the deflection angle must lie between 0 and 90 degrees")
    bm = _beta_max(M1, g)
    tmax = _theta_of_beta(M1, bm, g)
    _need(theta < tmax, "the deflection exceeds the maximum for an attached oblique shock (the shock detaches)")
    mu = math.asin(1.0 / M1)
    if branch == "weak":
        b = _bisect(lambda x: _theta_of_beta(M1, x, g) - theta, mu, bm)
    else:
        b = _bisect(lambda x: _theta_of_beta(M1, x, g) - theta, bm, math.pi / 2)
    _th, Mn1, M2, p21, r21, p0r = _oblique_from_beta(M1, b, g)
    return _out(beta_deg=b / DEG, M2=M2, p2_p1=p21, T2_T1=p21 / r21, rho2_rho1=r21, p02_p01=p0r, Mn1=Mn1)


def _ref_oblique_state(M1, b, g):
    """Oblique shock at wave angle b from the normal-shock balances on the normal velocity component and an
    unchanged tangential component: returns (theta, M2, p2/p1, rho2/rho1, T2/T1, p02/p01)."""
    Mn1 = M1 * math.sin(b)
    if Mn1 <= 1.0 + 1e-4:
        return 0.0, M1, 1.0, 1.0, 1.0, 1.0
    r2, Vn2, p2, Vn1 = _ref_shock(Mn1, g)
    Vt = Vn1 / math.tan(b)                                   # same scaling: V1 = Vn1 / sin b, Vt = V1 cos b
    th = b - math.atan2(Vn2, Vt)
    T21 = p2 / r2
    M2 = math.hypot(Vn2, Vt) / math.sqrt(g * T21)
    x1, x2 = _ref_x_of_M(M1, g), _ref_x_of_M(M2, g)
    return th, M2, p2, r2, T21, p2 * x1 / x2


def ref_oblique_shock(M1, theta, branch="weak", gamma=1.4):
    from scipy.optimize import brentq, minimize_scalar
    g = gamma
    mu = math.asin(1.0 / M1)
    res = minimize_scalar(lambda b: -_ref_oblique_state(M1, b, g)[0], bounds=(mu, math.pi / 2), method="bounded",
                          options={"xatol": 1e-10})
    bm = res.x
    f = lambda b: _ref_oblique_state(M1, b, g)[0] - theta     # noqa: E731
    if branch == "weak":
        b = brentq(f, math.asin(min(1.0, (1.0 + 2e-4) / M1)), bm, xtol=1e-15, rtol=8.9e-16, maxiter=500)
    else:
        b = brentq(f, bm, math.pi / 2 - 1e-12, xtol=1e-15, rtol=8.9e-16, maxiter=500)
    _th, M2, p21, r21, T21, p0r = _ref_oblique_state(M1, b, g)
    return {"beta_deg": math.degrees(b), "M2": M2, "p2_p1": p21, "T2_T1": T21, "rho2_rho1": r21, "p02_p01": p0r, "Mn1": M1 * math.sin(b)}


def _obl_random(rng):
    g = rng.choice([1.4, rng.uniform(1.15, 1.67)])
    M1 = rng.uniform(1.3, 6.0)
    tmax = _theta_of_beta(M1, _beta_max(M1, g), g)
    return {"M1": M1, "theta": rng.uniform(0.05, 0.9) * tmax, **({"branch": rng.choice(["weak", "strong"])} if rng.random() < 0.6 else {}),
            **({} if g == 1.4 else {"gamma": g})}


_add("aero_oblique_shock",
     name="Oblique shock from the deflection angle", topic="compressible flow / oblique shocks",
     purpose="The wave angle, downstream Mach number and the pressure, temperature, density and stagnation-pressure ratios of an attached oblique shock turning a supersonic flow by a deflection angle (weak or strong solution).",
     inputs=[_numin("M1", "upstream Mach number (> 1)"), _phys("theta", "angle", "flow deflection (wedge half-) angle (deg, rad)"),
             _choice("branch", ["weak", "strong"], "weak (default) or strong shock solution", required=False), _GAMMA_IN],
     outputs=[_o("beta_deg", "number", "shock wave angle in degrees"), _o("M2", "number", "downstream Mach number"),
              _o("p2_p1", "number", "pressure ratio p2/p1"), _o("T2_T1", "number", "temperature ratio T2/T1"),
              _o("rho2_rho1", "number", "density ratio rho2/rho1"), _o("p02_p01", "number", "stagnation pressure ratio"),
              _o("Mn1", "number", "upstream Mach number normal to the shock")],
     primary="beta_deg", conditions=["1 < M1 <= 20", "0 < theta < theta_max(M1) (attached shock)", "1.05 <= gamma <= 1.7"],
     method="theta-beta-M relation tan(theta) = 2 cot(beta) (M1^2 sin^2 beta - 1)/(M1^2 (g + cos 2 beta) + 2), solved by bisection on the chosen branch; normal-shock relations on Mn1 = M1 sin beta; M2 = Mn2 / sin(beta - theta)",
     latex=r"\tan\theta = 2\cot\beta\,\frac{M_1^2\sin^2\beta - 1}{M_1^2(\gamma + \cos 2\beta) + 2},\quad M_{n1} = M_1\sin\beta,\quad M_2 = \frac{M_{n2}}{\sin(\beta - \theta)}",
     run=oblique_shock,
     parse=lambda q: _pairs(q, r"oblique shock(?: from (?:the )?deflection(?: angle)?)?",
                            {"m1": "M1", "m": "M1", "theta": "theta", "deflection": "theta", "branch": "branch", "solution": "branch", **_GNAMES},
                            phys={"theta"}, words={"branch"}),
     reference=ref_oblique_shock,
     wrong=_safe(lambda M1, theta, branch="weak", gamma=1.4: oblique_shock(M1, theta, "strong" if branch == "weak" else "weak", gamma)),
     random_inputs=_obl_random,
     templates=["Oblique shock: M1 = 2, theta = 10 deg", "Oblique shock from the deflection angle: M1 = 3, theta = 20 deg, branch = strong"],
     examples=[{"inputs": {"M1": 2.0, "theta": _theta_of_beta(2.0, 40 * DEG, 1.4)}, "outputs": {"beta_deg": 40.0, "Mn1": 2 * math.sin(40 * DEG)}},
               {"inputs": {"M1": 3.0, "theta": _theta_of_beta(3.0, 80 * DEG, 1.4), "branch": "strong"}, "outputs": {"beta_deg": 80.0}}],
     questions=[{"q": "Oblique shock: M1 = 2, theta = " + repr(math.degrees(_theta_of_beta(2.0, 40 * DEG, 1.4))) + " deg", "outputs": {"beta_deg": 40.0}},
                {"q": "Oblique shock: M1 = 3, theta = " + repr(math.degrees(_theta_of_beta(3.0, 80 * DEG, 1.4))) + " deg, branch = strong", "outputs": {"beta_deg": 80.0}},
                {"q": "Oblique shock: M1 = 2, theta = 30 deg", "outcome": "INVALID_INPUT"},
                {"q": "Oblique shock: M1 = 2", "outcome": "NEEDS_INPUT"}])


# ================================================================================================ expansions
def _nu(M, g):
    a = math.sqrt((g + 1.0) / (g - 1.0))
    s = math.sqrt(M * M - 1.0)
    return a * math.atan(s / a) - math.atan(s)


def _M_of_nu(nu, g):
    hi = 2.0
    while _nu(hi, g) < nu:
        hi *= 2.0
    return _bisect(lambda M: _nu(M, g) - nu, 1.0, hi)


def _ref_nu(M, g):
    """Prandtl-Meyer function by numerical integration of d nu = sqrt(M^2 - 1)/(1 + (g-1)/2 M^2) dM/M, with M = cosh u."""
    import numpy as np
    k = 0.5 * (g - 1.0)
    return _gl(lambda u: np.sinh(u) ** 2 / ((1.0 + k * np.cosh(u) ** 2) * np.cosh(u)), 0.0, math.acosh(M), n=128)


def _ref_M_of_nu(nu, g):
    from scipy.optimize import brentq
    return math.cosh(brentq(lambda u: _ref_nu(math.cosh(u), g) - nu, 0.0, math.acosh(200.0), xtol=1e-15, rtol=8.9e-16, maxiter=500))


def prandtl_meyer(M, gamma=1.4):
    g = _gam(gamma)
    _need(isinstance(M, (int, float)) and 1.0 <= M <= 50.0, "the Mach number must lie between 1 and 50")
    return _out(nu_deg=_nu(M, g) / DEG)


_add("aero_prandtl_meyer",
     name="Prandtl-Meyer angle of a Mach number", topic="compressible flow / expansion waves",
     purpose="The Prandtl-Meyer function nu(M): the angle through which a sonic flow must turn isentropically to reach Mach M.",
     inputs=[_numin("M", "Mach number (>= 1)"), _GAMMA_IN],
     outputs=[_o("nu_deg", "number", "Prandtl-Meyer angle in degrees")],
     primary="nu_deg", conditions=["1 <= M <= 50", "1.05 <= gamma <= 1.7"],
     method="nu = sqrt((g+1)/(g-1)) atan(sqrt((g-1)/(g+1)(M^2 - 1))) - atan(sqrt(M^2 - 1))",
     latex=r"\nu(M) = \sqrt{\frac{\gamma+1}{\gamma-1}}\arctan\sqrt{\frac{\gamma-1}{\gamma+1}(M^2-1)} - \arctan\sqrt{M^2-1}",
     run=prandtl_meyer,
     parse=lambda q: _pairs(q, r"prandtl[- ]meyer (?:angle|function)(?: of (?:a )?mach(?: number)?)?", _MG_NAMES),
     reference=lambda M, gamma=1.4: {"nu_deg": math.degrees(_ref_nu(M, gamma))},
     wrong=_safe(lambda M, gamma=1.4: _out(nu_deg=(math.atan(math.sqrt((gamma - 1) / (gamma + 1) * (M * M - 1))) - math.atan(math.sqrt(M * M - 1))) / DEG)),
     random_inputs=lambda rng: {"M": rng.uniform(1.05, 10.0), **({"gamma": rng.uniform(1.1, 1.67)} if rng.random() < 0.5 else {})},
     templates=["Prandtl-Meyer angle: M = 2", "Prandtl-Meyer function of Mach number: M = 3.2, gamma = 1.3"],
     examples=[{"inputs": {"M": 1.0}, "outputs": {"nu_deg": 0.0}},
               {"inputs": {"M": 2.0}, "outputs": {"nu_deg": (6 ** 0.5 * math.atan(0.5 ** 0.5) - math.atan(3 ** 0.5)) / DEG}}],
     questions=[{"q": "Prandtl-Meyer angle: M = 2", "outputs": {"nu_deg": (6 ** 0.5 * math.atan(0.5 ** 0.5) - math.atan(3 ** 0.5)) / DEG}},
                {"q": "Prandtl-Meyer angle: M = 1", "outputs": {"nu_deg": 0.0}},
                {"q": "Prandtl-Meyer angle: M = 0.5", "outcome": "INVALID_INPUT"},
                {"q": "Prandtl-Meyer angle:", "outcome": "NEEDS_INPUT"}])


def mach_from_prandtl_meyer(nu, gamma=1.4):
    g = _gam(gamma)
    numax = (math.sqrt((g + 1.0) / (g - 1.0)) - 1.0) * math.pi / 2
    _need(isinstance(nu, (int, float)) and 0 <= nu < _nu(50.0, g), "the Prandtl-Meyer angle must lie between 0 and nu(50)")
    return _out(M=_M_of_nu(nu, g) if nu > 0 else 1.0, nu_max_deg=numax / DEG)


def ref_mach_from_prandtl_meyer(nu, gamma=1.4):
    from scipy.integrate import quad
    numax, _ = quad(lambda m: math.sqrt(m * m - 1) / (1 + 0.5 * (gamma - 1) * m * m) / m, 1.0, math.inf, epsabs=0, epsrel=1e-13, limit=400)
    return {"M": _ref_M_of_nu(nu, gamma) if nu > 0 else 1.0, "nu_max_deg": math.degrees(numax)}


_add("aero_mach_from_prandtl_meyer",
     name="Mach number from the Prandtl-Meyer angle", topic="compressible flow / expansion waves",
     purpose="The supersonic Mach number whose Prandtl-Meyer angle is nu, with the maximum turning angle of the gas (expansion to vacuum).",
     inputs=[_phys("nu", "angle", "Prandtl-Meyer angle (deg, rad)"), _GAMMA_IN],
     outputs=[_o("M", "number", "Mach number"), _o("nu_max_deg", "number", "maximum Prandtl-Meyer angle (M -> infinity) in degrees")],
     primary="M", conditions=["0 <= nu < nu(M = 50)", "1.05 <= gamma <= 1.7"],
     method="invert nu(M) by bisection; nu_max = (sqrt((g+1)/(g-1)) - 1) pi/2",
     latex=r"\nu(M) = \nu,\quad \nu_{max} = \frac{\pi}{2}\left(\sqrt{\frac{\gamma+1}{\gamma-1}} - 1\right)",
     run=mach_from_prandtl_meyer,
     parse=lambda q: _pairs(q, r"mach (?:number )?from (?:the )?prandtl[- ]meyer (?:angle|function)", {"nu": "nu", "angle": "nu", **_GNAMES}, phys={"nu"}),
     reference=ref_mach_from_prandtl_meyer,
     wrong=_safe(lambda nu, gamma=1.4: _out(M=1.0 / math.sin(nu), nu_max_deg=math.sqrt((gamma + 1.0) / (gamma - 1.0)) * 90.0)),
     random_inputs=lambda rng: {"nu": rng.uniform(2.0, 80.0) * DEG, **({"gamma": rng.uniform(1.15, 1.67)} if rng.random() < 0.5 else {})},
     templates=["Mach from Prandtl-Meyer angle: nu = 26.38 deg", "Mach number from the Prandtl-Meyer function: nu = 0.5 rad, gamma = 1.3"],
     examples=[{"inputs": {"nu": 6 ** 0.5 * math.atan(0.5 ** 0.5) - math.atan(3 ** 0.5)}, "outputs": {"M": 2.0, "nu_max_deg": (6 ** 0.5 - 1) * 90.0}},
               {"inputs": {"nu": 0.0}, "outputs": {"M": 1.0}}],
     questions=[{"q": "Mach from Prandtl-Meyer angle: nu = " + repr(math.degrees(6 ** 0.5 * math.atan(0.5 ** 0.5) - math.atan(3 ** 0.5))) + " deg", "outputs": {"M": 2.0}},
                {"q": "Mach from Prandtl-Meyer angle: nu = 0 deg", "outputs": {"M": 1.0}},
                {"q": "Mach from Prandtl-Meyer angle: nu = 150 deg", "outcome": "INVALID_INPUT"},
                {"q": "Mach from Prandtl-Meyer angle: gamma = 1.4", "outcome": "NEEDS_INPUT"}])


def expansion_fan(M1, theta, gamma=1.4):
    g = _gam(gamma)
    _need(isinstance(M1, (int, float)) and 1.0 <= M1 <= 20.0, "the upstream Mach number must lie between 1 and 20")
    _need(0 < theta < math.pi / 2, "the turning angle must lie between 0 and 90 degrees")
    nu1 = _nu(M1, g)
    nu2 = nu1 + theta
    _need(nu2 < _nu(50.0, g), "the turn exceeds what the gas can expand through (downstream Mach above 50)")
    M2 = _M_of_nu(nu2, g)
    pr = (_t0t(M1, g) / _t0t(M2, g)) ** (g / (g - 1.0))
    return _out(M2=M2, nu1_deg=nu1 / DEG, nu2_deg=nu2 / DEG, p2_p1=pr, T2_T1=_t0t(M1, g) / _t0t(M2, g), rho2_rho1=pr ** (1.0 / g))


def ref_expansion_fan(M1, theta, gamma=1.4):
    g = gamma
    nu1 = _ref_nu(M1, g)
    M2 = _ref_M_of_nu(nu1 + theta, g)
    pr = _ref_x_of_M(M2, g) / _ref_x_of_M(M1, g)
    return {"M2": M2, "nu1_deg": math.degrees(nu1), "nu2_deg": math.degrees(nu1 + theta), "p2_p1": pr,
            "T2_T1": pr ** ((g - 1.0) / g), "rho2_rho1": pr ** (1.0 / g)}


def _fan_wrong(M1, theta, gamma=1.4):
    g = gamma
    nu1 = _nu(M1, g)
    M2 = _M_of_nu(nu1 + theta, g)
    pr = (_t0t(M1, g) / _t0t(M2, g)) ** (1.0 / (g - 1.0))               # density exponent used for pressure
    return _out(M2=M2 * 1.0, nu1_deg=nu1 / DEG, nu2_deg=(nu1 + theta) / DEG, p2_p1=pr, T2_T1=_t0t(M1, g) / _t0t(M2, g), rho2_rho1=pr ** (1.0 / g))


_add("aero_expansion_fan",
     name="Prandtl-Meyer expansion fan", topic="compressible flow / expansion waves",
     purpose="The downstream Mach number and the pressure, temperature and density ratios when a supersonic flow turns away from itself by an angle through a centred expansion fan.",
     inputs=[_numin("M1", "upstream Mach number (>= 1)"), _phys("theta", "angle", "turning angle of the flow (deg, rad)"), _GAMMA_IN],
     outputs=[_o("M2", "number", "downstream Mach number"), _o("nu1_deg", "number", "upstream Prandtl-Meyer angle (deg)"),
              _o("nu2_deg", "number", "downstream Prandtl-Meyer angle (deg)"), _o("p2_p1", "number", "pressure ratio p2/p1"),
              _o("T2_T1", "number", "temperature ratio T2/T1"), _o("rho2_rho1", "number", "density ratio rho2/rho1")],
     primary="p2_p1", conditions=["1 <= M1 <= 20", "0 < theta < 90 deg", "downstream Mach below 50", "1.05 <= gamma <= 1.7"],
     method="nu2 = nu(M1) + theta; M2 from nu(M2) = nu2; p2/p1 = [(1 + (g-1)/2 M1^2)/(1 + (g-1)/2 M2^2)]^(g/(g-1)) (isentropic)",
     latex=r"\nu(M_2) = \nu(M_1) + \theta,\quad \frac{p_2}{p_1} = \left[\frac{1 + \frac{\gamma-1}{2}M_1^2}{1 + \frac{\gamma-1}{2}M_2^2}\right]^{\frac{\gamma}{\gamma-1}}",
     run=expansion_fan,
     parse=lambda q: _pairs(q, r"(?:prandtl[- ]meyer )?expansion fan", {"m1": "M1", "m": "M1", "theta": "theta", "turn": "theta", **_GNAMES}, phys={"theta"}),
     reference=ref_expansion_fan,
     wrong=_safe(_fan_wrong),
     random_inputs=lambda rng: {"M1": rng.uniform(1.05, 5.0), "theta": rng.uniform(1.0, 30.0) * DEG,
                                **({"gamma": rng.uniform(1.15, 1.67)} if rng.random() < 0.5 else {})},
     templates=["Expansion fan: M1 = 2, theta = 10 deg", "Prandtl-Meyer expansion fan: M1 = 1.5, theta = 15 deg, gamma = 1.3"],
     examples=[{"inputs": {"M1": 1.0, "theta": 6 ** 0.5 * math.atan(0.5 ** 0.5) - math.atan(3 ** 0.5)}, "outputs": {"M2": 2.0, "T2_T1": 1.2 / 1.8, "nu1_deg": 0.0}}],
     questions=[{"q": "Expansion fan: M1 = 1, theta = " + repr(math.degrees(6 ** 0.5 * math.atan(0.5 ** 0.5) - math.atan(3 ** 0.5))) + " deg", "outputs": {"M2": 2.0}},
                {"q": "Expansion fan: M1 = 0.5, theta = 10 deg", "outcome": "INVALID_INPUT"},
                {"q": "Expansion fan: M1 = 2", "outcome": "NEEDS_INPUT"}])


def normal_shock_from_pressure_ratio(p21, gamma=1.4):
    g = _gam(gamma)
    _need(isinstance(p21, (int, float)) and 1.0 < p21 <= 500.0, "the static pressure ratio p2/p1 must lie between 1 and 500")
    M1 = math.sqrt((p21 - 1.0) * (g + 1.0) / (2.0 * g) + 1.0)
    M2, _p, r21, p0r = _shock(M1, g)
    return _out(M1=M1, M2=M2, T2_T1=p21 / r21, rho2_rho1=r21, p02_p01=p0r)


def _nsp_wrong(p21, gamma=1.4):
    M1 = math.sqrt((p21 - 1.0) * (gamma + 1.0) / (2.0 * gamma))             # the "+ 1" under the root forgotten
    M2, _p, r21, p0r = _shock(max(M1, 1.0 + 1e-9), gamma)
    return _out(M1=M1, M2=M2, T2_T1=p21 / r21, rho2_rho1=r21, p02_p01=p0r)


def ref_normal_shock_from_pressure_ratio(p21, gamma=1.4):
    from scipy.optimize import brentq
    M1 = brentq(lambda M: _ref_shock(M, gamma)[2] - p21, 1.0001, 60.0, xtol=1e-14, rtol=8.9e-16, maxiter=500)
    d = _ref_shock_all(M1, gamma)
    return {"M1": M1, "M2": d["M2"], "T2_T1": d["T2_T1"], "rho2_rho1": d["rho2_rho1"], "p02_p01": d["p02_p01"]}


_add("aero_normal_shock_from_pressure_ratio",
     name="Normal shock from its pressure ratio", topic="compressible flow / normal shocks",
     purpose="The upstream and downstream Mach numbers and the temperature, density and stagnation-pressure ratios of a normal shock of known static pressure ratio p2/p1 (shock strength).",
     inputs=[_numin("p21", "static pressure ratio across the shock p2/p1 (> 1)"), _GAMMA_IN],
     outputs=[_o("M1", "number", "upstream Mach number"), _o("M2", "number", "downstream Mach number"),
              _o("T2_T1", "number", "temperature ratio"), _o("rho2_rho1", "number", "density ratio"),
              _o("p02_p01", "number", "stagnation pressure ratio")],
     primary="M1", conditions=["1 < p2/p1 <= 500", "1.05 <= gamma <= 1.7"],
     method="M1 = sqrt((g+1)/(2g) (p2/p1 - 1) + 1), then the normal-shock relations",
     latex=r"M_1 = \sqrt{\frac{\gamma+1}{2\gamma}\left(\frac{p_2}{p_1} - 1\right) + 1}",
     run=normal_shock_from_pressure_ratio,
     parse=lambda q: _pairs(q, r"normal shock from (?:its |the )?pressure ratio", {"p21": "p21", "p2/p1": "p21", "ratio": "p21", **_GNAMES}),
     reference=ref_normal_shock_from_pressure_ratio,
     wrong=_safe(_nsp_wrong),
     random_inputs=lambda rng: {"p21": _lu(rng, 1.2, 100.0), **({"gamma": rng.uniform(1.1, 1.67)} if rng.random() < 0.5 else {})},
     templates=["Normal shock from pressure ratio: p2/p1 = 4.5", "Normal shock from its pressure ratio: p21 = 10, gamma = 1.3"],
     examples=[{"inputs": {"p21": 4.5}, "outputs": {"M1": 2.0, "rho2_rho1": 8 / 3}},
               {"inputs": {"p21": 1 + 2.8 / 2.4 * 8}, "outputs": {"M1": 3.0}}],
     questions=[{"q": "Normal shock from pressure ratio: p2/p1 = 4.5", "outputs": {"M1": 2.0}},
                {"q": "Normal shock from pressure ratio: p2/p1 = 0.5", "outcome": "INVALID_INPUT"},
                {"q": "Normal shock from pressure ratio: gamma = 1.4", "outcome": "NEEDS_INPUT"}])


def oblique_from_wave_angle(M1, beta, gamma=1.4):
    g = _gam(gamma)
    _need(isinstance(M1, (int, float)) and 1.0 < M1 <= 20.0, "the upstream Mach number must lie between 1 and 20")
    mu = math.asin(1.0 / M1)
    _need(mu < beta <= math.pi / 2, "the wave angle must lie between the Mach angle asin(1/M1) and 90 degrees")
    th, Mn1, M2, p21, r21, p0r = _oblique_from_beta(M1, beta, g)
    if beta == math.pi / 2:
        th = 0.0
    return _out(theta_deg=th / DEG, M2=M2, p2_p1=p21, T2_T1=p21 / r21, rho2_rho1=r21, p02_p01=p0r, Mn1=Mn1)


def ref_oblique_from_wave_angle(M1, beta, gamma=1.4):
    th, M2, p21, r21, T21, p0r = _ref_oblique_state(M1, beta, gamma)
    return {"theta_deg": math.degrees(th), "M2": M2, "p2_p1": p21, "T2_T1": T21, "rho2_rho1": r21, "p02_p01": p0r, "Mn1": M1 * math.sin(beta)}


def _owa_random(rng):
    M1 = rng.uniform(1.3, 8.0)
    mu = math.asin(1.0 / M1)
    return {"M1": M1, "beta": mu + rng.uniform(0.05, 0.98) * (math.pi / 2 - mu), **({"gamma": rng.uniform(1.15, 1.67)} if rng.random() < 0.5 else {})}


_add("aero_oblique_shock_from_wave_angle",
     name="Oblique shock from the wave angle", topic="compressible flow / oblique shocks",
     purpose="The flow deflection, downstream Mach number and the property ratios of an oblique shock of known wave angle (for example measured on a schlieren photograph).",
     inputs=[_numin("M1", "upstream Mach number (> 1)"), _phys("beta", "angle", "shock wave angle (deg, rad)"), _GAMMA_IN],
     outputs=[_o("theta_deg", "number", "flow deflection angle in degrees"), _o("M2", "number", "downstream Mach number"),
              _o("p2_p1", "number", "pressure ratio"), _o("T2_T1", "number", "temperature ratio"), _o("rho2_rho1", "number", "density ratio"),
              _o("p02_p01", "number", "stagnation pressure ratio"), _o("Mn1", "number", "upstream normal Mach number")],
     primary="theta_deg", conditions=["1 < M1 <= 20", "asin(1/M1) < beta <= 90 deg", "1.05 <= gamma <= 1.7"],
     method="theta from the theta-beta-M relation; normal-shock relations on Mn1 = M1 sin beta; M2 = Mn2/sin(beta - theta)",
     latex=r"\tan\theta = 2\cot\beta\,\frac{M_1^2\sin^2\beta - 1}{M_1^2(\gamma + \cos 2\beta) + 2}",
     run=oblique_from_wave_angle,
     parse=lambda q: _pairs(q, r"oblique shock from (?:the )?wave angle", {"m1": "M1", "m": "M1", "beta": "beta", "wave_angle": "beta", **_GNAMES}, phys={"beta"}),
     reference=ref_oblique_from_wave_angle,
     wrong=_safe(lambda M1, beta, gamma=1.4: (lambda o: {**o, "theta_deg": _r(o["theta_deg"] * 2.0)})(oblique_from_wave_angle(M1, beta, gamma))),
     random_inputs=_owa_random,
     templates=["Oblique shock from wave angle: M1 = 2, beta = 40 deg", "Oblique shock from the wave angle: M1 = 3, beta = 30 deg, gamma = 1.3"],
     examples=[{"inputs": {"M1": 2.0, "beta": math.pi / 2}, "outputs": {"theta_deg": 0.0, "p2_p1": 4.5, "M2": 1 / 3 ** 0.5}},
               {"inputs": {"M1": 2.0, "beta": 40 * DEG}, "outputs": {"theta_deg": math.degrees(_theta_of_beta(2.0, 40 * DEG, 1.4))}}],
     questions=[{"q": "Oblique shock from wave angle: M1 = 2, beta = 90 deg", "outputs": {"p2_p1": 4.5}},
                {"q": "Oblique shock from wave angle: M1 = 2, beta = 20 deg", "outcome": "INVALID_INPUT"},
                {"q": "Oblique shock from wave angle: M1 = 2", "outcome": "NEEDS_INPUT"}])


def max_deflection(M1, gamma=1.4):
    g = _gam(gamma)
    _need(isinstance(M1, (int, float)) and 1.01 <= M1 <= 20.0, "the upstream Mach number must lie between 1.01 and 20")
    bm = _beta_max(M1, g)
    th, Mn1, M2, p21, r21, p0r = _oblique_from_beta(M1, bm, g)
    return _out(theta_max_deg=th / DEG, beta_deg=bm / DEG, M2=M2, p2_p1=p21)


def ref_max_deflection(M1, gamma=1.4):
    """The maximum of theta(beta) of the conservation-law shock, located as the root of d theta/d beta, at 30 digits (mpmath)."""
    import mpmath as mp
    mp.mp.dps = 30
    g, M = mp.mpf(gamma), mp.mpf(M1)

    def rho_ratio(Mn):
        V1 = Mn * mp.sqrt(g)
        c = g / (g - 1)
        h = lambda r: c * ((1 + V1 ** 2) * r - V1 ** 2) + V1 ** 2 / 2 - (c + V1 ** 2 / 2) * r ** 2   # noqa: E731
        lo = 1 + mp.mpf("1e-3") * (Mn ** 2 - 1) / Mn ** 2
        return mp.findroot(h, (lo, (g + 1) / (g - 1)), solver="anderson")

    def theta(b):
        r = rho_ratio(M * mp.sin(b))
        return b - mp.atan2(mp.sin(b) / r, mp.cos(b))          # tangential component kept, normal component / r

    mu = mp.asin(1 / M)
    b0 = mp.mpf(_ref_beta_guess(M1, gamma))
    bstar = mp.findroot(lambda b: mp.diff(theta, b), b0)
    r = rho_ratio(M * mp.sin(bstar))
    Mn1 = M * mp.sin(bstar)
    V1 = Mn1 * mp.sqrt(g)
    Vn2 = V1 / r
    p2 = 1 + V1 ** 2 - V1 * Vn2
    Vt = V1 * mp.cos(bstar) / mp.sin(bstar)
    M2 = mp.sqrt(Vn2 ** 2 + Vt ** 2) / mp.sqrt(g * p2 / r)
    _ = mu
    return {"theta_max_deg": float(mp.degrees(theta(bstar))), "beta_deg": float(mp.degrees(bstar)), "M2": float(M2), "p2_p1": float(p2)}


def _ref_beta_guess(M1, g):
    from scipy.optimize import minimize_scalar
    mu = math.asin(1.0 / M1)
    return minimize_scalar(lambda b: -_ref_oblique_state(M1, b, g)[0], bounds=(mu, math.pi / 2), method="bounded", options={"xatol": 1e-9}).x


_add("aero_max_deflection_angle",
     name="Maximum deflection angle of an attached oblique shock", topic="compressible flow / oblique shocks",
     purpose="The largest wedge (deflection) angle for which an oblique shock stays attached at a given upstream Mach number, with the wave angle, downstream Mach number and pressure ratio at that limit; beyond it the shock detaches.",
     inputs=[_numin("M1", "upstream Mach number (>= 1.01)"), _GAMMA_IN],
     outputs=[_o("theta_max_deg", "number", "maximum deflection angle in degrees"), _o("beta_deg", "number", "wave angle at the maximum, degrees"),
              _o("M2", "number", "downstream Mach number at the maximum"), _o("p2_p1", "number", "pressure ratio at the maximum")],
     primary="theta_max_deg", conditions=["1.01 <= M1 <= 20", "1.05 <= gamma <= 1.7"],
     method="sin^2 beta* = [(g+1)M^2/4 - 1 + sqrt((g+1)(1 + (g-1)M^2/2 + (g+1)M^4/16))]/(g M^2); theta_max = theta(beta*)",
     latex=r"\sin^2\beta^* = \frac{\frac{\gamma+1}{4}M_1^2 - 1 + \sqrt{(\gamma+1)\left(1 + \frac{\gamma-1}{2}M_1^2 + \frac{\gamma+1}{16}M_1^4\right)}}{\gamma M_1^2}",
     run=max_deflection,
     parse=lambda q: _pairs(q, r"maximum deflection angle|max(?:imum)? deflection|detachment angle", {"m1": "M1", "m": "M1", **_GNAMES}),
     reference=ref_max_deflection,
     wrong=_safe(lambda M1, gamma=1.4: (lambda b: (lambda t: _out(theta_max_deg=t[0] / DEG, beta_deg=b / DEG, M2=t[2], p2_p1=t[3]))(_oblique_from_beta(M1, b, gamma)))(
         math.asin(math.sqrt(((gamma + 1.0) * M1 * M1 / 4.0 - 1.0 + math.sqrt((gamma + 1.0) * (1.0 + 0.5 * (gamma - 1.0) * M1 * M1))) / (gamma * M1 * M1))))),
     random_inputs=lambda rng: {"M1": rng.uniform(1.2, 10.0), **({"gamma": rng.uniform(1.15, 1.67)} if rng.random() < 0.5 else {})},
     templates=["Maximum deflection angle: M1 = 2", "Max deflection: M1 = 3, gamma = 1.3"],
     examples=[{"inputs": {"M1": 2.0}, "outputs": {"beta_deg": math.degrees(_beta_max(2.0, 1.4))}},
               {"inputs": {"M1": 3.0}, "outputs": {"theta_max_deg": math.degrees(_theta_of_beta(3.0, _beta_max(3.0, 1.4), 1.4))}}],
     questions=[{"q": "Maximum deflection angle: M1 = 2", "outputs": {"beta_deg": math.degrees(_beta_max(2.0, 1.4))}},
                {"q": "Maximum deflection angle: M1 = 0.9", "outcome": "INVALID_INPUT"},
                {"q": "Maximum deflection angle: gamma = 1.4", "outcome": "NEEDS_INPUT"}])


# ================================================================================================ Fanno and Rayleigh flow
def _ref_exp_int(fun, M_from, M_to):
    """exp of the integral of fun(m) dm from M_from to M_to, computed in u = ln m (fun given as a numpy function)."""
    import numpy as np
    return math.exp(_gl(lambda u: fun(np.exp(u)) * np.exp(u), math.log(M_from), math.log(M_to), n=128))


def fanno(M, gamma=1.4):
    g = _gam(gamma)
    _need(isinstance(M, (int, float)) and 0.05 <= M <= 10.0 and M != 1.0, "the Mach number must lie between 0.05 and 10 (not 1)")
    m2 = M * M
    tr = (g + 1.0) / (2.0 + (g - 1.0) * m2)
    fld = (1.0 - m2) / (g * m2) + (g + 1.0) / (2.0 * g) * math.log((g + 1.0) * m2 / (2.0 + (g - 1.0) * m2))
    return _out(fLD=fld, p_pstar=math.sqrt(tr) / M, T_Tstar=tr, rho_rhostar=1.0 / (M * math.sqrt(tr)),
                p0_p0star=(1.0 / tr) ** ((g + 1.0) / (2.0 * (g - 1.0))) / M)


def ref_fanno(M, gamma=1.4):
    import numpy as np
    g, k = gamma, 0.5 * (gamma - 1.0)
    fld = math.log(_ref_exp_int(lambda m: 2.0 * (1.0 - m * m) / (g * m ** 3 * (1.0 + k * m * m)), M, 1.0))
    pr = _ref_exp_int(lambda m: (1.0 + (g - 1.0) * m * m) / ((1.0 + k * m * m) * m), M, 1.0)
    tr = _ref_exp_int(lambda m: 2.0 * k * m / (1.0 + k * m * m), M, 1.0)
    rr = _ref_exp_int(lambda m: 1.0 / m - k * m / (1.0 + k * m * m), M, 1.0)
    p0r = _ref_exp_int(lambda m: (1.0 - m * m) / (m * (1.0 + k * m * m)), M, 1.0)
    _ = np
    return {"fLD": fld, "p_pstar": pr, "T_Tstar": tr, "rho_rhostar": rr, "p0_p0star": p0r}


_add("aero_fanno_flow",
     name="Fanno flow (adiabatic duct flow with friction)", topic="compressible flow / Fanno flow",
     purpose="For adiabatic flow with wall friction in a constant-area duct: the friction length f L*/D needed to reach Mach 1 from Mach M, and the pressure, temperature, density and stagnation-pressure ratios to the sonic (choked) state.",
     inputs=[_numin("M", "Mach number at the duct station (0.05 to 10, not 1)"), _GAMMA_IN],
     outputs=[_o("fLD", "number", "f L*/D with the Darcy friction factor f (= 4 times the Fanning factor)"), _o("p_pstar", "number", "p/p*"),
              _o("T_Tstar", "number", "T/T*"), _o("rho_rhostar", "number", "rho/rho* (= V*/V)"), _o("p0_p0star", "number", "p0/p0*")],
     primary="fLD", conditions=["0.05 <= M <= 10", "adiabatic, constant area, constant friction factor", "1.05 <= gamma <= 1.7"],
     method="f L*/D = (1 - M^2)/(g M^2) + (g+1)/(2g) ln[(g+1)M^2/(2 + (g-1)M^2)]; T/T* = (g+1)/(2 + (g-1)M^2); p/p* = sqrt(T/T*)/M; p0/p0* = (T*/T)^((g+1)/(2(g-1)))/M",
     latex=r"\frac{\bar f L^*}{D} = \frac{1 - M^2}{\gamma M^2} + \frac{\gamma+1}{2\gamma}\ln\frac{(\gamma+1)M^2}{2 + (\gamma-1)M^2},\quad \frac{p}{p^*} = \frac{1}{M}\sqrt{\frac{\gamma+1}{2 + (\gamma-1)M^2}}",
     run=fanno,
     parse=lambda q: _pairs(q, r"fanno flow(?: \(adiabatic duct flow with friction\))?|fanno", _MG_NAMES),
     reference=ref_fanno,
     wrong=_safe(lambda M, gamma=1.4: (lambda o: {**o, "fLD": _r(o["fLD"] / 4.0)})(fanno(M, gamma))),
     random_inputs=lambda rng: {"M": rng.choice([rng.uniform(0.1, 0.95), rng.uniform(1.05, 4.0)]), **({"gamma": rng.uniform(1.1, 1.67)} if rng.random() < 0.5 else {})},
     templates=["Fanno flow: M = 0.3", "Fanno flow: M = 2.5, gamma = 1.4"],
     examples=[{"inputs": {"M": 2.0}, "outputs": {"T_Tstar": 2.4 / 3.6, "p_pstar": (2.4 / 3.6) ** 0.5 / 2}},
               {"inputs": {"M": 0.5}, "outputs": {"T_Tstar": 2.4 / 2.1, "fLD": 0.75 / 0.35 + 2.4 / 2.8 * math.log(0.6 / 2.1)}}],
     questions=[{"q": "Fanno flow: M = 2", "outputs": {"T_Tstar": 2.4 / 3.6}},
                {"q": "Fanno flow: M = 1", "outcome": "INVALID_INPUT"},
                {"q": "Fanno flow: gamma = 1.4", "outcome": "NEEDS_INPUT"}])


def rayleigh(M, gamma=1.4):
    g = _gam(gamma)
    _need(isinstance(M, (int, float)) and 0.05 <= M <= 10.0, "the Mach number must lie between 0.05 and 10")
    m2 = M * M
    pr = (1.0 + g) / (1.0 + g * m2)
    return _out(T0_T0star=(g + 1.0) * m2 * (2.0 + (g - 1.0) * m2) / (1.0 + g * m2) ** 2, T_Tstar=m2 * pr * pr, p_pstar=pr,
                rho_rhostar=1.0 / (m2 * pr), p0_p0star=pr * ((2.0 + (g - 1.0) * m2) / (g + 1.0)) ** (g / (g - 1.0)))


def ref_rayleigh(M, gamma=1.4):
    g, k = gamma, 0.5 * (gamma - 1.0)
    dlp = lambda m: -2.0 * g * m / (1.0 + g * m * m)                       # noqa: E731  momentum dp = -rho V dV
    dlT = lambda m: 2.0 * (1.0 - g * m * m) / (m * (1.0 + g * m * m))      # noqa: E731
    dlr = lambda m: -2.0 / (m * (1.0 + g * m * m))                         # noqa: E731  continuity
    dls = lambda m: 2.0 * k * m / (1.0 + k * m * m)                        # noqa: E731  d ln(1 + k M^2)
    return {"T0_T0star": _ref_exp_int(lambda m: dlT(m) + dls(m), 1.0, M), "T_Tstar": _ref_exp_int(dlT, 1.0, M),
            "p_pstar": _ref_exp_int(dlp, 1.0, M), "rho_rhostar": _ref_exp_int(dlr, 1.0, M),
            "p0_p0star": _ref_exp_int(lambda m: dlp(m) + g / (g - 1.0) * dls(m), 1.0, M)}


_add("aero_rayleigh_flow",
     name="Rayleigh flow (duct flow with heat addition)", topic="compressible flow / Rayleigh flow",
     purpose="For frictionless constant-area flow with heat addition: the stagnation temperature, temperature, pressure, density and stagnation-pressure ratios to the sonic (thermally choked) state at a Mach number.",
     inputs=[_numin("M", "Mach number (0.05 to 10)"), _GAMMA_IN],
     outputs=[_o("T0_T0star", "number", "T0/T0*"), _o("T_Tstar", "number", "T/T*"), _o("p_pstar", "number", "p/p*"),
              _o("rho_rhostar", "number", "rho/rho*"), _o("p0_p0star", "number", "p0/p0*")],
     primary="T0_T0star", conditions=["0.05 <= M <= 10", "frictionless, constant area", "1.05 <= gamma <= 1.7"],
     method="p/p* = (1+g)/(1 + g M^2); T/T* = M^2 (p/p*)^2; T0/T0* = (g+1)M^2(2 + (g-1)M^2)/(1 + g M^2)^2; p0/p0* = (p/p*)[(2 + (g-1)M^2)/(g+1)]^(g/(g-1))",
     latex=r"\frac{p}{p^*} = \frac{1+\gamma}{1 + \gamma M^2},\quad \frac{T_0}{T_0^*} = \frac{(\gamma+1)M^2\left[2 + (\gamma-1)M^2\right]}{(1 + \gamma M^2)^2}",
     run=rayleigh,
     parse=lambda q: _pairs(q, r"rayleigh flow(?: \(duct flow with heat addition\))?|rayleigh line", _MG_NAMES),
     reference=ref_rayleigh,
     wrong=_safe(lambda M, gamma=1.4: (lambda o: {**o, "T0_T0star": _r(o["T0_T0star"] * (1.0 + gamma * M * M) / (1.0 + gamma))})(rayleigh(M, gamma))),
     random_inputs=lambda rng: {"M": rng.choice([rng.uniform(0.1, 0.95), rng.uniform(1.05, 5.0)]), **({"gamma": rng.uniform(1.1, 1.67)} if rng.random() < 0.5 else {})},
     templates=["Rayleigh flow: M = 0.4", "Rayleigh flow: M = 2, gamma = 1.4"],
     examples=[{"inputs": {"M": 1.0}, "outputs": {"T0_T0star": 1.0, "p_pstar": 1.0, "p0_p0star": 1.0}},
               {"inputs": {"M": 2.0}, "outputs": {"p_pstar": 2.4 / 6.6, "T0_T0star": 2.4 * 4 * 3.6 / 6.6 ** 2}}],
     questions=[{"q": "Rayleigh flow: M = 2", "outputs": {"p_pstar": 2.4 / 6.6}},
                {"q": "Rayleigh flow: M = 20", "outcome": "INVALID_INPUT"},
                {"q": "Rayleigh flow:", "outcome": "NEEDS_INPUT"}])


# ================================================================================================ nozzles and rocket engines
def _vdk(g):
    """Vandenkerckhove function Gamma(gamma) = sqrt(gamma) (2/(gamma+1))^((gamma+1)/(2(gamma-1)))."""
    return math.sqrt(g) * (2.0 / (g + 1.0)) ** ((g + 1.0) / (2.0 * (g - 1.0)))


def _ref_G(x, g):
    """Non-dimensional isentropic mass flux rho V / (p0/sqrt(R T0)) at pressure ratio x = p/p0 (Euler integral)."""
    return x ** (1.0 / g) * math.sqrt(2.0 * _ref_I(x, g))


def _ref_throat(g):
    """Throat pressure ratio x* as the maximum of the mass flux: root of d ln(G^2)/dx = 0, i.e. (2/g) I(x) = x^(1 - 1/g)."""
    from scipy.optimize import brentq
    return brentq(lambda x: 2.0 / g * _ref_I(x, g) - x ** (1.0 - 1.0 / g), 1e-6, 1.0 - 1e-15, xtol=1e-16, rtol=8.9e-16, maxiter=500)


def _ref_exit_x(eps, g):
    """Supersonic exit pressure ratio for an area ratio eps: mass conservation G(x_e) eps = G(x*)."""
    from scipy.optimize import brentq
    xs = _ref_throat(g)
    Gs = _ref_G(xs, g)
    return math.exp(brentq(lambda lx: _ref_G(math.exp(lx), g) * eps - Gs, -120.0, math.log(xs), xtol=1e-15, rtol=8.9e-16, maxiter=500)), xs, Gs


def _ref_nozzle(pc, Tc, gamma, Mw, eps, pa):
    R = RU / Mw
    xe, xs, Gs = _ref_exit_x(eps, gamma)
    ve = math.sqrt(2.0 * R * Tc * _ref_I(xe, gamma))
    cf = Gs * math.sqrt(2.0 * _ref_I(xe, gamma)) + (xe - pa / pc) * eps
    return {"xe": xe, "Gs": Gs, "ve": ve, "cf": cf, "cstar": math.sqrt(R * Tc) / Gs, "Me": math.sqrt(_ref_M2_of_x(xe, gamma)),
            "cf_vac": cf + pa / pc * eps}


def choked_mass_flow(p0, T0, A, gamma=1.4, R=R_AIR):
    g = _gam(gamma)
    _pos("the stagnation pressure, stagnation temperature, throat area and gas constant", p0, T0, A, R)
    t = 2.0 / (g + 1.0)
    return _out(mdot=_vdk(g) * p0 * A / math.sqrt(R * T0), p_star=p0 * t ** (g / (g - 1.0)), T_star=T0 * t,
                V_star=math.sqrt(g * R * T0 * t))


def ref_choked_mass_flow(p0, T0, A, gamma=1.4, R=R_AIR):
    xs = _ref_throat(gamma)
    return {"mdot": _ref_G(xs, gamma) * p0 * A / math.sqrt(R * T0), "p_star": xs * p0, "T_star": T0 * xs ** ((gamma - 1.0) / gamma),
            "V_star": math.sqrt(2.0 * R * T0 * _ref_I(xs, gamma))}


_add("aero_choked_mass_flow",
     name="Choked mass flow through a nozzle throat", topic="nozzles / choked flow",
     purpose="The maximum (choked) mass flow of a perfect gas through a sonic throat from the stagnation pressure and temperature, with the throat (critical) pressure, temperature and speed.",
     inputs=[_phys("p0", "pressure", "stagnation (reservoir) pressure"), _phys("T0", "temperature", "stagnation temperature"),
             _phys("A", "area", "throat area (m^2, cm^2, mm^2)"), _GAMMA_IN,
             _phys("R", "specific_gas_constant", "specific gas constant (default air, 287.053 J/(kg*K))", required=False)],
     outputs=[_o("mdot", "mass_flow_rate", "choked mass flow rate"), _o("p_star", "pressure", "throat pressure"),
              _o("T_star", "temperature", "throat temperature"), _o("V_star", "speed", "throat (sonic) speed")],
     primary="mdot", conditions=["p0, T0, A, R > 0", "isentropic flow to a sonic throat", "1.05 <= gamma <= 1.7"],
     method="mdot = Gamma p0 A/sqrt(R T0), Gamma = sqrt(g)(2/(g+1))^((g+1)/(2(g-1))); p*/p0 = (2/(g+1))^(g/(g-1)); T*/T0 = 2/(g+1)",
     latex=r"\dot m = \frac{p_0 A^*}{\sqrt{R T_0}}\sqrt{\gamma}\left(\frac{2}{\gamma+1}\right)^{\frac{\gamma+1}{2(\gamma-1)}}",
     run=choked_mass_flow,
     parse=lambda q: _pairs(q, r"choked mass flow(?: through a nozzle throat)?|choked flow", {"p0": "p0", "pt": "p0", "t0": "T0", "tt": "T0", "a": "A",
                                                                                            "at": "A", "r": "R", **_GNAMES}, phys={"p0", "T0", "A", "R"}),
     reference=ref_choked_mass_flow,
     wrong=_safe(lambda p0, T0, A, gamma=1.4, R=R_AIR: (lambda o: {**o, "mdot": _r(o["mdot"] * math.sqrt(gamma))})(choked_mass_flow(p0, T0, A, gamma, R))),
     random_inputs=lambda rng: {"p0": _lu(rng, 1e4, 2e7), "T0": rng.uniform(200, 3500), "A": _lu(rng, 1e-6, 1.0),
                                **({"gamma": rng.uniform(1.1, 1.67)} if rng.random() < 0.5 else {}), **({"R": rng.uniform(150, 4200)} if rng.random() < 0.5 else {})},
     templates=["Choked mass flow: p0 = 500 kPa, T0 = 300 K, A = 2 cm^2", "Choked mass flow through a nozzle throat: p0 = 7 MPa, T0 = 3200 K, A = 0.05 m^2, gamma = 1.2, R = 350 J/(kg*K)"],
     examples=[{"inputs": {"p0": 1e5, "T0": 300.0, "A": 1e-3}, "outputs": {"T_star": 250.0, "mdot": _vdk(1.4) * 100.0 / math.sqrt(R_AIR * 300.0)}},
               {"inputs": {"p0": 1e5, "T0": 300.0, "A": 1e-3, "gamma": 1.4, "R": 300.0}, "outputs": {"V_star": math.sqrt(1.4 * 300.0 * 250.0)}}],
     questions=[{"q": "Choked mass flow: p0 = 100 kPa, T0 = 300 K, A = 10 cm^2", "outputs": {"T_star": 250.0}},
                {"q": "Choked mass flow: p0 = 100 kPa, T0 = 300 K, A = 0 cm^2", "outcome": "INVALID_INPUT"},
                {"q": "Choked mass flow: p0 = 100 kPa, T0 = 300 K", "outcome": "NEEDS_INPUT"}])


def _rkt_in(pc, Tc, gamma, Mw):
    g = _gam(gamma)
    _pos("the chamber pressure, chamber temperature and molar mass", pc, Tc, Mw)
    _need(1e-3 <= Mw <= 0.3, "the molar mass must lie between 1 and 300 g/mol")
    return g, RU / Mw


def characteristic_velocity(Tc, Mw, gamma):
    g, R = _rkt_in(1.0, Tc, gamma, Mw)
    return _out(c_star=math.sqrt(R * Tc) / _vdk(g), Gamma=_vdk(g))


_ROCKET_NAMES = {"pc": "pc", "p0": "pc", "tc": "Tc", "t0": "Tc", "mw": "Mw", "molar_mass": "Mw", "m_molar": "Mw", "at": "At",
                 "a_t": "At", "eps": "eps", "epsilon": "eps", "ae/at": "eps", "ar": "eps", "area_ratio": "eps", "pa": "pa",
                 "p_amb": "pa", "pe": "pe", "f": "F", "thrust": "F", **_GNAMES}
_RK_RANDOM = lambda rng: {"pc": _lu(rng, 5e5, 2.5e7), "Tc": rng.uniform(1500, 3800), "Mw": rng.uniform(0.002, 0.035),   # noqa: E731
                          "gamma": rng.uniform(1.12, 1.35)}

_add("aero_characteristic_velocity",
     name="Characteristic velocity c* of a rocket propellant", topic="rocket propulsion / characteristic velocity",
     purpose="The characteristic exhaust velocity c* = pc At / mdot of ideal rocket combustion products from the chamber temperature, molar mass and ratio of specific heats, with the Vandenkerckhove function.",
     inputs=[_phys("Tc", "temperature", "chamber (combustion) temperature"), _phys("Mw", "molar_mass", "molar mass of the combustion products (g/mol, kg/kmol)"),
             _numin("gamma", "ratio of specific heats of the products")],
     outputs=[_o("c_star", "speed", "characteristic velocity c*"), _o("Gamma", "number", "Vandenkerckhove function Gamma(gamma)")],
     primary="c_star", conditions=["Tc > 0", "1 <= Mw <= 300 g/mol", "1.05 <= gamma <= 1.7", "ideal, frozen, isentropic flow"],
     method="c* = sqrt(R Tc)/Gamma, R = Ru/Mw, Gamma = sqrt(g)(2/(g+1))^((g+1)/(2(g-1)))",
     latex=r"c^* = \frac{\sqrt{R T_c}}{\Gamma},\quad \Gamma = \sqrt{\gamma}\left(\frac{2}{\gamma+1}\right)^{\frac{\gamma+1}{2(\gamma-1)}}",
     run=characteristic_velocity,
     parse=lambda q: _pairs(q, r"characteristic velocity(?: c\*)?(?: of a rocket propellant)?|c\* of a propellant", _ROCKET_NAMES, phys={"Tc", "Mw"}),
     reference=lambda Tc, Mw, gamma: (lambda xs: {"c_star": math.sqrt(RU / Mw * Tc) / _ref_G(xs, gamma), "Gamma": _ref_G(xs, gamma)})(_ref_throat(gamma)),
     wrong=_safe(lambda Tc, Mw, gamma: _out(c_star=math.sqrt(gamma * RU / Mw * Tc) / _vdk(gamma), Gamma=_vdk(gamma))),
     random_inputs=lambda rng: {k: v for k, v in _RK_RANDOM(rng).items() if k in ("Tc", "Mw", "gamma")},
     templates=["Characteristic velocity: Tc = 3500 K, Mw = 22 g/mol, gamma = 1.2", "Characteristic velocity c*: Tc = 3200 K, Mw = 13.5 kg/kmol, gamma = 1.25"],
     examples=[{"inputs": {"Tc": 3000.0, "Mw": 0.02, "gamma": 1.2}, "outputs": {"c_star": math.sqrt(RU / 0.02 * 3000.0) / _vdk(1.2)}}],
     questions=[{"q": "Characteristic velocity: Tc = 3000 K, Mw = 20 g/mol, gamma = 1.2", "outputs": {"c_star": math.sqrt(RU / 0.02 * 3000.0) / _vdk(1.2)}},
                {"q": "Characteristic velocity: Tc = 3000 K, Mw = 20 g/mol, gamma = 2.5", "outcome": "INVALID_INPUT"},
                {"q": "Characteristic velocity: Tc = 3000 K, Mw = 20 g/mol", "outcome": "NEEDS_INPUT"}])


def _sep_check(pe, pa):
    _need(pa == 0 or pe >= 0.4 * pa, "the nozzle is so overexpanded (pe < 0.4 pa) that the flow is likely to separate; ideal thrust does not apply")


def thrust_coefficient(gamma, eps, pc, pa):
    g = _gam(gamma)
    _pos("the chamber pressure", pc)
    _need(isinstance(eps, (int, float)) and 1.0 < eps <= 500.0, "the nozzle area ratio must lie between 1 and 500")
    _need(isinstance(pa, (int, float)) and 0 <= pa < pc, "the ambient pressure must be at least 0 and below the chamber pressure")
    Me = _M_of_area(eps, g, True)
    x = _p0p(Me, g) ** -1.0
    _sep_check(x * pc, pa)
    mom = _vdk(g) * math.sqrt(2.0 * g / (g - 1.0) * (1.0 - x ** ((g - 1.0) / g)))
    return _out(CF=mom + (x - pa / pc) * eps, pe_pc=x, Me=Me, CF_vac=mom + x * eps)


def ref_thrust_coefficient(gamma, eps, pc, pa):
    d = _ref_nozzle(pc, 1000.0, gamma, 0.02, eps, pa)
    return {"CF": d["cf"], "pe_pc": d["xe"], "Me": d["Me"], "CF_vac": d["cf_vac"]}


def _cf_random(rng):
    g = rng.uniform(1.12, 1.4)
    eps = _lu(rng, 2.0, 150.0)
    pc = _lu(rng, 1e6, 2e7)
    x = 1.0 / _p0p(_M_of_area(eps, g, True), g)
    pa = 0.0 if rng.random() < 0.3 else min(101325.0, x * pc / rng.uniform(0.45, 3.0))
    return {"gamma": g, "eps": eps, "pc": pc, "pa": pa}


_add("aero_thrust_coefficient",
     name="Nozzle thrust coefficient", topic="rocket propulsion / thrust coefficient",
     purpose="The thrust coefficient CF = F/(pc At) of an ideal rocket nozzle of given expansion area ratio at an ambient pressure, with the exit pressure ratio and Mach number and the vacuum thrust coefficient.",
     inputs=[_numin("gamma", "ratio of specific heats of the exhaust"), _numin("eps", "nozzle expansion area ratio Ae/At (> 1)"),
             _phys("pc", "pressure", "chamber pressure"), _phys("pa", "pressure", "ambient pressure (0 for vacuum)")],
     outputs=[_o("CF", "number", "thrust coefficient"), _o("pe_pc", "number", "exit-to-chamber pressure ratio"),
              _o("Me", "number", "exit Mach number"), _o("CF_vac", "number", "vacuum thrust coefficient")],
     primary="CF", conditions=["1 < eps <= 500", "0 <= pa < pc", "pe >= 0.4 pa (no flow separation)", "1.05 <= gamma <= 1.7"],
     method="Me from the area ratio (supersonic); pe/pc isentropic; CF = Gamma sqrt(2g/(g-1)(1 - (pe/pc)^((g-1)/g))) + (pe - pa)/pc eps",
     latex=r"C_F = \sqrt{\frac{2\gamma^2}{\gamma-1}\left(\frac{2}{\gamma+1}\right)^{\frac{\gamma+1}{\gamma-1}}\left[1 - \left(\frac{p_e}{p_c}\right)^{\frac{\gamma-1}{\gamma}}\right]} + \frac{p_e - p_a}{p_c}\frac{A_e}{A_t}",
     run=thrust_coefficient,
     parse=lambda q: _pairs(q, r"(?:nozzle )?thrust coefficient", _ROCKET_NAMES, phys={"pc", "pa"}),
     reference=ref_thrust_coefficient,
     wrong=_safe(lambda gamma, eps, pc, pa: (lambda o: {**o, "CF": o["CF_vac"]})(thrust_coefficient(gamma, eps, pc, pa)) if pa > 0 else
                 (lambda o: {**o, "CF": _r(o["CF"] - o["pe_pc"] * eps), "CF_vac": _r(o["CF_vac"] - o["pe_pc"] * eps)})(thrust_coefficient(gamma, eps, pc, pa))),
     random_inputs=_cf_random,
     templates=["Thrust coefficient: gamma = 1.2, eps = 40, pc = 7 MPa, pa = 0 Pa", "Nozzle thrust coefficient: gamma = 1.25, eps = 12, pc = 10 MPa, pa = 101.325 kPa"],
     examples=[{"inputs": {"gamma": 1.4, "eps": 1.6875, "pc": 1e6, "pa": 0.0}, "outputs": {"Me": 2.0, "pe_pc": 1.8 ** -3.5,
                                                                                          "CF": _vdk(1.4) * math.sqrt(7.0 * (1 - 1 / 1.8)) + 1.8 ** -3.5 * 1.6875}}],
     questions=[{"q": "Thrust coefficient: gamma = 1.4, eps = 1.6875, pc = 1 MPa, pa = 0 Pa", "outputs": {"Me": 2.0}},
                {"q": "Thrust coefficient: gamma = 1.2, eps = 40, pc = 1 MPa, pa = 101325 Pa", "outcome": "INVALID_INPUT"},
                {"q": "Thrust coefficient: gamma = 1.2, eps = 40, pc = 7 MPa", "outcome": "NEEDS_INPUT"}])


def ideal_exhaust_velocity(Tc, Mw, gamma, pc, pe):
    g, R = _rkt_in(pc, Tc, gamma, Mw)
    _need(isinstance(pe, (int, float)) and 0 <= pe < pc, "the exit pressure must be at least 0 and below the chamber pressure")
    x = pe / pc
    ve = math.sqrt(2.0 * g / (g - 1.0) * R * Tc * (1.0 - x ** ((g - 1.0) / g)))
    Me = math.sqrt(2.0 / (g - 1.0) * (x ** (-(g - 1.0) / g) - 1.0)) if x > 0 else math.inf
    _need(math.isfinite(Me), "an exit pressure of zero gives an infinite exit Mach number; use a positive exit pressure")
    return _out(ve=ve, Isp=ve / G0, Me=Me)


def ref_ideal_exhaust_velocity(Tc, Mw, gamma, pc, pe):
    x = pe / pc
    ve = math.sqrt(2.0 * RU / Mw * Tc * _ref_I(x, gamma))
    return {"ve": ve, "Isp": ve / G0, "Me": math.sqrt(_ref_M2_of_x(x, gamma))}


_add("aero_ideal_exhaust_velocity",
     name="Ideal rocket exhaust velocity", topic="rocket propulsion / exhaust velocity",
     purpose="The exhaust velocity of an ideal rocket nozzle expanding combustion products from the chamber to the exit pressure, with the specific impulse at that exit pressure (optimum expansion) and the exit Mach number.",
     inputs=[_phys("Tc", "temperature", "chamber temperature"), _phys("Mw", "molar_mass", "molar mass of the products (g/mol)"),
             _numin("gamma", "ratio of specific heats"), _phys("pc", "pressure", "chamber pressure"), _phys("pe", "pressure", "nozzle exit pressure (> 0)")],
     outputs=[_o("ve", "speed", "exhaust velocity"), _o("Isp", "time", "specific impulse ve/g0 (exit pressure = ambient)"),
              _o("Me", "number", "exit Mach number")],
     primary="ve", conditions=["0 < pe < pc", "1 <= Mw <= 300 g/mol", "1.05 <= gamma <= 1.7", "ideal, frozen, isentropic expansion"],
     method="ve = sqrt(2g/(g-1) (Ru/Mw) Tc [1 - (pe/pc)^((g-1)/g)])",
     latex=r"v_e = \sqrt{\frac{2\gamma}{\gamma-1}\frac{R_u T_c}{\mathcal{M}}\left[1 - \left(\frac{p_e}{p_c}\right)^{\frac{\gamma-1}{\gamma}}\right]}",
     run=ideal_exhaust_velocity,
     parse=lambda q: _pairs(q, r"(?:ideal )?(?:rocket )?exhaust velocity", _ROCKET_NAMES, phys={"Tc", "Mw", "pc", "pe"}),
     reference=ref_ideal_exhaust_velocity,
     wrong=_safe(lambda Tc, Mw, gamma, pc, pe: (lambda o: {**o, "ve": _r(o["ve"] / math.sqrt(2.0)), "Isp": _r(o["Isp"] / math.sqrt(2.0))})(ideal_exhaust_velocity(Tc, Mw, gamma, pc, pe))),
     random_inputs=lambda rng: (lambda d: {**{k: d[k] for k in ("Tc", "Mw", "gamma", "pc")}, "pe": d["pc"] / _lu(rng, 2.0, 2000.0)})(_RK_RANDOM(rng)),
     templates=["Exhaust velocity: Tc = 3500 K, Mw = 22 g/mol, gamma = 1.2, pc = 7 MPa, pe = 101.325 kPa",
                "Ideal rocket exhaust velocity: Tc = 3300 K, Mw = 12 g/mol, gamma = 1.25, pc = 20 MPa, pe = 10 kPa"],
     examples=[{"inputs": {"Tc": 3000.0, "Mw": 0.02, "gamma": 1.4, "pc": 1e6, "pe": 1e6 / 1.8 ** 3.5},
                "outputs": {"Me": 2.0, "ve": math.sqrt(7.0 * RU / 0.02 * 3000.0 * (1 - 1 / 1.8))}}],
     questions=[{"q": "Exhaust velocity: Tc = 3000 K, Mw = 20 g/mol, gamma = 1.4, pc = 1 MPa, pe = " + repr(1e3 / 1.8 ** 3.5) + " kPa", "outputs": {"Me": 2.0}},
                {"q": "Exhaust velocity: Tc = 3000 K, Mw = 20 g/mol, gamma = 1.2, pc = 1 MPa, pe = 2 MPa", "outcome": "INVALID_INPUT"},
                {"q": "Exhaust velocity: Tc = 3000 K, Mw = 20 g/mol, gamma = 1.2, pc = 1 MPa", "outcome": "NEEDS_INPUT"}])


def optimum_area_ratio(gamma, pc, pa):
    g = _gam(gamma)
    _pos("the chamber and ambient pressures", pc, pa)
    _need(pc / pa >= 2.0 and pc / pa <= 1e6, "the chamber-to-ambient pressure ratio must lie between 2 and 1e6")
    x = pa / pc
    Me = math.sqrt(2.0 / (g - 1.0) * (x ** (-(g - 1.0) / g) - 1.0))
    return _out(eps=_area(Me, g), Me=Me, CF=_vdk(g) * math.sqrt(2.0 * g / (g - 1.0) * (1.0 - x ** ((g - 1.0) / g))))


def ref_optimum_area_ratio(gamma, pc, pa):
    x = pa / pc
    xs = _ref_throat(gamma)
    Gs = _ref_G(xs, gamma)
    return {"eps": Gs / _ref_G(x, gamma), "Me": math.sqrt(_ref_M2_of_x(x, gamma)), "CF": Gs * math.sqrt(2.0 * _ref_I(x, gamma))}


_add("aero_optimum_area_ratio",
     name="Optimum nozzle area ratio", topic="rocket propulsion / nozzle design",
     purpose="The nozzle expansion area ratio that expands the exhaust exactly to the ambient pressure (optimum expansion, maximum thrust at that pressure), with the exit Mach number and the thrust coefficient.",
     inputs=[_numin("gamma", "ratio of specific heats"), _phys("pc", "pressure", "chamber pressure"), _phys("pa", "pressure", "ambient pressure")],
     outputs=[_o("eps", "number", "optimum area ratio Ae/At"), _o("Me", "number", "exit Mach number"), _o("CF", "number", "thrust coefficient at optimum expansion")],
     primary="eps", conditions=["2 <= pc/pa <= 1e6", "1.05 <= gamma <= 1.7"],
     method="pe = pa: Me = sqrt(2/(g-1)[(pc/pa)^((g-1)/g) - 1]); eps = A/A*(Me); CF = Gamma sqrt(2g/(g-1)(1 - (pa/pc)^((g-1)/g)))",
     latex=r"M_e = \sqrt{\frac{2}{\gamma-1}\left[\left(\frac{p_c}{p_a}\right)^{\frac{\gamma-1}{\gamma}} - 1\right]},\quad \varepsilon = \frac{A}{A^*}(M_e)",
     run=optimum_area_ratio,
     parse=lambda q: _pairs(q, r"optimum (?:nozzle )?area ratio|optimum expansion ratio", _ROCKET_NAMES, phys={"pc", "pa"}),
     reference=ref_optimum_area_ratio,
     wrong=_safe(lambda gamma, pc, pa: (lambda o: {**o, "eps": _r(_area(o["Me"], gamma) ** 2)})(optimum_area_ratio(gamma, pc, pa))),
     random_inputs=lambda rng: {"gamma": rng.uniform(1.12, 1.4), "pc": _lu(rng, 1e6, 2e7), "pa": _lu(rng, 1e3, 101325.0)},
     templates=["Optimum area ratio: gamma = 1.2, pc = 7 MPa, pa = 101.325 kPa", "Optimum nozzle area ratio: gamma = 1.25, pc = 20 MPa, pa = 1 kPa"],
     examples=[{"inputs": {"gamma": 1.4, "pc": 1.8 ** 3.5 * 1e5, "pa": 1e5}, "outputs": {"Me": 2.0, "eps": 1.6875}}],
     questions=[{"q": "Optimum area ratio: gamma = 1.4, pc = " + repr(1.8 ** 3.5 * 100) + " kPa, pa = 100 kPa", "outputs": {"eps": 1.6875}},
                {"q": "Optimum area ratio: gamma = 1.2, pc = 100 kPa, pa = 100 kPa", "outcome": "INVALID_INPUT"},
                {"q": "Optimum area ratio: gamma = 1.2, pc = 7 MPa", "outcome": "NEEDS_INPUT"}])


def rocket_performance(pc, Tc, Mw, gamma, At, eps, pa):
    g, R = _rkt_in(pc, Tc, gamma, Mw)
    _pos("the throat area", At)
    _need(isinstance(eps, (int, float)) and 1.0 < eps <= 500.0, "the nozzle area ratio must lie between 1 and 500")
    _need(isinstance(pa, (int, float)) and 0 <= pa < pc, "the ambient pressure must be at least 0 and below the chamber pressure")
    Me = _M_of_area(eps, g, True)
    x = 1.0 / _p0p(Me, g)
    _sep_check(x * pc, pa)
    cstar = math.sqrt(R * Tc) / _vdk(g)
    mdot = pc * At / cstar
    ve = math.sqrt(2.0 * g / (g - 1.0) * R * Tc * (1.0 - x ** ((g - 1.0) / g)))
    F = mdot * ve + (x * pc - pa) * eps * At
    return _out(F=F, mdot=mdot, Isp=F / (mdot * G0), CF=F / (pc * At), c_star=cstar, pe=x * pc, ve=ve, Me=Me)


def ref_rocket_performance(pc, Tc, Mw, gamma, At, eps, pa):
    d = _ref_nozzle(pc, Tc, gamma, Mw, eps, pa)
    mdot = pc * At / d["cstar"]
    F = d["cf"] * pc * At
    return {"F": F, "mdot": mdot, "Isp": F / (mdot * G0), "CF": d["cf"], "c_star": d["cstar"], "pe": d["xe"] * pc, "ve": d["ve"], "Me": d["Me"]}


def _rp_random(rng):
    d = _RK_RANDOM(rng)
    c = _cf_random(rng)
    pe = d["pc"] / _p0p(_M_of_area(c["eps"], d["gamma"], True), d["gamma"])
    return {**d, "At": _lu(rng, 1e-4, 0.5), "eps": c["eps"], "pa": 0.0 if c["pa"] == 0 else min(101325.0, pe / rng.uniform(0.45, 3.0))}


def _rp_wrong(pc, Tc, Mw, gamma, At, eps, pa):
    o = rocket_performance(pc, Tc, Mw, gamma, At, eps, pa)
    F = o["mdot"] * o["ve"]                                                   # the pressure thrust left out
    if abs(F - o["F"]) <= 1e-6 * abs(o["F"]):
        F = o["F"] * 1.05
    return {**o, "F": _r(F), "Isp": _r(F / (o["mdot"] * G0)), "CF": _r(F / (pc * At))}


_add("aero_rocket_engine_performance",
     name="Ideal rocket engine performance", topic="rocket propulsion / engine performance",
     purpose="Thrust, mass flow, specific impulse, thrust coefficient, characteristic velocity and the exit pressure, velocity and Mach number of an ideal rocket engine from its chamber conditions, propellant products, throat area, expansion ratio and the ambient pressure.",
     inputs=[_phys("pc", "pressure", "chamber pressure"), _phys("Tc", "temperature", "chamber temperature"),
             _phys("Mw", "molar_mass", "molar mass of the products (g/mol)"), _numin("gamma", "ratio of specific heats"),
             _phys("At", "area", "throat area"), _numin("eps", "expansion area ratio Ae/At"), _phys("pa", "pressure", "ambient pressure (0 for vacuum)")],
     outputs=[_o("F", "force", "thrust"), _o("mdot", "mass_flow_rate", "propellant mass flow"), _o("Isp", "time", "specific impulse"),
              _o("CF", "number", "thrust coefficient"), _o("c_star", "speed", "characteristic velocity"), _o("pe", "pressure", "exit pressure"),
              _o("ve", "speed", "exit (exhaust) velocity"), _o("Me", "number", "exit Mach number")],
     primary="F", conditions=["1 < eps <= 500", "0 <= pa < pc", "pe >= 0.4 pa (no flow separation)", "1 <= Mw <= 300 g/mol", "1.05 <= gamma <= 1.7",
                              "ideal, frozen, isentropic, one-dimensional nozzle flow"],
     method="mdot = pc At/c*; Me from eps; pe = pc (1 + (g-1)/2 Me^2)^(-g/(g-1)); ve from the isentropic expansion; F = mdot ve + (pe - pa) Ae; Isp = F/(mdot g0)",
     latex=r"F = \dot m v_e + (p_e - p_a)A_e,\quad \dot m = \frac{p_c A_t}{c^*},\quad I_{sp} = \frac{F}{\dot m g_0}",
     run=rocket_performance,
     parse=lambda q: _pairs(q, r"(?:ideal )?rocket engine performance|rocket performance", _ROCKET_NAMES, phys={"pc", "Tc", "Mw", "At", "pa"}),
     reference=ref_rocket_performance,
     wrong=_safe(_rp_wrong),
     random_inputs=_rp_random,
     templates=["Rocket engine performance: pc = 7 MPa, Tc = 3500 K, Mw = 22 g/mol, gamma = 1.2, At = 0.05 m^2, eps = 40, pa = 0 Pa",
                "Ideal rocket engine performance: pc = 10 MPa, Tc = 3300 K, Mw = 20 g/mol, gamma = 1.22, At = 0.1 m^2, eps = 12, pa = 101.325 kPa"],
     examples=[{"inputs": {"pc": 1e6, "Tc": 3000.0, "Mw": 0.02, "gamma": 1.4, "At": 0.01, "eps": 1.6875, "pa": 1e6 / 1.8 ** 3.5},
                "outputs": {"Me": 2.0, "mdot": 1e6 * 0.01 * _vdk(1.4) / math.sqrt(RU / 0.02 * 3000.0),
                            "F": 1e6 * 0.01 * _vdk(1.4) / math.sqrt(RU / 0.02 * 3000.0) * math.sqrt(7.0 * RU / 0.02 * 3000.0 * (1 - 1 / 1.8))}}],
     questions=[{"q": "Rocket engine performance: pc = 1 MPa, Tc = 3000 K, Mw = 20 g/mol, gamma = 1.4, At = 0.01 m^2, eps = 1.6875, pa = 0 Pa", "outputs": {"Me": 2.0}},
                {"q": "Rocket engine performance: pc = 1 MPa, Tc = 3000 K, Mw = 20 g/mol, gamma = 1.2, At = 0.01 m^2, eps = 40, pa = 2 MPa", "outcome": "INVALID_INPUT"},
                {"q": "Rocket engine performance: pc = 1 MPa, Tc = 3000 K, Mw = 20 g/mol, gamma = 1.2, eps = 40, pa = 0 Pa", "outcome": "NEEDS_INPUT"}])


def throat_area_for_thrust(F, pc, Tc, Mw, gamma, eps, pa):
    _pos("the thrust", F)
    o = rocket_performance(pc, Tc, Mw, gamma, 1.0, eps, pa)
    _need(o["CF"] > 0, "the thrust coefficient is not positive at this ambient pressure")
    At = F / (o["CF"] * pc)
    return _out(At=At, Ae=At * eps, mdot=pc * At / o["c_star"], Isp=o["Isp"])


def ref_throat_area_for_thrust(F, pc, Tc, Mw, gamma, eps, pa):
    d = _ref_nozzle(pc, Tc, gamma, Mw, eps, pa)
    At = F / (d["cf"] * pc)
    return {"At": At, "Ae": At * eps, "mdot": pc * At / d["cstar"], "Isp": d["cf"] * d["cstar"] / G0}


_add("aero_throat_area_for_thrust",
     name="Throat area of a rocket engine for a required thrust", topic="rocket propulsion / engine sizing",
     purpose="The throat and exit areas, mass flow and specific impulse of an ideal rocket engine sized to give a required thrust at an ambient pressure.",
     inputs=[_phys("F", "force", "required thrust (N, kN, lbf)"), _phys("pc", "pressure", "chamber pressure"), _phys("Tc", "temperature", "chamber temperature"),
             _phys("Mw", "molar_mass", "molar mass of the products"), _numin("gamma", "ratio of specific heats"),
             _numin("eps", "expansion area ratio Ae/At"), _phys("pa", "pressure", "ambient pressure (0 for vacuum)")],
     outputs=[_o("At", "area", "throat area"), _o("Ae", "area", "exit area"), _o("mdot", "mass_flow_rate", "propellant mass flow"),
              _o("Isp", "time", "specific impulse")],
     primary="At", conditions=["F > 0", "1 < eps <= 500", "0 <= pa < pc", "pe >= 0.4 pa", "1.05 <= gamma <= 1.7"],
     method="At = F/(CF pc); Ae = eps At; mdot = pc At/c*; Isp = c* CF/g0",
     latex=r"A_t = \frac{F}{C_F\,p_c},\quad \dot m = \frac{p_c A_t}{c^*},\quad I_{sp} = \frac{c^* C_F}{g_0}",
     run=throat_area_for_thrust,
     parse=lambda q: _pairs(q, r"throat area(?: of a rocket engine)? for (?:a )?(?:required )?thrust", _ROCKET_NAMES, phys={"F", "pc", "Tc", "Mw", "pa"}),
     reference=ref_throat_area_for_thrust,
     wrong=_safe(lambda F, pc, Tc, Mw, gamma, eps, pa: (lambda o: {**o, "At": _r(o["At"] * eps), "Ae": _r(o["Ae"] * eps)})(throat_area_for_thrust(F, pc, Tc, Mw, gamma, eps, pa))),
     random_inputs=lambda rng: (lambda d: {"F": _lu(rng, 1e2, 1e7), **{k: d[k] for k in ("pc", "Tc", "Mw", "gamma", "eps", "pa")}})(_rp_random(rng)),
     templates=["Throat area for thrust: F = 100 kN, pc = 7 MPa, Tc = 3500 K, Mw = 22 g/mol, gamma = 1.2, eps = 40, pa = 0 Pa",
                "Throat area of a rocket engine for a required thrust: F = 2 MN, pc = 10 MPa, Tc = 3300 K, Mw = 20 g/mol, gamma = 1.22, eps = 15, pa = 101.325 kPa"],
     examples=[{"inputs": {"F": 1000.0, "pc": 1e6, "Tc": 3000.0, "Mw": 0.02, "gamma": 1.4, "eps": 1.6875, "pa": 1e6 / 1.8 ** 3.5},
                "outputs": {"At": 1000.0 / (1e6 * _vdk(1.4) * math.sqrt(7.0 * (1 - 1 / 1.8)))}}],
     questions=[{"q": "Throat area for thrust: F = 1 kN, pc = 1 MPa, Tc = 3000 K, Mw = 20 g/mol, gamma = 1.4, eps = 1.6875, pa = " + repr(1e3 / 1.8 ** 3.5) + " kPa",
                 "outputs": {"At": 1000.0 / (1e6 * _vdk(1.4) * math.sqrt(7.0 * (1 - 1 / 1.8)))}},
                {"q": "Throat area for thrust: F = -1 kN, pc = 1 MPa, Tc = 3000 K, Mw = 20 g/mol, gamma = 1.4, eps = 2, pa = 0 Pa", "outcome": "INVALID_INPUT"},
                {"q": "Throat area for thrust: F = 1 kN, pc = 1 MPa, Tc = 3000 K, Mw = 20 g/mol, gamma = 1.4, eps = 2", "outcome": "NEEDS_INPUT"}])


def nozzle_normal_shock(pc, pb, eps, gamma=1.4):
    g = _gam(gamma)
    _pos("the chamber and back pressures", pc, pb)
    _need(isinstance(eps, (int, float)) and 1.05 <= eps <= 100.0, "the nozzle area ratio must lie between 1.05 and 100")
    Msub, Msup = _M_of_area(eps, g, False), _M_of_area(eps, g, True)
    p_hi = pc / _p0p(Msub, g)                                   # subsonic isentropic exit: the shock reaches the throat
    _M2e, p21e, _r21, _p0r = _shock(Msup, g)
    p_lo = pc / _p0p(Msup, g) * p21e                             # the normal shock stands at the exit
    _need(p_lo < pb < p_hi, "a normal shock stands inside the nozzle only for back pressures between the exit-shock and the critical subsonic pressures")
    target = pb / pc * eps
    Me = _bisect(lambda M: _area(M, g) / _p0p(M, g) - target, 1e-9, 1.0)
    p0r = pb / pc * _p0p(Me, g)
    M1 = _bisect(lambda M: _shock(M, g)[3] - p0r, 1.0, Msup)
    M2 = _shock(M1, g)[0]
    return _out(As_At=_area(M1, g), M1=M1, M2=M2, Me=Me, p02_p01=p0r)


def ref_nozzle_normal_shock(pc, pb, eps, gamma=1.4):
    """March through the nozzle for a trial shock Mach number M1: supersonic area from the area-velocity integral, the
    shock from the conservation laws, the new sonic area from mass conservation, the subsonic exit by the integral; then
    choose M1 so the exit pressure equals the back pressure (brentq)."""
    from scipy.optimize import brentq
    g = gamma
    Msup = _ref_M_of_area(eps, g, True)

    def state(M1):
        sh = _ref_shock_all(M1, g)
        Me = _ref_M_of_area(eps * sh["p02_p01"], g, False)
        return sh, Me, pc * sh["p02_p01"] * _ref_x_of_M(Me, g)

    M1 = brentq(lambda M: state(M)[2] - pb, 1.0 + 1e-6, Msup, xtol=1e-14, rtol=8.9e-16, maxiter=300)
    sh, Me, _pe = state(M1)
    return {"As_At": math.exp(_ref_lnA(M1, g)), "M1": M1, "M2": sh["M2"], "Me": Me, "p02_p01": sh["p02_p01"]}


def _nns_pb(pc, M1, eps, g):
    """The back pressure that holds a normal shock at Mach M1 in a nozzle of area ratio eps (worked examples)."""
    p0r = _shock(M1, g)[3]
    return pc * p0r / _p0p(_M_of_area(eps * p0r, g, False), g)


def _nns_random(rng):
    g = rng.choice([1.4, rng.uniform(1.15, 1.67)])
    eps = rng.uniform(1.3, 6.0)
    pc = _lu(rng, 2e5, 5e6)
    Msub, Msup = _M_of_area(eps, g, False), _M_of_area(eps, g, True)
    M1 = 1.0 + rng.uniform(0.08, 0.95) * (Msup - 1.0)
    _M2, _p21, _r21, p0r = _shock(M1, g)
    Me = _M_of_area(eps * p0r, g, False)
    pb = pc * p0r / _p0p(Me, g)
    return {"pc": pc, "pb": pb, "eps": eps, **({} if g == 1.4 else {"gamma": g})}


_add("aero_nozzle_normal_shock",
     name="Normal shock position in a converging-diverging nozzle", topic="nozzles / off-design operation",
     purpose="Where a normal shock stands in the diverging part of a converging-diverging nozzle for a given back pressure: the area ratio at the shock, the Mach numbers before and after it, the exit Mach number and the stagnation-pressure loss.",
     inputs=[_phys("pc", "pressure", "reservoir (stagnation) pressure"), _phys("pb", "pressure", "back pressure at the exit"),
             _numin("eps", "exit-to-throat area ratio Ae/At"), _GAMMA_IN],
     outputs=[_o("As_At", "number", "area ratio at the shock As/At"), _o("M1", "number", "Mach number just before the shock"),
              _o("M2", "number", "Mach number just after the shock"), _o("Me", "number", "exit Mach number (subsonic)"),
              _o("p02_p01", "number", "stagnation pressure ratio across the shock")],
     primary="As_At", conditions=["1.05 <= eps <= 100", "back pressure between the exit-shock and the critical subsonic exit pressures", "1.05 <= gamma <= 1.7"],
     method="mass flow: pe Ae/(p01 At) = (p/p0)(A/A*) at Me (subsonic) gives Me; p02/p01 = pe/(p01 (p/p0)(Me)); M1 from the normal-shock p02/p01; As/At = A/A*(M1)",
     latex=r"\frac{p_e A_e}{p_{01} A_t} = \frac{p_e}{p_{02}}\frac{A_e}{A_2^*}\Big|_{M_e},\quad \frac{p_{02}}{p_{01}} = \frac{A_t}{A_2^*}",
     run=nozzle_normal_shock,
     parse=lambda q: _pairs(q, r"normal shock (?:position )?in a (?:converging-diverging |c-d |cd )?nozzle", {"pc": "pc", "p0": "pc", "pb": "pb", "p_back": "pb",
                                                                                                          "eps": "eps", "ae/at": "eps", "ar": "eps", **_GNAMES}, phys={"pc", "pb"}),
     reference=ref_nozzle_normal_shock,
     wrong=_safe(lambda pc, pb, eps, gamma=1.4: (lambda o: {**o, "As_At": _r(_area(o["M2"], gamma))})(nozzle_normal_shock(pc, pb, eps, gamma))),
     random_inputs=_nns_random,
     templates=["Normal shock in a nozzle: pc = 1 MPa, pb = 700 kPa, eps = 3", "Normal shock position in a converging-diverging nozzle: pc = 500 kPa, pb = 400 kPa, eps = 2, gamma = 1.4"],
     examples=[{"inputs": {"pc": 1e6, "pb": _nns_pb(1e6, 2.0, 3.0, 1.4), "eps": 3.0}, "outputs": {"M1": 2.0, "As_At": 1.6875}},
               {"inputs": {"pc": 1e6, "pb": _nns_pb(1e6, 1.5, 2.0, 1.4), "eps": 2.0}, "outputs": {"M1": 1.5, "As_At": _area(1.5, 1.4)}}],
     questions=[{"q": "Normal shock in a nozzle: pc = 1 MPa, pb = " + repr(_nns_pb(1e3, 2.0, 3.0, 1.4)) + " kPa, eps = 3", "outputs": {"M1": 2.0}},
                {"q": "Normal shock in a nozzle: pc = 1 MPa, pb = 999 kPa, eps = 3", "outcome": "INVALID_INPUT"},
                {"q": "Normal shock in a nozzle: pc = 1 MPa, eps = 3", "outcome": "NEEDS_INPUT"}])


def critical_mach(Cp0, rule):
    _need(isinstance(Cp0, (int, float)) and -3.0 <= Cp0 < -0.01, "the incompressible minimum pressure coefficient must lie between -3 and -0.01")
    _need(rule in ("prandtl-glauert", "karman-tsien"), "the compressibility rule must be prandtl-glauert or karman-tsien")
    g = GAMMA_AIR

    def cp_cr(M):
        return 2.0 / (g * M * M) * ((_t0t(M, g) / _t0t(1.0, g)) ** (g / (g - 1.0)) - 1.0)

    def cp(M):
        b = math.sqrt(1.0 - M * M)
        return Cp0 / b if rule == "prandtl-glauert" else Cp0 / (b + M * M / (1.0 + b) * Cp0 / 2.0)

    top = 1.0 - 1e-12
    if rule == "karman-tsien":                                  # the Karman-Tsien form is singular where its denominator vanishes
        top = _bisect(lambda m: math.sqrt(1.0 - m * m) + m * m / (1.0 + math.sqrt(1.0 - m * m)) * Cp0 / 2.0, 1e-3, 1.0) * (1.0 - 1e-12)
    M = _bisect(lambda m: cp(m) - cp_cr(m), 1e-3, top)
    return _out(M_cr=M, Cp_cr=cp_cr(M))


def ref_critical_mach(Cp0, rule):
    """Sonic-point pressure from the Euler integral: Cp* = (p*/p_inf - 1)/(g M^2/2), p*/p_inf = x(1)/x(M); brentq on M."""
    from scipy.optimize import brentq
    g = GAMMA_AIR
    xs = _ref_x_of_M(1.0, g)

    def cp_cr(M):
        return (xs / _ref_x_of_M(M, g) - 1.0) / (0.5 * g * M * M)

    def cp(M):
        b = math.sqrt(1.0 - M * M)
        return Cp0 / b if rule == "prandtl-glauert" else Cp0 / (b + M * M / (1.0 + b) * Cp0 / 2.0)

    top = 1.0 - 1e-9
    if rule == "karman-tsien":
        top = brentq(lambda m: math.sqrt(1.0 - m * m) + m * m / (1.0 + math.sqrt(1.0 - m * m)) * Cp0 / 2.0, 1e-3, 1.0, xtol=1e-15) * (1.0 - 1e-10)
    M = brentq(lambda m: cp(m) - cp_cr(m), 1e-3, top, xtol=1e-15, rtol=8.9e-16, maxiter=500)
    return {"M_cr": M, "Cp_cr": cp_cr(M)}


def _crit_wrong(Cp0, rule):
    g = GAMMA_AIR
    f = lambda m: Cp0 - 2.0 / (g * m * m) * ((_t0t(m, g) / 1.2) ** 3.5 - 1.0)      # noqa: E731  no compressibility correction
    M = _bisect(f, 1e-3, 1.0 - 1e-12)
    return _out(M_cr=M, Cp_cr=Cp0)


_add("aero_critical_mach",
     name="Critical Mach number of an airfoil", topic="aerodynamics / compressibility",
     purpose="The free-stream Mach number at which the flow first reaches sonic speed at the airfoil's minimum-pressure point, from its incompressible minimum pressure coefficient and a compressibility rule (Prandtl-Glauert or Karman-Tsien), air (gamma = 1.4).",
     inputs=[_numin("Cp0", "incompressible minimum pressure coefficient (negative)"),
             _choice("rule", ["prandtl-glauert", "karman-tsien"], "compressibility correction: prandtl-glauert or karman-tsien")],
     outputs=[_o("M_cr", "number", "critical Mach number"), _o("Cp_cr", "number", "critical (sonic) pressure coefficient at M_cr")],
     primary="M_cr", conditions=["-3 <= Cp0 < -0.01", "subsonic, small-perturbation compressibility rules", "air, gamma = 1.4"],
     method="Cp_cr(M) = 2/(g M^2) [((1 + (g-1)/2 M^2)/(1 + (g-1)/2))^(g/(g-1)) - 1]; equate to Cp0/sqrt(1 - M^2) (or the Karman-Tsien form) and solve",
     latex=r"C_{p,cr} = \frac{2}{\gamma M^2}\left[\left(\frac{2 + (\gamma-1)M^2}{\gamma+1}\right)^{\frac{\gamma}{\gamma-1}} - 1\right] = \frac{C_{p0}}{\sqrt{1 - M^2}}",
     run=critical_mach,
     parse=lambda q: _pairs(q, r"critical mach(?: number)?(?: of an airfoil)?", {"cp0": "Cp0", "cpmin": "Cp0", "rule": "rule", "correction": "rule"}, words={"rule"}),
     reference=ref_critical_mach,
     wrong=_safe(_crit_wrong),
     random_inputs=lambda rng: {"Cp0": -_lu(rng, 0.05, 2.0), "rule": rng.choice(["prandtl-glauert", "karman-tsien"])},
     templates=["Critical Mach: Cp0 = -0.43, rule = prandtl-glauert", "Critical Mach number of an airfoil: Cp0 = -0.6, rule = karman-tsien"],
     examples=[{"inputs": {"Cp0": -0.43, "rule": "prandtl-glauert"}, "outputs": {"M_cr": critical_mach(-0.43, "prandtl-glauert")["M_cr"]}}],
     questions=[{"q": "Critical Mach: Cp0 = -0.43, rule = prandtl-glauert", "outputs": {"M_cr": critical_mach(-0.43, "prandtl-glauert")["M_cr"]}},
                {"q": "Critical Mach: Cp0 = 0.5, rule = prandtl-glauert", "outcome": "INVALID_INPUT"},
                {"q": "Critical Mach: Cp0 = -0.43", "outcome": "NEEDS_INPUT"}])


# ================================================================================================ rocket performance
def _ivp(f, t1, y0, t0=0.0):
    """scipy DOP853 integration of a non-dimensional ODE to t1 (reference side only); returns the final state."""
    from scipy.integrate import solve_ivp
    s = solve_ivp(f, (t0, t1), y0, method="DOP853", rtol=1e-13, atol=1e-15)
    if not s.success:
        raise RuntimeError(s.message)
    return s.y[:, -1]


def _ref_burn(R, g_c_tb=0.0):
    """Constant-thrust, constant-mass-flow burn in units m0 = 1, c = 1, tb = 1: integrate m' = -(1 - 1/R),
    v' = (1 - 1/R)/m - g tb/c, h' = v from t = 0 to 1; returns (v, h) at burnout."""
    k = 1.0 - 1.0 / R
    y = _ivp(lambda t, y: [-k, k / y[0] - g_c_tb, y[1]], 1.0, [1.0, 0.0, 0.0])
    return y[1], y[2]


def _isp_in(Isp):
    _need(isinstance(Isp, (int, float)) and 1.0 <= Isp <= 10000.0, "the specific impulse must lie between 1 s and 10000 s")


def rocket_delta_v(Isp, m0, mf):
    _isp_in(Isp)
    _pos("the initial and final masses", m0, mf)
    _need(mf < m0, "the final (burnout) mass must be smaller than the initial mass")
    return _out(dv=Isp * G0 * math.log(m0 / mf))


_add("aero_rocket_delta_v",
     name="Rocket delta-v from specific impulse (Tsiolkovsky)", topic="rocket performance / rocket equation",
     purpose="The ideal velocity change of a rocket (no gravity or drag) from its specific impulse and its initial and burnout masses.",
     inputs=[_phys("Isp", "time", "specific impulse (s)"), _phys("m0", "mass", "initial (lift-off or ignition) mass"),
             _phys("mf", "mass", "final (burnout) mass")],
     outputs=[_o("dv", "speed", "ideal velocity change delta-v")],
     primary="dv", conditions=["1 s <= Isp <= 10000 s", "0 < mf < m0", "no gravity, no drag"],
     method="dv = Isp g0 ln(m0/mf)", latex=r"\Delta v = I_{sp}\,g_0\ln\frac{m_0}{m_f}",
     run=rocket_delta_v,
     parse=lambda q: _pairs(q, r"rocket delta-?v(?: from specific impulse)?(?: \(tsiolkovsky\))?|tsiolkovsky", {"isp": "Isp", "m0": "m0", "mf": "mf", "m1": "mf"}, phys={"Isp", "m0", "mf"}),
     reference=lambda Isp, m0, mf: {"dv": Isp * G0 * _ref_burn(m0 / mf)[0]},
     wrong=_safe(lambda Isp, m0, mf: _out(dv=Isp * G0 * math.log10(m0 / mf))),
     random_inputs=lambda rng: (lambda m0: {"Isp": rng.uniform(150, 4500), "m0": m0, "mf": m0 / _lu(rng, 1.05, 30.0)})(_lu(rng, 10.0, 3e6)),
     templates=["Rocket delta-v: Isp = 300 s, m0 = 50000 kg, mf = 15000 kg", "Tsiolkovsky: Isp = 450 s, m0 = 120 t, mf = 30 t"],
     examples=[{"inputs": {"Isp": 300.0, "m0": math.e * 1000.0, "mf": 1000.0}, "outputs": {"dv": 300.0 * G0}},
               {"inputs": {"Isp": 450.0, "m0": 4000.0, "mf": 1000.0}, "outputs": {"dv": 450.0 * G0 * math.log(4.0)}}],
     questions=[{"q": "Rocket delta-v: Isp = 450 s, m0 = 4000 kg, mf = 1000 kg", "outputs": {"dv": 450.0 * G0 * math.log(4.0)}},
                {"q": "Rocket delta-v: Isp = 300 s, m0 = 1000 kg, mf = 2000 kg", "outcome": "INVALID_INPUT"},
                {"q": "Rocket delta-v: Isp = 300 s, m0 = 1000 kg", "outcome": "NEEDS_INPUT"}])


def propellant_for_delta_v(dv, Isp, mf):
    _isp_in(Isp)
    _pos("the velocity change and the burnout mass", dv, mf)
    R = math.exp(dv / (Isp * G0))
    _need(R <= 1e4, "the mass ratio would exceed 10000")
    return _out(m0=mf * R, mp=mf * (R - 1.0), R=R)


def ref_propellant_for_delta_v(dv, Isp, mf):
    """Momentum balance m dv = -c dm integrated backwards in v (scipy DOP853) from the burnout mass."""
    c = Isp * G0
    m = _ivp(lambda v, y: [-y[0]], -dv / c, [1.0])[0]               # unit burnout mass, v in units of c
    return {"m0": m * mf, "mp": (m - 1.0) * mf, "R": m}


_add("aero_propellant_for_delta_v",
     name="Propellant mass for a required delta-v", topic="rocket performance / rocket equation",
     purpose="The propellant mass, initial mass and mass ratio a rocket stage needs to give a velocity change, from its specific impulse and burnout mass (structure plus payload).",
     inputs=[_phys("dv", "speed", "required velocity change (m/s, km/s)"), _phys("Isp", "time", "specific impulse (s)"),
             _phys("mf", "mass", "burnout mass (dry mass plus payload)")],
     outputs=[_o("m0", "mass", "initial mass"), _o("mp", "mass", "propellant mass"), _o("R", "number", "mass ratio m0/mf")],
     primary="mp", conditions=["dv > 0", "1 s <= Isp <= 10000 s", "mf > 0", "mass ratio <= 10000"],
     method="R = exp(dv/(Isp g0)); m0 = R mf; mp = (R - 1) mf",
     latex=r"\frac{m_0}{m_f} = e^{\Delta v/(I_{sp} g_0)},\quad m_p = m_f\left(e^{\Delta v/(I_{sp} g_0)} - 1\right)",
     run=propellant_for_delta_v,
     parse=lambda q: _pairs(q, r"propellant (?:mass )?for (?:a )?(?:required )?delta-?v", {"dv": "dv", "delta_v": "dv", "isp": "Isp", "mf": "mf", "m_dry": "mf"}, phys={"dv", "Isp", "mf"}),
     reference=ref_propellant_for_delta_v,
     wrong=_safe(lambda dv, Isp, mf: (lambda R: _out(m0=mf * R, mp=mf * (R - 1.0), R=R))(1.0 + dv / (Isp * G0))),
     random_inputs=lambda rng: {"dv": rng.uniform(100.0, 12000.0), "Isp": rng.uniform(250.0, 4500.0), "mf": _lu(rng, 10.0, 1e5)},
     templates=["Propellant for delta-v: dv = 4 km/s, Isp = 320 s, mf = 2000 kg", "Propellant mass for a required delta-v: dv = 1500 m/s, Isp = 450 s, mf = 3 t"],
     examples=[{"inputs": {"dv": 300.0 * G0, "Isp": 300.0, "mf": 1000.0}, "outputs": {"m0": 1000.0 * math.e, "mp": 1000.0 * (math.e - 1.0)}}],
     questions=[{"q": "Propellant for delta-v: dv = " + repr(300.0 * G0) + " m/s, Isp = 300 s, mf = 1000 kg", "outputs": {"R": math.e}},
                {"q": "Propellant for delta-v: dv = -3 km/s, Isp = 300 s, mf = 1000 kg", "outcome": "INVALID_INPUT"},
                {"q": "Propellant for delta-v: dv = 3 km/s, Isp = 300 s", "outcome": "NEEDS_INPUT"}])


def _vert_in(Isp, m0, mf, tb, g):
    _isp_in(Isp)
    _pos("the masses, burn time and gravity", m0, mf, tb, g)
    _need(mf < m0, "the burnout mass must be smaller than the initial mass")
    c = Isp * G0
    _need(c * (m0 - mf) / tb > 1.001 * m0 * g, "the thrust must exceed the initial weight (thrust-to-weight above 1)")
    return c, m0 / mf


def gravity_loss(Isp, m0, mf, tb, g=G0):
    c, R = _vert_in(Isp, m0, mf, tb, g)
    dvi = c * math.log(R)
    return _out(v_b=dvi - g * tb, dv_ideal=dvi, loss=g * tb)


def ref_gravity_loss(Isp, m0, mf, tb, g=G0):
    c, R = Isp * G0, m0 / mf
    vb = _ref_burn(R, g * tb / c)[0] * c
    vi = _ref_burn(R)[0] * c
    return {"v_b": vb, "dv_ideal": vi, "loss": vi - vb}


def _vert_random(rng):
    Isp = rng.uniform(200.0, 450.0)
    m0 = _lu(rng, 100.0, 3e6)
    R = _lu(rng, 1.5, 12.0)
    tw = rng.uniform(1.2, 4.0)                                 # initial thrust-to-weight ratio
    tb = Isp * G0 * (m0 - m0 / R) / (tw * m0 * G0)
    return {"Isp": Isp, "m0": m0, "mf": m0 / R, "tb": tb, **({"g": rng.uniform(1.6, 9.81)} if rng.random() < 0.3 else {})}


_VERT_NAMES = {"isp": "Isp", "m0": "m0", "mf": "mf", "tb": "tb", "t_b": "tb", "burn_time": "tb", "g": "g"}

_add("aero_delta_v_gravity_loss",
     name="Gravity loss of a vertical rocket burn", topic="rocket performance / gravity losses",
     purpose="The burnout speed of a rocket climbing vertically under constant thrust and constant gravity (no drag), its ideal delta-v and the gravity loss g tb.",
     inputs=[_phys("Isp", "time", "specific impulse"), _phys("m0", "mass", "initial mass"), _phys("mf", "mass", "burnout mass"),
             _phys("tb", "time", "burn time"), _phys("g", "acceleration", "gravitational acceleration (default 9.80665 m/s^2)", required=False)],
     outputs=[_o("v_b", "speed", "vertical speed at burnout"), _o("dv_ideal", "speed", "ideal delta-v (no gravity)"),
              _o("loss", "speed", "gravity loss")],
     primary="v_b", conditions=["0 < mf < m0", "thrust-to-weight above 1 at lift-off", "constant thrust and mass flow, constant g, no drag, vertical flight"],
     method="v_b = Isp g0 ln(m0/mf) - g tb", latex=r"v_b = I_{sp}g_0\ln\frac{m_0}{m_f} - g\,t_b",
     run=gravity_loss,
     parse=lambda q: _pairs(q, r"gravity loss(?: of a vertical (?:rocket )?burn)?|delta-?v with gravity loss", _VERT_NAMES, phys={"Isp", "m0", "mf", "tb", "g"}),
     reference=ref_gravity_loss,
     wrong=_safe(lambda Isp, m0, mf, tb, g=G0: (lambda c: _out(v_b=c * math.log(m0 / mf) - 0.5 * g * tb, dv_ideal=c * math.log(m0 / mf), loss=0.5 * g * tb))(Isp * G0)),
     random_inputs=_vert_random,
     templates=["Gravity loss: Isp = 300 s, m0 = 100 t, mf = 30 t, tb = 120 s", "Gravity loss of a vertical burn: Isp = 250 s, m0 = 500 kg, mf = 200 kg, tb = 30 s, g = 1.62 m/s^2"],
     examples=[{"inputs": {"Isp": 300.0, "m0": 1000.0 * math.e, "mf": 1000.0, "tb": 100.0}, "outputs": {"v_b": 300.0 * G0 - 100.0 * G0, "loss": 100.0 * G0}}],
     questions=[{"q": "Gravity loss: Isp = 300 s, m0 = 4 t, mf = 1 t, tb = 100 s", "outputs": {"loss": 100.0 * G0}},
                {"q": "Gravity loss: Isp = 300 s, m0 = 4 t, mf = 1 t, tb = 1000 s", "outcome": "INVALID_INPUT"},
                {"q": "Gravity loss: Isp = 300 s, m0 = 4 t, mf = 1 t", "outcome": "NEEDS_INPUT"}])


def vertical_rocket(Isp, m0, mf, tb, g=G0):
    c, R = _vert_in(Isp, m0, mf, tb, g)
    vb = c * math.log(R) - g * tb
    hb = c * tb * (1.0 - math.log(R) / (R - 1.0)) - 0.5 * g * tb * tb
    _need(vb > 0, "the rocket does not climb at burnout")
    return _out(v_b=vb, h_b=hb, h_max=hb + vb * vb / (2.0 * g), t_apex=tb + vb / g)


def ref_vertical_rocket(Isp, m0, mf, tb, g=G0):
    from scipy.integrate import quad
    c = Isp * G0
    v, h = _ref_burn(m0 / mf, g * tb / c)
    vb, hb = v * c, h * c * tb
    dh, _ = quad(lambda w: w / g, 0.0, vb, epsabs=0, epsrel=1e-13)        # coast: dh/dv = -v/g
    dt, _ = quad(lambda w: 1.0 / g, 0.0, vb, epsabs=0, epsrel=1e-13)
    return {"v_b": vb, "h_b": hb, "h_max": hb + dh, "t_apex": tb + dt}


_add("aero_vertical_rocket_trajectory",
     name="Burnout height and apogee of a vertical rocket", topic="rocket performance / sounding rockets",
     purpose="Burnout speed and height, apogee height and time to apogee of a rocket launched vertically under constant thrust and mass flow and constant gravity, neglecting drag.",
     inputs=[_phys("Isp", "time", "specific impulse"), _phys("m0", "mass", "initial mass"), _phys("mf", "mass", "burnout mass"),
             _phys("tb", "time", "burn time"), _phys("g", "acceleration", "gravitational acceleration (default 9.80665 m/s^2)", required=False)],
     outputs=[_o("v_b", "speed", "burnout speed"), _o("h_b", "length", "burnout height"), _o("h_max", "length", "apogee height"),
              _o("t_apex", "time", "time from lift-off to apogee")],
     primary="h_max", conditions=["0 < mf < m0", "thrust-to-weight above 1", "constant thrust, constant g, no drag", "vertical flight"],
     method="v_b = c ln R - g tb; h_b = c tb (1 - ln R/(R - 1)) - g tb^2/2; h_max = h_b + v_b^2/(2g); t_apex = tb + v_b/g (c = Isp g0, R = m0/mf)",
     latex=r"h_b = c\,t_b\left(1 - \frac{\ln R}{R - 1}\right) - \frac{1}{2}g t_b^2,\quad h_{max} = h_b + \frac{v_b^2}{2g}",
     run=vertical_rocket,
     parse=lambda q: _pairs(q, r"(?:burnout height and apogee of a )?vertical rocket(?: trajectory)?|sounding rocket apogee", _VERT_NAMES, phys={"Isp", "m0", "mf", "tb", "g"}),
     reference=ref_vertical_rocket,
     wrong=_safe(lambda Isp, m0, mf, tb, g=G0: (lambda o: {**o, "h_b": _r(o["h_b"] + 0.5 * g * tb * tb), "h_max": _r(o["h_max"] + 0.5 * g * tb * tb)})(vertical_rocket(Isp, m0, mf, tb, g))),
     random_inputs=_vert_random,
     templates=["Vertical rocket: Isp = 250 s, m0 = 1000 kg, mf = 400 kg, tb = 40 s", "Burnout height and apogee of a vertical rocket: Isp = 220 s, m0 = 50 kg, mf = 20 kg, tb = 10 s"],
     examples=[{"inputs": {"Isp": 300.0, "m0": 1000.0 * math.e, "mf": 1000.0, "tb": 100.0},
                "outputs": {"v_b": 200.0 * G0, "h_b": 300.0 * G0 * 100.0 * (1 - 1 / (math.e - 1)) - 0.5 * G0 * 1e4}}],
     questions=[{"q": "Vertical rocket: Isp = 300 s, m0 = " + repr(math.e * 1000) + " kg, mf = 1000 kg, tb = 100 s", "outputs": {"t_apex": 300.0}},
                {"q": "Vertical rocket: Isp = 300 s, m0 = 1000 kg, mf = 400 kg, tb = 1000 s", "outcome": "INVALID_INPUT"},
                {"q": "Vertical rocket: Isp = 300 s, m0 = 1000 kg, mf = 400 kg", "outcome": "NEEDS_INPUT"}])


def multistage_delta_v(Isp, m0, mf):
    _need(len(Isp) == len(m0) == len(mf), "give the same number of stages in Isp, m0 and mf")
    dvs = []
    for i, (s, a, b) in enumerate(zip(Isp, m0, mf)):
        _isp_in(s)
        _need(0 < b < a, f"stage {i + 1}: the burnout mass must be positive and below the ignition mass")
        if i > 0:
            _need(a < mf[i - 1], f"stage {i + 1}: its ignition mass must be below the burnout mass of stage {i} (a stage is dropped)")
        dvs.append(s * G0 * math.log(a / b))
    return _out(dv=sum(dvs), dv_stages=dvs)


def ref_multistage_delta_v(Isp, m0, mf):
    dvs = [s * G0 * _ref_burn(a / b)[0] for s, a, b in zip(Isp, m0, mf)]
    return {"dv": sum(dvs), "dv_stages": dvs}


def _ms_random(rng):
    n = rng.randint(2, 4)
    m = _lu(rng, 1e3, 3e6)
    isp, m0, mf = [], [], []
    for _ in range(n):
        m0.append(m)
        mf.append(m / _lu(rng, 1.5, 8.0))
        isp.append(rng.uniform(250.0, 460.0))
        m = mf[-1] * rng.uniform(0.2, 0.8)
    return {"Isp": isp, "m0": m0, "mf": mf}


_add("aero_multistage_delta_v",
     name="Delta-v of a multistage rocket", topic="rocket performance / staging",
     purpose="The total ideal velocity change of a multistage rocket and the contribution of each stage, from each stage's specific impulse and the vehicle mass at its ignition and at its burnout.",
     inputs=[_vecin("Isp", "specific impulse of each stage in seconds, e.g. '[290, 340]'", 1, 8),
             _vecin("m0", "vehicle mass at each stage's ignition in kilograms", 1, 8),
             _vecin("mf", "vehicle mass at each stage's burnout in kilograms", 1, 8)],
     outputs=[_o("dv", "speed", "total delta-v"), _o("dv_stages", "number", "delta-v of each stage in m/s")],
     primary="dv", conditions=["equal numbers of stages", "0 < mf < m0 for each stage", "each ignition mass below the previous burnout mass", "no gravity or drag"],
     method="dv = sum of Isp_i g0 ln(m0_i/mf_i)", latex=r"\Delta v = \sum_i I_{sp,i}\,g_0\ln\frac{m_{0,i}}{m_{f,i}}",
     run=multistage_delta_v,
     parse=lambda q: _pairs(q, r"(?:delta-?v of a )?multistage rocket(?: delta-?v)?|staged rocket delta-?v", {"isp": "Isp", "m0": "m0", "mf": "mf"}, vectors={"Isp", "m0", "mf"}),
     reference=ref_multistage_delta_v,
     wrong=_safe(lambda Isp, m0, mf: (lambda d: _out(dv=Isp[0] * G0 * math.log(m0[0] / mf[-1]), dv_stages=d))(multistage_delta_v(Isp, m0, mf)["dv_stages"])),
     random_inputs=_ms_random,
     templates=["Multistage rocket: Isp = [290, 340], m0 = [120000, 30000], mf = [45000, 9000]",
                "Delta-v of a multistage rocket: Isp = [270, 320, 450], m0 = [500000, 120000, 25000], mf = [180000, 40000, 8000]"],
     examples=[{"inputs": {"Isp": "300, 300", "m0": [math.e * 1000.0, 500.0], "mf": [1000.0, 500.0 / math.e]}, "outputs": {"dv": 600.0 * G0, "dv_stages": [300 * G0, 300 * G0]}}],
     questions=[{"q": "Multistage rocket: Isp = [300, 300], m0 = [" + repr(math.e * 1000) + ", 500], mf = [1000, " + repr(500 / math.e) + "]", "outputs": {"dv": 600.0 * G0}},
                {"q": "Multistage rocket: Isp = [300, 300], m0 = [1000, 2000], mf = [500, 100]", "outcome": "INVALID_INPUT"},
                {"q": "Multistage rocket: Isp = [300, 300], m0 = [1000, 400]", "outcome": "NEEDS_INPUT"}])


def identical_stages(n, dv, Isp, eps):
    _isp_in(Isp)
    _pos("the velocity change", dv)
    _need(isinstance(n, int) and 1 <= n <= 6, "the number of stages must be 1 to 6")
    _need(isinstance(eps, (int, float)) and 0.01 <= eps <= 0.5, "the structural coefficient must lie between 0.01 and 0.5")
    R = math.exp(dv / (n * Isp * G0))
    _need(eps * R < 1.0 - 1e-6, "the velocity change is not reachable with this structural coefficient (eps R >= 1)")
    lam = (1.0 - eps * R) / (R - 1.0)
    pi = (lam / (1.0 + lam)) ** n
    return _out(lam=lam, payload_fraction=pi, R=R, gross_per_payload=1.0 / pi)


def ref_identical_stages(n, dv, Isp, eps):
    """Payload ratio found by brentq so that n stages, each flown as an ODE burn from 1 + lambda to lambda + eps
    (unit stage mass), give the required delta-v; the stack is then built stage by stage."""
    from scipy.optimize import brentq
    c = Isp * G0
    lam = brentq(lambda L: n * c * _ref_burn((1.0 + L) / (L + eps))[0] - dv, 1e-12, 1e6, xtol=1e-16, rtol=8.9e-16, maxiter=500)
    stack = 1.0
    for _ in range(n):
        stack *= (1.0 + lam) / lam
    return {"lam": lam, "payload_fraction": 1.0 / stack, "R": (1.0 + lam) / (lam + eps), "gross_per_payload": stack}


def _is_random(rng):
    n = rng.randint(1, 4)
    Isp = rng.uniform(280.0, 460.0)
    eps = rng.uniform(0.05, 0.15)
    Rmax = 1.0 / eps
    R = 1.0 + rng.uniform(0.1, 0.8) * (Rmax - 1.0)
    return {"n": n, "dv": n * Isp * G0 * math.log(R), "Isp": Isp, "eps": eps}


_add("aero_identical_stages_payload",
     name="Payload fraction of a rocket with identical stages", topic="rocket performance / staging",
     purpose="For a rocket of n identical stages (same specific impulse and structural coefficient, equal delta-v split): the payload ratio of each stage, the overall payload fraction, the stage mass ratio and the gross lift-off mass per kilogram of payload.",
     inputs=[_intin("n", "number of stages (1 to 6)", 1, 6), _phys("dv", "speed", "total delta-v"), _phys("Isp", "time", "specific impulse of every stage"),
             _numin("eps", "structural coefficient ms/(ms + mp) of every stage")],
     outputs=[_o("lam", "number", "payload ratio of each stage (payload above / stage mass)"), _o("payload_fraction", "number", "overall payload fraction m_payload/m0"),
              _o("R", "number", "mass ratio of each stage"), _o("gross_per_payload", "number", "lift-off mass per unit payload mass")],
     primary="payload_fraction", conditions=["1 <= n <= 6", "0.01 <= eps <= 0.5", "eps R < 1", "identical stages, equal delta-v split"],
     method="R = exp(dv/(n Isp g0)); lambda = (1 - eps R)/(R - 1); payload fraction = (lambda/(1 + lambda))^n",
     latex=r"R = e^{\Delta v/(n I_{sp} g_0)},\quad \lambda = \frac{1 - \varepsilon R}{R - 1},\quad \frac{m_{pl}}{m_0} = \left(\frac{\lambda}{1+\lambda}\right)^n",
     run=identical_stages,
     parse=lambda q: _pairs(q, r"payload fraction(?: of a rocket)? with identical stages|identical stages", {"n": "n", "stages": "n", "dv": "dv", "isp": "Isp",
                                                                                                         "eps": "eps", "epsilon": "eps"}, phys={"dv", "Isp"}),
     reference=ref_identical_stages,
     wrong=_safe(lambda n, dv, Isp, eps: (lambda R: (lambda lam: _out(lam=lam, payload_fraction=(lam / (1 + lam)) ** n, R=R, gross_per_payload=((1 + lam) / lam) ** n))(
         (1.0 - eps * R) / R))(math.exp(dv / (n * Isp * G0)))),
     random_inputs=_is_random,
     templates=["Identical stages: n = 2, dv = 9 km/s, Isp = 350 s, eps = 0.1", "Payload fraction with identical stages: n = 3, dv = 9500 m/s, Isp = 320 s, eps = 0.08"],
     examples=[{"inputs": {"n": 1, "dv": 300.0 * G0 * math.log(4.0), "Isp": 300.0, "eps": 0.1}, "outputs": {"R": 4.0, "lam": 0.2, "payload_fraction": 0.2 / 1.2}}],
     questions=[{"q": "Identical stages: n = 1, dv = " + repr(300.0 * G0 * math.log(4.0)) + " m/s, Isp = 300 s, eps = 0.1", "outputs": {"lam": 0.2}},
                {"q": "Identical stages: n = 1, dv = 20 km/s, Isp = 300 s, eps = 0.1", "outcome": "INVALID_INPUT"},
                {"q": "Identical stages: n = 2, dv = 9 km/s, Isp = 350 s", "outcome": "NEEDS_INPUT"}])


# ================================================================================================ orbits
def _orb_alt(h, lo=100e3, hi=1e9):
    _need(isinstance(h, (int, float)) and lo <= h <= hi, f"the altitude must lie between {lo / 1e3:g} km and {hi / 1e3:g} km")


def _hapsira():
    """Import hapsira for a reference. numba (used by hapsira) imports a module named `coverage` when it can; the
    factory's tools folder, first on sys.path while certifying, holds an unrelated coverage.py, so it is hidden here."""
    import os
    import sys
    mod = sys.modules.get("coverage")
    if mod is not None and not hasattr(mod, "types"):
        del sys.modules["coverage"]
    saved = list(sys.path)
    sys.path[:] = [p for p in sys.path if not os.path.exists(os.path.join(p or ".", "coverage.py"))]
    try:
        import hapsira.bodies
        import hapsira.iod.izzo
        import hapsira.maneuver
        import hapsira.twobody  # noqa: F401
    finally:
        sys.path[:] = saved


def _hap_circ(h):
    _hapsira()
    import astropy.units as u
    import numpy as np
    from hapsira.bodies import Earth
    from hapsira.twobody import Orbit
    o = Orbit.circular(Earth, alt=h * u.m)
    return float(np.linalg.norm(o.v.to_value(u.m / u.s))), float(o.period.to_value(u.s))


def circular_orbit(h):
    _orb_alt(h)
    r = R_E + h
    v = math.sqrt(MU_E / r)
    T = 2 * math.pi * math.sqrt(r ** 3 / MU_E)
    return _out(v=v, T=T, T_min=T / 60.0, revs_per_day=DAY / T)


_add("aero_circular_orbit",
     name="Circular orbit at an altitude above the Earth", topic="orbits / circular orbits",
     purpose="Orbital speed, period and revolutions per day of a circular orbit at an altitude above the Earth's equatorial radius.",
     inputs=[_phys("h", "length", "orbit altitude above the Earth (km)")],
     outputs=[_o("v", "speed", "circular orbital speed"), _o("T", "time", "orbital period"), _o("T_min", "number", "period in minutes"),
              _o("revs_per_day", "number", "revolutions per day (86400 s)")],
     primary="v", conditions=["100 km <= h <= 1e6 km", "two-body Earth, mu = 3.986004418e14 m^3/s^2, R = 6378.1366 km"],
     method="r = R + h; v = sqrt(mu/r); T = 2 pi sqrt(r^3/mu)", latex=r"v = \sqrt{\frac{\mu}{R + h}},\quad T = 2\pi\sqrt{\frac{(R + h)^3}{\mu}}",
     run=circular_orbit,
     parse=lambda q: _pairs(q, r"circular orbit(?: at (?:an )?altitude)?(?: above the earth)?", _ALT_NAMES, phys={"h"}),
     reference=lambda h: (lambda vt: {"v": vt[0], "T": vt[1], "T_min": vt[1] / 60.0, "revs_per_day": DAY / vt[1]})(_hap_circ(h)),
     wrong=_safe(lambda h: (lambda r: (lambda v, T: _out(v=v, T=T, T_min=T / 60.0, revs_per_day=DAY / T))(math.sqrt(MU_E / r), 2 * math.pi * math.sqrt(r ** 3 / MU_E)))(h)),
     random_inputs=lambda rng: {"h": _lu(rng, 150e3, 4e8)},
     templates=["Circular orbit: h = 400 km", "Circular orbit at altitude: h = 35786 km"],
     examples=[{"inputs": {"h": 3 * R_E}, "outputs": {"v": math.sqrt(MU_E / (4 * R_E)), "T": 2 * math.pi * math.sqrt((4 * R_E) ** 3 / MU_E)}}],
     questions=[{"q": "Circular orbit: h = " + repr(3 * R_E / 1e3) + " km", "outputs": {"v": math.sqrt(MU_E / (4 * R_E))}},
                {"q": "Circular orbit: h = 50 km", "outcome": "INVALID_INPUT"},
                {"q": "Circular orbit:", "outcome": "NEEDS_INPUT"}])


def escape_speed(h):
    _need(isinstance(h, (int, float)) and 0 <= h <= 1e9, "the altitude must lie between 0 and 1e6 km")
    r = R_E + h
    ve, vc = math.sqrt(2 * MU_E / r), math.sqrt(MU_E / r)
    return _out(v_esc=ve, v_circ=vc, dv=ve - vc)


def ref_escape_speed(h):
    from scipy.integrate import quad
    r = R_E + h
    w, _ = quad(lambda s: MU_E / r * math.exp(-s), 0.0, math.inf, epsabs=0, epsrel=2e-14)   # work per kg from r to infinity (x = r e^s)
    ve = math.sqrt(2.0 * w)
    vc = _hap_circ(h)[0] if h >= 1.0 else math.sqrt(MU_E / r)
    return {"v_esc": ve, "v_circ": vc, "dv": ve - vc}


_add("aero_escape_speed",
     name="Escape speed at an altitude above the Earth", topic="orbits / escape",
     purpose="The speed needed to escape the Earth's gravity from an altitude, the circular orbital speed there and the burn needed to escape from that circular orbit.",
     inputs=[_phys("h", "length", "altitude above the Earth (km); 0 = the surface")],
     outputs=[_o("v_esc", "speed", "escape speed"), _o("v_circ", "speed", "circular orbital speed at that altitude"),
              _o("dv", "speed", "burn from the circular orbit to escape")],
     primary="v_esc", conditions=["0 <= h <= 1e6 km", "two-body Earth, no atmosphere"],
     method="v_esc = sqrt(2 mu/(R + h)); v_circ = sqrt(mu/(R + h))", latex=r"v_{esc} = \sqrt{\frac{2\mu}{R + h}} = \sqrt{2}\,v_{circ}",
     run=escape_speed,
     parse=lambda q: _pairs(q, r"escape speed(?: at (?:an )?altitude)?(?: above the earth)?|escape velocity", _ALT_NAMES, phys={"h"}),
     reference=ref_escape_speed,
     wrong=_safe(lambda h: (lambda r: _out(v_esc=math.sqrt(MU_E / r) * 2.0, v_circ=math.sqrt(MU_E / r), dv=math.sqrt(MU_E / r)))(R_E + h)),
     random_inputs=lambda rng: {"h": _lu(rng, 1e3, 4e8)},
     templates=["Escape speed: h = 300 km", "Escape speed at altitude: h = 0 km"],
     examples=[{"inputs": {"h": 0.0}, "outputs": {"v_esc": math.sqrt(2 * MU_E / R_E)}},
               {"inputs": {"h": R_E}, "outputs": {"v_circ": math.sqrt(MU_E / (2 * R_E)), "v_esc": math.sqrt(MU_E / R_E)}}],
     questions=[{"q": "Escape speed: h = 0 km", "outputs": {"v_esc": math.sqrt(2 * MU_E / R_E)}},
                {"q": "Escape speed: h = -10 km", "outcome": "INVALID_INPUT"},
                {"q": "Escape speed:", "outcome": "NEEDS_INPUT"}])


def altitude_for_period(T):
    _pos("the period", T)
    r = (MU_E * (T / (2 * math.pi)) ** 2) ** (1.0 / 3.0)
    h = r - R_E
    _orb_alt(h)
    return _out(h=h, h_km=h / 1e3, v=math.sqrt(MU_E / r))


def ref_altitude_for_period(T):
    from scipy.optimize import brentq
    h = brentq(lambda x: _hap_circ(x)[1] - T, 50e3, 2e9, xtol=1e-9, rtol=8.9e-16, maxiter=300)
    return {"h": h, "h_km": h / 1e3, "v": _hap_circ(h)[0]}


_add("aero_altitude_for_period",
     name="Circular-orbit altitude for an orbital period", topic="orbits / circular orbits",
     purpose="The altitude above the Earth of the circular orbit with a given period (for example a geostationary orbit, one sidereal day), and its orbital speed.",
     inputs=[_phys("T", "time", "orbital period (s, min, h, day)")],
     outputs=[_o("h", "length", "orbit altitude"), _o("h_km", "number", "altitude in km"), _o("v", "speed", "orbital speed")],
     primary="h", conditions=["the altitude between 100 km and 1e6 km", "two-body Earth"],
     method="r = (mu (T/(2 pi))^2)^(1/3); h = r - R", latex=r"R + h = \left(\mu\,\frac{T^2}{4\pi^2}\right)^{1/3}",
     run=altitude_for_period,
     parse=lambda q: _pairs(q, r"(?:circular[- ]orbit )?altitude for (?:an )?(?:orbital )?period", {"t": "T", "period": "T"}, phys={"T"}),
     reference=ref_altitude_for_period,
     wrong=_safe(lambda T: (lambda r: _out(h=r - R_E, h_km=(r - R_E) / 1e3, v=math.sqrt(MU_E / r)))((MU_E * (T / (2 * math.pi)) ** 2) ** 0.5)),
     random_inputs=lambda rng: {"T": _lu(rng, 5300.0, 3e6)},
     templates=["Altitude for period: T = 86164.0905 s", "Circular-orbit altitude for an orbital period: T = 90 min"],
     examples=[{"inputs": {"T": 2 * math.pi * math.sqrt((4 * R_E) ** 3 / MU_E)}, "outputs": {"h": 3 * R_E}}],
     questions=[{"q": "Altitude for period: T = " + repr(2 * math.pi * math.sqrt((4 * R_E) ** 3 / MU_E)) + " s", "outputs": {"h": 3 * R_E}},
                {"q": "Altitude for period: T = 60 min", "outcome": "INVALID_INPUT"},
                {"q": "Altitude for period:", "outcome": "NEEDS_INPUT"}])


_HOH24 = math.sqrt(MU_E / (2 * R_E)) * (math.sqrt(4 / 3) - 1) + math.sqrt(MU_E / (4 * R_E)) * (1 - math.sqrt(2 / 3))   # Hohmann 2R -> 4R


def bielliptic(h1, h2, hb):
    _orb_alt(h1)
    _orb_alt(h2)
    _orb_alt(hb)
    r1, r2, rb = R_E + h1, R_E + h2, R_E + hb
    _need(r1 < r2 <= rb, "the bi-elliptic transfer here is outward: h1 < h2 <= hb")
    mu = MU_E
    a1, a2 = (r1 + rb) / 2, (r2 + rb) / 2
    dv1 = math.sqrt(mu * (2 / r1 - 1 / a1)) - math.sqrt(mu / r1)
    dv2 = math.sqrt(mu * (2 / rb - 1 / a2)) - math.sqrt(mu * (2 / rb - 1 / a1))
    dv3 = math.sqrt(mu * (2 / r2 - 1 / a2)) - math.sqrt(mu / r2)
    t = math.pi * (math.sqrt(a1 ** 3 / mu) + math.sqrt(a2 ** 3 / mu))
    ah = (r1 + r2) / 2
    dvh = (math.sqrt(mu * (2 / r1 - 1 / ah)) - math.sqrt(mu / r1)) + (math.sqrt(mu / r2) - math.sqrt(mu * (2 / r2 - 1 / ah)))
    return _out(dv1=dv1, dv2=dv2, dv3=dv3, dv=dv1 + dv2 + dv3, t=t, t_days=t / DAY, dv_hohmann=dvh)


def ref_bielliptic(h1, h2, hb):
    _hapsira()
    import astropy.units as u
    import numpy as np
    from hapsira.bodies import Earth
    from hapsira.maneuver import Maneuver
    from hapsira.twobody import Orbit
    o = Orbit.circular(Earth, alt=h1 * u.m)
    m = Maneuver.bielliptic(o, (R_E + hb) * u.m, (R_E + h2) * u.m)
    dvs = [float(np.linalg.norm(dv.to_value(u.m / u.s))) for _, dv in m.impulses]
    t = float(m.get_total_time().to_value(u.s))
    mh = Maneuver.hohmann(o, (R_E + h2) * u.m)
    dvh = sum(float(np.linalg.norm(dv.to_value(u.m / u.s))) for _, dv in mh.impulses)
    return {"dv1": dvs[0], "dv2": dvs[1], "dv3": dvs[2], "dv": sum(dvs), "t": t, "t_days": t / DAY, "dv_hohmann": dvh}


def _bi_random(rng):
    h1 = _lu(rng, 200e3, 2e6)
    r2 = (R_E + h1) * _lu(rng, 1.5, 30.0)
    rb = r2 * _lu(rng, 1.05, 20.0)
    return {"h1": h1, "h2": r2 - R_E, "hb": rb - R_E}


_add("aero_bielliptic_transfer",
     name="Bi-elliptic transfer between circular orbits", topic="orbits / orbital transfers",
     purpose="The three burns, total delta-v and transfer time of a bi-elliptic transfer from a lower to a higher circular orbit around the Earth through an intermediate apoapsis, with the Hohmann delta-v for comparison.",
     inputs=[_phys("h1", "length", "altitude of the initial circular orbit"), _phys("h2", "length", "altitude of the final circular orbit"),
             _phys("hb", "length", "altitude of the intermediate apoapsis (>= h2)")],
     outputs=[_o("dv1", "speed", "first burn"), _o("dv2", "speed", "second burn (at the intermediate apoapsis)"), _o("dv3", "speed", "third burn (circularization)"),
              _o("dv", "speed", "total delta-v"), _o("t", "time", "total transfer time"), _o("t_days", "number", "transfer time in days"),
              _o("dv_hohmann", "speed", "total delta-v of the Hohmann transfer between the same orbits")],
     primary="dv", conditions=["100 km <= h1 < h2 <= hb", "coplanar circular orbits, impulsive burns, two-body Earth"],
     method="two transfer ellipses (r1, rb) and (r2, rb): burns from vis-viva at r1, rb and r2; t = pi (sqrt(a1^3/mu) + sqrt(a2^3/mu))",
     latex=r"v = \sqrt{\mu\left(\frac{2}{r} - \frac{1}{a}\right)},\quad t = \pi\left(\sqrt{\frac{a_1^3}{\mu}} + \sqrt{\frac{a_2^3}{\mu}}\right)",
     run=bielliptic,
     parse=lambda q: _pairs(q, r"bi-?elliptic transfer(?: between circular orbits)?", {"h1": "h1", "h2": "h2", "hb": "hb", "h_b": "hb"}, phys={"h1", "h2", "hb"}),
     reference=ref_bielliptic,
     wrong=_safe(lambda h1, h2, hb: (lambda o: {**o, "t": _r(2 * o["t"]), "t_days": _r(2 * o["t_days"]), "dv2": _r(o["dv2"] * 2), "dv": _r(o["dv"] + o["dv2"])})(bielliptic(h1, h2, hb))),
     random_inputs=_bi_random,
     templates=["Bi-elliptic transfer: h1 = 300 km, h2 = 100000 km, hb = 200000 km", "Bielliptic transfer: h1 = 500 km, h2 = 35786 km, hb = 80000 km"],
     examples=[{"inputs": {"h1": R_E, "h2": 3 * R_E, "hb": 3 * R_E}, "outputs": {"dv": _HOH24, "dv_hohmann": _HOH24}}],
     questions=[{"q": "Bi-elliptic transfer: h1 = 300 km, h2 = 200 km, hb = 400 km", "outcome": "INVALID_INPUT"},
                {"q": "Bi-elliptic transfer: h1 = 300 km, h2 = 100000 km", "outcome": "NEEDS_INPUT"}])


_MU_KM = MU_E / 1e9


def _stumpff(z):
    if abs(z) < 1e-3:
        return 0.5 - z / 24 + z * z / 720 - z ** 3 / 40320, 1 / 6 - z / 120 + z * z / 5040 - z ** 3 / 362880
    if z > 0:
        s = math.sqrt(z)
        return 2 * math.sin(s / 2) ** 2 / z, (s - math.sin(s)) / s ** 3
    s = math.sqrt(-z)
    return 2 * math.sinh(s / 2) ** 2 / (-z), (math.sinh(s) - s) / s ** 3


def lambert(r1, r2, tof, direction="prograde"):
    _need(len(r1) == 3 and len(r2) == 3, "the positions must be vectors of three components (km)")
    _pos("the time of flight", tof)
    _need(direction in ("prograde", "retrograde"), "the direction must be prograde or retrograde")
    n1, n2 = math.sqrt(sum(x * x for x in r1)), math.sqrt(sum(x * x for x in r2))
    _need(n1 > 0 and n2 > 0, "the positions must not be at the centre of the Earth")
    cz = r1[0] * r2[1] - r1[1] * r2[0]
    c = max(-1.0, min(1.0, sum(a * b for a, b in zip(r1, r2)) / (n1 * n2)))
    dth = math.acos(c)
    if (direction == "prograde") != (cz >= 0):
        dth = 2 * math.pi - dth
    _need(math.radians(2) <= dth <= math.radians(358) and abs(dth - math.pi) >= math.radians(2),
          "the transfer angle must be between 2 and 358 degrees and not within 2 degrees of 180 (the transfer plane is undefined)")
    A = math.sin(dth) * math.sqrt(n1 * n2 / (1 - math.cos(dth)))
    smu = math.sqrt(_MU_KM)

    def y(z):
        C, S = _stumpff(z)
        return n1 + n2 + A * (z * S - 1) / math.sqrt(C)

    def F(z):
        C, S = _stumpff(z)
        yz = y(z)
        if yz <= 0:
            return -smu * tof - 1.0
        return (yz / C) ** 1.5 * S + A * math.sqrt(yz) - smu * tof

    hi = (2 * math.pi - 1e-4) ** 2
    _need(F(hi) > 0, "the time of flight is too long for a zero-revolution transfer")
    lo = -4.0
    while F(lo) > 0:
        lo *= 2
        _need(lo > -1e7, "no zero-revolution solution found")
    z = _bisect(F, lo, hi, 400)
    yz = y(z)
    _need(yz > 0, "no zero-revolution solution found")
    f = 1 - yz / n1
    g = A * math.sqrt(yz / _MU_KM)
    gd = 1 - yz / n2
    v1 = [(b - f * a) / g for a, b in zip(r1, r2)]
    v2 = [(gd * b - a) / g for a, b in zip(r1, r2)]
    snap = lambda v: [0.0 if abs(x) <= 1e-13 * max(abs(w) for w in v) else x for x in v]      # noqa: E731
    return _out(v1=snap(v1), v2=snap(v2), dtheta_deg=dth / DEG)


def ref_lambert(r1, r2, tof, direction="prograde"):
    _hapsira()
    import astropy.units as u
    import numpy as np
    from hapsira.bodies import Earth
    from hapsira.iod import izzo
    v1, v2 = izzo.lambert(Earth.k, np.array(r1, dtype=float) * u.km, np.array(r2, dtype=float) * u.km, tof * u.s,
                          prograde=(direction == "prograde"), rtol=1e-13, numiter=200)
    a, b = v1.to_value(u.km / u.s), v2.to_value(u.km / u.s)
    snap = lambda v: [0.0 if abs(x) <= 1e-12 * max(abs(w) for w in v) else float(x) for x in v]   # noqa: E731
    c = float(np.dot(r1, r2) / (np.linalg.norm(r1) * np.linalg.norm(r2)))
    th = math.acos(max(-1.0, min(1.0, c)))
    cz = float(np.cross(r1, r2)[2])
    if (direction == "prograde") != (cz >= 0):
        th = 2 * math.pi - th
    return {"v1": snap(a), "v2": snap(b), "dtheta_deg": math.degrees(th)}


def _lam_random(rng):
    n1 = rng.uniform(6800.0, 42000.0)
    n2 = rng.uniform(6800.0, 42000.0)
    inc = rng.uniform(0.0, 1.2)
    th = rng.choice([rng.uniform(20.0, 160.0), rng.uniform(200.0, 340.0)]) * DEG
    u0 = rng.uniform(0, 2 * math.pi)

    def pos(nrm, uu):
        return [nrm * math.cos(uu), nrm * math.sin(uu) * math.cos(inc), nrm * math.sin(uu) * math.sin(inc)]
    a = (n1 + n2) / 2
    tof = rng.uniform(0.3, 1.5) * math.pi * math.sqrt(a ** 3 / _MU_KM) * th / math.pi
    return {"r1": pos(n1, u0), "r2": pos(n2, u0 + th), "tof": tof, **({"direction": "prograde"} if rng.random() < 0.5 else {})}


_add("aero_lambert_problem",
     name="Lambert's problem (orbit through two positions in a given time)", topic="orbits / orbit determination and targeting",
     purpose="The velocities at departure and arrival of the zero-revolution Earth orbit that goes from one position to another in a given time of flight (prograde or retrograde), with the transfer angle.",
     inputs=[_vecin("r1", "departure position vector in km, e.g. '[7000, 0, 0]'", 3, 3), _vecin("r2", "arrival position vector in km", 3, 3),
             _phys("tof", "time", "time of flight"), _choice("direction", ["prograde", "retrograde"], "prograde (default) or retrograde transfer", required=False)],
     outputs=[_o("v1", "number", "departure velocity vector in km/s"), _o("v2", "number", "arrival velocity vector in km/s"),
              _o("dtheta_deg", "number", "transfer angle in degrees")],
     primary="dtheta_deg", conditions=["2 deg <= transfer angle <= 358 deg, not within 2 deg of 180", "tof > 0", "zero revolutions, two-body Earth (mu = 398600.4418 km^3/s^2)"],
     method="universal variables: solve sqrt(mu) t = (y/C)^1.5 S + A sqrt(y) for z by bisection (Stumpff functions C, S); f = 1 - y/r1, g = A sqrt(y/mu), g' = 1 - y/r2; v1 = (r2 - f r1)/g, v2 = (g' r2 - r1)/g",
     latex=r"\sqrt{\mu}\,t = \left(\frac{y}{C(z)}\right)^{3/2}S(z) + A\sqrt{y},\quad \mathbf v_1 = \frac{\mathbf r_2 - f\mathbf r_1}{g},\quad \mathbf v_2 = \frac{\dot g\,\mathbf r_2 - \mathbf r_1}{g}",
     run=lambert,
     parse=lambda q: _pairs(q, r"lambert(?:'s)? problem(?: \(orbit through two positions in a given time\))?|lambert", {"r1": "r1", "r2": "r2", "tof": "tof", "t": "tof",
                                                                                                                      "direction": "direction"},
                            phys={"tof"}, words={"direction"}, vectors={"r1", "r2"}),
     reference=ref_lambert,
     wrong=_safe(lambda r1, r2, tof, direction="prograde": lambert(r1, r2, tof, "retrograde" if direction == "prograde" else "prograde")),
     random_inputs=_lam_random,
     templates=["Lambert problem: r1 = [5000, 10000, 2100], r2 = [-14600, 2500, 7000], tof = 1 h",
                "Lambert's problem: r1 = [7000, 0, 0], r2 = [0, 8000, 1000], tof = 3600 s, direction = prograde"],
     examples=[{"inputs": {"r1": "7000, 0, 0", "r2": "0, 7000, 0", "tof": math.pi / 2 * math.sqrt(7000.0 ** 3 / _MU_KM)},
                "outputs": {"v1": [0.0, math.sqrt(_MU_KM / 7000.0), 0.0], "dtheta_deg": 90.0}}],
     questions=[{"q": "Lambert problem: r1 = [7000, 0, 0], r2 = [0, 7000, 0], tof = " + repr(math.pi / 2 * math.sqrt(7000.0 ** 3 / _MU_KM)) + " s",
                 "outputs": {"v2": [-math.sqrt(_MU_KM / 7000.0), 0.0, 0.0]}},
                {"q": "Lambert problem: r1 = [7000, 0, 0], r2 = [-7000, 1, 0], tof = 1 h", "outcome": "INVALID_INPUT"},
                {"q": "Lambert problem: r1 = [7000, 0, 0], r2 = [0, 7000, 0]", "outcome": "NEEDS_INPUT"}])


def _j2_elements(hp, ha, i):
    _orb_alt(hp, 100e3, 1e8)
    ha = hp if ha is None else ha
    _orb_alt(ha, 100e3, 1e8)
    _need(ha >= hp, "the apogee altitude must not be below the perigee altitude")
    _need(DEG <= i <= 179 * DEG, "the inclination must lie between 1 and 179 degrees (the node and perigee are undefined for an equatorial orbit)")
    rp, ra = R_E + hp, R_E + ha
    a = (rp + ra) / 2
    e = (ra - rp) / (ra + rp)
    p = a * (1 - e * e)
    n = math.sqrt(MU_E / a ** 3)
    return a, e, p, n


def nodal_regression(hp, i, ha=None):
    a, e, p, n = _j2_elements(hp, ha, i)
    rate = -1.5 * n * J2_E * (R_E / p) ** 2 * math.cos(i)
    if abs(math.cos(i)) < 1e-12:
        rate = 0.0
    return _out(raan_rate_deg_day=rate / DEG * DAY, raan_rate=rate)


def _ref_gauss_rates(hp, ha, i, want="node"):
    """Orbit-averaged Gauss variational equations under the J2 acceleration, averaged in time over one orbit
    (dt = r^2/h d nu, trapezoid rule over the true anomaly, spectrally exact for a periodic integrand)."""
    import numpy as np
    rp, ra = R_E + hp, R_E + (hp if ha is None else ha)
    a, e = (rp + ra) / 2, (ra - rp) / (ra + rp)
    p = a * (1 - e * e)
    h = math.sqrt(MU_E * p)
    w = 0.7                                                       # argument of perigee (the average does not depend on it)
    nu = np.linspace(0.0, 2 * np.pi, 4096, endpoint=False)
    r = p / (1 + e * np.cos(nu))
    u = w + nu
    k = -1.5 * J2_E * MU_E * R_E ** 2 / r ** 4
    Racc = k * (1 - 3 * math.sin(i) ** 2 * np.sin(u) ** 2)
    Sacc = k * (math.sin(i) ** 2 * np.sin(2 * u))
    Wacc = k * (math.sin(2 * i) * np.sin(u))
    dt = r ** 2 / h
    T = float(np.sum(dt))
    if want == "node":
        dO = r * np.sin(u) * Wacc / (h * math.sin(i))
        return float(np.sum(dO * dt)) / T
    dO_cos = r * np.sin(u) * math.cos(i) * Wacc / (h * math.sin(i))
    dw = (-p * np.cos(nu) * Racc + (p + r) * np.sin(nu) * Sacc) / (h * e) - dO_cos
    return float(np.sum(dw * dt)) / T


def _j2_random(rng):
    hp = _lu(rng, 200e3, 2e7)
    d = {"hp": hp, "i": rng.uniform(5.0, 175.0) * DEG}
    if rng.random() < 0.6:
        d["ha"] = hp + _lu(rng, 1e3, 3e7)
    return d


_add("aero_j2_nodal_regression",
     name="Nodal regression from the Earth's oblateness (J2)", topic="orbits / perturbations",
     purpose="The secular drift of the right ascension of the ascending node caused by the Earth's equatorial bulge (J2), from the perigee and apogee altitudes and the inclination.",
     inputs=[_phys("hp", "length", "perigee altitude (or the altitude of a circular orbit)"), _phys("i", "angle", "inclination (deg)"),
             _phys("ha", "length", "apogee altitude (default: circular orbit)", required=False)],
     outputs=[_o("raan_rate_deg_day", "number", "node drift rate in degrees per day (negative = westward regression)"),
              _o("raan_rate", "angular_frequency", "node drift rate")],
     primary="raan_rate_deg_day", conditions=["100 km <= hp <= ha <= 1e5 km", "1 deg <= i <= 179 deg", "first-order secular J2 theory, J2 = 1.08263e-3"],
     method="dOmega/dt = -(3/2) n J2 (R/p)^2 cos i, n = sqrt(mu/a^3), p = a(1 - e^2)",
     latex=r"\dot\Omega = -\frac{3}{2}\,n\,J_2\left(\frac{R}{p}\right)^2\cos i",
     run=nodal_regression,
     parse=lambda q: _pairs(q, r"nodal regression(?: from the earth'?s oblateness)?(?: \(j2\))?|j2 nodal (?:regression|precession)|raan drift",
                            {"hp": "hp", "h": "hp", "ha": "ha", "i": "i", "inc": "i", "inclination": "i"}, phys={"hp", "ha", "i"}),
     reference=lambda hp, i, ha=None: (lambda r: {"raan_rate_deg_day": math.degrees(r) * DAY, "raan_rate": r})(_ref_gauss_rates(hp, ha, i, "node")),
     wrong=_safe(lambda hp, i, ha=None: (lambda o: {**o, "raan_rate_deg_day": _r(o["raan_rate_deg_day"] / math.cos(i) * math.sin(i)), "raan_rate": _r(o["raan_rate"] / math.cos(i) * math.sin(i))})(nodal_regression(hp, i, ha))),
     random_inputs=_j2_random,
     templates=["Nodal regression: hp = 700 km, i = 98 deg", "J2 nodal regression: hp = 250 km, ha = 35786 km, i = 28.5 deg"],
     examples=[{"inputs": {"hp": 3 * R_E, "i": math.pi / 3}, "outputs": {"raan_rate": -0.75 * math.sqrt(MU_E / (4 * R_E) ** 3) * J2_E / 16}},
               {"inputs": {"hp": 3 * R_E, "i": math.pi / 2}, "outputs": {"raan_rate": 0.0}}],
     questions=[{"q": "Nodal regression: hp = " + repr(3 * R_E / 1e3) + " km, i = 60 deg", "outputs": {"raan_rate": -0.75 * math.sqrt(MU_E / (4 * R_E) ** 3) * J2_E / 16}},
                {"q": "Nodal regression: hp = 700 km, i = 200 deg", "outcome": "INVALID_INPUT"},
                {"q": "Nodal regression: hp = 700 km", "outcome": "NEEDS_INPUT"}])


def apsidal_rotation(hp, ha, i):
    a, e, p, n = _j2_elements(hp, ha, i)
    _need(e >= 1e-4, "the orbit must be eccentric (apogee above perigee) for its perigee to be defined")
    rate = 0.75 * n * J2_E * (R_E / p) ** 2 * (5 * math.cos(i) ** 2 - 1)
    return _out(argp_rate_deg_day=rate / DEG * DAY, argp_rate=rate)


def _aps_random(rng):
    hp = _lu(rng, 200e3, 2e7)
    return {"hp": hp, "ha": hp + _lu(rng, 2e4, 3e7), "i": rng.uniform(5.0, 175.0) * DEG}


_add("aero_j2_apsidal_rotation",
     name="Rotation of the line of apsides from J2", topic="orbits / perturbations",
     purpose="The secular rotation of the argument of perigee of an eccentric Earth orbit caused by J2 (zero at the critical inclination 63.4 degrees, as used by Molniya orbits).",
     inputs=[_phys("hp", "length", "perigee altitude"), _phys("ha", "length", "apogee altitude"), _phys("i", "angle", "inclination (deg)")],
     outputs=[_o("argp_rate_deg_day", "number", "perigee rotation rate in degrees per day"), _o("argp_rate", "angular_frequency", "perigee rotation rate")],
     primary="argp_rate_deg_day", conditions=["100 km <= hp < ha <= 1e5 km", "eccentricity >= 1e-4", "1 deg <= i <= 179 deg", "first-order secular J2 theory"],
     method="domega/dt = (3/4) n J2 (R/p)^2 (5 cos^2 i - 1)",
     latex=r"\dot\omega = \frac{3}{4}\,n\,J_2\left(\frac{R}{p}\right)^2\left(5\cos^2 i - 1\right)",
     run=apsidal_rotation,
     parse=lambda q: _pairs(q, r"rotation of the line of apsides(?: from j2)?|apsidal rotation|j2 apsidal (?:rotation|precession)|perigee drift",
                            {"hp": "hp", "ha": "ha", "i": "i", "inc": "i", "inclination": "i"}, phys={"hp", "ha", "i"}),
     reference=lambda hp, ha, i: (lambda r: {"argp_rate_deg_day": math.degrees(r) * DAY, "argp_rate": r})(_ref_gauss_rates(hp, ha, i, "perigee")),
     wrong=_safe(lambda hp, ha, i: (lambda o: {**o, "argp_rate_deg_day": _r(o["argp_rate_deg_day"] * (5 * math.cos(i) ** 2 + 1) / (5 * math.cos(i) ** 2 - 1)),
                                               "argp_rate": _r(o["argp_rate"] * (5 * math.cos(i) ** 2 + 1) / (5 * math.cos(i) ** 2 - 1))})(apsidal_rotation(hp, ha, i))),
     random_inputs=_aps_random,
     templates=["Apsidal rotation: hp = 500 km, ha = 39800 km, i = 63.4 deg", "Rotation of the line of apsides from J2: hp = 300 km, ha = 35786 km, i = 28.5 deg"],
     examples=[{"inputs": {"hp": 2 * R_E, "ha": 4 * R_E, "i": math.pi / 2}, "outputs": {"argp_rate": -0.75 * math.sqrt(MU_E / (4 * R_E) ** 3) * J2_E * (R_E / (4 * R_E * (1 - 0.0625))) ** 2}}],
     questions=[{"q": "Apsidal rotation: hp = 500 km, ha = 500 km, i = 63.4 deg", "outcome": "INVALID_INPUT"},
                {"q": "Apsidal rotation: hp = 500 km, ha = 39800 km", "outcome": "NEEDS_INPUT"}])


def sun_synchronous(h):
    _orb_alt(h, 100e3, 6e6)
    r = R_E + h
    n = math.sqrt(MU_E / r ** 3)
    c = -SSO_RATE / (1.5 * n * J2_E * (R_E / r) ** 2)
    _need(c >= -1.0, "no sun-synchronous circular orbit exists at this altitude (the J2 drift is too slow)")
    i = math.acos(c)
    return _out(i_deg=i / DEG, period_min=2 * math.pi / n / 60.0)


def ref_sun_synchronous(h):
    from scipy.optimize import brentq
    i = brentq(lambda x: _ref_gauss_rates(h, None, x, "node") - SSO_RATE, math.pi / 2 + 1e-9, math.pi - 1e-9, xtol=1e-15, rtol=8.9e-16, maxiter=300)
    return {"i_deg": math.degrees(i), "period_min": _hap_circ(h)[1] / 60.0}


_add("aero_sun_synchronous_inclination",
     name="Inclination of a sun-synchronous orbit", topic="orbits / perturbations",
     purpose="The inclination at which J2 makes a circular orbit's plane turn eastward at one revolution per year (sun-synchronous), at an altitude, with the orbital period.",
     inputs=[_phys("h", "length", "altitude of the circular orbit (km)")],
     outputs=[_o("i_deg", "number", "sun-synchronous inclination in degrees (retrograde, > 90)"), _o("period_min", "number", "orbital period in minutes")],
     primary="i_deg", conditions=["100 km <= h <= 6000 km (none exists above about 5970 km)", "circular orbit, first-order J2, node rate 360 deg per 365.2421897 days"],
     method="cos i = -(dOmega/dt)_sun / ((3/2) n J2 (R/r)^2)", latex=r"\cos i = -\frac{\dot\Omega_\odot}{\frac{3}{2}\,n\,J_2\,(R/r)^2}",
     run=sun_synchronous,
     parse=lambda q: _pairs(q, r"(?:inclination of a )?sun-?synchronous(?: orbit)?(?: inclination)?", _ALT_NAMES, phys={"h"}),
     reference=ref_sun_synchronous,
     wrong=_safe(lambda h: (lambda o: {**o, "i_deg": _r(180.0 - o["i_deg"])})(sun_synchronous(h))),
     random_inputs=lambda rng: {"h": rng.uniform(200e3, 5000e3)},
     templates=["Sun-synchronous orbit: h = 700 km", "Inclination of a sun-synchronous orbit: h = 500 km"],
     examples=[{"inputs": {"h": 700e3}, "outputs": {"i_deg": math.degrees(math.acos(-SSO_RATE / (1.5 * math.sqrt(MU_E / (R_E + 700e3) ** 3) * J2_E * (R_E / (R_E + 700e3)) ** 2)))}}],
     questions=[{"q": "Sun-synchronous orbit: h = 700 km", "outputs": {"i_deg": math.degrees(math.acos(-SSO_RATE / (1.5 * math.sqrt(MU_E / (R_E + 700e3) ** 3) * J2_E * (R_E / (R_E + 700e3)) ** 2)))}},
                {"q": "Sun-synchronous orbit: h = 7000 km", "outcome": "INVALID_INPUT"},
                {"q": "Sun-synchronous orbit:", "outcome": "NEEDS_INPUT"}])


# ================================================================================================ re-entry
def _entry_in(m, CD, A, VE, gam, H, rho0):
    _pos("the mass, drag coefficient, area, entry speed, scale height and surface density", m, CD, A, VE, H, rho0)
    _need(0 < gam <= math.pi / 2, "the entry flight-path angle (below the horizontal) must lie between 0 and 90 degrees")
    _need(gam >= 2 * DEG, "the straight-line ballistic (Allen-Eggers) model needs an entry angle of at least 2 degrees")
    return m / (CD * A)


def entry_peak(m, CD, A, VE, gamma_e, H=7200.0, rho0=1.225):
    beta = _entry_in(m, CD, A, VE, gamma_e, H, rho0)
    s = math.sin(gamma_e)
    hs = H * math.log(rho0 * H / (beta * s))
    _need(hs > 0, "the deceleration would peak below sea level (the body reaches the ground first)")
    amax = VE * VE * s / (2 * math.e * H)
    return _out(n_max=amax / G0, a_max=amax, h_peak=hs, V_peak=VE * math.exp(-0.5))


def _ref_entry_V(h, beta, s, H, rho0, VE):
    """Speed at altitude h (real or complex) from dV/dh = rho V/(2 beta sin gamma), integrated numerically from 60 scale
    heights up (Gauss-Legendre in h)."""
    import numpy as np
    top = 60.0 * H
    integral = _gl_c(lambda x: rho0 * np.exp(-x / H), h, top)
    return VE * np.exp(-integral / (2 * beta * s))


def _gl_c(f, a, b, n=160):
    import numpy as np
    if n not in _LEG:
        _LEG[n] = np.polynomial.legendre.leggauss(n)
    x, w = _LEG[n]
    t = 0.5 * (b - a) * x + 0.5 * (b + a)
    return 0.5 * (b - a) * np.dot(w, f(t))


def ref_entry_peak(m, CD, A, VE, gamma_e, H=7200.0, rho0=1.225):
    """Peak of the deceleration rho V^2/(2 beta) along the numerically integrated trajectory: its derivative in h by the
    complex step, root by brentq."""
    import numpy as np
    from scipy.optimize import brentq
    beta, s = m / (CD * A), math.sin(gamma_e)

    def dec(h):
        return rho0 * np.exp(-h / H) * _ref_entry_V(h, beta, s, H, rho0, VE) ** 2 / (2 * beta)

    step = 1e-30
    ddec = lambda h: float(np.imag(dec(h + 1j * step)) / step)           # noqa: E731
    hs = brentq(ddec, 0.0, 40.0 * H, xtol=1e-12, rtol=8.9e-16, maxiter=300)
    amax = float(np.real(dec(hs)))
    return {"n_max": amax / G0, "a_max": amax, "h_peak": hs, "V_peak": float(np.real(_ref_entry_V(hs, beta, s, H, rho0, VE)))}


def _entry_random(rng):
    H = rng.choice([7200.0, rng.uniform(6000.0, 11000.0)])
    rho0 = rng.choice([1.225, rng.uniform(0.01, 65.0)])
    g = rng.uniform(5.0, 80.0) * DEG
    hs = rng.uniform(5e3, 25e3) * H / 7200.0
    beta = rho0 * H / (math.sin(g) * math.exp(hs / H))
    CD = rng.uniform(0.5, 1.6)
    A = _lu(rng, 0.01, 10.0)
    return {"m": beta * CD * A, "CD": CD, "A": A, "VE": rng.uniform(2000.0, 11500.0), "gamma_e": g,
            **({} if H == 7200.0 else {"H": H}), **({} if rho0 == 1.225 else {"rho0": rho0})}


_ENTRY_NAMES = {"m": "m", "mass": "m", "cd": "CD", "a": "A", "area": "A", "ve": "VE", "v_e": "VE", "v": "VE", "gamma_e": "gamma_e", "gamma": "gamma_e",
                "fpa": "gamma_e", "H": "H", "h_s": "H", "hs": "H", "scale_height": "H", "rho0": "rho0", "h": "h"}

_add("aero_ballistic_entry_peak_deceleration",
     name="Peak deceleration of a ballistic re-entry (Allen-Eggers)", topic="re-entry / ballistic entry",
     purpose="The peak deceleration (in g), the altitude where it occurs and the speed there for a body entering an exponential atmosphere on a straight ballistic path, from its mass, drag coefficient and reference area (ballistic coefficient m/(CD A)), entry speed and flight-path angle.",
     inputs=[_phys("m", "mass", "mass"), _numin("CD", "drag coefficient"), _phys("A", "area", "drag reference area"),
             _phys("VE", "speed", "entry speed"), _phys("gamma_e", "angle", "entry flight-path angle below the horizontal (deg)"),
             _phys("H", "length", "atmospheric scale height (default 7.2 km)", required=False),
             _phys("rho0", "density", "surface density of the exponential atmosphere (default 1.225 kg/m^3)", required=False)],
     outputs=[_o("n_max", "number", "peak deceleration in g (9.80665 m/s^2)"), _o("a_max", "acceleration", "peak deceleration"),
              _o("h_peak", "length", "altitude of peak deceleration"), _o("V_peak", "speed", "speed at peak deceleration")],
     primary="n_max", conditions=["2 deg <= gamma_e <= 90 deg", "peak above sea level", "straight-line path, gravity and lift neglected, exponential atmosphere rho0 exp(-h/H)"],
     method="beta = m/(CD A); a_max = VE^2 sin(gamma)/(2 e H); h* = H ln(rho0 H/(beta sin gamma)); V* = VE e^(-1/2)",
     latex=r"a_{max} = \frac{V_E^2\sin\gamma_E}{2eH},\quad h^* = H\ln\frac{\rho_0 H}{\beta\sin\gamma_E},\quad V^* = V_E\,e^{-1/2},\quad \beta = \frac{m}{C_D A}",
     run=entry_peak,
     parse=lambda q: _pairs(q, r"peak deceleration(?: of a ballistic re-?entry)?(?: \(allen-eggers\))?|ballistic entry peak deceleration", _ENTRY_NAMES,
                            phys={"m", "A", "VE", "gamma_e", "H", "rho0"}),
     reference=ref_entry_peak,
     wrong=_safe(lambda m, CD, A, VE, gamma_e, H=7200.0, rho0=1.225: (lambda o: {**o, "n_max": _r(o["n_max"] * math.e), "a_max": _r(o["a_max"] * math.e)})(entry_peak(m, CD, A, VE, gamma_e, H, rho0))),
     random_inputs=_entry_random,
     templates=["Peak deceleration: m = 1000 kg, CD = 1.2, A = 1.5 m^2, VE = 7.5 km/s, gamma_e = 30 deg",
                "Peak deceleration of a ballistic re-entry: m = 250 kg, CD = 1, A = 0.4 m^2, VE = 6 km/s, gamma_e = 10 deg, H = 8 km"],
     examples=[{"inputs": {"m": 1000.0, "CD": 1.0, "A": 1.0, "VE": 7000.0, "gamma_e": math.pi / 2},
                "outputs": {"a_max": 7000.0 ** 2 / (2 * math.e * 7200.0), "h_peak": 7200.0 * math.log(1.225 * 7200.0 / 1000.0)}}],
     questions=[{"q": "Peak deceleration: m = 1000 kg, CD = 1, A = 1 m^2, VE = 7 km/s, gamma_e = 90 deg", "outputs": {"a_max": 7000.0 ** 2 / (2 * math.e * 7200.0)}},
                {"q": "Peak deceleration: m = 1000000 kg, CD = 1, A = 1 m^2, VE = 7 km/s, gamma_e = 90 deg", "outcome": "INVALID_INPUT"},
                {"q": "Peak deceleration: m = 1000 kg, CD = 1, A = 1 m^2, VE = 7 km/s", "outcome": "NEEDS_INPUT"}])


def entry_velocity(m, CD, A, VE, gamma_e, h, H=7200.0, rho0=1.225):
    beta = _entry_in(m, CD, A, VE, gamma_e, H, rho0)
    _need(isinstance(h, (int, float)) and 0 <= h <= 40 * H, "the altitude must lie between sea level and 40 scale heights")
    s = math.sin(gamma_e)
    rho = rho0 * math.exp(-h / H)
    V = VE * math.exp(-rho * H / (2 * beta * s))
    a = rho * V * V / (2 * beta)
    return _out(V=V, n=a / G0, a=a)


def ref_entry_velocity(m, CD, A, VE, gamma_e, h, H=7200.0, rho0=1.225):
    beta, s = m / (CD * A), math.sin(gamma_e)
    top = 60.0 * H
    # straight-line ballistic entry integrated in altitude: dV/dh = rho V / (2 beta sin gamma)   (scipy DOP853)
    lnV = _ivp(lambda x, y: [rho0 * math.exp(-x / H) / (2 * beta * s)], h, [math.log(VE)], t0=top)[0]
    V = math.exp(lnV)
    a = rho0 * math.exp(-h / H) * V * V / (2 * beta)
    return {"V": V, "n": a / G0, "a": a}


_add("aero_ballistic_entry_velocity",
     name="Speed and deceleration of a ballistic re-entry at an altitude", topic="re-entry / ballistic entry",
     purpose="The speed and the deceleration of a body on a straight ballistic entry into an exponential atmosphere when it reaches a given altitude (Allen-Eggers).",
     inputs=[_phys("m", "mass", "mass"), _numin("CD", "drag coefficient"), _phys("A", "area", "drag reference area"),
             _phys("VE", "speed", "entry speed"), _phys("gamma_e", "angle", "entry flight-path angle below the horizontal (deg)"),
             _phys("h", "length", "altitude"),
             _phys("H", "length", "atmospheric scale height (default 7.2 km)", required=False),
             _phys("rho0", "density", "surface density (default 1.225 kg/m^3)", required=False)],
     outputs=[_o("V", "speed", "speed at the altitude"), _o("n", "number", "deceleration in g"), _o("a", "acceleration", "deceleration")],
     primary="V", conditions=["2 deg <= gamma_e <= 90 deg", "0 <= h <= 40 H", "straight-line path, gravity and lift neglected, exponential atmosphere"],
     method="V = VE exp(-rho0 H e^(-h/H)/(2 beta sin gamma)); a = rho V^2/(2 beta), beta = m/(CD A)",
     latex=r"V = V_E\exp\left(-\frac{\rho_0 H e^{-h/H}}{2\beta\sin\gamma_E}\right),\quad a = \frac{\rho V^2}{2\beta}",
     run=entry_velocity,
     parse=lambda q: _pairs(q, r"(?:speed and deceleration of a )?ballistic re-?entry at (?:an )?altitude|entry speed at altitude", _ENTRY_NAMES,
                            phys={"m", "A", "VE", "gamma_e", "h", "H", "rho0"}),
     reference=ref_entry_velocity,
     wrong=_safe(lambda m, CD, A, VE, gamma_e, h, H=7200.0, rho0=1.225: entry_velocity(m, CD, A, VE, math.pi / 2 - gamma_e if gamma_e < math.pi / 2 - 2 * DEG else gamma_e / 2, h, H, rho0)),
     random_inputs=lambda rng: (lambda d: {**d, "h": rng.uniform(0.0, 12.0) * d.get("H", 7200.0)})(_entry_random(rng)),
     templates=["Ballistic re-entry at altitude: m = 1000 kg, CD = 1.2, A = 1.5 m^2, VE = 7.5 km/s, gamma_e = 30 deg, h = 30 km",
                "Speed and deceleration of a ballistic re-entry at an altitude: m = 250 kg, CD = 1, A = 0.4 m^2, VE = 6 km/s, gamma_e = 10 deg, h = 20 km, H = 8 km"],
     examples=[{"inputs": {"m": 1000.0, "CD": 1.0, "A": 1.0, "VE": 7000.0, "gamma_e": math.pi / 2, "h": 7200.0 * math.log(1.225 * 7200.0 / 1000.0)},
                "outputs": {"V": 7000.0 * math.exp(-0.5)}}],
     questions=[{"q": "Ballistic re-entry at altitude: m = 1000 kg, CD = 1, A = 1 m^2, VE = 7 km/s, gamma_e = 90 deg, h = 500 km", "outcome": "INVALID_INPUT"},
                {"q": "Ballistic re-entry at altitude: m = 1000 kg, CD = 1, A = 1 m^2, VE = 7 km/s, gamma_e = 90 deg", "outcome": "NEEDS_INPUT"}])


# ================================================================================================ aircraft performance
def _K(AR, e):
    _need(isinstance(AR, (int, float)) and 1.0 <= AR <= 50.0, "the aspect ratio must lie between 1 and 50")
    _need(isinstance(e, (int, float)) and 0.3 <= e <= 1.0, "the Oswald efficiency factor must lie between 0.3 and 1")
    return 1.0 / (math.pi * e * AR)


def _cd0(CD0):
    _need(isinstance(CD0, (int, float)) and 0.002 <= CD0 <= 0.2, "the zero-lift drag coefficient must lie between 0.002 and 0.2")


def _cs_argmin(f, lo, hi):
    """Stationary point of an analytic f on [lo, hi]: the root of f' computed by the complex step (reference side)."""
    from scipy.optimize import brentq
    step = 1e-30
    return brentq(lambda x: f(complex(x, step)).imag / step, lo, hi, xtol=1e-300, rtol=8.9e-16, maxiter=500)


def drag_polar_optimum(CD0, AR, e):
    _cd0(CD0)
    K = _K(AR, e)
    cl_md, cl_mp, cl_mr = math.sqrt(CD0 / K), math.sqrt(3 * CD0 / K), math.sqrt(CD0 / (3 * K))
    return _out(CL_md=cl_md, LD_max=1.0 / (2.0 * math.sqrt(CD0 * K)), CL_mp=cl_mp, LD_mp=cl_mp / (4.0 * CD0),
                CL_mr=cl_mr, LD_mr=cl_mr / (CD0 * 4.0 / 3.0))


def ref_drag_polar_optimum(CD0, AR, e):
    K = 1.0 / (math.pi * e * AR)
    cd = lambda cl: CD0 + K * cl * cl                      # noqa: E731
    a = _cs_argmin(lambda cl: cd(cl) / cl, 1e-4, 20.0)
    b = _cs_argmin(lambda cl: cd(cl) / cl ** 1.5, 1e-4, 20.0)
    c = _cs_argmin(lambda cl: cd(cl) / cl ** 0.5, 1e-4, 20.0)
    return {"CL_md": a, "LD_max": a / cd(a), "CL_mp": b, "LD_mp": b / cd(b), "CL_mr": c, "LD_mr": c / cd(c)}


_POLAR_NAMES = {"cd0": "CD0", "ar": "AR", "a": "AR", "aspect_ratio": "AR", "e": "e", "oswald": "e", "w": "W", "weight": "W", "s": "S",
                "wing_area": "S", "h": "h", "altitude": "h", "t": "T", "thrust": "T", "p": "P", "power": "P", "eta": "eta", "eta_p": "eta"}
_POLAR_RANDOM = lambda rng: {"CD0": rng.uniform(0.012, 0.05), "AR": rng.uniform(4.0, 14.0), "e": rng.uniform(0.6, 0.95)}   # noqa: E731

_add("aero_drag_polar_optimum",
     name="Best lift coefficients and lift-to-drag ratios of a drag polar", topic="aircraft performance / drag polar",
     purpose="For a parabolic drag polar CD = CD0 + CL^2/(pi e AR): the lift coefficient and L/D for minimum drag (maximum L/D), for minimum power (best propeller endurance) and for the jet's best range (maximum sqrt(CL)/CD).",
     inputs=[_numin("CD0", "zero-lift (parasite) drag coefficient"), _numin("AR", "wing aspect ratio b^2/S"), _numin("e", "Oswald span efficiency factor")],
     outputs=[_o("CL_md", "number", "lift coefficient for minimum drag"), _o("LD_max", "number", "maximum lift-to-drag ratio"),
              _o("CL_mp", "number", "lift coefficient for minimum power required"), _o("LD_mp", "number", "L/D at minimum power"),
              _o("CL_mr", "number", "lift coefficient for best jet range (max sqrt(CL)/CD)"), _o("LD_mr", "number", "L/D at best jet range")],
     primary="LD_max", conditions=["0.002 <= CD0 <= 0.2", "1 <= AR <= 50", "0.3 <= e <= 1", "parabolic drag polar"],
     method="K = 1/(pi e AR); CL_md = sqrt(CD0/K), (L/D)max = 1/(2 sqrt(CD0 K)); CL_mp = sqrt(3 CD0/K), CD = 4 CD0; CL_mr = sqrt(CD0/(3K)), CD = 4/3 CD0",
     latex=r"C_{L,md} = \sqrt{\frac{C_{D0}}{K}},\quad \left(\frac{L}{D}\right)_{max} = \frac{1}{2\sqrt{C_{D0}K}},\quad K = \frac{1}{\pi e A\!R}",
     run=drag_polar_optimum,
     parse=lambda q: _pairs(q, r"(?:best lift coefficients and lift-to-drag ratios of a )?drag polar(?: optimum)?|maximum lift-to-drag ratio|l/d max", _POLAR_NAMES),
     reference=ref_drag_polar_optimum,
     wrong=_safe(lambda CD0, AR, e: (lambda K: _out(CL_md=math.sqrt(CD0 / K), LD_max=1.0 / (2.0 * math.sqrt(CD0 * K)), CL_mp=math.sqrt(3 * CD0 / K),
                                                   LD_mp=math.sqrt(3 * CD0 / K) / (4 * CD0), CL_mr=math.sqrt(CD0 / (3 * K)), LD_mr=math.sqrt(CD0 / (3 * K)) * 3 / (4 * CD0)))(1.0 / (e * AR))),
     random_inputs=_POLAR_RANDOM,
     templates=["Drag polar: CD0 = 0.025, AR = 8, e = 0.8", "Maximum lift-to-drag ratio: CD0 = 0.018, AR = 10, e = 0.85"],
     examples=[{"inputs": {"CD0": 0.02, "AR": 10.0, "e": 0.8}, "outputs": {"CL_md": math.sqrt(0.02 * math.pi * 8.0), "LD_max": 1 / (2 * math.sqrt(0.02 / (8 * math.pi)))}}],
     questions=[{"q": "Drag polar: CD0 = 0.02, AR = 10, e = 0.8", "outputs": {"LD_max": 1 / (2 * math.sqrt(0.02 / (8 * math.pi)))}},
                {"q": "Drag polar: CD0 = 0.02, AR = 10, e = 1.5", "outcome": "INVALID_INPUT"},
                {"q": "Drag polar: CD0 = 0.02, AR = 10", "outcome": "NEEDS_INPUT"}])


def _rho_at(h):
    return _isa(h)[2]


def _ref_rho(h):
    return _fluids_atm(h).rho


def _ws_in(W, S):
    _pos("the weight and the wing area", W, S)


def min_drag_power_speeds(W, S, h, CD0, AR, e):
    _ws_in(W, S)
    _cd0(CD0)
    K = _K(AR, e)
    rho = _rho_at(h)
    vmd = math.sqrt(2 * W / (rho * S) * math.sqrt(K / CD0))
    vmp = vmd / 3 ** 0.25

    def drag(V):
        q = 0.5 * rho * V * V
        return q * S * CD0 + K * W * W / (q * S)
    return _out(V_md=vmd, D_min=2 * W * math.sqrt(K * CD0), V_mp=vmp, P_min=drag(vmp) * vmp, V_mr=vmd * 3 ** 0.25)


def ref_min_drag_power_speeds(W, S, h, CD0, AR, e):
    K = 1.0 / (math.pi * e * AR)
    rho = _ref_rho(h)
    drag = lambda V: 0.5 * rho * V * V * S * CD0 + K * W * W / (0.5 * rho * V * V * S)    # noqa: E731
    v0 = math.sqrt(2 * W / (rho * S))
    a = _cs_argmin(drag, 1e-3 * v0, 1e3 * v0)
    b = _cs_argmin(lambda V: drag(V) * V, 1e-3 * v0, 1e3 * v0)
    c = _cs_argmin(lambda V: drag(V) / V, 1e-3 * v0, 1e3 * v0)
    return {"V_md": a, "D_min": drag(a), "V_mp": b, "P_min": drag(b) * b, "V_mr": c}


_PERF_RANDOM = lambda rng: {"W": _lu(rng, 5e3, 3e6), "S": _lu(rng, 10.0, 500.0), "h": rng.uniform(0.0, 13000.0), **_POLAR_RANDOM(rng)}   # noqa: E731

_add("aero_min_drag_power_speeds",
     name="Speeds for minimum drag, minimum power and best jet range", topic="aircraft performance / level flight",
     purpose="In steady level flight at an altitude: the speed for minimum drag and the minimum drag, the speed for minimum power required and that power, and the jet's best-range speed, for a parabolic drag polar.",
     inputs=[_phys("W", "force", "aircraft weight (N, kN, lbf)"), _phys("S", "area", "wing area (m^2, ft^2)"), _phys("h", "length", "altitude (standard atmosphere)"),
             _numin("CD0", "zero-lift drag coefficient"), _numin("AR", "aspect ratio"), _numin("e", "Oswald efficiency factor")],
     outputs=[_o("V_md", "speed", "speed for minimum drag (max L/D)"), _o("D_min", "force", "minimum drag"), _o("V_mp", "speed", "speed for minimum power required"),
              _o("P_min", "power", "minimum power required"), _o("V_mr", "speed", "best-range speed of a jet (max V/D)")],
     primary="V_md", conditions=["W, S > 0", "-1 km <= h <= 47 km", "parabolic drag polar", "steady level flight, L = W"],
     method="V_md = sqrt(2W/(rho S) sqrt(K/CD0)); D_min = 2W sqrt(K CD0); V_mp = V_md/3^(1/4); V_mr = 3^(1/4) V_md",
     latex=r"V_{md} = \sqrt{\frac{2W}{\rho S}\sqrt{\frac{K}{C_{D0}}}},\quad D_{min} = 2W\sqrt{K C_{D0}},\quad V_{mp} = \frac{V_{md}}{3^{1/4}}",
     run=min_drag_power_speeds,
     parse=lambda q: _pairs(q, r"speeds? for minimum drag(?:, minimum power and best jet range)?|minimum drag speed", _POLAR_NAMES, phys={"W", "S", "h"}),
     reference=ref_min_drag_power_speeds,
     wrong=_safe(lambda W, S, h, CD0, AR, e: (lambda o: {**o, "V_mp": _r(o["V_md"] * 3 ** 0.25), "V_mr": _r(o["V_md"] / 3 ** 0.25)})(min_drag_power_speeds(W, S, h, CD0, AR, e))),
     random_inputs=_PERF_RANDOM,
     templates=["Speeds for minimum drag: W = 50 kN, S = 30 m^2, h = 3000 m, CD0 = 0.025, AR = 8, e = 0.8",
                "Minimum drag speed: W = 600 kN, S = 120 m^2, h = 11 km, CD0 = 0.02, AR = 9.5, e = 0.85"],
     examples=[{"inputs": {"W": 1.0e4, "S": 20.0, "h": 0.0, "CD0": 0.02, "AR": 10.0, "e": 0.8}, "outputs": {"D_min": 2e4 * math.sqrt(0.02 / (8 * math.pi))}}],
     questions=[{"q": "Speeds for minimum drag: W = 10 kN, S = 20 m^2, h = 0 m, CD0 = 0.02, AR = 10, e = 0.8", "outputs": {"D_min": 2e4 * math.sqrt(0.02 / (8 * math.pi))}},
                {"q": "Speeds for minimum drag: W = -10 kN, S = 20 m^2, h = 0 m, CD0 = 0.02, AR = 10, e = 0.8", "outcome": "INVALID_INPUT"},
                {"q": "Speeds for minimum drag: W = 10 kN, S = 20 m^2, CD0 = 0.02, AR = 10, e = 0.8", "outcome": "NEEDS_INPUT"}])


def _tsfc_in(c):
    _need(isinstance(c, (int, float)) and 1e-7 <= c <= 1e-3, "the thrust-specific fuel consumption must lie between 1e-7 and 1e-3 kg/(N s)")


def _bsfc_in(c):
    _need(isinstance(c, (int, float)) and 1e-9 <= c <= 1e-5, "the power-specific fuel consumption must lie between 1e-9 and 1e-5 kg/J")


def _ld_in(LD):
    _need(isinstance(LD, (int, float)) and 1.0 <= LD <= 60.0, "the lift-to-drag ratio must lie between 1 and 60")


def _w01(W0, W1):
    _pos("the initial and final weights", W0, W1)
    _need(W1 < W0, "the final weight must be below the initial weight")


def breguet_jet(V, c, LD, W0, W1):
    _pos("the flight speed", V)
    _tsfc_in(c)
    _ld_in(LD)
    _w01(W0, W1)
    R = V * LD / (G0 * c) * math.log(W0 / W1)
    return _out(R=R, R_km=R / 1e3, t=R / V, t_h=R / V / 3600.0)


def ref_breguet_jet(V, c, LD, W0, W1):
    from scipy.integrate import quad
    # cruise at constant V and L/D: dW/dt = -g0 c T = -g0 c W/(L/D), dx = V dt, integrated over the fuel burned
    t, _ = quad(lambda W: LD / (G0 * c * W), W1, W0, epsabs=0, epsrel=1e-13)
    R, _ = quad(lambda W: V * LD / (G0 * c * W), W1, W0, epsabs=0, epsrel=1e-13)
    return {"R": R, "R_km": R / 1e3, "t": t, "t_h": t / 3600.0}


_FUEL_NAMES = {"v": "V", "speed": "V", "c": "c", "tsfc": "c", "sfc": "c", "bsfc": "c", "ld": "LD", "l/d": "LD", "w0": "W0", "w1": "W1", "wf": "W1",
               "r": "R", "range": "R", "eta": "eta", "eta_p": "eta", "cl": "CL", "cd": "CD", "s": "S", "h": "h"}

_add("aero_breguet_range_jet",
     name="Breguet range of a jet aircraft", topic="aircraft performance / range and endurance",
     purpose="The cruise range and flight time of a jet aircraft at constant speed and lift-to-drag ratio (cruise-climb) from its thrust-specific fuel consumption and the weights at the start and end of cruise.",
     inputs=[_phys("V", "speed", "cruise true airspeed"), _phys("c", "thrust_specific_fuel_consumption", "thrust-specific fuel consumption (kg/(N*s), 1/h, lb/(lbf*h))"),
             _numin("LD", "lift-to-drag ratio"), _phys("W0", "force", "weight at the start of cruise"), _phys("W1", "force", "weight at the end of cruise")],
     outputs=[_o("R", "length", "range"), _o("R_km", "number", "range in km"), _o("t", "time", "flight time"), _o("t_h", "number", "flight time in hours")],
     primary="R", conditions=["V > 0", "0 < W1 < W0", "1 <= L/D <= 60", "constant V, L/D and TSFC (cruise-climb)"],
     method="R = (V/(g0 c)) (L/D) ln(W0/W1), c the mass-flow TSFC", latex=r"R = \frac{V}{g_0 c}\,\frac{L}{D}\,\ln\frac{W_0}{W_1}",
     run=breguet_jet,
     parse=lambda q: _pairs(q, r"breguet range(?: of a jet(?: aircraft)?)?|jet breguet range|jet range", _FUEL_NAMES, phys={"V", "c", "W0", "W1"}),
     reference=ref_breguet_jet,
     wrong=_safe(lambda V, c, LD, W0, W1: (lambda R: _out(R=R, R_km=R / 1e3, t=R / V, t_h=R / V / 3600.0))(V * LD / (G0 * c) * (W0 - W1) / W0)),
     random_inputs=lambda rng: (lambda W0: {"V": rng.uniform(100.0, 280.0), "c": rng.uniform(1e-5, 3e-5), "LD": rng.uniform(8.0, 22.0), "W0": W0,
                                           "W1": W0 * rng.uniform(0.55, 0.95)})(_lu(rng, 5e4, 4e6)),
     templates=["Breguet range: V = 230 m/s, c = 0.6 1/h, LD = 17, W0 = 2000 kN, W1 = 1500 kN",
                "Jet range: V = 450 kt, c = 0.55 lb/(lbf*h), LD = 15, W0 = 160000 lbf, W1 = 120000 lbf"],
     examples=[{"inputs": {"V": 200.0, "c": 1 / G0 / 3600.0, "LD": 15.0, "W0": math.e * 1e5, "W1": 1e5}, "outputs": {"R": 200.0 * 15.0 * 3600.0, "t_h": 15.0}}],
     questions=[{"q": "Breguet range: V = 200 m/s, c = 1 1/h, LD = 15, W0 = " + repr(math.e * 100) + " kN, W1 = 100 kN", "outputs": {"t_h": 15.0}},
                {"q": "Breguet range: V = 200 m/s, c = 0.6 1/h, LD = 15, W0 = 100 kN, W1 = 150 kN", "outcome": "INVALID_INPUT"},
                {"q": "Breguet range: V = 200 m/s, c = 0.6 1/h, LD = 15, W0 = 100 kN", "outcome": "NEEDS_INPUT"}])


def breguet_endurance_jet(c, LD, W0, W1):
    _tsfc_in(c)
    _ld_in(LD)
    _w01(W0, W1)
    E = LD / (G0 * c) * math.log(W0 / W1)
    return _out(E=E, E_h=E / 3600.0)


_add("aero_breguet_endurance_jet",
     name="Endurance of a jet aircraft", topic="aircraft performance / range and endurance",
     purpose="The time a jet aircraft can stay airborne at constant lift-to-drag ratio from its thrust-specific fuel consumption and initial and final weights (independent of speed).",
     inputs=[_phys("c", "thrust_specific_fuel_consumption", "thrust-specific fuel consumption (kg/(N*s), 1/h, lb/(lbf*h))"),
             _numin("LD", "lift-to-drag ratio"), _phys("W0", "force", "initial weight"), _phys("W1", "force", "final weight")],
     outputs=[_o("E", "time", "endurance"), _o("E_h", "number", "endurance in hours")],
     primary="E", conditions=["0 < W1 < W0", "1 <= L/D <= 60", "constant L/D and TSFC"],
     method="E = (1/(g0 c)) (L/D) ln(W0/W1)", latex=r"E = \frac{1}{g_0 c}\,\frac{L}{D}\,\ln\frac{W_0}{W_1}",
     run=breguet_endurance_jet,
     parse=lambda q: _pairs(q, r"endurance of a jet(?: aircraft)?|jet endurance", _FUEL_NAMES, phys={"c", "W0", "W1"}),
     reference=lambda c, LD, W0, W1: (lambda E: {"E": E, "E_h": E / 3600.0})(__import__("scipy.integrate", fromlist=["quad"]).quad(lambda W: LD / (G0 * c * W), W1, W0, epsabs=0, epsrel=1e-13)[0]),
     wrong=_safe(lambda c, LD, W0, W1: (lambda E: _out(E=E, E_h=E / 3600.0))(LD / c * math.log(W0 / W1))),
     random_inputs=lambda rng: (lambda W0: {"c": rng.uniform(1e-5, 3e-5), "LD": rng.uniform(8.0, 22.0), "W0": W0, "W1": W0 * rng.uniform(0.55, 0.95)})(_lu(rng, 5e4, 4e6)),
     templates=["Jet endurance: c = 0.6 1/h, LD = 18, W0 = 300 kN, W1 = 240 kN", "Endurance of a jet aircraft: c = 0.0000185 kg/(N*s), LD = 16, W0 = 90000 lbf, W1 = 70000 lbf"],
     examples=[{"inputs": {"c": 1 / G0 / 3600.0, "LD": 15.0, "W0": math.e * 1e5, "W1": 1e5}, "outputs": {"E_h": 15.0}}],
     questions=[{"q": "Jet endurance: c = 1 1/h, LD = 15, W0 = " + repr(math.e * 100) + " kN, W1 = 100 kN", "outputs": {"E_h": 15.0}},
                {"q": "Jet endurance: c = 0.6 1/h, LD = 100, W0 = 300 kN, W1 = 240 kN", "outcome": "INVALID_INPUT"},
                {"q": "Jet endurance: c = 0.6 1/h, LD = 15, W0 = 300 kN", "outcome": "NEEDS_INPUT"}])


def jet_fuel_for_range(R, V, c, LD, W0):
    _pos("the range and the speed", R, V)
    _tsfc_in(c)
    _ld_in(LD)
    _pos("the initial weight", W0)
    W1 = W0 * math.exp(-R * G0 * c / (V * LD))
    _need(W1 >= 0.2 * W0, "the fuel needed would exceed 80% of the initial weight")
    return _out(W1=W1, Wf=W0 - W1, mf=(W0 - W1) / G0)


def ref_jet_fuel_for_range(R, V, c, LD, W0):
    # dW/dx = -g0 c W/(V L/D), integrated over the distance (DOP853), in units of W0 and of the stated range
    k = R * G0 * c / (V * LD)
    w = _ivp(lambda x, y: [-k * y[0]], 1.0, [1.0])[0]
    return {"W1": w * W0, "Wf": (1.0 - w) * W0, "mf": (1.0 - w) * W0 / G0}


_add("aero_jet_fuel_for_range",
     name="Fuel a jet needs for a cruise range", topic="aircraft performance / range and endurance",
     purpose="The fuel weight (and mass) a jet aircraft burns cruising a distance at constant speed and L/D, and its weight at the end, from the Breguet relation.",
     inputs=[_phys("R", "length", "cruise distance (km, nmi)"), _phys("V", "speed", "cruise speed"),
             _phys("c", "thrust_specific_fuel_consumption", "thrust-specific fuel consumption"), _numin("LD", "lift-to-drag ratio"),
             _phys("W0", "force", "weight at the start of cruise")],
     outputs=[_o("W1", "force", "weight at the end of cruise"), _o("Wf", "force", "fuel weight burned"), _o("mf", "mass", "fuel mass burned")],
     primary="Wf", conditions=["R, V > 0", "fuel at most 80% of W0", "constant V, L/D and TSFC"],
     method="W1 = W0 exp(-R g0 c/(V L/D)); Wf = W0 - W1", latex=r"W_1 = W_0\exp\left(-\frac{R\,g_0 c}{V\,(L/D)}\right)",
     run=jet_fuel_for_range,
     parse=lambda q: _pairs(q, r"fuel (?:a jet needs )?for (?:a )?(?:cruise )?range|jet fuel for range", _FUEL_NAMES, phys={"R", "V", "c", "W0"}),
     reference=ref_jet_fuel_for_range,
     wrong=_safe(lambda R, V, c, LD, W0: (lambda W1: _out(W1=W1, Wf=W0 - W1, mf=(W0 - W1) / G0))(W0 * (1 - R * G0 * c / (V * LD)))),
     random_inputs=lambda rng: {"R": _lu(rng, 2e5, 1.2e7), "V": rng.uniform(150.0, 260.0), "c": rng.uniform(1.2e-5, 2.5e-5), "LD": rng.uniform(12.0, 20.0), "W0": _lu(rng, 5e4, 4e6)},
     templates=["Fuel for range: R = 5000 km, V = 230 m/s, c = 0.6 1/h, LD = 17, W0 = 2000 kN", "Jet fuel for range: R = 2500 nmi, V = 470 kt, c = 0.58 lb/(lbf*h), LD = 16, W0 = 170000 lbf"],
     examples=[{"inputs": {"R": 200.0 * 15.0 * 3600.0, "V": 200.0, "c": 1 / G0 / 3600.0, "LD": 15.0, "W0": math.e * 1e5}, "outputs": {"W1": 1e5}}],
     questions=[{"q": "Fuel for range: R = 10800 km, V = 200 m/s, c = 1 1/h, LD = 15, W0 = " + repr(math.e * 100) + " kN", "outputs": {"W1": 1e5}},
                {"q": "Fuel for range: R = 100000 km, V = 200 m/s, c = 1 1/h, LD = 15, W0 = 100 kN", "outcome": "INVALID_INPUT"},
                {"q": "Fuel for range: R = 1000 km, V = 200 m/s, c = 1 1/h, LD = 15", "outcome": "NEEDS_INPUT"}])


def _clcd_in(CL, CD):
    _need(isinstance(CL, (int, float)) and 0.05 <= CL <= 3.0, "the lift coefficient must lie between 0.05 and 3")
    _need(isinstance(CD, (int, float)) and 0.003 <= CD <= 0.5 and CD < CL, "the drag coefficient must lie between 0.003 and 0.5 and below CL")


def jet_range_constant_altitude(W0, W1, S, h, CL, CD, c):
    _w01(W0, W1)
    _pos("the wing area", S)
    _clcd_in(CL, CD)
    _tsfc_in(c)
    rho = _rho_at(h)
    R = 2.0 / (G0 * c) * math.sqrt(2.0 / (rho * S)) * math.sqrt(CL) / CD * (math.sqrt(W0) - math.sqrt(W1))
    t = CL / CD / (G0 * c) * math.log(W0 / W1)
    return _out(R=R, R_km=R / 1e3, t=t, t_h=t / 3600.0)


def ref_jet_range_constant_altitude(W0, W1, S, h, CL, CD, c):
    from scipy.integrate import quad
    rho = _ref_rho(h)
    V = lambda W: math.sqrt(2 * W / (rho * S * CL))                          # noqa: E731  lift = weight at constant CL and altitude
    dt = lambda W: (CL / CD) / (G0 * c * W)                                  # noqa: E731  dW/dt = -g0 c D, D = W CD/CL
    R, _ = quad(lambda W: V(W) * dt(W), W1, W0, epsabs=0, epsrel=1e-13)
    t, _ = quad(dt, W1, W0, epsabs=0, epsrel=1e-13)
    return {"R": R, "R_km": R / 1e3, "t": t, "t_h": t / 3600.0}


_add("aero_jet_range_constant_altitude",
     name="Jet range at constant altitude and lift coefficient", topic="aircraft performance / range and endurance",
     purpose="The range and flight time of a jet cruising at a constant altitude and constant lift coefficient, so that its speed falls as fuel burns (the second Breguet form).",
     inputs=[_phys("W0", "force", "initial weight"), _phys("W1", "force", "final weight"), _phys("S", "area", "wing area"), _phys("h", "length", "cruise altitude"),
             _numin("CL", "cruise lift coefficient"), _numin("CD", "cruise drag coefficient"),
             _phys("c", "thrust_specific_fuel_consumption", "thrust-specific fuel consumption")],
     outputs=[_o("R", "length", "range"), _o("R_km", "number", "range in km"), _o("t", "time", "flight time"), _o("t_h", "number", "flight time in hours")],
     primary="R", conditions=["0 < W1 < W0", "0.05 <= CL <= 3", "CD < CL", "constant altitude, CL and TSFC"],
     method="R = (2/(g0 c)) sqrt(2/(rho S)) (sqrt(CL)/CD) (sqrt(W0) - sqrt(W1)); t = (CL/CD)/(g0 c) ln(W0/W1)",
     latex=r"R = \frac{2}{g_0 c}\sqrt{\frac{2}{\rho S}}\,\frac{C_L^{1/2}}{C_D}\left(\sqrt{W_0} - \sqrt{W_1}\right)",
     run=jet_range_constant_altitude,
     parse=lambda q: _pairs(q, r"jet range at constant altitude(?: and lift coefficient)?", _FUEL_NAMES, phys={"W0", "W1", "S", "h", "c"}),
     reference=ref_jet_range_constant_altitude,
     wrong=_safe(lambda W0, W1, S, h, CL, CD, c: (lambda o: {**o, "R": _r(o["R"] / 2), "R_km": _r(o["R_km"] / 2)})(jet_range_constant_altitude(W0, W1, S, h, CL, CD, c))),
     random_inputs=lambda rng: (lambda W0, CL: {"W0": W0, "W1": W0 * rng.uniform(0.55, 0.95), "S": _lu(rng, 20.0, 500.0), "h": rng.uniform(3000.0, 13000.0),
                                                "CL": CL, "CD": CL / rng.uniform(10.0, 20.0), "c": rng.uniform(1e-5, 3e-5)})(_lu(rng, 5e4, 4e6), rng.uniform(0.3, 0.7)),
     templates=["Jet range at constant altitude: W0 = 600 kN, W1 = 450 kN, S = 120 m^2, h = 11 km, CL = 0.5, CD = 0.03, c = 0.6 1/h"],
     examples=[{"inputs": {"W0": 4e5, "W1": 1e5, "S": 100.0, "h": 0.0, "CL": 0.5, "CD": 0.05, "c": 1e-5},
                "outputs": {"R": 2 / (G0 * 1e-5) * math.sqrt(2 / (RHO_SL * 100.0)) * math.sqrt(0.5) / 0.05 * (math.sqrt(4e5) - math.sqrt(1e5))}}],
     questions=[{"q": "Jet range at constant altitude: W0 = 400 kN, W1 = 100 kN, S = 100 m^2, h = 0 m, CL = 0.5, CD = 0.05, c = 0.00001 kg/(N*s)",
                 "outputs": {"t": 10 / (G0 * 1e-5) * math.log(4)}},
                {"q": "Jet range at constant altitude: W0 = 400 kN, W1 = 100 kN, S = 100 m^2, h = 0 m, CL = 0.5, CD = 0.8, c = 0.00001 kg/(N*s)", "outcome": "INVALID_INPUT"},
                {"q": "Jet range at constant altitude: W0 = 400 kN, W1 = 100 kN, S = 100 m^2, CL = 0.5, CD = 0.05, c = 0.00001 kg/(N*s)", "outcome": "NEEDS_INPUT"}])


def _eta_in(eta):
    _need(isinstance(eta, (int, float)) and 0.1 <= eta <= 1.0, "the propeller efficiency must lie between 0.1 and 1")


def breguet_prop(eta, c, LD, W0, W1):
    _eta_in(eta)
    _bsfc_in(c)
    _ld_in(LD)
    _w01(W0, W1)
    R = eta / (G0 * c) * LD * math.log(W0 / W1)
    return _out(R=R, R_km=R / 1e3)


def ref_breguet_prop(eta, c, LD, W0, W1):
    # fuel weight flow g0 c P_shaft with P_shaft = D V/eta: dW/dx = -g0 c W/(eta L/D), integrated over the distance (DOP853)
    k = G0 * c / (eta * LD)
    x = _ivp(lambda w, y: [1.0 / (k * w)], 1.0, [0.0], t0=W1 / W0)[0]
    return {"R": x, "R_km": x / 1e3}


_add("aero_breguet_range_prop",
     name="Breguet range of a propeller aircraft", topic="aircraft performance / range and endurance",
     purpose="The cruise range of a propeller-driven aircraft at constant lift-to-drag ratio and propeller efficiency from its power-specific fuel consumption and initial and final weights.",
     inputs=[_numin("eta", "propeller efficiency"), _phys("c", "power_specific_fuel_consumption", "power-specific fuel consumption (kg/J, g/(kW*h), lb/(hp*h))"),
             _numin("LD", "lift-to-drag ratio"), _phys("W0", "force", "initial weight"), _phys("W1", "force", "final weight")],
     outputs=[_o("R", "length", "range"), _o("R_km", "number", "range in km")],
     primary="R", conditions=["0.1 <= eta <= 1", "0 < W1 < W0", "1 <= L/D <= 60", "constant L/D, efficiency and BSFC"],
     method="R = (eta/(g0 c)) (L/D) ln(W0/W1), c the mass flow per shaft power", latex=r"R = \frac{\eta_p}{g_0 c_P}\,\frac{L}{D}\,\ln\frac{W_0}{W_1}",
     run=breguet_prop,
     parse=lambda q: _pairs(q, r"breguet range of a propeller(?: aircraft)?|propeller breguet range|prop(?:eller)? range", _FUEL_NAMES, phys={"c", "W0", "W1"}),
     reference=ref_breguet_prop,
     wrong=_safe(lambda eta, c, LD, W0, W1: (lambda R: _out(R=R, R_km=R / 1e3))(1.0 / (eta * G0 * c) * LD * math.log(W0 / W1))),
     random_inputs=lambda rng: (lambda W0: {"eta": rng.uniform(0.6, 0.9), "c": rng.uniform(5e-8, 1.2e-7), "LD": rng.uniform(8.0, 20.0), "W0": W0,
                                           "W1": W0 * rng.uniform(0.6, 0.95)})(_lu(rng, 5e3, 5e5)),
     templates=["Propeller range: eta = 0.8, c = 0.45 lb/(hp*h), LD = 14, W0 = 12 kN, W1 = 10 kN",
                "Breguet range of a propeller aircraft: eta = 0.85, c = 250 g/(kW*h), LD = 16, W0 = 200 kN, W1 = 170 kN"],
     examples=[{"inputs": {"eta": 0.8, "c": 1e-7, "LD": 15.0, "W0": math.e * 1e4, "W1": 1e4}, "outputs": {"R": 0.8 / (G0 * 1e-7) * 15.0}}],
     questions=[{"q": "Propeller range: eta = 0.8, c = 0.0000001 kg/J, LD = 15, W0 = " + repr(math.e * 10) + " kN, W1 = 10 kN", "outputs": {"R": 0.8 / (G0 * 1e-7) * 15.0}},
                {"q": "Propeller range: eta = 1.5, c = 0.0000001 kg/J, LD = 15, W0 = 20 kN, W1 = 10 kN", "outcome": "INVALID_INPUT"},
                {"q": "Propeller range: eta = 0.8, c = 0.0000001 kg/J, LD = 15, W0 = 20 kN", "outcome": "NEEDS_INPUT"}])


def prop_endurance(eta, c, CL, CD, S, h, W0, W1):
    _eta_in(eta)
    _bsfc_in(c)
    _clcd_in(CL, CD)
    _pos("the wing area", S)
    _w01(W0, W1)
    rho = _rho_at(h)
    E = eta / (G0 * c) * CL ** 1.5 / CD * math.sqrt(2 * rho * S) * (1 / math.sqrt(W1) - 1 / math.sqrt(W0))
    return _out(E=E, E_h=E / 3600.0)


def ref_prop_endurance(eta, c, CL, CD, S, h, W0, W1):
    from scipy.integrate import quad
    rho = _ref_rho(h)

    def dt(W):                                                     # dW/dt = -g0 c P_shaft, P_shaft = D V/eta
        V = math.sqrt(2 * W / (rho * S * CL))
        return eta / (G0 * c * (W * CD / CL) * V)
    E, _ = quad(dt, W1, W0, epsabs=0, epsrel=1e-13)
    return {"E": E, "E_h": E / 3600.0}


_add("aero_prop_endurance_constant_altitude",
     name="Endurance of a propeller aircraft at constant altitude", topic="aircraft performance / range and endurance",
     purpose="How long a propeller-driven aircraft can loiter at a constant altitude and lift coefficient, from its propeller efficiency, power-specific fuel consumption, drag polar point, wing area and weights.",
     inputs=[_numin("eta", "propeller efficiency"), _phys("c", "power_specific_fuel_consumption", "power-specific fuel consumption"),
             _numin("CL", "lift coefficient"), _numin("CD", "drag coefficient"), _phys("S", "area", "wing area"), _phys("h", "length", "altitude"),
             _phys("W0", "force", "initial weight"), _phys("W1", "force", "final weight")],
     outputs=[_o("E", "time", "endurance"), _o("E_h", "number", "endurance in hours")],
     primary="E", conditions=["0.1 <= eta <= 1", "0 < W1 < W0", "0.05 <= CL <= 3", "constant altitude and CL"],
     method="E = (eta/(g0 c)) (CL^1.5/CD) sqrt(2 rho S) (W1^(-1/2) - W0^(-1/2))",
     latex=r"E = \frac{\eta_p}{g_0 c_P}\,\frac{C_L^{3/2}}{C_D}\sqrt{2\rho S}\left(W_1^{-1/2} - W_0^{-1/2}\right)",
     run=prop_endurance,
     parse=lambda q: _pairs(q, r"endurance of a propeller(?: aircraft)?(?: at constant altitude)?|propeller endurance", _FUEL_NAMES, phys={"c", "S", "h", "W0", "W1"}),
     reference=ref_prop_endurance,
     wrong=_safe(lambda eta, c, CL, CD, S, h, W0, W1: (lambda o: {**o, "E": _r(o["E"] / math.sqrt(CL)), "E_h": _r(o["E_h"] / math.sqrt(CL))})(prop_endurance(eta, c, CL, CD, S, h, W0, W1))),
     random_inputs=lambda rng: (lambda W0, CL: {"eta": rng.uniform(0.6, 0.9), "c": rng.uniform(5e-8, 1.2e-7), "CL": CL, "CD": CL / rng.uniform(8.0, 18.0),
                                                "S": _lu(rng, 10.0, 100.0), "h": rng.uniform(0.0, 8000.0), "W0": W0, "W1": W0 * rng.uniform(0.6, 0.95)})(_lu(rng, 5e3, 3e5), rng.uniform(0.4, 1.2)),
     templates=["Propeller endurance: eta = 0.8, c = 0.45 lb/(hp*h), CL = 0.9, CD = 0.06, S = 16 m^2, h = 2000 m, W0 = 11 kN, W1 = 9 kN"],
     examples=[{"inputs": {"eta": 0.8, "c": 1e-7, "CL": 1.0, "CD": 0.05, "S": 20.0, "h": 0.0, "W0": 4e4, "W1": 1e4},
                "outputs": {"E": 0.8 / (G0 * 1e-7) * 20.0 * math.sqrt(2 * RHO_SL * 20.0) * (1 / 100.0 - 1 / 200.0)}}],
     questions=[{"q": "Propeller endurance: eta = 0.8, c = 0.0000001 kg/J, CL = 1, CD = 0.05, S = 20 m^2, h = 0 m, W0 = 40 kN, W1 = 10 kN",
                 "outputs": {"E": 0.8 / (G0 * 1e-7) * 20.0 * math.sqrt(2 * RHO_SL * 20.0) * (1 / 100.0 - 1 / 200.0)}},
                {"q": "Propeller endurance: eta = 0.8, c = 0.0000001 kg/J, CL = 1, CD = 0.05, S = 20 m^2, h = 0 m, W0 = 10 kN, W1 = 40 kN", "outcome": "INVALID_INPUT"},
                {"q": "Propeller endurance: eta = 0.8, c = 0.0000001 kg/J, CL = 1, CD = 0.05, S = 20 m^2, h = 0 m, W0 = 40 kN", "outcome": "NEEDS_INPUT"}])


def _roll_terms(W, S, h, mu, CL, CD):
    _ws_in(W, S)
    _need(isinstance(mu, (int, float)) and 0 <= mu <= 0.8, "the friction coefficient must lie between 0 and 0.8")
    _need(isinstance(CL, (int, float)) and 0 <= CL <= 3.0, "the ground-run lift coefficient must lie between 0 and 3")
    _need(isinstance(CD, (int, float)) and 0.005 <= CD <= 1.0, "the ground-run drag coefficient must lie between 0.005 and 1")
    rho = _rho_at(h)
    return G0 * rho * S * (CD - mu * CL) / (2 * W)


def takeoff_roll(W, S, h, T, mu, CL, CD, VLO):
    B = _roll_terms(W, S, h, mu, CL, CD)
    _pos("the thrust and the lift-off speed", T, VLO)
    A = G0 * (T / W - mu)
    _need(A > 0 and A - B * VLO * VLO > 0, "the aircraft would stop accelerating before the lift-off speed (thrust too low)")
    x = B * VLO * VLO / A
    s = -math.log1p(-x) / (2 * B) if abs(x) > 1e-8 else VLO * VLO / (2 * A) * (1 + x / 2 + x * x / 3)
    if abs(x) < 1e-6:
        t = VLO / A * (1 + x / 3 + x * x / 5)
    elif B > 0:
        t = math.atanh(math.sqrt(x)) / math.sqrt(A * B)
    else:
        t = math.atan(math.sqrt(-x)) / math.sqrt(-A * B)
    return _out(s=s, t=t)


def ref_takeoff_roll(W, S, h, T, mu, CL, CD, VLO):
    from scipy.integrate import quad
    rho = _ref_rho(h)
    m = W / G0

    def acc(V):                                       # thrust, drag, rolling friction on the weight not carried by the wing
        L = 0.5 * rho * V * V * S * CL
        D = 0.5 * rho * V * V * S * CD
        return (T - D - mu * (W - L)) / m
    s, _ = quad(lambda V: V / acc(V), 0.0, VLO, epsabs=0, epsrel=1e-13)
    t, _ = quad(lambda V: 1.0 / acc(V), 0.0, VLO, epsabs=0, epsrel=1e-13)
    return {"s": s, "t": t}


def _to_random(rng):
    W = _lu(rng, 5e3, 3e6)
    S = W / rng.uniform(1000.0, 7000.0)
    h = rng.uniform(0.0, 3000.0)
    CLmax = rng.uniform(1.4, 2.6)
    VLO = 1.2 * math.sqrt(2 * W / (_rho_at(h) * S * CLmax))
    return {"W": W, "S": S, "h": h, "T": W * rng.uniform(0.2, 0.45), "mu": rng.uniform(0.01, 0.05), "CL": rng.uniform(0.1, 0.8),
            "CD": rng.uniform(0.03, 0.12), "VLO": VLO}


_ROLL_NAMES = {"w": "W", "weight": "W", "s": "S", "h": "h", "t": "T", "thrust": "T", "mu": "mu", "mu_r": "mu", "mu_b": "mu", "cl": "CL", "cd": "CD",
               "vlo": "VLO", "v_lo": "VLO", "vtd": "VTD", "v_td": "VTD", "tr": "Tr", "t_rev": "Tr", "reverse_thrust": "Tr"}

_add("aero_takeoff_ground_roll",
     name="Takeoff ground roll", topic="aircraft performance / takeoff and landing",
     purpose="The ground-roll distance and time to lift-off speed of an aircraft accelerating under constant thrust against aerodynamic drag and rolling friction on a level runway.",
     inputs=[_phys("W", "force", "weight"), _phys("S", "area", "wing area"), _phys("h", "length", "airfield altitude (standard atmosphere)"),
             _phys("T", "force", "thrust (constant during the roll)"), _numin("mu", "rolling friction coefficient"),
             _numin("CL", "lift coefficient during the ground run"), _numin("CD", "drag coefficient during the ground run"),
             _phys("VLO", "speed", "lift-off speed")],
     outputs=[_o("s", "length", "ground-roll distance"), _o("t", "time", "ground-roll time")],
     primary="s", conditions=["thrust large enough to keep accelerating up to VLO", "0 <= mu <= 0.8", "constant thrust, level runway, no wind"],
     method="dV/dt = A - B V^2, A = g (T/W - mu), B = g rho S (CD - mu CL)/(2W); s = ln(A/(A - B VLO^2))/(2B); t = artanh(VLO sqrt(B/A))/sqrt(A B)",
     latex=r"s = \frac{1}{2B}\ln\frac{A}{A - BV_{LO}^2},\quad A = g\left(\frac{T}{W} - \mu\right),\quad B = \frac{g\rho S}{2W}(C_D - \mu C_L)",
     run=takeoff_roll,
     parse=lambda q: _pairs(q, r"takeoff (?:ground )?roll|take-off (?:ground )?roll", _ROLL_NAMES, phys={"W", "S", "h", "T", "VLO"}),
     reference=ref_takeoff_roll,
     wrong=_safe(lambda W, S, h, T, mu, CL, CD, VLO: (lambda A: _out(s=VLO * VLO / (2 * A), t=VLO / A))(G0 * (T / W - mu))),
     random_inputs=_to_random,
     templates=["Takeoff roll: W = 60 kN, S = 25 m^2, h = 0 m, T = 15 kN, mu = 0.02, CL = 0.3, CD = 0.05, VLO = 50 m/s",
                "Takeoff ground roll: W = 700 kN, S = 125 m^2, h = 1500 m, T = 220 kN, mu = 0.02, CL = 0.6, CD = 0.08, VLO = 150 kt"],
     examples=[{"inputs": {"W": 1e4, "S": 10.0, "h": 0.0, "T": 3000.0, "mu": 0.02, "CL": 1.0, "CD": 0.02, "VLO": 40.0}, "outputs": {"s": 1600.0 / (2 * G0 * 0.28), "t": 40.0 / (G0 * 0.28)}}],
     questions=[{"q": "Takeoff roll: W = 10 kN, S = 10 m^2, h = 0 m, T = 3 kN, mu = 0.02, CL = 1, CD = 0.02, VLO = 40 m/s", "outputs": {"s": 1600.0 / (2 * G0 * 0.28)}},
                {"q": "Takeoff roll: W = 10 kN, S = 10 m^2, h = 0 m, T = 0.1 kN, mu = 0.02, CL = 1, CD = 0.02, VLO = 40 m/s", "outcome": "INVALID_INPUT"},
                {"q": "Takeoff roll: W = 10 kN, S = 10 m^2, h = 0 m, T = 3 kN, mu = 0.02, CL = 1, CD = 0.02", "outcome": "NEEDS_INPUT"}])


def landing_roll(W, S, h, VTD, mu, CL, CD, Tr=0.0):
    B = _roll_terms(W, S, h, mu, CL, CD)
    _pos("the touchdown speed", VTD)
    _need(isinstance(Tr, (int, float)) and Tr >= 0, "the reverse thrust must not be negative")
    A = G0 * (Tr / W + mu)
    _need(A > 0 and A + B * VTD * VTD > 0, "the aircraft would not decelerate (no braking, reverse thrust or drag)")
    x = B * VTD * VTD / A
    s = math.log1p(x) / (2 * B) if abs(x) > 1e-8 else VTD * VTD / (2 * A) * (1 - x / 2 + x * x / 3)
    if abs(x) < 1e-6:
        t = VTD / A * (1 - x / 3 + x * x / 5)
    elif B > 0:
        t = math.atan(math.sqrt(x)) / math.sqrt(A * B)
    else:
        t = math.atanh(math.sqrt(-x)) / math.sqrt(-A * B)
    return _out(s=s, t=t)


def ref_landing_roll(W, S, h, VTD, mu, CL, CD, Tr=0.0):
    from scipy.integrate import quad
    rho = _ref_rho(h)
    m = W / G0

    def dec(V):                                       # reverse thrust, drag and braking friction on the weight on the wheels
        L = 0.5 * rho * V * V * S * CL
        D = 0.5 * rho * V * V * S * CD
        return (Tr + D + mu * (W - L)) / m
    s, _ = quad(lambda V: V / dec(V), 0.0, VTD, epsabs=0, epsrel=1e-13)
    t, _ = quad(lambda V: 1.0 / dec(V), 0.0, VTD, epsabs=0, epsrel=1e-13)
    return {"s": s, "t": t}


def _ld_random(rng):
    W = _lu(rng, 5e3, 3e6)
    S = W / rng.uniform(1000.0, 7000.0)
    h = rng.uniform(0.0, 3000.0)
    VTD = 1.15 * math.sqrt(2 * W / (_rho_at(h) * S * rng.uniform(1.8, 3.0)))
    d = {"W": W, "S": S, "h": h, "VTD": VTD, "mu": rng.uniform(0.2, 0.5), "CL": rng.uniform(0.0, 0.5), "CD": rng.uniform(0.05, 0.2)}
    if rng.random() < 0.5:
        d["Tr"] = W * rng.uniform(0.05, 0.3)
    return d


_add("aero_landing_ground_roll",
     name="Landing ground roll", topic="aircraft performance / takeoff and landing",
     purpose="The ground-roll distance and time from touchdown to a stop of an aircraft braking on a level runway, with aerodynamic drag, lift (which unloads the wheels) and optional reverse thrust.",
     inputs=[_phys("W", "force", "weight at landing"), _phys("S", "area", "wing area"), _phys("h", "length", "airfield altitude"),
             _phys("VTD", "speed", "touchdown speed"), _numin("mu", "braking friction coefficient"), _numin("CL", "lift coefficient during the roll"),
             _numin("CD", "drag coefficient during the roll"), _phys("Tr", "force", "reverse thrust (default 0)", required=False)],
     outputs=[_o("s", "length", "ground-roll distance"), _o("t", "time", "ground-roll time")],
     primary="s", conditions=["0 <= mu <= 0.8", "the net deceleration positive throughout", "level runway, no wind, constant reverse thrust"],
     method="-dV/dt = A + B V^2, A = g (Tr/W + mu), B = g rho S (CD - mu CL)/(2W); s = ln(1 + B VTD^2/A)/(2B); t = atan(VTD sqrt(B/A))/sqrt(A B)",
     latex=r"s = \frac{1}{2B}\ln\left(1 + \frac{B V_{TD}^2}{A}\right),\quad A = g\left(\frac{T_r}{W} + \mu\right),\quad B = \frac{g\rho S}{2W}(C_D - \mu C_L)",
     run=landing_roll,
     parse=lambda q: _pairs(q, r"landing (?:ground )?roll", _ROLL_NAMES, phys={"W", "S", "h", "VTD", "Tr"}),
     reference=ref_landing_roll,
     wrong=_safe(lambda W, S, h, VTD, mu, CL, CD, Tr=0.0: (lambda A: _out(s=VTD * VTD / A, t=VTD / A))(G0 * (Tr / W + mu))),
     random_inputs=_ld_random,
     templates=["Landing roll: W = 55 kN, S = 25 m^2, h = 0 m, VTD = 40 m/s, mu = 0.4, CL = 0.2, CD = 0.1",
                "Landing ground roll: W = 600 kN, S = 125 m^2, h = 500 m, VTD = 130 kt, mu = 0.35, CL = 0.1, CD = 0.12, Tr = 80 kN"],
     examples=[{"inputs": {"W": 1e4, "S": 10.0, "h": 0.0, "VTD": 30.0, "mu": 0.4, "CL": 0.25, "CD": 0.1}, "outputs": {"s": 900.0 / (2 * G0 * 0.4)}}],
     questions=[{"q": "Landing roll: W = 10 kN, S = 10 m^2, h = 0 m, VTD = 30 m/s, mu = 0.4, CL = 0.25, CD = 0.1", "outputs": {"s": 900.0 / (2 * G0 * 0.4)}},
                {"q": "Landing roll: W = 10 kN, S = 10 m^2, h = 0 m, VTD = 30 m/s, mu = 0.4, CL = 0.25, CD = 0.1, Tr = -5 kN", "outcome": "INVALID_INPUT"},
                {"q": "Landing roll: W = 10 kN, S = 10 m^2, h = 0 m, VTD = 30 m/s, mu = 0.4, CL = 0.25", "outcome": "NEEDS_INPUT"}])


def _turn_out(V, n, g):
    w = g * math.sqrt(n * n - 1) / V
    return _out(n=n, phi_deg=math.degrees(math.acos(1.0 / n)), R=V / w, omega=w, omega_deg=w / DEG, t_360=2 * math.pi / w)


def level_turn_bank(V, phi, g=G0):
    _pos("the speed and gravity", V, g)
    _need(isinstance(phi, (int, float)) and 1 * DEG <= phi <= 85 * DEG, "the bank angle must lie between 1 and 85 degrees")
    return _turn_out(V, 1.0 / math.cos(phi), g)


def _ref_turn_sim(V, phi, g):
    """Point-mass simulation of a banked turn: the lift vector is turned about the velocity by the bank angle (scipy
    Rotation), its magnitude set by brentq so the vertical acceleration vanishes, then the 3-D motion is integrated
    (DOP853) and the turn rate and radius are measured from the trajectory (heading change and chord)."""
    import numpy as np
    from scipy.integrate import solve_ivp
    from scipy.optimize import brentq
    from scipy.spatial.transform import Rotation
    lift_dir0 = Rotation.from_rotvec([phi, 0.0, 0.0]).apply([0.0, 0.0, 1.0])       # velocity along x, bank about x
    Lm = brentq(lambda L: L * lift_dir0[2] - g, 0.0, 100.0 * g, xtol=1e-300, rtol=8.9e-16)   # lift per unit mass

    def rhs(t, y):
        v = y[3:]
        vh = v / np.linalg.norm(v)
        up = np.array([0.0, 0.0, 1.0]) - vh[2] * vh
        up /= np.linalg.norm(up)
        side = np.cross(vh, up)
        lift = Lm * (math.cos(phi) * up - math.sin(phi) * side)
        return np.concatenate([v, lift - np.array([0.0, 0.0, g])])
    tau = 0.2 * V / g
    s = solve_ivp(rhs, (0.0, tau), [0.0, 0.0, 0.0, V, 0.0, 0.0], method="DOP853", rtol=1e-13, atol=1e-12 * V * tau)
    x, y, vx, vy = s.y[0, -1], s.y[1, -1], s.y[3, -1], s.y[4, -1]
    dpsi = abs(math.atan2(vy, vx))
    w = dpsi / tau
    R = math.hypot(x, y) / (2 * math.sin(dpsi / 2))
    return {"n": Lm / g, "R": R, "omega": w, "omega_deg": math.degrees(w), "t_360": 2 * math.pi / w}


_TURN_NAMES = {"v": "V", "speed": "V", "phi": "phi", "bank": "phi", "bank_angle": "phi", "n": "n", "load_factor": "n", "g": "g"}

_add("aero_level_turn_bank",
     name="Level coordinated turn at a bank angle", topic="aircraft performance / turning flight",
     purpose="The load factor, turn radius, turn rate and the time for a full circle of an aircraft in a level, coordinated turn at a true airspeed and bank angle.",
     inputs=[_phys("V", "speed", "true airspeed"), _phys("phi", "angle", "bank angle (deg)"), _phys("g", "acceleration", "gravity (default 9.80665 m/s^2)", required=False)],
     outputs=[_o("n", "number", "load factor L/W"), _o("phi_deg", "number", "bank angle in degrees"), _o("R", "length", "turn radius"),
              _o("omega", "angular_frequency", "turn rate"), _o("omega_deg", "number", "turn rate in degrees per second"), _o("t_360", "time", "time for a 360-degree turn")],
     primary="R", conditions=["1 deg <= phi <= 85 deg", "V > 0", "level, coordinated turn at constant speed"],
     method="n = 1/cos(phi); R = V^2/(g tan(phi)); omega = g tan(phi)/V", latex=r"n = \frac{1}{\cos\phi},\quad R = \frac{V^2}{g\tan\phi},\quad \omega = \frac{g\tan\phi}{V}",
     run=level_turn_bank,
     parse=lambda q: _pairs(q, r"level (?:coordinated )?turn at (?:a )?bank(?: angle)?|turn radius and rate at (?:a )?bank(?: angle)?|banked turn", _TURN_NAMES, phys={"V", "phi", "g"}),
     reference=lambda V, phi, g=G0: (lambda d: {**d, "phi_deg": math.degrees(phi)})(_ref_turn_sim(V, phi, g)),
     wrong=_safe(lambda V, phi, g=G0: (lambda w: _out(n=1 / math.cos(phi), phi_deg=phi / DEG, R=V / w, omega=w, omega_deg=w / DEG, t_360=2 * math.pi / w))(g * math.sin(phi) / V)),
     random_inputs=lambda rng: {"V": rng.uniform(20.0, 300.0), "phi": rng.uniform(5.0, 80.0) * DEG, **({"g": rng.uniform(3.0, 9.81)} if rng.random() < 0.2 else {})},
     templates=["Level turn at bank angle: V = 100 m/s, phi = 30 deg", "Banked turn: V = 250 kt, phi = 45 deg"],
     examples=[{"inputs": {"V": 100.0, "phi": math.pi / 4}, "outputs": {"n": 2 ** 0.5, "R": 1e4 / G0, "omega": G0 / 100.0}},
               {"inputs": {"V": 50.0, "phi": math.pi / 3}, "outputs": {"n": 2.0}}],
     questions=[{"q": "Level turn at bank angle: V = 100 m/s, phi = 45 deg", "outputs": {"R": 1e4 / G0}},
                {"q": "Level turn at bank angle: V = 100 m/s, phi = 89 deg", "outcome": "INVALID_INPUT"},
                {"q": "Level turn at bank angle: V = 100 m/s", "outcome": "NEEDS_INPUT"}])


def level_turn_load_factor(V, n, g=G0):
    _pos("the speed and gravity", V, g)
    _need(isinstance(n, (int, float)) and 1.0002 <= n <= 11.5, "the load factor must lie between 1.0002 and 11.5")
    return _turn_out(V, n, g)


def ref_level_turn_load_factor(V, n, g=G0):
    from scipy.optimize import brentq
    from scipy.spatial.transform import Rotation
    phi = brentq(lambda p: n * Rotation.from_rotvec([p, 0.0, 0.0]).apply([0.0, 0.0, 1.0])[2] - 1.0, 0.0, math.pi / 2, xtol=1e-300, rtol=8.9e-16)
    d = _ref_turn_sim(V, phi, g)
    return {"n": n, "phi_deg": math.degrees(phi), "R": d["R"], "omega": d["omega"], "omega_deg": d["omega_deg"], "t_360": d["t_360"]}


_add("aero_level_turn_load_factor",
     name="Level turn at a load factor", topic="aircraft performance / turning flight",
     purpose="The bank angle, turn radius, turn rate and time for a full circle of a level coordinated turn flown at a true airspeed and load factor (g).",
     inputs=[_phys("V", "speed", "true airspeed"), _numin("n", "load factor (g)"), _phys("g", "acceleration", "gravity (default 9.80665 m/s^2)", required=False)],
     outputs=[_o("n", "number", "load factor"), _o("phi_deg", "number", "bank angle in degrees"), _o("R", "length", "turn radius"),
              _o("omega", "angular_frequency", "turn rate"), _o("omega_deg", "number", "turn rate in degrees per second"), _o("t_360", "time", "time for a 360-degree turn")],
     primary="R", conditions=["1.0002 <= n <= 11.5", "V > 0", "level, coordinated turn at constant speed"],
     method="cos(phi) = 1/n; R = V^2/(g sqrt(n^2 - 1)); omega = g sqrt(n^2 - 1)/V", latex=r"R = \frac{V^2}{g\sqrt{n^2 - 1}},\quad \omega = \frac{g\sqrt{n^2-1}}{V}",
     run=level_turn_load_factor,
     parse=lambda q: _pairs(q, r"level turn at (?:a )?load factor|turn at (?:a )?load factor", _TURN_NAMES, phys={"V", "g"}),
     reference=ref_level_turn_load_factor,
     wrong=_safe(lambda V, n, g=G0: (lambda w: _out(n=n, phi_deg=math.degrees(math.acos(1 / n)), R=V / w, omega=w, omega_deg=w / DEG, t_360=2 * math.pi / w))(g * n / V)),
     random_inputs=lambda rng: {"V": rng.uniform(20.0, 300.0), "n": rng.uniform(1.05, 9.0)},
     templates=["Level turn at load factor: V = 120 m/s, n = 2", "Turn at a load factor: V = 400 kt, n = 6"],
     examples=[{"inputs": {"V": 100.0, "n": 2.0}, "outputs": {"phi_deg": 60.0, "R": 1e4 / (G0 * 3 ** 0.5)}}],
     questions=[{"q": "Level turn at load factor: V = 100 m/s, n = 2", "outputs": {"phi_deg": 60.0}},
                {"q": "Level turn at load factor: V = 100 m/s, n = 0.5", "outcome": "INVALID_INPUT"},
                {"q": "Level turn at load factor: V = 100 m/s", "outcome": "NEEDS_INPUT"}])


def time_to_climb(RC0, Hc, h1, h2):
    _pos("the sea-level rate of climb and the absolute ceiling", RC0, Hc)
    _need(isinstance(h1, (int, float)) and isinstance(h2, (int, float)) and 0 <= h1 < h2 < Hc, "the altitudes must satisfy 0 <= h1 < h2 < absolute ceiling")
    t = Hc / RC0 * math.log((Hc - h1) / (Hc - h2))
    return _out(t=t, t_min=t / 60.0)


_add("aero_time_to_climb",
     name="Time to climb with a linearly decreasing rate of climb", topic="aircraft performance / climb",
     purpose="The time to climb between two altitudes when the maximum rate of climb falls linearly from its sea-level value to zero at the absolute ceiling (the standard textbook model).",
     inputs=[_phys("RC0", "speed", "maximum rate of climb at sea level (m/s, ft/min)"), _phys("Hc", "length", "absolute ceiling"),
             _phys("h1", "length", "starting altitude"), _phys("h2", "length", "final altitude")],
     outputs=[_o("t", "time", "time to climb"), _o("t_min", "number", "time to climb in minutes")],
     primary="t", conditions=["0 <= h1 < h2 < Hc", "RC(h) = RC0 (1 - h/Hc) (stated linear model)"],
     method="t = integral of dh/RC = (Hc/RC0) ln((Hc - h1)/(Hc - h2))", latex=r"t = \int_{h_1}^{h_2}\frac{dh}{RC_0(1 - h/H_c)} = \frac{H_c}{RC_0}\ln\frac{H_c - h_1}{H_c - h_2}",
     run=time_to_climb,
     parse=lambda q: _pairs(q, r"time to climb(?: with a linearly decreasing rate of climb)?", {"rc0": "RC0", "roc": "RC0", "hc": "Hc", "ceiling": "Hc", "h1": "h1", "h2": "h2"},
                            phys={"RC0", "Hc", "h1", "h2"}),
     reference=lambda RC0, Hc, h1, h2: (lambda t: {"t": t, "t_min": t / 60.0})(__import__("scipy.integrate", fromlist=["quad"]).quad(lambda h: 1.0 / (RC0 * (1.0 - h / Hc)), h1, h2, epsabs=0, epsrel=1e-13)[0]),
     wrong=_safe(lambda RC0, Hc, h1, h2: (lambda t: _out(t=t, t_min=t / 60.0))((h2 - h1) / RC0)),
     random_inputs=lambda rng: (lambda Hc: (lambda h1: {"RC0": rng.uniform(2.0, 80.0), "Hc": Hc, "h1": h1, "h2": h1 + rng.uniform(0.05, 0.95) * (Hc - h1)})(rng.uniform(0.0, 0.5) * Hc))(rng.uniform(3000.0, 20000.0)),
     templates=["Time to climb: RC0 = 1500 ft/min, Hc = 18000 ft, h1 = 0 ft, h2 = 10000 ft", "Time to climb: RC0 = 20 m/s, Hc = 15 km, h1 = 1 km, h2 = 11 km"],
     examples=[{"inputs": {"RC0": 10.0, "Hc": 10000.0, "h1": 0.0, "h2": 10000.0 * (1 - 1 / math.e)}, "outputs": {"t": 1000.0}}],
     questions=[{"q": "Time to climb: RC0 = 10 m/s, Hc = 10 km, h1 = 0 m, h2 = " + repr(10000.0 * (1 - 1 / math.e)) + " m", "outputs": {"t": 1000.0}},
                {"q": "Time to climb: RC0 = 10 m/s, Hc = 10 km, h1 = 0 m, h2 = 12 km", "outcome": "INVALID_INPUT"},
                {"q": "Time to climb: RC0 = 10 m/s, Hc = 10 km, h1 = 0 m", "outcome": "NEEDS_INPUT"}])


def _rc_jet(W, S, rho, T, CD0, K):
    a = 0.5 * rho * S * CD0
    b = 2 * K * W * W / (rho * S)
    V = math.sqrt((T + math.sqrt(T * T + 12 * a * b)) / (6 * a))
    rc = V * (T - a * V * V - b / (V * V)) / W
    return V, rc


def max_rc_jet(W, S, h, T, CD0, AR, e):
    _ws_in(W, S)
    _cd0(CD0)
    K = _K(AR, e)
    _pos("the thrust", T)
    _need(T > 2 * W * math.sqrt(K * CD0), "the thrust is below the minimum drag: the aircraft cannot hold level flight")
    _need(T < 0.8 * W, "the thrust-to-weight ratio must be below 0.8 (small climb angle, lift = weight)")
    V, rc = _rc_jet(W, S, _rho_at(h), T, CD0, K)
    return _out(RC_max=rc, V=V, gamma_deg=math.degrees(math.asin(rc / V)))


def ref_max_rc_jet(W, S, h, T, CD0, AR, e):
    K = 1.0 / (math.pi * e * AR)
    rho = _ref_rho(h)
    rc = lambda V: V * (T - 0.5 * rho * V * V * S * CD0 - K * W * W / (0.5 * rho * V * V * S)) / W   # noqa: E731  excess power / W
    v0 = math.sqrt(2 * W / (rho * S))
    V = _cs_argmin(rc, 1e-2 * v0, 1e2 * v0)
    return {"RC_max": rc(V), "V": V, "gamma_deg": math.degrees(math.asin(rc(V) / V))}


def _rc_random(rng):
    d = _PERF_RANDOM(rng)
    K = 1.0 / (math.pi * d["e"] * d["AR"])
    dmin = 2 * d["W"] * math.sqrt(K * d["CD0"])
    return {**d, "T": dmin * rng.uniform(1.2, 6.0) if dmin * 6.0 < 0.8 * d["W"] else min(0.79 * d["W"], dmin * rng.uniform(1.2, 2.0))}


_add("aero_max_rate_of_climb_jet",
     name="Maximum rate of climb of a jet", topic="aircraft performance / climb",
     purpose="The maximum rate of climb of a jet aircraft (constant thrust) at an altitude, the speed at which it occurs and the climb angle there, for a parabolic drag polar.",
     inputs=[_phys("W", "force", "weight"), _phys("S", "area", "wing area"), _phys("h", "length", "altitude"), _phys("T", "force", "available thrust"),
             _numin("CD0", "zero-lift drag coefficient"), _numin("AR", "aspect ratio"), _numin("e", "Oswald efficiency factor")],
     outputs=[_o("RC_max", "speed", "maximum rate of climb"), _o("V", "speed", "speed for the maximum rate of climb"), _o("gamma_deg", "number", "climb angle at that speed, degrees")],
     primary="RC_max", conditions=["minimum drag < T < 0.8 W", "small climb angle (lift = weight)", "parabolic drag polar"],
     method="RC = V (T - D)/W with D = a V^2 + b/V^2, a = rho S CD0/2, b = 2 K W^2/(rho S); dRC/dV = 0 gives V^2 = (T + sqrt(T^2 + 12 a b))/(6a)",
     latex=r"V^2 = \frac{T + \sqrt{T^2 + 12ab}}{6a},\quad RC_{max} = \frac{V(T - aV^2 - b/V^2)}{W}",
     run=max_rc_jet,
     parse=lambda q: _pairs(q, r"maximum rate of climb of a jet|jet maximum rate of climb|max(?:imum)? rate of climb \(jet\)", _POLAR_NAMES, phys={"W", "S", "h", "T"}),
     reference=ref_max_rc_jet,
     wrong=_safe(lambda W, S, h, T, CD0, AR, e: (lambda K, rho: (lambda V: (lambda rc: _out(RC_max=rc, V=V, gamma_deg=math.degrees(math.asin(min(1.0, rc / V)))))(
         V * (T - 0.5 * rho * V * V * S * CD0 - K * W * W / (0.5 * rho * V * V * S)) / W))(math.sqrt(2 * W / (rho * S) * math.sqrt(K / CD0))))(_K(AR, e), _rho_at(h))),
     random_inputs=_rc_random,
     templates=["Maximum rate of climb of a jet: W = 100 kN, S = 30 m^2, h = 0 m, T = 30 kN, CD0 = 0.02, AR = 7, e = 0.8"],
     examples=[{"inputs": {"W": 1e5, "S": 30.0, "h": 0.0, "T": 3e4, "CD0": 0.02, "AR": 7.0, "e": 0.8},
                "outputs": {"RC_max": _rc_jet(1e5, 30.0, RHO_SL, 3e4, 0.02, 1 / (math.pi * 5.6))[1]}}],
     questions=[{"q": "Maximum rate of climb of a jet: W = 100 kN, S = 30 m^2, h = 0 m, T = 1 kN, CD0 = 0.02, AR = 7, e = 0.8", "outcome": "INVALID_INPUT"},
                {"q": "Maximum rate of climb of a jet: W = 100 kN, S = 30 m^2, h = 0 m, CD0 = 0.02, AR = 7, e = 0.8", "outcome": "NEEDS_INPUT"}])


def max_climb_angle_jet(W, S, h, T, CD0, AR, e):
    _ws_in(W, S)
    _cd0(CD0)
    K = _K(AR, e)
    _pos("the thrust", T)
    dmin = 2 * W * math.sqrt(K * CD0)
    _need(dmin < T < 0.8 * W, "the thrust must exceed the minimum drag and stay below 0.8 W")
    rho = _rho_at(h)
    V = math.sqrt(2 * W / (rho * S) * math.sqrt(K / CD0))
    s = (T - dmin) / W
    return _out(gamma_max_deg=math.degrees(math.asin(s)), V=V, RC=V * s)


def ref_max_climb_angle_jet(W, S, h, T, CD0, AR, e):
    K = 1.0 / (math.pi * e * AR)
    rho = _ref_rho(h)
    sg = lambda V: (T - 0.5 * rho * V * V * S * CD0 - K * W * W / (0.5 * rho * V * V * S)) / W   # noqa: E731  sin(gamma) = (T - D)/W
    v0 = math.sqrt(2 * W / (rho * S))
    V = _cs_argmin(sg, 1e-2 * v0, 1e2 * v0)
    return {"gamma_max_deg": math.degrees(math.asin(sg(V))), "V": V, "RC": V * sg(V)}


_add("aero_max_climb_angle_jet",
     name="Steepest climb angle of a jet", topic="aircraft performance / climb",
     purpose="The maximum climb angle of a jet aircraft (constant thrust, small-angle approximation lift = weight), the speed for it and the rate of climb at that speed.",
     inputs=[_phys("W", "force", "weight"), _phys("S", "area", "wing area"), _phys("h", "length", "altitude"), _phys("T", "force", "available thrust"),
             _numin("CD0", "zero-lift drag coefficient"), _numin("AR", "aspect ratio"), _numin("e", "Oswald efficiency factor")],
     outputs=[_o("gamma_max_deg", "number", "maximum climb angle in degrees"), _o("V", "speed", "speed for the steepest climb"), _o("RC", "speed", "rate of climb at that speed")],
     primary="gamma_max_deg", conditions=["minimum drag < T < 0.8 W", "lift = weight (small angle)", "parabolic drag polar"],
     method="sin(gamma_max) = T/W - 2 sqrt(K CD0) at the minimum-drag speed", latex=r"\sin\gamma_{max} = \frac{T}{W} - \frac{1}{(L/D)_{max}}",
     run=max_climb_angle_jet,
     parse=lambda q: _pairs(q, r"steepest climb angle of a jet|maximum climb angle(?: of a jet)?|max(?:imum)? climb angle \(jet\)", _POLAR_NAMES, phys={"W", "S", "h", "T"}),
     reference=ref_max_climb_angle_jet,
     wrong=_safe(lambda W, S, h, T, CD0, AR, e: (lambda o: {**o, "gamma_max_deg": _r(math.degrees(math.asin(T / W))), "RC": _r(o["V"] * T / W)})(max_climb_angle_jet(W, S, h, T, CD0, AR, e))),
     random_inputs=_rc_random,
     templates=["Maximum climb angle: W = 100 kN, S = 30 m^2, h = 0 m, T = 30 kN, CD0 = 0.02, AR = 7, e = 0.8"],
     examples=[{"inputs": {"W": 1e5, "S": 30.0, "h": 0.0, "T": 3e4, "CD0": 0.02, "AR": 7.0, "e": 0.8},
                "outputs": {"gamma_max_deg": math.degrees(math.asin(0.3 - 2 * math.sqrt(0.02 / (math.pi * 5.6))))}}],
     questions=[{"q": "Maximum climb angle: W = 100 kN, S = 30 m^2, h = 0 m, T = 30 kN, CD0 = 0.02, AR = 7, e = 0.8",
                 "outputs": {"gamma_max_deg": math.degrees(math.asin(0.3 - 2 * math.sqrt(0.02 / (math.pi * 5.6))))}},
                {"q": "Maximum climb angle: W = 100 kN, S = 30 m^2, h = 0 m, T = 1 kN, CD0 = 0.02, AR = 7, e = 0.8", "outcome": "INVALID_INPUT"},
                {"q": "Maximum climb angle: W = 100 kN, S = 30 m^2, h = 0 m, CD0 = 0.02, AR = 7, e = 0.8", "outcome": "NEEDS_INPUT"}])


def max_rc_prop(W, S, h, P, eta, CD0, AR, e):
    _ws_in(W, S)
    _cd0(CD0)
    K = _K(AR, e)
    _eta_in(eta)
    _pos("the shaft power", P)
    rho = _rho_at(h)
    V = math.sqrt(2 * W / (rho * S)) * (K / (3 * CD0)) ** 0.25
    preq = (0.5 * rho * V * V * S * CD0 + K * W * W / (0.5 * rho * V * V * S)) * V
    rc = (eta * P - preq) / W
    _need(rc > 0, "the available power is below the minimum power required")
    _need(rc / V < 0.5, "the climb would be steeper than 30 degrees (the small-angle model does not apply)")
    return _out(RC_max=rc, V=V, gamma_deg=math.degrees(math.asin(rc / V)))


def ref_max_rc_prop(W, S, h, P, eta, CD0, AR, e):
    K = 1.0 / (math.pi * e * AR)
    rho = _ref_rho(h)
    rc = lambda V: (eta * P - (0.5 * rho * V * V * S * CD0 + K * W * W / (0.5 * rho * V * V * S)) * V) / W   # noqa: E731
    v0 = math.sqrt(2 * W / (rho * S))
    V = _cs_argmin(rc, 1e-2 * v0, 1e2 * v0)
    return {"RC_max": rc(V), "V": V, "gamma_deg": math.degrees(math.asin(rc(V) / V))}


def _rcp_random(rng):
    d = _PERF_RANDOM(rng)
    d["h"] = rng.uniform(0.0, 6000.0)
    W, S = d["W"], d["S"]
    K = 1.0 / (math.pi * d["e"] * d["AR"])
    rho = _rho_at(d["h"])
    V = math.sqrt(2 * W / (rho * S)) * (K / (3 * d["CD0"])) ** 0.25
    preq = (0.5 * rho * V * V * S * d["CD0"] + K * W * W / (0.5 * rho * V * V * S)) * V
    eta = rng.uniform(0.6, 0.9)
    rc = V * rng.uniform(0.02, 0.3)
    return {**d, "eta": eta, "P": (preq + rc * W) / eta}


_add("aero_max_rate_of_climb_prop",
     name="Maximum rate of climb of a propeller aircraft", topic="aircraft performance / climb",
     purpose="The maximum rate of climb of a propeller aircraft (constant shaft power and propeller efficiency) at an altitude, the speed for it (minimum power required) and the climb angle there.",
     inputs=[_phys("W", "force", "weight"), _phys("S", "area", "wing area"), _phys("h", "length", "altitude"), _phys("P", "power", "shaft power (W, kW, hp)"),
             _numin("eta", "propeller efficiency"), _numin("CD0", "zero-lift drag coefficient"), _numin("AR", "aspect ratio"), _numin("e", "Oswald efficiency factor")],
     outputs=[_o("RC_max", "speed", "maximum rate of climb"), _o("V", "speed", "speed for the maximum rate of climb"), _o("gamma_deg", "number", "climb angle in degrees")],
     primary="RC_max", conditions=["available power above the minimum power required", "climb angle below 30 degrees (lift = weight)", "parabolic drag polar"],
     method="RC = (eta P - D V)/W is largest at the minimum-power speed V = sqrt(2W/(rho S)) (K/(3 CD0))^(1/4)",
     latex=r"RC_{max} = \frac{\eta_p P - P_{R,min}}{W},\quad V_{mp} = \sqrt{\frac{2W}{\rho S}}\left(\frac{K}{3C_{D0}}\right)^{1/4}",
     run=max_rc_prop,
     parse=lambda q: _pairs(q, r"maximum rate of climb of a propeller(?: aircraft)?|propeller maximum rate of climb|max(?:imum)? rate of climb \(prop(?:eller)?\)", _POLAR_NAMES, phys={"W", "S", "h", "P"}),
     reference=ref_max_rc_prop,
     wrong=_safe(lambda W, S, h, P, eta, CD0, AR, e: (lambda o: {**o, "RC_max": _r(o["RC_max"] + (1 - eta) * P / W)})(max_rc_prop(W, S, h, P, eta, CD0, AR, e))),
     random_inputs=_rcp_random,
     templates=["Maximum rate of climb of a propeller aircraft: W = 11 kN, S = 16 m^2, h = 0 m, P = 120 kW, eta = 0.8, CD0 = 0.03, AR = 7.5, e = 0.8"],
     examples=[{"inputs": {"W": 1.1e4, "S": 16.0, "h": 0.0, "P": 1.2e5, "eta": 0.8, "CD0": 0.03, "AR": 7.5, "e": 0.8},
                "outputs": {"V": math.sqrt(2 * 1.1e4 / (RHO_SL * 16.0)) * (1 / (math.pi * 6.0) / 0.09) ** 0.25}}],
     questions=[{"q": "Maximum rate of climb of a propeller aircraft: W = 11 kN, S = 16 m^2, h = 0 m, P = 1 kW, eta = 0.8, CD0 = 0.03, AR = 7.5, e = 0.8", "outcome": "INVALID_INPUT"},
                {"q": "Maximum rate of climb of a propeller aircraft: W = 11 kN, S = 16 m^2, h = 0 m, eta = 0.8, CD0 = 0.03, AR = 7.5, e = 0.8", "outcome": "NEEDS_INPUT"}])


def absolute_ceiling_jet(W, S, T0, CD0, AR, e):
    _ws_in(W, S)
    _cd0(CD0)
    K = _K(AR, e)
    _pos("the sea-level thrust", T0)
    sig = 2 * W * math.sqrt(K * CD0) / T0
    _need(sig < 1.0, "the sea-level thrust is below the minimum drag: the aircraft cannot fly level even at sea level")
    rho = sig * RHO_SL
    _need(rho >= RHO_MIN, "the ceiling lies above 47 km")
    h = _geom(_H_from_rho(rho))
    return _out(h_abs=h, h_abs_ft=h / FT, sigma=sig)


def ref_absolute_ceiling_jet(W, S, T0, CD0, AR, e):
    from scipy.optimize import brentq
    K = 1.0 / (math.pi * e * AR)
    rho0 = _ref_rho(0.0)

    def excess(h):                                    # thrust T0 rho/rho0 minus the minimum drag found by optimization at h
        rho = _ref_rho(h)
        drag = lambda V: 0.5 * rho * V * V * S * CD0 + K * W * W / (0.5 * rho * V * V * S)   # noqa: E731
        v0 = math.sqrt(2 * W / (rho * S))
        return T0 * rho / rho0 - drag(_cs_argmin(drag, 1e-3 * v0, 1e3 * v0)).real
    h = brentq(excess, 0.0, H_MAX, xtol=1e-9, rtol=8.9e-16, maxiter=300)
    return {"h_abs": h, "h_abs_ft": h / FT, "sigma": _ref_rho(h) / rho0}


def _ceil_random(rng):
    d = _POLAR_RANDOM(rng)
    W = _lu(rng, 5e3, 3e6)
    K = 1.0 / (math.pi * d["e"] * d["AR"])
    sig = rng.uniform(0.1, 0.9)
    return {**d, "W": W, "S": W / rng.uniform(1000.0, 7000.0), "T0": 2 * W * math.sqrt(K * d["CD0"]) / sig}


_add("aero_absolute_ceiling_jet",
     name="Absolute ceiling of a jet aircraft", topic="aircraft performance / ceilings",
     purpose="The absolute ceiling of a jet aircraft (where the available thrust just equals the minimum drag) when the thrust falls in proportion to the air density, T = T0 rho/rho0.",
     inputs=[_phys("W", "force", "weight"), _phys("S", "area", "wing area"), _phys("T0", "force", "sea-level thrust"),
             _numin("CD0", "zero-lift drag coefficient"), _numin("AR", "aspect ratio"), _numin("e", "Oswald efficiency factor")],
     outputs=[_o("h_abs", "length", "absolute ceiling"), _o("h_abs_ft", "number", "absolute ceiling in feet"), _o("sigma", "number", "density ratio at the ceiling")],
     primary="h_abs", conditions=["minimum drag below the sea-level thrust", "ceiling below 47 km", "thrust proportional to density (stated lapse)", "parabolic drag polar"],
     method="T0 sigma = D_min = 2W sqrt(K CD0): sigma = 2W sqrt(K CD0)/T0; the height of that density in the standard atmosphere",
     latex=r"\sigma_{abs} = \frac{2W\sqrt{K C_{D0}}}{T_0} = \frac{W}{T_0 (L/D)_{max}}",
     run=absolute_ceiling_jet,
     parse=lambda q: _pairs(q, r"absolute ceiling(?: of a jet(?: aircraft)?)?", {**_POLAR_NAMES, "t0": "T0", "t_sl": "T0"}, phys={"W", "S", "T0"}),
     reference=ref_absolute_ceiling_jet,
     wrong=_safe(lambda W, S, T0, CD0, AR, e: (lambda sig: (lambda h: _out(h_abs=h, h_abs_ft=h / FT, sigma=sig))(_geom(_H_from_p(sig * P_SL))))(2 * W * math.sqrt(_K(AR, e) * CD0) / T0)),
     random_inputs=_ceil_random,
     templates=["Absolute ceiling: W = 100 kN, S = 30 m^2, T0 = 30 kN, CD0 = 0.02, AR = 7, e = 0.8"],
     examples=[{"inputs": {"W": 1e5, "S": 30.0, "T0": 4e5 * math.sqrt(0.02 / (math.pi * 5.6)), "CD0": 0.02, "AR": 7.0, "e": 0.8}, "outputs": {"sigma": 0.5}}],
     questions=[{"q": "Absolute ceiling: W = 100 kN, S = 30 m^2, T0 = 1 kN, CD0 = 0.02, AR = 7, e = 0.8", "outcome": "INVALID_INPUT"},
                {"q": "Absolute ceiling: W = 100 kN, S = 30 m^2, CD0 = 0.02, AR = 7, e = 0.8", "outcome": "NEEDS_INPUT"}])


def service_ceiling_jet(W, S, T0, CD0, AR, e, RCs=0.508):
    _ws_in(W, S)
    _cd0(CD0)
    K = _K(AR, e)
    _pos("the sea-level thrust and the service-ceiling rate of climb", T0, RCs)
    _need(T0 < 0.8 * W, "the sea-level thrust-to-weight ratio must be below 0.8")

    def rc(h):
        rho = _rho_at(h)
        return _rc_jet(W, S, rho, T0 * rho / RHO_SL, CD0, K)[1]
    _need(rc(0.0) > RCs, "the aircraft cannot climb at the service-ceiling rate even at sea level")
    _need(rc(H_MAX) < RCs, "the service ceiling lies above 47 km")
    h = _bisect(lambda x: rc(x) - RCs, 0.0, H_MAX)
    return _out(h_s=h, h_s_ft=h / FT)


def ref_service_ceiling_jet(W, S, T0, CD0, AR, e, RCs=0.508):
    from scipy.optimize import brentq
    K = 1.0 / (math.pi * e * AR)
    rho0 = _ref_rho(0.0)

    def rcmax(h):
        rho = _ref_rho(h)
        T = T0 * rho / rho0
        rcf = lambda V: V * (T - 0.5 * rho * V * V * S * CD0 - K * W * W / (0.5 * rho * V * V * S)) / W   # noqa: E731
        v0 = math.sqrt(2 * W / (rho * S))
        return rcf(_cs_argmin(rcf, 1e-2 * v0, 1e2 * v0)).real
    h = brentq(lambda x: rcmax(x) - RCs, 0.0, H_MAX, xtol=1e-9, rtol=8.9e-16, maxiter=300)
    return {"h_s": h, "h_s_ft": h / FT}


def _sc_random(rng):
    for _ in range(100):
        d = _ceil_random(rng)
        d.pop("T0")
        K = 1.0 / (math.pi * d["e"] * d["AR"])
        T0 = d["W"] * rng.uniform(0.2, 0.6)
        if T0 > 2 * d["W"] * math.sqrt(K * d["CD0"]) * 1.3:
            return {**d, "T0": T0, **({"RCs": rng.uniform(0.3, 2.5)} if rng.random() < 0.4 else {})}
    return {**d, "T0": d["W"] * 0.6}


_add("aero_service_ceiling_jet",
     name="Service ceiling of a jet aircraft", topic="aircraft performance / ceilings",
     purpose="The service ceiling of a jet aircraft: the altitude where its maximum rate of climb has fallen to 100 ft/min (0.508 m/s, or a stated value), with thrust proportional to density.",
     inputs=[_phys("W", "force", "weight"), _phys("S", "area", "wing area"), _phys("T0", "force", "sea-level thrust"),
             _numin("CD0", "zero-lift drag coefficient"), _numin("AR", "aspect ratio"), _numin("e", "Oswald efficiency factor"),
             _phys("RCs", "speed", "rate of climb that defines the ceiling (default 100 ft/min = 0.508 m/s)", required=False)],
     outputs=[_o("h_s", "length", "service ceiling"), _o("h_s_ft", "number", "service ceiling in feet")],
     primary="h_s", conditions=["T0 < 0.8 W", "climb possible at sea level", "ceiling below 47 km", "thrust proportional to density", "small climb angle"],
     method="solve RC_max(h) = RCs by bisection, with RC_max from the optimum climb speed V^2 = (T + sqrt(T^2 + 12ab))/(6a) and T = T0 rho/rho0",
     latex=r"RC_{max}(h_s) = RC_s,\quad T(h) = T_0\frac{\rho(h)}{\rho_0}",
     run=service_ceiling_jet,
     parse=lambda q: _pairs(q, r"service ceiling(?: of a jet(?: aircraft)?)?", {**_POLAR_NAMES, "t0": "T0", "t_sl": "T0", "rcs": "RCs", "rc": "RCs"}, phys={"W", "S", "T0", "RCs"}),
     reference=ref_service_ceiling_jet,
     wrong=_safe(lambda W, S, T0, CD0, AR, e, RCs=0.508: service_ceiling_jet(W, S, T0, CD0, AR, e, 1e-6)),     # the absolute ceiling (RC = 0) given instead
     random_inputs=_sc_random,
     templates=["Service ceiling: W = 100 kN, S = 30 m^2, T0 = 30 kN, CD0 = 0.02, AR = 7, e = 0.8"],
     examples=[{"inputs": {"W": 1e5, "S": 30.0, "T0": 3e4, "CD0": 0.02, "AR": 7.0, "e": 0.8}, "outputs": {"h_s": service_ceiling_jet(1e5, 30.0, 3e4, 0.02, 7.0, 0.8)["h_s"]}}],
     questions=[{"q": "Service ceiling: W = 100 kN, S = 30 m^2, T0 = 1 kN, CD0 = 0.02, AR = 7, e = 0.8", "outcome": "INVALID_INPUT"},
                {"q": "Service ceiling: W = 100 kN, S = 30 m^2, CD0 = 0.02, AR = 7, e = 0.8", "outcome": "NEEDS_INPUT"}])


# ================================================================================================ static stability
def neutral_point(h_ac, a_w, a_t, St_S, lt_c, deps, h_cg, eta_t=1.0):
    for nm, v, lo, hi in (("the wing aerodynamic-centre position", h_ac, -1.0, 2.0), ("the wing lift slope", a_w, 0.5, 8.0),
                          ("the tail lift slope", a_t, 0.5, 8.0), ("the tail-to-wing area ratio", St_S, 0.01, 1.0),
                          ("the tail arm", lt_c, 0.5, 15.0), ("the downwash gradient", deps, 0.0, 0.9),
                          ("the centre-of-gravity position", h_cg, -1.0, 2.0), ("the tail efficiency", eta_t, 0.5, 1.2)):
        _need(isinstance(v, (int, float)) and lo <= v <= hi, f"{nm} must lie between {lo:g} and {hi:g}")
    b = eta_t * St_S * a_t * (1 - deps)
    a = a_w + b
    hn = h_ac + b * lt_c / a
    sm = hn - h_cg
    return _out(h_n=hn, SM=sm, CL_alpha=a, Cm_alpha=0.0 if abs(sm) < 1e-12 else -a * sm)


def ref_neutral_point(h_ac, a_w, a_t, St_S, lt_c, deps, h_cg, eta_t=1.0):
    """Moment build-up: wing and tail lift forces as vectors at their chordwise stations (numpy cross products), the
    pitching moment about a trial point at two angles of attack, and the neutral point as the point where its slope
    vanishes (brentq)."""
    import numpy as np
    from scipy.optimize import brentq
    q_S_c = 1.0                                         # unit dynamic pressure, wing area and mean chord

    def forces(alpha):
        Lw = np.array([0.0, 0.0, a_w * alpha * q_S_c])
        Lt = np.array([0.0, 0.0, eta_t * St_S * a_t * (alpha * (1 - deps) - 0.03) * q_S_c])    # tail incidence offset -0.03 rad
        return [(np.array([h_ac, 0.0, 0.0]), Lw), (np.array([h_ac + lt_c, 0.0, 0.0]), Lt)]

    def cm(alpha, x):                                   # nose-up moment about the point x (x aft, z up): the y component of r x F
        return sum(float(np.cross(r - np.array([x, 0.0, 0.0]), F)[1]) for r, F in forces(alpha))

    def slope(x):
        return (cm(0.08, x) - cm(0.02, x)) / 0.06
    hn = brentq(slope, -50.0, 50.0, xtol=1e-14, rtol=8.9e-16)
    cla = (sum(F[2] for _r, F in forces(0.08)) - sum(F[2] for _r, F in forces(0.02))) / 0.06
    return {"h_n": hn, "SM": hn - h_cg, "CL_alpha": cla, "Cm_alpha": slope(h_cg)}


def _np_random(rng):
    return {"h_ac": rng.uniform(0.2, 0.3), "a_w": rng.uniform(4.0, 6.0), "a_t": rng.uniform(3.0, 5.5), "St_S": rng.uniform(0.1, 0.35),
            "lt_c": rng.uniform(2.0, 6.0), "deps": rng.uniform(0.2, 0.5), "h_cg": rng.uniform(0.1, 0.5), **({"eta_t": rng.uniform(0.8, 1.0)} if rng.random() < 0.5 else {})}


_add("aero_neutral_point",
     name="Neutral point and static margin (wing and tail)", topic="stability / longitudinal static stability",
     purpose="The stick-fixed neutral point, the static margin, the airplane lift-curve slope and the pitching-moment slope Cm_alpha of a wing-tail aircraft (positions as fractions of the mean aerodynamic chord, aft of its leading edge).",
     inputs=[_numin("h_ac", "wing-body aerodynamic centre / mean chord"), _numin("a_w", "wing lift-curve slope (per rad)"),
             _numin("a_t", "tail lift-curve slope (per rad)"), _numin("St_S", "tail area / wing area"),
             _numin("lt_c", "distance from the wing aerodynamic centre to the tail aerodynamic centre / mean chord"),
             _numin("deps", "downwash gradient d(epsilon)/d(alpha)"), _numin("h_cg", "centre of gravity / mean chord"),
             _numin("eta_t", "tail efficiency (dynamic-pressure ratio, default 1)", required=False)],
     outputs=[_o("h_n", "number", "neutral point / mean chord"), _o("SM", "number", "static margin h_n - h_cg (positive = stable)"),
              _o("CL_alpha", "number", "airplane lift-curve slope (per rad)"), _o("Cm_alpha", "number", "pitching-moment slope about the cg (per rad)")],
     primary="h_n", conditions=["linear aerodynamics (small angles)", "thrust and fuselage moment effects neglected beyond h_ac", "stated ranges of each input"],
     method="a = a_w + eta (St/S) a_t (1 - deps); h_n = h_ac + eta (St/S)(lt/c) a_t (1 - deps)/a; SM = h_n - h_cg; Cm_alpha = -a SM",
     latex=r"h_n = h_{ac} + \eta\,\bar V_H\,\frac{a_t}{a}\left(1 - \frac{d\varepsilon}{d\alpha}\right),\quad \bar V_H = \frac{S_t l_t}{S\bar c},\quad C_{m\alpha} = -a\,(h_n - h_{cg})",
     run=neutral_point,
     parse=lambda q: _pairs(q, r"neutral point(?: and static margin)?(?: \(wing and tail\))?|static margin", {"h_ac": "h_ac", "a_w": "a_w", "a_t": "a_t", "st_s": "St_S",
                                                                                                       "st/s": "St_S", "lt_c": "lt_c", "lt/c": "lt_c", "deps": "deps",
                                                                                                       "de/da": "deps", "h_cg": "h_cg", "eta_t": "eta_t", "eta": "eta_t"}),
     reference=ref_neutral_point,
     wrong=_safe(lambda h_ac, a_w, a_t, St_S, lt_c, deps, h_cg, eta_t=1.0: (lambda b: (lambda a: _out(h_n=h_ac + b * lt_c / a_w, SM=h_ac + b * lt_c / a_w - h_cg,
                                                                                                          CL_alpha=a, Cm_alpha=-a * (h_ac + b * lt_c / a_w - h_cg)))(a_w + b))(eta_t * St_S * a_t * (1 - deps))),
     random_inputs=_np_random,
     templates=["Neutral point: h_ac = 0.25, a_w = 5, a_t = 4, St_S = 0.2, lt_c = 4, deps = 0.4, h_cg = 0.3",
                "Static margin: h_ac = 0.24, a_w = 4.8, a_t = 4.2, St/S = 0.18, lt/c = 3.5, de/da = 0.35, h_cg = 0.28, eta_t = 0.9"],
     examples=[{"inputs": {"h_ac": 0.25, "a_w": 5.0, "a_t": 5.0, "St_S": 0.2, "lt_c": 4.0, "deps": 0.5, "h_cg": 0.3}, "outputs": {"h_n": 0.25 + 0.5 * 4 / 5.5, "CL_alpha": 5.5}}],
     questions=[{"q": "Neutral point: h_ac = 0.25, a_w = 5, a_t = 5, St_S = 0.2, lt_c = 4, deps = 0.5, h_cg = 0.3", "outputs": {"h_n": 0.25 + 0.5 * 4 / 5.5}},
                {"q": "Neutral point: h_ac = 0.25, a_w = 5, a_t = 5, St_S = 0.2, lt_c = 4, deps = 1.5, h_cg = 0.3", "outcome": "INVALID_INPUT"},
                {"q": "Neutral point: h_ac = 0.25, a_w = 5, a_t = 5, St_S = 0.2, lt_c = 4, deps = 0.5", "outcome": "NEEDS_INPUT"}])


# ================================================================================================ propellers and rotors (momentum theory)
def _disk_in(T, D, rho):
    _pos("the thrust, the disk diameter and the air density", T, D, rho)
    return math.pi * D * D / 4


def actuator_disk_hover(T, D, rho):
    A = _disk_in(T, D, rho)
    v = math.sqrt(T / (2 * rho * A))
    return _out(v_i=v, P_ideal=T * v, w=2 * v)


def ref_actuator_disk_hover(T, D, rho):
    """Stream-tube balances solved together (scipy fsolve): mass rho A v, momentum T = mdot w, energy P = mdot w^2/2 = T v."""
    A = math.pi * D * D / 4
    s = math.sqrt(T / (rho * A))                              # velocity scale (no solution assumed)
    k = rho * A * s * s / T                                    # = 1 in these units
    v, w = _fsolve_scaled(lambda x: [k * x[0] * x[1] - 1.0, x[0] - 0.5 * k * x[0] * x[1] ** 2], [0.3, 2.0], s)
    return {"v_i": v, "P_ideal": T * v, "w": w}


def _fsolve_scaled(f, x0, s):
    from scipy.optimize import fsolve
    x = fsolve(f, x0, xtol=1e-15)
    x = fsolve(f, x, xtol=1e-15)
    return [float(x[0]) * s, float(x[1]) * s]


_add("aero_actuator_disk_hover",
     name="Ideal hover power of a rotor (momentum theory)", topic="propellers and rotors / momentum theory",
     purpose="The induced velocity at the disk, the ideal (induced) power and the far-wake velocity of a hovering rotor or static propeller from its thrust, diameter and the air density (actuator-disk momentum theory).",
     inputs=[_phys("T", "force", "thrust"), _phys("D", "length", "rotor (disk) diameter"), _phys("rho", "density", "air density")],
     outputs=[_o("v_i", "speed", "induced velocity at the disk"), _o("P_ideal", "power", "ideal induced power"), _o("w", "speed", "far-wake velocity")],
     primary="P_ideal", conditions=["T, D, rho > 0", "ideal actuator disk: uniform, inviscid, no swirl"],
     method="v_i = sqrt(T/(2 rho A)), A = pi D^2/4; P = T v_i; w = 2 v_i", latex=r"v_i = \sqrt{\frac{T}{2\rho A}},\quad P_i = T v_i = \frac{T^{3/2}}{\sqrt{2\rho A}}",
     run=actuator_disk_hover,
     parse=lambda q: _pairs(q, r"ideal hover power(?: of a rotor)?(?: \(momentum theory\))?|hover (?:induced )?power|actuator disk hover", {"t": "T", "thrust": "T", "d": "D",
                                                                                                                                 "diameter": "D", "rho": "rho", "density": "rho"}, phys={"T", "D", "rho"}),
     reference=ref_actuator_disk_hover,
     wrong=_safe(lambda T, D, rho: (lambda v: _out(v_i=v, P_ideal=T * v, w=2 * v))(math.sqrt(T / (rho * math.pi * D * D / 4)))),
     random_inputs=lambda rng: {"T": _lu(rng, 10.0, 5e5), "D": _lu(rng, 0.2, 25.0), "rho": rng.uniform(0.4, 1.3)},
     templates=["Hover power: T = 20 kN, D = 10 m, rho = 1.225 kg/m^3", "Ideal hover power of a rotor: T = 50 N, D = 0.4 m, rho = 1.2 kg/m^3"],
     examples=[{"inputs": {"T": 2 * math.pi, "D": 2.0, "rho": 1.0}, "outputs": {"v_i": 1.0, "P_ideal": 2 * math.pi, "w": 2.0}}],
     questions=[{"q": "Hover power: T = " + repr(2 * math.pi) + " N, D = 2 m, rho = 1 kg/m^3", "outputs": {"v_i": 1.0}},
                {"q": "Hover power: T = 20 kN, D = 0 m, rho = 1.225 kg/m^3", "outcome": "INVALID_INPUT"},
                {"q": "Hover power: T = 20 kN, D = 10 m", "outcome": "NEEDS_INPUT"}])


def actuator_disk_forward(T, V, D, rho):
    A = _disk_in(T, D, rho)
    _pos("the flight speed", V)
    k = T / (2 * rho * A)
    v = k / (V / 2 + math.sqrt(V * V / 4 + k))                # = -V/2 + sqrt(V^2/4 + k), written without cancellation
    P = T * (V + v)
    return _out(v_i=v, P_ideal=P, eta_ideal=T * V / P, w=2 * v)


def ref_actuator_disk_forward(T, V, D, rho):
    """Stream-tube balances solved numerically (brentq): mdot = rho A (V + v); T = mdot w; P = T (V + v) = mdot ((V + w)^2 - V^2)/2."""
    from scipy.optimize import brentq
    A = math.pi * D * D / 4
    wake = lambda v: T / (rho * A * (V + v))                                  # noqa: E731  momentum: T = mdot w
    # energy: the power T (V + v) delivered at the disk equals the kinetic-energy flux mdot ((V + w)^2 - V^2)/2
    # ((V + w)^2 - V^2 written as w (2V + w) to avoid cancellation when w << V)
    v = brentq(lambda v: T * (V + v) - 0.5 * rho * A * (V + v) * wake(v) * (2 * V + wake(v)), 1e-12 * (V + 1.0), 10.0 * (V + math.sqrt(T / (rho * A))),
               xtol=1e-300, rtol=8.9e-16, maxiter=500)
    w = wake(v)
    P = 0.5 * rho * A * (V + v) * w * (2 * V + w)
    return {"v_i": v, "P_ideal": P, "eta_ideal": T * V / P, "w": w}


_add("aero_actuator_disk_forward",
     name="Ideal propeller in axial flight (momentum theory)", topic="propellers and rotors / momentum theory",
     purpose="The induced velocity, ideal power, ideal (Froude) propulsive efficiency and far-wake velocity increment of a propeller producing a thrust at a flight speed (actuator-disk momentum theory).",
     inputs=[_phys("T", "force", "thrust"), _phys("V", "speed", "flight (axial) speed"), _phys("D", "length", "propeller diameter"), _phys("rho", "density", "air density")],
     outputs=[_o("v_i", "speed", "induced velocity at the disk"), _o("P_ideal", "power", "ideal power absorbed"), _o("eta_ideal", "number", "ideal propulsive efficiency T V/P"),
              _o("w", "speed", "far-wake velocity increment")],
     primary="eta_ideal", conditions=["T, V, D, rho > 0", "ideal actuator disk"],
     method="v = -V/2 + sqrt(V^2/4 + T/(2 rho A)); P = T (V + v); eta = V/(V + v); w = 2v",
     latex=r"v_i = -\frac{V}{2} + \sqrt{\frac{V^2}{4} + \frac{T}{2\rho A}},\quad \eta_i = \frac{V}{V + v_i}",
     run=actuator_disk_forward,
     parse=lambda q: _pairs(q, r"ideal propeller(?: in axial flight)?(?: \(momentum theory\))?|actuator disk(?: in)? forward flight|propeller momentum theory",
                            {"t": "T", "thrust": "T", "v": "V", "speed": "V", "d": "D", "diameter": "D", "rho": "rho", "density": "rho"}, phys={"T", "V", "D", "rho"}),
     reference=ref_actuator_disk_forward,
     wrong=_safe(lambda T, V, D, rho: (lambda v: _out(v_i=v, P_ideal=T * (V + v), eta_ideal=V / (V + v), w=2 * v))(math.sqrt(T / (2 * rho * math.pi * D * D / 4)))),
     random_inputs=lambda rng: {"T": _lu(rng, 50.0, 1e5), "V": rng.uniform(5.0, 150.0), "D": _lu(rng, 0.3, 6.0), "rho": rng.uniform(0.4, 1.3)},
     templates=["Ideal propeller: T = 3 kN, V = 60 m/s, D = 2 m, rho = 1.225 kg/m^3", "Propeller momentum theory: T = 600 lbf, V = 120 kt, D = 6 ft, rho = 1.1 kg/m^3"],
     examples=[{"inputs": {"T": 8 * math.pi, "V": 3.0, "D": 2.0, "rho": 1.0}, "outputs": {"v_i": 1.0, "eta_ideal": 0.75, "P_ideal": 32 * math.pi}}],
     questions=[{"q": "Ideal propeller: T = " + repr(8 * math.pi) + " N, V = 3 m/s, D = 2 m, rho = 1 kg/m^3", "outputs": {"eta_ideal": 0.75}},
                {"q": "Ideal propeller: T = 3 kN, V = -60 m/s, D = 2 m, rho = 1.225 kg/m^3", "outcome": "INVALID_INPUT"},
                {"q": "Ideal propeller: T = 3 kN, V = 60 m/s, D = 2 m", "outcome": "NEEDS_INPUT"}])


# ================================================================================================ wings and airfoils
def _J(n, a, b):
    if n == 0:
        return b - a
    return (math.sin(n * b) - math.sin(n * a)) / n


def _camber_int(n, m, p):
    """Integral over theta in (0, pi) of dz/dx cos(n theta) for the NACA 4-digit mean line, dz/dx = k (p - x),
    x = (1 - cos theta)/2, k = 2m/p^2 ahead of the maximum camber and 2m/(1-p)^2 behind it (closed form)."""
    if m == 0:
        return 0.0
    tp = math.acos(1 - 2 * p)
    total = 0.0
    for k, a, b in ((2 * m / p ** 2, 0.0, tp), (2 * m / (1 - p) ** 2, tp, math.pi)):
        c0 = p - 0.5
        total += k * (c0 * _J(n, a, b) + 0.25 * (_J(abs(n - 1), a, b) + _J(n + 1, a, b)))
    return total


def naca4_thin_airfoil(m, p, alpha):
    _need(isinstance(m, (int, float)) and 0 <= m <= 0.095, "the maximum camber must lie between 0 and 0.095 (0 to 9.5% chord)")
    _need(m == 0 or (isinstance(p, (int, float)) and 0.05 <= p <= 0.95), "the position of maximum camber must lie between 0.05 and 0.95 chord")
    _need(isinstance(alpha, (int, float)) and abs(alpha) <= 15 * DEG, "the angle of attack must lie within +-15 degrees (thin-airfoil theory, unstalled)")
    I0, I1, I2 = _camber_int(0, m, p), _camber_int(1, m, p), _camber_int(2, m, p)
    aL0 = -(I1 - I0) / math.pi
    A1, A2 = 2 * I1 / math.pi, 2 * I2 / math.pi
    CL = 2 * math.pi * (alpha - aL0)
    cm4 = math.pi / 4 * (A2 - A1)
    out = _out(alpha_L0_deg=aL0 / DEG, CL=CL, Cm_c4=cm4, Cm_le=-(CL / 4 + math.pi / 4 * (A1 - A2)))
    if m == 0:
        out["alpha_L0_deg"], out["Cm_c4"] = 0.0, 0.0
    if abs(out["CL"]) < 1e-12:
        out["CL"] = 0.0
    return out


def ref_naca4_thin_airfoil(m, p, alpha):
    from scipy.integrate import quad
    if m == 0:
        CL = 2 * math.pi * alpha
        return {"alpha_L0_deg": 0.0, "CL": CL, "Cm_c4": 0.0, "Cm_le": -CL / 4}

    def dzdx(th):                                        # NACA 4-digit mean line slope at x = (1 - cos theta)/2
        x = 0.5 * (1 - math.cos(th))
        return 2 * m / p ** 2 * (p - x) if x < p else 2 * m / (1 - p) ** 2 * (p - x)
    tp = math.acos(1 - 2 * p)

    def integ(f):
        a, _ = quad(f, 0.0, tp, epsabs=0, epsrel=2e-14, limit=200)
        b, _ = quad(f, tp, math.pi, epsabs=0, epsrel=2e-14, limit=200)
        return a + b
    aL0 = -integ(lambda t: dzdx(t) * (math.cos(t) - 1)) / math.pi
    A0 = alpha - integ(dzdx) / math.pi
    A1 = 2 / math.pi * integ(lambda t: dzdx(t) * math.cos(t))
    A2 = 2 / math.pi * integ(lambda t: dzdx(t) * math.cos(2 * t))
    CL = math.pi * (2 * A0 + A1)
    return {"alpha_L0_deg": math.degrees(aL0), "CL": CL, "Cm_c4": math.pi / 4 * (A2 - A1), "Cm_le": -(CL / 4 + math.pi / 4 * (A1 - A2))}


def _naca_parse(q):
    """'NACA 2412 thin airfoil: alpha = 4 deg' (the 4-digit code gives m and p) or 'Thin airfoil NACA 4-digit: m = 0.02, p = 0.4, alpha = 4 deg'."""
    if not isinstance(q, str):
        return None
    mm = re.match(r"\s*naca\s*(\d)(\d)(\d\d)\s+thin[- ]airfoil(?: theory)?\s*:\s*(.*)$", q, re.I)
    if mm:
        rest = _pairs("x: " + mm.group(4), "x", {"alpha": "alpha", "aoa": "alpha"}, phys={"alpha"})
        if rest is None:
            return None
        return {"m": int(mm.group(1)) / 100.0, "p": int(mm.group(2)) / 10.0, **rest}
    return _pairs(q, r"thin[- ]airfoil naca 4-digit|naca 4-digit thin[- ]airfoil", {"m": "m", "p": "p", "alpha": "alpha", "aoa": "alpha"}, phys={"alpha"})


_add("aero_naca4_thin_airfoil",
     name="Thin-airfoil theory for a NACA 4-digit airfoil", topic="aerodynamics / thin-airfoil theory",
     purpose="The zero-lift angle, lift coefficient and the pitching-moment coefficients about the quarter chord and the leading edge of a cambered NACA 4-digit airfoil at an angle of attack, from thin-airfoil theory.",
     inputs=[_numin("m", "maximum camber as a fraction of the chord (first digit / 100)"), _numin("p", "chordwise position of maximum camber (second digit / 10; 0 for a symmetric section)"),
             _phys("alpha", "angle", "angle of attack (deg)")],
     outputs=[_o("alpha_L0_deg", "number", "zero-lift angle of attack in degrees"), _o("CL", "number", "section lift coefficient"),
              _o("Cm_c4", "number", "moment coefficient about the quarter chord"), _o("Cm_le", "number", "moment coefficient about the leading edge")],
     primary="CL", conditions=["0 <= m <= 0.095", "0.05 <= p <= 0.95 when m > 0", "|alpha| <= 15 deg", "thin airfoil, inviscid, incompressible, unstalled"],
     method="Glauert: A0 = alpha - (1/pi) int dz/dx dtheta, An = (2/pi) int dz/dx cos(n theta) dtheta; CL = pi(2A0 + A1); Cm_c/4 = pi/4 (A2 - A1); integrals of the NACA mean line in closed form",
     latex=r"C_L = 2\pi(\alpha - \alpha_{L0}),\quad \alpha_{L0} = -\frac{1}{\pi}\int_0^\pi \frac{dz}{dx}(\cos\theta - 1)\,d\theta,\quad C_{m,c/4} = \frac{\pi}{4}(A_2 - A_1)",
     run=naca4_thin_airfoil,
     parse=_naca_parse,
     reference=ref_naca4_thin_airfoil,
     wrong=_safe(lambda m, p, alpha: (lambda o: {**o, "CL": _r(o["CL"] / 2), "Cm_le": _r(o["Cm_le"] + o["CL"] / 8)})(naca4_thin_airfoil(m, p, alpha))),
     random_inputs=lambda rng: {"m": rng.uniform(0.01, 0.07), "p": rng.uniform(0.2, 0.6), "alpha": rng.uniform(-6.0, 12.0) * DEG},
     templates=["NACA 2412 thin airfoil: alpha = 4 deg", "Thin airfoil NACA 4-digit: m = 0.04, p = 0.4, alpha = 6 deg"],
     examples=[{"inputs": {"m": 0.0, "p": 0.4, "alpha": 0.1}, "outputs": {"CL": 0.2 * math.pi, "Cm_c4": 0.0, "Cm_le": -0.05 * math.pi}}],
     questions=[{"q": "NACA 0012 thin airfoil: alpha = 0.1 rad", "outputs": {"CL": 0.2 * math.pi}},
                {"q": "NACA 2412 thin airfoil: alpha = 30 deg", "outcome": "INVALID_INPUT"},
                {"q": "NACA 2412 thin airfoil:", "outcome": "NEEDS_INPUT"}])


def elliptic_wing(AR, alpha, a0=2 * math.pi):
    _need(isinstance(AR, (int, float)) and 2.0 <= AR <= 40.0, "the aspect ratio must lie between 2 and 40 (lifting-line theory)")
    _need(isinstance(a0, (int, float)) and 3.0 <= a0 <= 7.5, "the section lift slope must lie between 3 and 7.5 per radian")
    _need(isinstance(alpha, (int, float)) and abs(alpha) <= 15 * DEG, "the angle of attack (from zero lift) must lie within +-15 degrees")
    a = a0 / (1 + a0 / (math.pi * AR))
    CL = a * alpha
    out = _out(CL_alpha=a, CL=CL, CDi=CL * CL / (math.pi * AR), alpha_i_deg=CL / (math.pi * AR) / DEG)
    if alpha == 0:
        out.update(CL=0.0, CDi=0.0, alpha_i_deg=0.0)
    return out


def ref_elliptic_wing(AR, alpha, a0=2 * math.pi):
    """Prandtl's lifting-line (monoplane) equation solved by collocation for an elliptic planform (numpy linear solve):
    alpha = sum A_n sin(n theta) (mu + n/sin theta), mu = pi AR/(a0 sin theta) ... for c = c0 sin theta."""
    import numpy as np
    N = 7
    ns = np.arange(1, 2 * N, 2)                           # symmetric loading: odd terms
    th = np.linspace(math.pi / (4 * N), math.pi / 2, N)
    c_over = np.sin(th)                                   # chord / root chord of the ellipse
    mu = a0 * (4.0 / (math.pi * AR)) * c_over / 4.0      # a0 c / (4 b) with c0/b = 4/(pi AR)
    M = np.array([[np.sin(n * t) * (1.0 + n * m_ / np.sin(t)) for n in ns] for t, m_ in zip(th, mu)])
    rhs = mu * alpha
    An = np.linalg.solve(M, rhs)
    CL = math.pi * AR * An[0]
    CDi = math.pi * AR * float(np.sum(ns * An ** 2))
    return {"CL_alpha": CL / alpha if alpha != 0 else a0 / (1 + a0 / (math.pi * AR)), "CL": CL, "CDi": CDi, "alpha_i_deg": math.degrees(An[0])}


_add("aero_elliptic_wing_lifting_line",
     name="Elliptic wing by lifting-line theory", topic="aerodynamics / finite wings",
     purpose="The lift-curve slope, lift coefficient, induced drag coefficient and induced angle of attack of an untwisted elliptic wing of a given aspect ratio at an angle of attack measured from zero lift (Prandtl lifting-line theory, span efficiency 1).",
     inputs=[_numin("AR", "aspect ratio"), _phys("alpha", "angle", "angle of attack from zero lift (deg)"),
             _numin("a0", "section lift-curve slope per radian (default 2 pi)", required=False)],
     outputs=[_o("CL_alpha", "number", "wing lift-curve slope per radian"), _o("CL", "number", "wing lift coefficient"),
              _o("CDi", "number", "induced drag coefficient"), _o("alpha_i_deg", "number", "induced angle of attack in degrees")],
     primary="CL", conditions=["2 <= AR <= 40", "3 <= a0 <= 7.5 per rad", "|alpha| <= 15 deg", "untwisted elliptic planform, attached flow"],
     method="a = a0/(1 + a0/(pi AR)); CL = a alpha; CDi = CL^2/(pi AR); alpha_i = CL/(pi AR)",
     latex=r"a = \frac{a_0}{1 + \frac{a_0}{\pi A\!R}},\quad C_{D_i} = \frac{C_L^2}{\pi A\!R},\quad \alpha_i = \frac{C_L}{\pi A\!R}",
     run=elliptic_wing,
     parse=lambda q: _pairs(q, r"elliptic wing(?: by lifting-line theory)?|lifting[- ]line elliptic wing", {"ar": "AR", "aspect_ratio": "AR", "alpha": "alpha", "aoa": "alpha", "a0": "a0"}, phys={"alpha"}),
     reference=ref_elliptic_wing,
     wrong=_safe(lambda AR, alpha, a0=2 * math.pi: (lambda a: _out(CL_alpha=a, CL=a * alpha, CDi=(a * alpha) ** 2 / AR, alpha_i_deg=a * alpha / AR / DEG))(a0 / (1 + a0 / AR))),
     random_inputs=lambda rng: {"AR": rng.uniform(3.0, 25.0), "alpha": rng.choice([-1, 1]) * rng.uniform(0.5, 12.0) * DEG, **({"a0": rng.uniform(5.0, 7.0)} if rng.random() < 0.5 else {})},
     templates=["Elliptic wing: AR = 8, alpha = 5 deg", "Elliptic wing by lifting-line theory: AR = 6, alpha = 4 deg, a0 = 6.1"],
     examples=[{"inputs": {"AR": 2.0, "alpha": 0.1}, "outputs": {"CL_alpha": 2 * math.pi / 2.0, "CL": 0.1 * math.pi}}],
     questions=[{"q": "Elliptic wing: AR = 2, alpha = 0.1 rad", "outputs": {"CL": 0.1 * math.pi}},
                {"q": "Elliptic wing: AR = 1, alpha = 5 deg", "outcome": "INVALID_INPUT"},
                {"q": "Elliptic wing: AR = 8", "outcome": "NEEDS_INPUT"}])


# ================================================================================================ additions: navigation, ram temperature, staging, coverage
def wind_triangle(TAS, course, wind_from, wind_speed):
    _pos("the true airspeed", TAS)
    _need(isinstance(wind_speed, (int, float)) and wind_speed >= 0, "the wind speed must not be negative")
    for nm, v in (("the course", course), ("the wind direction", wind_from)):
        _need(isinstance(v, (int, float)) and 0 <= v <= 2 * math.pi, f"{nm} must lie between 0 and 360 degrees")
    rel = wind_from - course
    x = wind_speed * math.sin(rel) / TAS
    _need(abs(x) < 0.99, "the crosswind is too strong for this airspeed (no heading holds the course)")
    wca = math.asin(x)
    gs = TAS * math.cos(wca) - wind_speed * math.cos(rel)
    _need(gs > 0.01 * TAS, "the headwind would stop the aircraft over the ground")
    hdg = math.degrees(course + wca) % 360.0
    return _out(heading_deg=hdg, wca_deg=math.degrees(wca), GS=gs, GS_kt=gs / KT)


def ref_wind_triangle(TAS, course, wind_from, wind_speed):
    """Vector solution: the heading (brentq) at which air velocity plus wind velocity points along the course (numpy)."""
    import numpy as np
    from scipy.optimize import brentq
    cvec = np.array([math.sin(course), math.cos(course)])                     # east, north
    wind = -wind_speed * np.array([math.sin(wind_from), math.cos(wind_from)])   # blowing from wind_from
    ground = lambda H: TAS * np.array([math.sin(H), math.cos(H)]) + wind      # noqa: E731
    cross = lambda H: float(cvec[0] * ground(H)[1] - cvec[1] * ground(H)[0])  # noqa: E731  off-course component
    H = brentq(cross, course - math.pi / 2, course + math.pi / 2, xtol=1e-15, rtol=8.9e-16, maxiter=300)
    g = ground(H)
    gs = float(np.dot(g, cvec))
    return {"heading_deg": math.degrees(H) % 360.0, "wca_deg": math.degrees(H - course), "GS": gs, "GS_kt": gs / KT}


def _wt_random(rng):
    for _ in range(100):
        d = {"TAS": rng.uniform(30.0, 260.0), "course": rng.uniform(5.0, 355.0) * DEG, "wind_from": rng.uniform(0.0, 359.0) * DEG}
        d["wind_speed"] = d["TAS"] * rng.uniform(0.02, 0.6)
        try:
            h = wind_triangle(**d)["heading_deg"]
        except ValueError:
            continue
        if 0.5 < h < 359.5:
            return d
    return d


_add("aero_wind_triangle",
     name="Wind triangle: heading and ground speed", topic="aircraft performance / navigation",
     purpose="The heading to fly, the wind correction angle and the ground speed for an aircraft of a given true airspeed to make good a true course in a wind (direction the wind blows from, and its speed).",
     inputs=[_phys("TAS", "speed", "true airspeed (kt, km/h, m/s)"), _phys("course", "angle", "true course to make good (deg)"),
             _phys("wind_from", "angle", "direction the wind blows from (deg true)"), _phys("wind_speed", "speed", "wind speed")],
     outputs=[_o("heading_deg", "number", "true heading to fly, degrees"), _o("wca_deg", "number", "wind correction angle, degrees (positive = right of course)"),
              _o("GS", "speed", "ground speed"), _o("GS_kt", "number", "ground speed in knots")],
     primary="GS", conditions=["crosswind component below 0.99 TAS", "positive ground speed", "angles in [0, 360] degrees"],
     method="WCA = asin(Vw sin(W - C)/TAS); heading = C + WCA; GS = TAS cos(WCA) - Vw cos(W - C)",
     latex=r"\sin\delta = \frac{V_w\sin(W - C)}{V},\quad V_g = V\cos\delta - V_w\cos(W - C)",
     run=wind_triangle,
     parse=lambda q: _pairs(q, r"wind triangle(?:: heading and ground speed)?|heading and ground speed", {"tas": "TAS", "v": "TAS", "course": "course", "tc": "course",
                                                                                                         "wind_from": "wind_from", "wind_dir": "wind_from", "wd": "wind_from",
                                                                                                         "wind_speed": "wind_speed", "ws": "wind_speed"},
                            phys={"TAS", "course", "wind_from", "wind_speed"}),
     reference=ref_wind_triangle,
     wrong=_safe(lambda TAS, course, wind_from, wind_speed: (lambda o: {**o, "heading_deg": _r((2 * math.degrees(course) - o["heading_deg"]) % 360.0), "wca_deg": _r(-o["wca_deg"])})(
         wind_triangle(TAS, course, wind_from, wind_speed))),
     random_inputs=_wt_random,
     templates=["Wind triangle: TAS = 120 kt, course = 090 deg, wind_from = 030 deg, wind_speed = 25 kt",
                "Heading and ground speed: TAS = 450 km/h, course = 250 deg, wind_from = 300 deg, wind_speed = 60 km/h"],
     examples=[{"inputs": {"TAS": 100.0, "course": math.pi / 2, "wind_from": math.pi / 2, "wind_speed": 20.0}, "outputs": {"heading_deg": 90.0, "GS": 80.0, "wca_deg": 0.0}},
               {"inputs": {"TAS": 100.0, "course": 0.5, "wind_from": 0.5 + math.pi / 2, "wind_speed": 50.0}, "outputs": {"wca_deg": 30.0, "GS": 100 * math.cos(math.pi / 6)}}],
     questions=[{"q": "Wind triangle: TAS = 100 kt, course = 90 deg, wind_from = 90 deg, wind_speed = 20 kt", "outputs": {"GS_kt": 80.0}},
                {"q": "Wind triangle: TAS = 100 kt, course = 90 deg, wind_from = 0 deg, wind_speed = 120 kt", "outcome": "INVALID_INPUT"},
                {"q": "Wind triangle: TAS = 100 kt, course = 90 deg, wind_from = 0 deg", "outcome": "NEEDS_INPUT"}])


def total_temperature(V, T):
    _pos("the true airspeed and the static temperature", V, T)
    _need(150.0 <= T <= 350.0, "the static air temperature must lie between 150 K and 350 K")
    cp = GAMMA_AIR * R_AIR / (GAMMA_AIR - 1)
    M = V / math.sqrt(GAMMA_AIR * R_AIR * T)
    _need(M < 5.0, "the flight Mach number must be below 5 (calorically perfect air)")
    dT = V * V / (2 * cp)
    return _out(T0=T + dT, dT=dT, M=M)


def ref_total_temperature(V, T):
    """Stagnation pressure ratio from the momentum (Euler) integral V^2/2 = integral of dp/rho, then the isentropic
    temperature ratio; the Mach number from the same integral with a^2 = gamma p/rho."""
    y = _ref_y_from_V(V, 1.0, 1.0 / (R_AIR * T))          # p = 1 Pa, rho = p/(R T)
    g = GAMMA_AIR
    T0 = T * y ** ((g - 1) / g)
    return {"T0": T0, "dT": T0 - T, "M": math.sqrt(_ref_M2_of_x(1.0 / y, g))}


_add("aero_total_temperature",
     name="Total (stagnation) air temperature in flight", topic="airspeeds / ram temperature rise",
     purpose="The total (stagnation) air temperature, the ram temperature rise and the Mach number of an aircraft flying at a true airspeed through air of a static temperature (full recovery, perfect gas, air gamma = 1.4).",
     inputs=[_phys("V", "speed", "true airspeed"), _phys("T", "temperature", "static (outside) air temperature")],
     outputs=[_o("T0", "temperature", "total (stagnation) temperature"), _o("dT", "temperature_difference", "ram temperature rise"), _o("M", "number", "flight Mach number")],
     primary="T0", conditions=["150 K <= T <= 350 K", "M < 5", "full recovery (recovery factor 1)"],
     method="T0 = T + V^2/(2 cp), cp = gamma R/(gamma - 1) = 1004.69 J/(kg K); M = V/sqrt(gamma R T)",
     latex=r"T_0 = T + \frac{V^2}{2c_p} = T\left(1 + \frac{\gamma-1}{2}M^2\right)",
     run=total_temperature,
     parse=lambda q: _pairs(q, r"total (?:\(stagnation\) )?(?:air )?temperature(?: in flight)?|ram temperature rise|stagnation temperature in flight",
                            {"v": "V", "tas": "V", "t": "T", "sat": "T", "oat": "T"}, phys={"V", "T"}),
     reference=ref_total_temperature,
     wrong=_safe(lambda V, T: (lambda dT: _out(T0=T + dT, dT=dT, M=V / math.sqrt(1.4 * R_AIR * T)))(V * V / (2 * R_AIR / 0.4))),
     random_inputs=lambda rng: (lambda T: {"T": T, "V": rng.uniform(0.1, 3.0) * math.sqrt(1.4 * R_AIR * T)})(rng.uniform(200.0, 320.0)),
     templates=["Total temperature: V = 250 m/s, T = -50 °C", "Ram temperature rise: V = 480 kt, T = 220 K"],
     examples=[{"inputs": {"V": math.sqrt(1.4 * R_AIR * 250.0), "T": 250.0}, "outputs": {"T0": 300.0, "M": 1.0, "dT": 50.0}}],
     questions=[{"q": "Total temperature: V = " + repr(math.sqrt(1.4 * R_AIR * 250.0)) + " m/s, T = 250 K", "outputs": {"T0": 300.0}},
                {"q": "Total temperature: V = 250 m/s, T = 500 K", "outcome": "INVALID_INPUT"},
                {"q": "Total temperature: V = 250 m/s", "outcome": "NEEDS_INPUT"}])


def _stack(R, eps, mpl):
    above, stages = mpl, []
    for Ri, ei in zip(reversed(R), reversed(eps)):
        mi = above * (Ri - 1.0) / (1.0 - Ri * ei)
        stages.append(mi)
        above += mi
    return above, list(reversed(stages))


def optimal_staging(Isp, eps, dv, m_pl):
    _need(2 <= len(Isp) <= 5 and len(Isp) == len(eps), "give 2 to 5 stages, the same number of specific impulses and structural coefficients")
    for s_, e_ in zip(Isp, eps):
        _isp_in(s_)
        _need(0.01 <= e_ <= 0.5, "each structural coefficient must lie between 0.01 and 0.5")
    _pos("the delta-v and the payload mass", dv, m_pl)
    c = [s_ * G0 for s_ in Isp]
    _need(dv < 0.999 * sum(ci * math.log(1.0 / ei) for ci, ei in zip(c, eps)), "the delta-v is not reachable with these stages")

    def dv_of(lam):
        return sum(ci * math.log((lam * ci - 1.0) / (lam * ci * ei)) for ci, ei in zip(c, eps))
    lo = max(1.0 / (ci * (1.0 - ei)) for ci, ei in zip(c, eps)) * (1.0 + 1e-15)
    hi = lo * 2.0
    while dv_of(hi) < dv:
        hi *= 2.0
    lam = _bisect(lambda x: dv_of(x) - dv, lo, hi)
    R = [(lam * ci - 1.0) / (lam * ci * ei) for ci, ei in zip(c, eps)]
    m0, stages = _stack(R, eps, m_pl)
    return _out(m0=m0, payload_fraction=m_pl / m0, R=R, stage_mass=stages)


def ref_optimal_staging(Isp, eps, dv, m_pl):
    """Direct numerical optimization: minimize ln(m0/m_pl) over the stage mass ratios with the last one fixed by the
    delta-v constraint; the stationary point is found from the complex-step gradient (scipy root, hybr)."""
    import numpy as np
    from scipy.optimize import root
    c = np.array(Isp, dtype=float) * G0
    e = np.array(eps, dtype=float)
    n = len(c)

    def lnR_all(x):
        last = (dv - np.dot(c[:-1], x)) / c[-1]
        return np.concatenate([x, [last]])

    def f(x):
        R = np.exp(lnR_all(x))
        return np.sum(np.log(R * (1 - e) / (1 - R * e)))

    def grad(x):
        g = np.zeros(n - 1)
        for i in range(n - 1):
            xc = np.array(x, dtype=complex)
            xc[i] += 1e-30j
            g[i] = f(xc).imag / 1e-30
        return g
    x0 = np.array([dv / (n * ci) for ci in c[:-1]])
    sol = root(grad, x0, method="hybr", tol=1e-15)
    sol = root(grad, sol.x, method="hybr", tol=1e-15)
    R = [float(v) for v in np.exp(lnR_all(sol.x))]
    m0, stages = _stack(R, list(e), m_pl)
    return {"m0": m0, "payload_fraction": m_pl / m0, "R": R, "stage_mass": stages}


def _os_random(rng):
    for _ in range(200):
        n = rng.randint(2, 4)
        Isp = [rng.uniform(260.0, 460.0) for _ in range(n)]
        eps = [rng.uniform(0.05, 0.15) for _ in range(n)]
        cmax = sum(s_ * G0 * math.log(1 / e_) for s_, e_ in zip(Isp, eps))
        dv = rng.uniform(0.3, 0.7) * cmax
        if all(math.exp(dv / (n * s_ * G0)) * e_ < 0.9 for s_, e_ in zip(Isp, eps)):
            return {"Isp": Isp, "eps": eps, "dv": dv, "m_pl": _lu(rng, 10.0, 5e4)}
    return {"Isp": [300.0, 300.0], "eps": [0.1, 0.1], "dv": 6000.0, "m_pl": 1000.0}


_add("aero_optimal_staging",
     name="Optimal staging of a multistage rocket", topic="rocket performance / staging",
     purpose="The lightest rocket that gives a payload a required delta-v with stages of given specific impulses and structural coefficients: the optimal mass ratio and mass of each stage, the lift-off mass and the payload fraction (Lagrange-multiplier solution).",
     inputs=[_vecin("Isp", "specific impulse of each stage in seconds, bottom stage first, e.g. '[300, 350, 450]'", 2, 5),
             _vecin("eps", "structural coefficient ms/(ms + mp) of each stage", 2, 5),
             _phys("dv", "speed", "required total delta-v"), _phys("m_pl", "mass", "payload mass")],
     outputs=[_o("m0", "mass", "minimum lift-off mass"), _o("payload_fraction", "number", "payload fraction m_pl/m0"),
              _o("R", "number", "optimal mass ratio of each stage (list)"), _o("stage_mass", "number", "mass of each stage, structure plus propellant, in kg (list)")],
     primary="m0", conditions=["2 to 5 stages", "0.01 <= eps <= 0.5", "delta-v below what the stages could reach with zero payload", "impulsive staging, no gravity or drag losses"],
     method="R_i = (lambda c_i - 1)/(lambda c_i eps_i) with the multiplier lambda from sum c_i ln R_i = dv (bisection); stage masses built from the payload up, m_i = m_above (R_i - 1)/(1 - R_i eps_i)",
     latex=r"R_i = \frac{\lambda c_i - 1}{\lambda c_i \varepsilon_i},\quad \sum_i c_i\ln R_i = \Delta v,\quad m_i = m_{above}\frac{R_i - 1}{1 - R_i\varepsilon_i}",
     run=optimal_staging,
     parse=lambda q: _pairs(q, r"optimal staging(?: of a multistage rocket)?", {"isp": "Isp", "eps": "eps", "epsilon": "eps", "dv": "dv", "m_pl": "m_pl", "payload": "m_pl", "mpl": "m_pl"},
                            phys={"dv", "m_pl"}, vectors={"Isp", "eps"}),
     reference=ref_optimal_staging,
     wrong=_safe(lambda Isp, eps, dv, m_pl: (lambda R: (lambda m0s: _out(m0=m0s[0], payload_fraction=m_pl / m0s[0], R=R, stage_mass=m0s[1]))(_stack(R, eps, m_pl)))(
         [math.exp(dv / (len(Isp) * s_ * G0)) for s_ in Isp])),
     random_inputs=_os_random,
     templates=["Optimal staging: Isp = [300, 350, 450], eps = [0.08, 0.1, 0.12], dv = 9 km/s, m_pl = 5000 kg",
                "Optimal staging of a multistage rocket: Isp = [290, 340], eps = [0.06, 0.1], dv = 7500 m/s, m_pl = 1 t"],
     examples=[{"inputs": {"Isp": "300, 300", "eps": "0.1, 0.1", "dv": 2 * 300 * G0 * math.log(4.0), "m_pl": 1000.0},
                "outputs": {"R": [4.0, 4.0], "stage_mass": [6000 * 3 / 0.6, 1000 * 3 / 0.6], "m0": 1000 * (1 + 5) ** 2}}],
     questions=[{"q": "Optimal staging: Isp = [300, 300], eps = [0.1, 0.1], dv = " + repr(2 * 300 * G0 * math.log(4.0)) + " m/s, m_pl = 1000 kg", "outputs": {"m0": 36000.0}},
                {"q": "Optimal staging: Isp = [300, 300], eps = [0.1, 0.1], dv = 20 km/s, m_pl = 1000 kg", "outcome": "INVALID_INPUT"},
                {"q": "Optimal staging: Isp = [300, 300], eps = [0.1, 0.1], dv = 9 km/s", "outcome": "NEEDS_INPUT"}])


def satellite_coverage(h, eps_min=0.0):
    _orb_alt(h, 100e3, 1e9)
    _need(isinstance(eps_min, (int, float)) and 0 <= eps_min <= 80 * DEG, "the minimum elevation angle must lie between 0 and 80 degrees")
    r = R_E + h
    eta = math.asin(R_E * math.cos(eps_min) / r)
    lam = math.pi / 2 - eps_min - eta
    d = R_E * math.sin(lam) / math.sin(eta)
    return _out(lambda_deg=math.degrees(lam), nadir_deg=math.degrees(eta), slant_range=d, coverage=(1 - math.cos(lam)) / 2)


def ref_satellite_coverage(h, eps_min=0.0):
    """Vector geometry (numpy): the ground point at central angle lambda where the satellite appears at the minimum
    elevation, found by brentq; the coverage fraction is the spherical-cap area by quadrature."""
    import numpy as np
    from scipy.integrate import quad
    from scipy.optimize import brentq
    S = np.array([0.0, R_E + h])

    def elev(lam):
        G = R_E * np.array([math.sin(lam), math.cos(lam)])
        los = S - G
        return math.asin(float(np.dot(los, G / R_E)) / float(np.linalg.norm(los)))
    lam = brentq(lambda x: elev(x) - eps_min, 1e-12, math.pi / 2, xtol=1e-15, rtol=8.9e-16, maxiter=300)
    G = R_E * np.array([math.sin(lam), math.cos(lam)])
    a, b = -S, G - S
    nadir = math.acos(float(np.dot(a, b)) / (np.linalg.norm(a) * np.linalg.norm(b)))
    cap, _ = quad(lambda t: 2 * math.pi * math.sin(t), 0.0, lam, epsabs=0, epsrel=1e-13)
    return {"lambda_deg": math.degrees(lam), "nadir_deg": math.degrees(nadir), "slant_range": float(np.linalg.norm(b)), "coverage": cap / (4 * math.pi)}


_add("aero_satellite_coverage",
     name="Coverage of a satellite at an altitude", topic="orbits / satellite coverage",
     purpose="The Earth central angle out to the edge of a satellite's coverage, the off-nadir angle and slant range to that edge, and the fraction of the Earth's surface in view, for a minimum elevation angle at the ground (0 = the geometric horizon).",
     inputs=[_phys("h", "length", "satellite altitude"), _phys("eps_min", "angle", "minimum elevation angle at the ground station (default 0 deg)", required=False)],
     outputs=[_o("lambda_deg", "number", "Earth central angle to the coverage edge, degrees"), _o("nadir_deg", "number", "off-nadir angle to the edge, degrees"),
              _o("slant_range", "length", "slant range to the coverage edge"), _o("coverage", "number", "fraction of the Earth's surface covered")],
     primary="lambda_deg", conditions=["100 km <= h <= 1e6 km", "0 <= eps_min <= 80 deg", "spherical Earth of radius 6378.1366 km"],
     method="sin(eta) = R cos(eps)/(R + h); lambda = 90 deg - eps - eta; d = R sin(lambda)/sin(eta); coverage = (1 - cos(lambda))/2",
     latex=r"\sin\eta = \frac{R\cos\varepsilon}{R + h},\quad \lambda = 90^\circ - \varepsilon - \eta,\quad d = \frac{R\sin\lambda}{\sin\eta},\quad f = \frac{1 - \cos\lambda}{2}",
     run=satellite_coverage,
     parse=lambda q: _pairs(q, r"(?:coverage of a )?satellite coverage(?: at (?:an )?altitude)?|coverage of a satellite(?: at (?:an )?altitude)?|satellite horizon",
                            {"h": "h", "altitude": "h", "eps_min": "eps_min", "elevation": "eps_min", "e_min": "eps_min"}, phys={"h", "eps_min"}),
     reference=ref_satellite_coverage,
     wrong=_safe(lambda h, eps_min=0.0: (lambda o: {**o, "coverage": _r((1 - math.cos(math.radians(o["lambda_deg"]))) / 4), "lambda_deg": _r(90.0 - math.degrees(eps_min) - o["nadir_deg"] / 2)})(satellite_coverage(h, eps_min))),
     random_inputs=lambda rng: {"h": _lu(rng, 200e3, 4e7), **({"eps_min": rng.uniform(1.0, 45.0) * DEG} if rng.random() < 0.6 else {})},
     templates=["Satellite coverage: h = 800 km, eps_min = 10 deg", "Coverage of a satellite at an altitude: h = 35786 km"],
     examples=[{"inputs": {"h": R_E}, "outputs": {"lambda_deg": 60.0, "nadir_deg": 30.0, "coverage": 0.25, "slant_range": R_E * 3 ** 0.5}}],
     questions=[{"q": "Satellite coverage: h = " + repr(R_E / 1e3) + " km", "outputs": {"lambda_deg": 60.0, "coverage": 0.25}},
                {"q": "Satellite coverage: h = 800 km, eps_min = 85 deg", "outcome": "INVALID_INPUT"},
                {"q": "Satellite coverage:", "outcome": "NEEDS_INPUT"}])


# ================================================================================================ definitional additions
# (owner, 2026-10-23: a capability that passes every gate ships; a weaker reference is documented). Stall speed,
# level-flight lift coefficient, wing loading, induced drag, glide ratio and range, rocket thrust and specific
# impulse, ballistic coefficient, and the flight condition at an altitude (Mach number, dynamic pressure, Reynolds
# number, equivalent airspeed).

def _brent_ref(f, a, b):
    from scipy.optimize import brentq
    return brentq(f, a, b, xtol=1e-300, rtol=8.9e-16, maxiter=500)


_FLT_NAMES = {"w": "W", "weight": "W", "s": "S", "wing_area": "S", "h": "h", "altitude": "h", "v": "V", "tas": "V", "clmax": "CLmax",
              "cl_max": "CLmax", "ar": "AR", "aspect_ratio": "AR", "e": "e", "oswald": "e"}


def stall_speed(W, S, CLmax, h):
    _ws_in(W, S)
    _need(isinstance(CLmax, (int, float)) and 0.5 <= CLmax <= 5.0, "the maximum lift coefficient must lie between 0.5 and 5")
    T, p, rho = _isa(h)
    V = math.sqrt(2 * W / (rho * S * CLmax))
    return _out(V_stall=V, V_stall_kt=V / KT, EAS_stall=V * math.sqrt(rho / RHO_SL), wing_loading=W / S)


def ref_stall_speed(W, S, CLmax, h):
    """The speed at which the lift at CLmax equals the weight (scipy brentq), with the density of fluids' US 1976
    atmosphere."""
    rho, rho0 = _ref_rho(h), _ref_rho(0.0)
    V = _brent_ref(lambda v: 0.5 * rho * v * v * S * CLmax - W, 1e-6, 1e5)
    return {"V_stall": V, "V_stall_kt": V / KT, "EAS_stall": V * math.sqrt(rho / rho0)}


_add("aero_stall_speed",
     name="Stall speed in level flight", topic="aircraft performance / level flight",
     purpose="The stall speed V_s = sqrt(2W/(rho S CL_max)) of an aircraft in level flight at an altitude of the standard atmosphere, as true and equivalent airspeed.",
     inputs=[_phys("W", "force", "aircraft weight (N, kN, lbf)"), _phys("S", "area", "wing area (m^2, ft^2)"),
             _numin("CLmax", "maximum lift coefficient"), _phys("h", "length", "altitude (standard atmosphere)")],
     outputs=[_o("V_stall", "speed", "stall speed (true airspeed)"), _o("V_stall_kt", "number", "stall speed in knots (TAS)"),
              _o("EAS_stall", "speed", "stall speed as equivalent airspeed"), _o("wing_loading", "wing_loading", "wing loading W/S")],
     primary="V_stall", conditions=["W, S > 0", "0.5 <= CL_max <= 5", "-1 km <= h <= 47 km", "level flight, load factor 1"],
     method="V_s = sqrt(2 W/(rho S CL_max)); EAS = V sqrt(rho/rho0)", latex=r"V_s = \sqrt{\frac{2W}{\rho S C_{L,max}}}",
     run=stall_speed,
     parse=lambda q: _pairs(q, r"stall speed(?: in level flight)?", _FLT_NAMES, phys={"W", "S", "h"}),
     reference=ref_stall_speed,
     wrong=_safe(lambda W, S, CLmax, h: (lambda o: {**o, "V_stall": _r(o["V_stall"] / math.sqrt(2)), "V_stall_kt": _r(o["V_stall_kt"] / math.sqrt(2)),
                                                    "EAS_stall": _r(o["EAS_stall"] / math.sqrt(2))})(stall_speed(W, S, CLmax, h))),
     random_inputs=lambda rng: {"W": _lu(rng, 5e3, 3e6), "S": _lu(rng, 10.0, 500.0), "CLmax": rng.uniform(1.0, 3.2), "h": rng.uniform(0.0, 13000.0)},
     templates=["Stall speed: W = 10 kN, S = 16 m^2, CLmax = 1.6, h = 0 m"],
     examples=[{"inputs": {"W": 1e4, "S": 16.0, "CLmax": 1.6, "h": 0.0}, "outputs": {"V_stall": math.sqrt(2e4 / (RHO_SL * 16 * 1.6)), "wing_loading": 625.0}},
               {"inputs": {"W": 6e5, "S": 120.0, "CLmax": 2.5, "h": 3000.0}, "outputs": {"V_stall": math.sqrt(1.2e6 / (_isa(3000.0)[2] * 120 * 2.5))}}],
     questions=[{"q": "Stall speed: W = 10 kN, S = 16 m^2, CLmax = 1.6, h = 0 m", "outputs": {"V_stall": math.sqrt(2e4 / (RHO_SL * 16 * 1.6))}},
                {"q": "Stall speed: W = 10 kN, S = 16 m^2, CLmax = 9, h = 0 m", "outcome": "INVALID_INPUT"},
                {"q": "Stall speed: W = 10 kN, S = 16 m^2, CLmax = 1.6", "outcome": "NEEDS_INPUT"}])


def level_flight_cl(W, S, V, h):
    _ws_in(W, S)
    _pos("the true airspeed", V)
    T, p, rho = _isa(h)
    q = 0.5 * rho * V * V
    return _out(CL=W / (q * S), q=q, wing_loading=W / S)


def ref_level_flight_cl(W, S, V, h):
    """The lift coefficient at which the lift equals the weight (scipy brentq), with the density of fluids' US 1976
    atmosphere."""
    rho = _ref_rho(h)
    CL = _brent_ref(lambda c: 0.5 * rho * V * V * S * c - W, 0.0, 1e6)
    return {"CL": CL, "q": 0.5 * rho * V * V}


_add("aero_level_flight_lift_coefficient",
     name="Lift coefficient in level flight", topic="aircraft performance / level flight",
     purpose="The lift coefficient CL = 2W/(rho V^2 S) needed for steady level flight at a true airspeed and altitude, with the dynamic pressure.",
     inputs=[_phys("W", "force", "aircraft weight"), _phys("S", "area", "wing area"), _phys("V", "speed", "true airspeed (m/s, kt, km/h)"),
             _phys("h", "length", "altitude (standard atmosphere)")],
     outputs=[_o("CL", "number", "lift coefficient"), _o("q", "pressure", "dynamic pressure"), _o("wing_loading", "wing_loading", "wing loading W/S")],
     primary="CL", conditions=["W, S, V > 0", "-1 km <= h <= 47 km", "level flight, L = W"],
     method="CL = W/(q S), q = rho V^2/2", latex=r"C_L = \frac{2W}{\rho V^2 S}",
     run=level_flight_cl,
     parse=lambda q: _pairs(q, r"level[- ]flight lift coefficient|lift coefficient in level flight", _FLT_NAMES, phys={"W", "S", "V", "h"}),
     reference=ref_level_flight_cl,
     wrong=_safe(lambda W, S, V, h: (lambda o: {**o, "CL": _r(o["CL"] / 2)})(level_flight_cl(W, S, V, h))),
     random_inputs=lambda rng: {"W": _lu(rng, 5e3, 3e6), "S": _lu(rng, 10.0, 500.0), "V": rng.uniform(40, 280), "h": rng.uniform(0.0, 13000.0)},
     templates=["Lift coefficient in level flight: W = 600 kN, S = 120 m^2, V = 230 m/s, h = 11 km"],
     examples=[{"inputs": {"W": 1e4, "S": 16.0, "V": 50.0, "h": 0.0}, "outputs": {"CL": 1e4 / (0.5 * RHO_SL * 2500 * 16)}},
               {"inputs": {"W": 6e5, "S": 120.0, "V": 230.0, "h": 11000.0}, "outputs": {"q": 0.5 * _isa(11000.0)[2] * 230 ** 2}}],
     questions=[{"q": "Lift coefficient in level flight: W = 10 kN, S = 16 m^2, V = 50 m/s, h = 0 m", "outputs": {"CL": 1e4 / (0.5 * RHO_SL * 2500 * 16)}},
                {"q": "Lift coefficient in level flight: W = 10 kN, S = 16 m^2, V = 50 m/s, h = 80 km", "outcome": "INVALID_INPUT"},
                {"q": "Lift coefficient in level flight: W = 10 kN, S = 16 m^2, V = 50 m/s", "outcome": "NEEDS_INPUT"}])


def wing_loading(W, S):
    _ws_in(W, S)
    return _out(wing_loading=W / S, wing_loading_kg_m2=W / (G0 * S), wing_loading_psf=W / S / (LBF / FT ** 2))


def ref_wing_loading(W, S):
    """The weight per area in exact rational arithmetic (sympy); the mass loading with scipy's g0 and the psf form with
    scipy's pound-force and foot."""
    import sympy as sp
    from scipy import constants as sc
    ws = sp.Rational(repr(W)) / sp.Rational(repr(S))
    return {"wing_loading": float(ws), "wing_loading_kg_m2": float(ws) / sc.g, "wing_loading_psf": float(ws) / (sc.pound_force / sc.foot ** 2)}


_add("aero_wing_loading",
     name="Wing loading", topic="aircraft performance / level flight",
     purpose="The wing loading W/S of an aircraft in N/m^2, as a mass per area (kg/m^2) and in lbf/ft^2.",
     inputs=[_phys("W", "force", "aircraft weight (N, kN, lbf)"), _phys("S", "area", "wing area (m^2, ft^2)")],
     outputs=[_o("wing_loading", "wing_loading", "wing loading W/S"), _o("wing_loading_kg_m2", "number", "wing loading as mass per area, kg/m^2"),
              _o("wing_loading_psf", "number", "wing loading in lbf/ft^2")],
     primary="wing_loading", conditions=["W, S > 0"], method="W/S; W/(g0 S); 1 lbf/ft^2 = 47.880 N/m^2", latex=r"\frac{W}{S}",
     run=wing_loading,
     parse=lambda q: _pairs(q, r"wing loading", _FLT_NAMES, phys={"W", "S"}),
     reference=ref_wing_loading,
     wrong=_safe(lambda W, S: _out(wing_loading=W / S / G0, wing_loading_kg_m2=W / S, wing_loading_psf=W / S / G0 / (LBF / FT ** 2))),
     random_inputs=lambda rng: {"W": _lu(rng, 1e3, 5e6), "S": _lu(rng, 5.0, 900.0)},
     templates=["Wing loading: W = 600 kN, S = 120 m^2"],
     examples=[{"inputs": {"W": 6e5, "S": 120.0}, "outputs": {"wing_loading": 5000.0, "wing_loading_kg_m2": 5000 / G0}},
               {"inputs": {"W": 2400 * LBF, "S": 160 * FT ** 2}, "outputs": {"wing_loading_psf": 15.0}}],
     questions=[{"q": "Wing loading: W = 600 kN, S = 120 m^2", "outputs": {"wing_loading": 5000.0}},
                {"q": "Wing loading: W = 2400 lbf, S = 160 ft^2", "outputs": {"wing_loading_psf": 15.0}},
                {"q": "Wing loading: W = 600 kN, S = 0 m^2", "outcome": "INVALID_INPUT"},
                {"q": "Wing loading: W = 600 kN", "outcome": "NEEDS_INPUT"}])


def induced_drag(W, S, V, h, AR, e):
    _ws_in(W, S)
    _pos("the true airspeed", V)
    K = _K(AR, e)
    T, p, rho = _isa(h)
    q = 0.5 * rho * V * V
    CL = W / (q * S)
    CDi = K * CL * CL
    Di = q * S * CDi
    return _out(CL=CL, CDi=CDi, Di=Di, P_induced=Di * V)


def ref_induced_drag(W, S, V, h, AR, e):
    """Trefftz-plane drag of the elliptic lift distribution carrying the weight (scipy quadrature of rho Gamma(y) w(y)
    over the span, w = Gamma0/(2b)), divided by the span efficiency e; density from fluids' US 1976 atmosphere."""
    from scipy.integrate import quad
    rho = _ref_rho(h)
    b = math.sqrt(AR * S)
    G0c = 4 * W / (rho * V * math.pi * b)                         # root circulation: rho V integral of Gamma = W
    w = G0c / (2 * b)
    Di = quad(lambda y: rho * G0c * math.sqrt(max(0.0, 1 - (2 * y / b) ** 2)) * w, -b / 2, b / 2, epsabs=0, epsrel=1e-13, limit=200)[0] / e
    q = 0.5 * rho * V * V
    return {"Di": Di, "CDi": Di / (q * S), "CL": W / (q * S), "P_induced": Di * V}


_add("aero_induced_drag",
     name="Induced drag in level flight", topic="aircraft performance / drag polar",
     purpose="The lift coefficient, induced drag coefficient CL^2/(pi e AR), induced drag and induced power of an aircraft in level flight at a true airspeed and altitude.",
     inputs=[_phys("W", "force", "aircraft weight"), _phys("S", "area", "wing area"), _phys("V", "speed", "true airspeed"),
             _phys("h", "length", "altitude (standard atmosphere)"), _numin("AR", "wing aspect ratio b^2/S"), _numin("e", "Oswald span efficiency factor")],
     outputs=[_o("CL", "number", "lift coefficient"), _o("CDi", "number", "induced drag coefficient"), _o("Di", "force", "induced drag"),
              _o("P_induced", "power", "induced power Di V")],
     primary="Di", conditions=["W, S, V > 0", "1 <= AR <= 50", "0.3 <= e <= 1", "level flight, L = W"],
     method="CL = W/(q S), CDi = CL^2/(pi e AR), Di = q S CDi", latex=r"C_{D_i} = \frac{C_L^2}{\pi e A\!R},\quad D_i = \frac{W^2}{q S\,\pi e A\!R}",
     run=induced_drag,
     parse=lambda q: _pairs(q, r"induced drag(?: in level flight)?", _FLT_NAMES, phys={"W", "S", "V", "h"}),
     reference=ref_induced_drag,
     wrong=_safe(lambda W, S, V, h, AR, e: (lambda o: {**o, "CDi": _r(o["CL"] ** 2 / (e * AR)), "Di": _r(o["Di"] * math.pi), "P_induced": _r(o["P_induced"] * math.pi)})(
         induced_drag(W, S, V, h, AR, e))),
     random_inputs=lambda rng: {"W": _lu(rng, 5e3, 3e6), "S": _lu(rng, 10.0, 500.0), "V": rng.uniform(40, 280), "h": rng.uniform(0.0, 13000.0),
                                "AR": rng.uniform(4.0, 14.0), "e": rng.uniform(0.6, 1.0)},
     templates=["Induced drag: W = 600 kN, S = 120 m^2, V = 230 m/s, h = 11 km, AR = 9.5, e = 0.85"],
     examples=[{"inputs": {"W": 1e4, "S": 16.0, "V": 50.0, "h": 0.0, "AR": 8.0, "e": 0.8},
                "outputs": {"Di": 1e8 / (0.5 * RHO_SL * 2500 * 16 * math.pi * 0.8 * 8)}}],
     questions=[{"q": "Induced drag: W = 10 kN, S = 16 m^2, V = 50 m/s, h = 0 m, AR = 8, e = 0.8", "outputs": {"Di": 1e8 / (0.5 * RHO_SL * 2500 * 16 * math.pi * 0.8 * 8)}},
                {"q": "Induced drag: W = 10 kN, S = 16 m^2, V = 50 m/s, h = 0 m, AR = 8, e = 1.4", "outcome": "INVALID_INPUT"},
                {"q": "Induced drag: W = 10 kN, S = 16 m^2, V = 50 m/s, h = 0 m, AR = 8", "outcome": "NEEDS_INPUT"}])


def glide(dh, LD, V=None):
    _pos("the height and the lift-to-drag ratio", dh, LD)
    _need(LD <= 100, "the lift-to-drag ratio must be at most 100")
    gamma = math.atan(1.0 / LD)
    out = dict(range=dh * LD, range_km=dh * LD / 1e3, glide_angle_deg=gamma / DEG)
    if V is not None:
        _pos("the airspeed", V)
        out["sink_rate"] = V * math.sin(gamma)
        out["time"] = dh / (V * math.sin(gamma))
    return _out(**out)


def ref_glide(dh, LD, V=None):
    """The unpowered glide integrated in time (scipy DOP853: dx/dt = V cos(gamma), dz/dt = -V sin(gamma), with
    tan(gamma) = D/L) until the height is used up (event); the angle by atan2."""
    from scipy.integrate import solve_ivp
    gamma = math.atan2(1.0, LD)
    Vv = V if V is not None else 50.0
    ev = lambda t, y: y[1]                                           # noqa: E731
    ev.terminal, ev.direction = True, -1
    tmax = 2 * dh / (Vv * math.sin(gamma))
    s = solve_ivp(lambda t, y: [Vv * math.cos(gamma), -Vv * math.sin(gamma)], (0, tmax), [0.0, dh], events=ev, method="DOP853", rtol=1e-13, atol=1e-12)
    x = float(s.y_events[0][0][0])
    out = {"range": x, "range_km": x / 1e3, "glide_angle_deg": math.degrees(gamma)}
    if V is not None:
        out["sink_rate"] = V * math.sin(gamma)
    return out


_add("aero_glide_range",
     name="Glide ratio, angle and range", topic="aircraft performance / gliding flight",
     purpose="The still-air glide range R = h (L/D) from a height h, the glide angle atan(D/L) and, with the airspeed, the sink rate and time aloft.",
     inputs=[_phys("dh", "length", "height to be lost"), _numin("LD", "lift-to-drag (glide) ratio"),
             _phys("V", "speed", "airspeed along the glide path (optional)", required=False)],
     outputs=[_o("range", "length", "glide range over the ground"), _o("range_km", "number", "glide range, km"),
              _o("glide_angle_deg", "number", "glide angle below the horizon, degrees"), _o("sink_rate", "speed", "sink rate V sin(gamma)"),
              _o("time", "time", "time to lose the height")],
     primary="range", conditions=["h > 0", "0 < L/D <= 100", "still air, steady glide"],
     method="tan(gamma) = D/L; R = h L/D; sink rate = V sin(gamma)", latex=r"R = h\,\frac{L}{D},\quad \tan\gamma = \frac{D}{L}",
     run=glide,
     parse=lambda q: _pairs(q, r"glide (?:ratio and )?range|glide ratio", {"h": "dh", "dh": "dh", "height": "dh", "ld": "LD", "l/d": "LD", "v": "V"},
                            phys={"dh", "V"}),
     reference=ref_glide,
     wrong=_safe(lambda dh, LD, V=None: (lambda o: {**o, "range": _r(dh * LD / math.sqrt(1 + LD * LD) * LD), "range_km": _r(dh * LD / math.sqrt(1 + LD * LD) * LD / 1e3)})(glide(dh, LD, V))
                 if LD < 3 else (lambda o: {**o, "range": _r(dh / LD), "range_km": _r(dh / LD / 1e3)})(glide(dh, LD, V))),
     random_inputs=lambda rng: {"dh": rng.uniform(100, 12000), "LD": rng.uniform(3, 60), **({"V": rng.uniform(15, 120)} if rng.random() < 0.5 else {})},
     templates=["Glide range: h = 2000 m, LD = 15", "Glide range: h = 2000 m, LD = 15, V = 30 m/s"],
     examples=[{"inputs": {"dh": 2000.0, "LD": 15.0}, "outputs": {"range": 30000.0, "glide_angle_deg": math.degrees(math.atan(1 / 15))}},
               {"inputs": {"dh": 1000.0, "LD": 40.0, "V": 25.0}, "outputs": {"range_km": 40.0, "sink_rate": 25 * math.sin(math.atan(1 / 40))}}],
     questions=[{"q": "Glide range: h = 2000 m, LD = 15", "outputs": {"range": 30000.0}},
                {"q": "Glide range: h = 1000 m, LD = 40, V = 25 m/s", "outputs": {"range_km": 40.0}},
                {"q": "Glide range: h = 2000 m, LD = -15", "outcome": "INVALID_INPUT"},
                {"q": "Glide range: h = 2000 m", "outcome": "NEEDS_INPUT"}])


def rocket_thrust(mdot, ve, pe=None, pa=None, Ae=None):
    _pos("the mass flow and the exhaust velocity", mdot, ve)
    given = [x is not None for x in (pe, pa, Ae)]
    _need(all(given) or not any(given), "give the exit pressure, ambient pressure and exit area together (or none of them)")
    Fp = 0.0
    if all(given):
        _need(pe >= 0 and pa >= 0, "pressures must be zero or positive")
        _pos("the exit area", Ae)
        Fp = (pe - pa) * Ae
    F = mdot * ve + Fp
    _need(F > 0, "the thrust must be positive")
    return _out(F=F, F_kN=F / 1e3, Isp=F / (mdot * G0), c_eff=F / mdot, F_pressure=Fp)


def ref_rocket_thrust(mdot, ve, pe=None, pa=None, Ae=None):
    """The momentum balance on a control volume around the engine: the momentum flux through the exit plane (scipy
    quadrature of rho u^2 over a uniform circular exit of area Ae, or a unit area) plus the pressure force; Isp with
    scipy's g0."""
    from scipy import constants as sc
    from scipy.integrate import quad
    A = Ae if Ae is not None else 1.0
    R = math.sqrt(A / math.pi)
    rho_u = mdot / A                                                # mass flux, uniform over the exit
    mom = quad(lambda r: rho_u * ve * 2 * math.pi * r, 0.0, R, epsabs=0, epsrel=1e-13)[0]
    Fp = (pe - pa) * Ae if Ae is not None else 0.0
    F = mom + Fp
    return {"F": F, "Isp": F / (mdot * sc.g), "c_eff": F / mdot, "F_pressure": Fp}


_add("aero_rocket_thrust",
     name="Rocket thrust and specific impulse from mass flow and exhaust velocity", topic="rockets / rocket engine performance",
     purpose="The thrust F = mdot ve + (pe - pa) Ae of a rocket engine, its specific impulse F/(mdot g0) and effective exhaust velocity, from the propellant mass flow and the exhaust velocity (with the pressure thrust when the exit conditions are given).",
     inputs=[_phys("mdot", "mass_flow_rate", "propellant mass flow rate (kg/s)"), _phys("ve", "speed", "exhaust velocity at the nozzle exit"),
             _phys("pe", "pressure", "exit pressure (optional, with pa and Ae)", required=False),
             _phys("pa", "pressure", "ambient pressure (optional)", required=False), _phys("Ae", "area", "nozzle exit area (optional)", required=False)],
     outputs=[_o("F", "force", "thrust"), _o("F_kN", "number", "thrust, kN"), _o("Isp", "time", "specific impulse F/(mdot g0)"),
              _o("c_eff", "speed", "effective exhaust velocity F/mdot"), _o("F_pressure", "force", "pressure thrust (pe - pa) Ae")],
     primary="F", conditions=["mdot, ve > 0", "pe, pa, Ae given together or not at all", "steady, one-dimensional exit flow"],
     method="F = mdot ve + (pe - pa) Ae; Isp = F/(mdot g0); c = F/mdot", latex=r"F = \dot m v_e + (p_e - p_a)A_e,\quad I_{sp} = \frac{F}{\dot m g_0}",
     run=rocket_thrust,
     parse=lambda q: _pairs(q, r"rocket thrust(?: and specific impulse)?|thrust and specific impulse",
                            {"mdot": "mdot", "m_dot": "mdot", "ve": "ve", "pe": "pe", "pa": "pa", "ae": "Ae"}, phys={"mdot", "ve", "pe", "pa", "Ae"}),
     reference=ref_rocket_thrust,
     wrong=_safe(lambda mdot, ve, pe=None, pa=None, Ae=None: (lambda o: {**o, "Isp": _r(o["F"] / mdot), "F": _r(mdot * ve), "F_kN": _r(mdot * ve / 1e3)} if Ae is not None
                                                             else {**o, "Isp": _r(o["F"] / mdot)})(rocket_thrust(mdot, ve, pe, pa, Ae))),
     random_inputs=lambda rng: {"mdot": _lu(rng, 0.1, 3000.0), "ve": rng.uniform(1500, 4500),
                                **({"pe": rng.uniform(2e3, 1.5e5), "pa": rng.choice([0.0, 101325.0, rng.uniform(0, 1e5)]), "Ae": _lu(rng, 0.01, 10.0)} if rng.random() < 0.5 else {})},
     templates=["Rocket thrust: mdot = 250 kg/s, ve = 3000 m/s", "Rocket thrust: mdot = 250 kg/s, ve = 3000 m/s, pe = 70 kPa, pa = 101.325 kPa, Ae = 1.2 m^2"],
     examples=[{"inputs": {"mdot": 250.0, "ve": 3000.0}, "outputs": {"F": 750000.0, "Isp": 3000 / G0}},
               {"inputs": {"mdot": 250.0, "ve": 3000.0, "pe": 70e3, "pa": 101325.0, "Ae": 1.2}, "outputs": {"F": 750000.0 + (70e3 - 101325.0) * 1.2}}],
     questions=[{"q": "Rocket thrust: mdot = 250 kg/s, ve = 3000 m/s", "outputs": {"F": 750000.0, "Isp": 3000 / G0}},
                {"q": "Rocket thrust: mdot = 250 kg/s, ve = 3000 m/s, pe = 70 kPa", "outcome": "INVALID_INPUT"},
                {"q": "Rocket thrust: mdot = 250 kg/s", "outcome": "NEEDS_INPUT"}])


def ballistic_coefficient(m, CD, A):
    _pos("the mass, drag coefficient and reference area", m, CD, A)
    _need(CD <= 5, "the drag coefficient must be at most 5")
    return _out(beta=m / (CD * A), beta_psf=m / (CD * A) * G0 / (LBF / FT ** 2))


def ref_ballistic_coefficient(m, CD, A):
    """beta as the ratio of weight to drag per unit dynamic pressure, m g/(CD A q) * q/g, in exact rational arithmetic
    (sympy); the lbf/ft^2 form with scipy's pound-force and foot."""
    import sympy as sp
    from scipy import constants as sc
    b = sp.Rational(repr(m)) / (sp.Rational(repr(CD)) * sp.Rational(repr(A)))
    return {"beta": float(b), "beta_psf": float(b) * sc.g / (sc.pound_force / sc.foot ** 2)}


_add("aero_ballistic_coefficient",
     name="Ballistic coefficient", topic="re-entry / ballistic coefficient",
     purpose="The ballistic coefficient beta = m/(CD A) of a body (mass per unit drag area), in kg/m^2 and as a weight per area in lbf/ft^2.",
     inputs=[_phys("m", "mass", "mass"), _numin("CD", "drag coefficient"), _phys("A", "area", "reference area")],
     outputs=[_o("beta", "ballistic_coefficient", "ballistic coefficient m/(CD A)"), _o("beta_psf", "number", "ballistic coefficient as weight per drag area, lbf/ft^2")],
     primary="beta", conditions=["m, CD, A > 0", "CD <= 5"], method="beta = m/(CD A)", latex=r"\beta = \frac{m}{C_D A}",
     run=ballistic_coefficient,
     parse=lambda q: _pairs(q, r"ballistic coefficient", {"m": "m", "mass": "m", "cd": "CD", "a": "A"}, phys={"m", "A"}),
     reference=ref_ballistic_coefficient,
     wrong=_safe(lambda m, CD, A: _out(beta=m * CD / A, beta_psf=m * CD / A * G0 / (LBF / FT ** 2))),
     random_inputs=lambda rng: {"m": _lu(rng, 1.0, 1e5), "CD": rng.uniform(0.1, 2.5), "A": _lu(rng, 0.01, 50.0)},
     templates=["Ballistic coefficient: m = 1000 kg, CD = 1.2, A = 1.5 m^2"],
     examples=[{"inputs": {"m": 1000.0, "CD": 1.2, "A": 1.5}, "outputs": {"beta": 1000 / 1.8}}, {"inputs": {"m": 50.0, "CD": 2.0, "A": 0.25}, "outputs": {"beta": 100.0}}],
     questions=[{"q": "Ballistic coefficient: m = 1000 kg, CD = 1.2, A = 1.5 m^2", "outputs": {"beta": 1000 / 1.8}},
                {"q": "Ballistic coefficient: m = 1000 kg, CD = -1.2, A = 1.5 m^2", "outcome": "INVALID_INPUT"},
                {"q": "Ballistic coefficient: m = 1000 kg, CD = 1.2", "outcome": "NEEDS_INPUT"}])


def flight_condition(V, h, L=None):
    _pos("the true airspeed", V)
    T, p, rho = _isa(h)
    a = math.sqrt(GAMMA_AIR * R_AIR * T)
    out = dict(M=V / a, q=0.5 * rho * V * V, EAS=V * math.sqrt(rho / RHO_SL), EAS_kt=V * math.sqrt(rho / RHO_SL) / KT, a=a, rho=rho)
    if L is not None:
        _pos("the reference length", L)
        out["Re"] = rho * V * L / _sutherland(T)
    return _out(**out)


def ref_flight_condition(V, h, L=None):
    """Speed of sound, density and viscosity from fluids' own US 1976 atmosphere; EAS from the dynamic pressure
    referred to sea-level density (sqrt(2q/rho0))."""
    at, a0 = _fluids_atm(h), _fluids_atm(0.0)
    q = 0.5 * at.rho * V * V
    eas = math.sqrt(2 * q / a0.rho)
    out = {"M": V / at.v_sonic, "q": q, "EAS": eas, "EAS_kt": eas / KT, "a": at.v_sonic, "rho": at.rho}
    if L is not None:
        out["Re"] = at.rho * V * L / at.mu
    return out


_add("aero_flight_condition",
     name="Mach number, dynamic pressure, equivalent airspeed and Reynolds number at an altitude", topic="standard atmosphere / flight condition",
     purpose="For a true airspeed at an altitude of the standard atmosphere: the Mach number, dynamic pressure, equivalent airspeed and (with a reference length) the Reynolds number.",
     inputs=[_phys("V", "speed", "true airspeed (m/s, kt, km/h)"), _phys("h", "length", "geometric altitude (standard atmosphere)"),
             _phys("L", "length", "reference length for the Reynolds number, e.g. the chord (optional)", required=False)],
     outputs=[_o("M", "number", "Mach number"), _o("q", "pressure", "dynamic pressure"), _o("EAS", "speed", "equivalent airspeed"),
              _o("EAS_kt", "number", "equivalent airspeed, knots"), _o("a", "speed", "speed of sound"), _o("rho", "density", "air density"),
              _o("Re", "number", "Reynolds number rho V L/mu (with L)")],
     primary="M", conditions=["V > 0", "-1 km <= h <= 47 km"],
     method="M = V/a, a = sqrt(gamma R T); q = rho V^2/2; EAS = V sqrt(rho/rho0); Re = rho V L/mu (Sutherland)",
     latex=r"M = \frac{V}{\sqrt{\gamma R T}},\quad q = \tfrac12\rho V^2,\quad V_E = V\sqrt{\rho/\rho_0},\quad Re = \frac{\rho V L}{\mu}",
     run=flight_condition,
     parse=lambda q: _pairs(q, r"flight condition|mach number at (?:an )?altitude|dynamic pressure at (?:an )?altitude|reynolds number at (?:an )?altitude|equivalent airspeed at (?:an )?altitude",
                            {"v": "V", "tas": "V", "h": "h", "altitude": "h", "l": "L", "chord": "L"}, phys={"V", "h", "L"}),
     reference=ref_flight_condition,
     wrong=_safe(lambda V, h, L=None: (lambda o: {**o, "M": _r(V / A_SL), "q": _r(0.5 * RHO_SL * V * V), "EAS": _r(V), "EAS_kt": _r(V / KT)})(flight_condition(V, h, L))),
     random_inputs=lambda rng: {"V": rng.uniform(30, 600), "h": rng.uniform(1000.0, 20000.0), **({"L": _lu(rng, 0.05, 30)} if rng.random() < 0.6 else {})},
     templates=["Flight condition: V = 230 m/s, h = 11 km", "Flight condition: V = 230 m/s, h = 11 km, L = 4 m"],
     examples=[{"inputs": {"V": 100.0, "h": 0.0}, "outputs": {"M": 100 / A_SL, "q": 0.5 * RHO_SL * 1e4, "EAS": 100.0}},
               {"inputs": {"V": 230.0, "h": 11000.0, "L": 4.0}, "outputs": {"q": 0.5 * _isa(11000.0)[2] * 230 ** 2}}],
     questions=[{"q": "Flight condition: V = 100 m/s, h = 0 m", "outputs": {"M": 100 / A_SL, "EAS": 100.0}},
                {"q": "Mach number at altitude: V = 230 m/s, h = 11 km", "outputs": {"q": 0.5 * _isa(11000.0)[2] * 230 ** 2}},
                {"q": "Flight condition: V = 230 m/s, h = 90 km", "outcome": "INVALID_INPUT"},
                {"q": "Flight condition: V = 230 m/s", "outcome": "NEEDS_INPUT"}])


# === END ===
