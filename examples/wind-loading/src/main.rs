use dosio::{Dos, IOTags};
use std::path::Path;
use std::time::Instant;
use windloading::WindLoads;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ANCHOR: wind_loading
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
    // ANCHOR_END: wind_loading
    // ANCHOR: wind_loading_dynamic
    println!("Playing the wind loads ...");
    let now = Instant::now();
    while let Some(_fem_forces) = wind_loading.outputs() {}
    println!(
        "{} sample wind loads played in {}ms",
        wind_loading.n_sample,
        now.elapsed().as_millis()
    );
    // ANCHOR_END: wind_loading_dynamic
    Ok(())
}
