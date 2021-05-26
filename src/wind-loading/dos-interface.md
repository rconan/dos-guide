## Dos Interface

From one CFD case, the wind loads time series are resampled to the sampling rate of a *Dos* simulation and saved into a Pickle file to be re-played later.
The wind loads are then loaded into the simulation and the wind loads to use with the simulation are selected according to the telescope structural component they are applied to.
```rust,ignore
{{#rustdoc_include examples/src/main.rs:wind_loading}}
```

The [`windloading`](https://rconan.github.io/windloading) crate implements only the `outputs` method of the *Dos* interface stepping through the time series of forces and moments at each call and returning `None` when the range limit has been reached.
```rust,ignore
{{#rustdoc_include examples/src/main.rs:wind_loading_dynamic}}
```

