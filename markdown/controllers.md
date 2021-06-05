# Controllers

The GMT has multiple controllers connecting sensors and actuators distributed across the telescope.
From a programming perspective, there are 2 categories of controllers:
 - controllers that are implemented using Simulink, converted to C and binded to Rust,
 - and controllers that are implemented directly in Rust.
