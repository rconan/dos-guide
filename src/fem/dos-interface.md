## Dos Interface

In the following, the FEM is discretized with a 1kHz sampling rate, the modal damping coefficient is set to 2% and the eigen frequencies are truncated at 75Hz.
Inputs are reduced to the wind loads of the M1 segments and the outputs are the rigid body motions of M1 and M2 segments.
```rust,ignore
{{#rustdoc_include ../../examples/fem/src/main.rs:fem}}
```

A constant wind pressure is applied to the M1 segments and the norm of the rigid body motion vectors is computed.
```rust,ignore
{{#rustdoc_include ../../examples/fem/src/main.rs:fem_dynamic}}
```
