## DOS Interface

The *DOS* interface for the optical model is implemented for 2 structures: `GmtOpticalModelInner` and `GmtOpticalSensorModelInner`.
`GmtOpticalModelInner` only performs ray tracing through the telescope and `GmtOpticalSensorModelInner` appends an optical sensor to the model,  Fourier propagating the wavefront derived from ray tracing to the sensor image plane. 
Both allows to alter the state of the M1 and M2 segment rigid body motions and segment surface figures.

### GMT Optical Model

A  `GmtOpticalModelInner` structure is created with the [`GmtOpticalModel`](https://rconan.github.io/crseo/crseo/dos/struct.GmtOpticalModel.html) builder that uses default templates for both the [GMT](https://rconan.github.io/crseo/crseo/struct.GMT.html) and the [source](https://rconan.github.io/crseo/crseo/struct.SOURCE.html) models.
The outputs of the model need to be specified as well, in the following, the model provides 2 outputs: the wavefront error rms and the PSSn are specified.

```rust,ignore
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
```

### GMT Optical Sensor Model

A  `GmtOpticalModeSensorlInner` structure is created with the [`GmtOpticalSensorModel`](https://rconan.github.io/crseo/crseo/dos/sensor/struct.GmtOpticalSensorModel.html) builder that uses default templates for the [GMT](https://rconan.github.io/crseo/crseo/struct.GMT.html), the [source](https://rconan.github.io/crseo/crseo/struct.SOURCE.html) and the optical sensor models.
In the following, the optical sensor is based on the [SH48](https://rconan.github.io/crseo/crseo/shackhartmann/struct.SH48.html) template but reduced to a single sensor.
The model has a single output: the data from the optical sensor.

```rust,ignore
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
```
