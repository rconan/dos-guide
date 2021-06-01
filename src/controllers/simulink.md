## Simulink

For the controllers that are implemented with Simulink, the crate [simulink-rs](https://github.com/rconan/simulink-rs.git) provides [macros](https://rconan.github.io/simulink-rs) to define the Rust binding interface to the C conversions of the Simulink controllers.

Using the `simulink-rs` crate, the following GMT controllers have been implemented in Rust and each of them also implements the *Dos* interface: 
 - the GMT [mount control system](https://github.com/rconan/mount-ctrl.git).
 - the GMT [M1 control system](https://github.com/rconan/m1-ctrl.git).

In the following model example, we combine together the FEM, the wind loads and both the mount and M1 control systems.

The model starts by adding the crates 
[dosio](https://github.com/rconan/dosio.git),
[fem](https://github.com/rconan/fem.git),
[m1-ctrl](https://github.com/rconan/m1-ctrl.git),
[mount-ctrl](https://github.com/rconan/mount-ctrl.git) and
[windloading](https://github.com/rconan/windloading.git)
to the list of dependencies in *Cargo.toml*.
```toml
{{#include ../../examples/controllers/simulink/Cargo.toml:depends}}
```

Then the model is laid out in *src/main.rs* with
 - the FEM, sampled at 1kHz, with a 2% proportionnal damping coefficient and a maximum eigen fequency of 75Hz; the inputs are the windloads and the mount drives; the outputs are the mount encoders, M1 hardpoints and M1 and M2 rigid body motions:
 ```rust,ignore
 {{#include ../../examples/controllers/simulink/src/main.rs:fem}}
 ```
 - the 1st 20s of wind loads on the top-end, the truss, M1 segment, M1 cells and M1 ASM reference bodies:
 ```rust,ignore
 {{#include ../../examples/controllers/simulink/src/main.rs:windloads}}
 ```
 - the 2 parts mount controller with the mount control that takes the mount encoder FEM outputs and transform them into mount drive commands with is combined with the encoder data and converted by the mount drive model into mount drive torques applied to the FEM:
 ```rust,ignore
 {{#include ../../examples/controllers/simulink/src/main.rs:mount_control}}
 ```
 - the 2 parts M1 controller with the M1 load cells model that combines the M1 hardpoint commands with the FEM hardpoint displacement output to produce the M1 load cell to hardpoint command for the M1 CG controller that computes M1 CG force and moments applied to both FEM inputs of the M1 segment and cells:
 ```rust,ignore
 {{#include ../../examples/controllers/simulink/src/main.rs:m1_control}}
 ```
 
Finally, the dynamic simulation is implemented such as wind loads are applied to the FEM of the telescope with the feedback loop controller between the mount 3 rotational axes drives and encoders and between M1 segments hardpoints and center of gravity:
 ```rust,ignore
 {{#include ../../examples/controllers/simulink/src/main.rs:feedback_loop}}
 ```
