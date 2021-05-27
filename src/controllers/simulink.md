## Simulink

For the controllers that are implemented with Simulink, the crate [simulink-rs](https://github.com/rconan/simulink-rs.git) provides [macros](https://rconan.github.io/simulink-rs) to define the Rust binding interface to the C conversions of the Simulink controllers.

Using the `simulink_rs` crate, the following GMT controllers have been implemented in Rust and each of them also implements the *Dos* interface: 
 - the GMT [mount control system](https://github.com/rconan/mount-ctrl.git).
The mount control system is closely related to the FEM as it takes inputs from the FEM outputs and send its outputs to the FEM inputs.
 - the GMT [M1 control system](https://github.com/rconan/m1-ctrl.git).
The M1 control system is related to both the wind loads and the FEM. M1 takes its inputs from some of the wind loads and send its outputs to the FEM inputs.

In the following model example, we combine together the FEM, the wind loads and both the mount and M1 control systems.

The model starts by adding the crates 
[dosio](https://github.com/rconan/dosio.git),
[fem](https://github.com/rconan/fem.git),
[m1-ctrl](https://github.com/rconan/m1-ctrl.git),
[mount-ctrl](https://github.com/rconan/mount-ctrl.git) and
[windloading](https://github.com/rconan/windloading.git)
to the list of dependencies.
```toml
{{#include ../../examples/controllers/simulink/Cargo.toml:depends}}
```
