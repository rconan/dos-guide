## DOS Interface

The *DOS* interface for the optical model is implemented for 2 structures: `GmtOpticalModelInner` and `GmtOpticalSensorModelInner`.
`GmtOpticalModelInner` only performs ray tracing through the telescope and `GmtOpticalSensorModelInner` appends an optical sensor to the model,  Fourier propagating the wavefront derived from ray tracing to the sensor image plane. 
Both allows to alter the state of the M1 and M2 segment rigid body motions and segment surface figures.

### GMT Optical Model

A  `GmtOpticalModelInner` structure is created with the [`GmtOpticalModel`](https://rconan.github.io/crseo/crseo/dos/struct.GmtOpticalModel.html) builder that uses default templates for both the [GMT](https://rconan.github.io/crseo/crseo/struct.GMT.html) and the [source](https://rconan.github.io/crseo/crseo/struct.SOURCE.html) models.
The outputs of the model need to be specified as well, in the following, the model provides 2 outputs: the wavefront error rms and the PSSn are specified.

```rust,no_run,noplayground
{{#rustdoc_include ../examples/optical-model_dos-interface/gmt_optical_model/src/main.rs}}
```

### GMT Optical Sensor Model

A  `GmtOpticalModeSensorlInner` structure is created with the [`GmtOpticalSensorModel`](https://rconan.github.io/crseo/crseo/dos/sensor/struct.GmtOpticalSensorModel.html) builder that uses default templates for the [GMT](https://rconan.github.io/crseo/crseo/struct.GMT.html), the [source](https://rconan.github.io/crseo/crseo/struct.SOURCE.html) and the optical sensor models.
In the following, the optical sensor is based on the [SH48](https://rconan.github.io/crseo/crseo/shackhartmann/struct.SH48.html) template but reduced to a single sensor.
The model has a single output: the data from the optical sensor.

```rust,no_run,noplayground
{{#rustdoc_include ../examples/optical-model_dos-interface/gmt_optical_sensor_model/src/main.rs}}
```
