## Implementation Rules

### Better architecture patterns must be evaluated and implemented after every implementation

### Breaking changes are encouraged 
- There are no consumers
- Code must be thin
- Code must be modular
- Code must be inherently safe

### Vertical Sclicing, Tracer Bullets, Steel threading (Only during implementation pass)

1. Construct a risk-retiring walking skeleton: a thin, production-shaped, test-bound vertical slice through the architecturally significant path.
2. Use it as an executable epistemic probe to falsify schema, API, and domain-model priors under real integration pressure.
3. Only after the slice exhibits stable invariants, regression safety, and coherent information-hiding boundaries should the design be horizontally generalized.
4. Scale the design horizontally, COMPLETELY. 
