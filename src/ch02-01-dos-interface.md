## DOS Interface

The *DOS* interface for the optical model is implemented for 2 structures: `GmtOpticalModelInner` and `GmtOpticalSensorModelInner`.
`GmtOpticalModelInner` only performs ray tracing through the telescope and `GmtOpticalSensorModelInner` appends an optical sensor to the model,  Fourier propagating the wavefront derived from ray tracing to the sensor image plane. 
Both allows to alter the state of the M1 and M2 segment rigid body motions and segment surface figures.

### GMT Optical Model

### GMT Optical Sensor Model
