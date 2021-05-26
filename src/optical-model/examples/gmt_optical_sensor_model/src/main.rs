use crseo::dos::GmtOpticalSensorModel;
use crseo::{shackhartmann::Geometric as WFS_TYPE, Builder, ShackHartmann, SH48};
use dosio::{io::jar, Dos};

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let mut gosm = GmtOpticalSensorModel::<ShackHartmann<WFS_TYPE>, SH48<WFS_TYPE>>::new()
        .sensor(SH48::<WFS_TYPE>::new().n_sensor(1))
        .build()?;
    println!("M1 mode: {}", gosm.gmt.get_m1_mode_type());
    println!("M2 mode: {}", gosm.gmt.get_m2_mode_type());
    println!("GS band: {}", gosm.src.get_photometric_band());

    let y = gosm.in_step_out(None)?.unwrap();
    let sensor_data: Option<Vec<f64>> = (&y[jar::SensorData::io()]).into();
    println!("sensor data size: {:}", sensor_data.unwrap().len());

    Ok(())
}
