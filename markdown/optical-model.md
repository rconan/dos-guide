# Optical Model

The GMT optical model defines the optical prescriptions of the GMT primary and secondary mirrors along with the functionnalities to perform ray tracing through the telescope down to the exit pupil.
Fourier optics based optical sensor models transform the wavefront in the telescope exit pupil into either a focal plane image, Shack-Hartmann subaperture images  or a pyramid four-quadrant pupil image, for example.
Noise can be added to the sensor images by the mean of a detector noise model.

At the core of the optical model is [CEO](https://github.com/rconan/CEO), a GPU optics simulator written in C++ using the CUDA API. 
A Rust binding for CEO is provided by the crate [crseo](https://github.com/rconan/crseo), that makes all the functionalities of CEO available through an idiomatic RUST API.
[crseo](https://rconan.github.io/crseo) provides an implementation of the DOS Rust interface allowing to seamlessly integrate CEO with the GMT Dynamics Optics Simulation framework.




