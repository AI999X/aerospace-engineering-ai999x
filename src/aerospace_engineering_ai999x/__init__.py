"""aerospace-engineering-ai999x — offline aerospace engineering answers with their formula, steps and verification.

    import aerospace_engineering_ai999x as m
    m.ask("Normal shock: M1 = 2").show()                                   # p2/p1 = 4.5, M2 = 0.57735
    m.ask("True airspeed: CAS = 250 kt, hp = 30000 ft").show()             # TAS = 202.41 m/s = 393.46 kt
    m.solve("aero_oblique_shock", M1=2, theta="10 deg")                    # weak shock, beta = 39.314 deg
    m.capabilities()                   # every certified capability

72 certified capabilities: the standard atmosphere (ISA properties, pressure, density and geopotential altitude),
airspeeds (calibrated and true airspeed, pitot Mach number, ram temperature), compressible flow (isentropic ratios,
area-Mach relation, normal and oblique shocks, maximum deflection, Prandtl-Meyer expansions, Fanno and Rayleigh flow,
critical Mach number), nozzles and rocket engines (choked flow, c*, thrust coefficient, exhaust velocity, optimum
expansion, engine performance and sizing, normal shock in a nozzle), rocket performance (Tsiolkovsky, propellant for a
delta-v, gravity loss, vertical flight, multistage delta-v, identical and optimal staging), aircraft performance (drag
polar optimum, minimum-drag and minimum-power speeds, Breguet range and endurance for jets and propeller aircraft,
takeoff and landing ground roll, level turns, climb rate and angle, absolute and service ceilings, wind triangle),
static stability (neutral point and static margin), propellers and rotors (momentum theory), wings and airfoils (NACA
4-digit thin-airfoil theory, elliptic-wing lifting line), orbits (circular orbits, escape, period-altitude, bi-elliptic
transfer, Lambert's problem, J2 nodal and apsidal drift, sun-synchronous inclination, satellite coverage) and ballistic
re-entry (Allen-Eggers). Altitudes are geometric heights in the US Standard Atmosphere 1976 (= ISA to 47 km).
Questions use the form "<capability phrase>: name = value, ..."; ask() also reads kt, nmi, fpm, lbf, lb, slug, hPa,
inHg, psi, °F, 1/h or lb/(lbf*h) for TSFC and lb/(hp*h) for BSFC; lists are written in brackets.
Built by the AI999X model machine from the same engine as every domain; a question that is not determined
is refused rather than guessed.

Creator / engineer: Ibnul Jaif Farabi. Company: AI999X.
"""
from __future__ import annotations

from pathlib import Path

try:                                       # the installed package carries its own copy of the engine
    from ._engine import Answer
    from ._engine.product import Product
except ImportError:                        # the factory's source tree, during development
    from ai999x_factory.runtime import Answer
    from ai999x_factory.runtime.product import Product

__version__ = "1.0.3"
_P = Product("aerospace_engineering", [Path(__file__).resolve().parent / "pack", Path(__file__).resolve().parents[3]])
pack, ask, solve, compile_problem = _P.pack, _P.ask, _P.solve, _P.compile_problem
formula, formulas, quantities, conditions, capabilities = _P.formula, _P.formulas, _P.quantities, _P.conditions, _P.capabilities
examples, family_examples, structured_example, to_si = _P.examples, _P.family_examples, _P.structured_example, _P.to_si
templates, procedure_examples = _P.templates, _P.procedure_examples
prove, derivations, about = _P.prove, _P.derivations, (lambda: {**_P.about(), "version": __version__})
knowledge, lookup, related = _P.knowledge, _P.lookup, _P.related
plot, plot_relation, search = _P.plot, _P.plot_relation, _P.search   # figures (SVG) and capability search ([search])
__all__ = ["ask", "solve", "prove", "derivations", "about", "formula", "formulas", "quantities", "conditions", "capabilities",
           "compile_problem", "pack", "Answer", "plot", "plot_relation", "search", "__version__"]
