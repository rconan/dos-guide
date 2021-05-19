# Introduction

The *Dynamic Optics Simulation*, or *DOS* for short, is the simulation software developed by the Integrated Modeling team of the GMTO project to write the end-to-end models of the Observatory.
As its core, the *DOS* provide a software framework to build the necessary interfaces with the many components of the end-to-end model like the [optical model](ch01-00-optical-model.md) or the [finite element model](ch02-00-fem.md). 
The *DOS* framework also provides all the possible inputs and outputs to the different model components.

The *DOS* framework is written in [Rust](https://www.rust-lang.org), the Rust language is a high level zero-cost abstraction language that is well suited for this kind of high performance computing application as, once compiled, and despite beeing a high level language, the runtime execution speed is comparable to what would have given a C or C++ equivalent program.
