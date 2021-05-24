use crseo::dos::GmtOpticalModel;
use dosio::{io::jar, Dos};

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let mut gom = GmtOpticalModel::new()
        .output(jar::SrcWfeRms::io())
        .output(jar::Pssn::io())
        .build()?;
    println!("M1 mode: {}", gom.gmt.get_m1_mode_type());
    println!("M2 mode: {}", gom.gmt.get_m2_mode_type());
    println!("GS band: {}", gom.src.get_photometric_band());
    println!("Outputs: {:#?}", gom.outputs);

    let y = gom.in_step_out(None)?;
    println!("y: {:#?}", y);

    Ok(())
}
