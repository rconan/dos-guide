## Dos Interface

From one CFD case, the wind loads time series are resampled to the sampling rate of a *Dos* simulation and saved into a Pickle file to be re-played later.
The wind loads are then loaded into the simulation and the wind loads to use with the simulation are selected according to the telescope structural component they are applied to.
```rust,ignore
# use dosio::{Dos, IOTags};
# use std::path::Path;
# use std::time::Instant;
# use windloading::WindLoads;
# 
# fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Loading the wind loads ...");
    let now = Instant::now();
    let fem_data_path = Path::new("data").join("20210225_1447_MT_mount_v202102_ASM_wind2");
    let mut wind_loading = WindLoads::from_pickle(
        fem_data_path.join("b2019_0z_30az_os_7ms.wind_loads_1kHz_100-400s.pkl"),
    )?
    .range(0.0, 20.0)
    .truss()?
    .m2_asm_topend()?
    .m1_segments()?
    .m1_cell()?
    .m2_asm_reference_bodies()?
    .build()?;
    println!("Wind loads loaded in {}ms", now.elapsed().as_millis());
    println!("wind loads outputs: {:#?}", wind_loading.outputs_tags());
#     println!("Playing the wind loads ...");
#     let now = Instant::now();
#     while let Some(_fem_forces) = wind_loading.outputs() {}
#     println!(
#         "{} sample wind loads played in {}ms",
#         wind_loading.n_sample,
#         now.elapsed().as_millis()
#     );
#     Ok(())
# }
```

The [`windloading`](https://rconan.github.io/windloading) crate implements only the `outputs` method of the *Dos* interface stepping through the time series of forces and moments at each call and returning `None` when the range limit has been reached.
```rust,ignore
# use dosio::{Dos, IOTags};
# use std::path::Path;
# use std::time::Instant;
# use windloading::WindLoads;
# 
# fn main() -> Result<(), Box<dyn std::error::Error>> {
#     println!("Loading the wind loads ...");
#     let now = Instant::now();
#     let fem_data_path = Path::new("data").join("20210225_1447_MT_mount_v202102_ASM_wind2");
#     let mut wind_loading = WindLoads::from_pickle(
#         fem_data_path.join("b2019_0z_30az_os_7ms.wind_loads_1kHz_100-400s.pkl"),
#     )?
#     .range(0.0, 20.0)
#     .truss()?
#     .m2_asm_topend()?
#     .m1_segments()?
#     .m1_cell()?
#     .m2_asm_reference_bodies()?
#     .build()?;
#     println!("Wind loads loaded in {}ms", now.elapsed().as_millis());
#     println!("wind loads outputs: {:#?}", wind_loading.outputs_tags());
    println!("Playing the wind loads ...");
    let now = Instant::now();
    while let Some(_fem_forces) = wind_loading.outputs() {}
    println!(
        "{} sample wind loads played in {}ms",
        wind_loading.n_sample,
        now.elapsed().as_millis()
    );
#     Ok(())
# }
```

