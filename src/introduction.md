# Introduction

The *Dynamic Optics Simulation*, or *DOS* for short, is the simulation software developed by the Integrated Modeling team of the GMTO project to write the end-to-end models of the Observatory.
As its core, the *DOS* provide a software framework to build the necessary interfaces with the many components of the end-to-end model like the [optical model](ch01-00-optical-model.md) or the [finite element model](ch02-00-fem.md). 
The *DOS* framework also provides all the possible inputs and outputs to the different model components.

The *DOS* framework is written in [Rust](https://www.rust-lang.org), the Rust language is a high level zero-cost abstraction language that is well suited for this kind of high performance computing application as, once compiled, and despite beeing a high level language, the runtime execution speed is comparable to what would have given a C or C++ equivalent program.

A *DOS* simulation is built from a collection of models, each model is implemented within a different Rust crate.
In each crate, there is an implementation of the *DOS* interface that allows the different models to communicate by passing data to each other.

The models currently available are listed in the table below:

| Model | Crate | Github | Doc |
| ----- | ----- | ------ | --- |
| Optical Model | crseo | <a href="https://github.com/rconan/crseo"><img style="display: inline! important" src="https://img.shields.io/badge/src-main-green.svg"></img></a> | <a href="https://rconan.github.io/crseo"><img style="display: inline! important" src="https://img.shields.io/badge/doc-current-green.svg"></img></a> |
| Finite Element Model | fem | <a href="https://github.com/rconan/fem/tree/dos"><img style="display: inline! important" src="https://img.shields.io/badge/src-dos-green.svg"></img></a> | <a href="https://rconan.github.io/fem"><img style="display: inline! important" src="https://img.shields.io/badge/doc-current-green.svg"></img></a> |
| Wind Loading | windloading | <a href="https://github.com/rconan/windloading"><img style="display: inline! important" src="https://img.shields.io/badge/src-main-green.svg"></img></a> | <a href="https://rconan.github.io/windloading"><img style="display: inline! important" src="https://img.shields.io/badge/doc-current-green.svg"></img></a> |
| Mount Control System | mount-ctrl | <a href="https://github.com/rconan/mount-ctrl"><img style="display: inline! important" src="https://img.shields.io/badge/src-main-green.svg"></img></a> | <a href="https://rconan.github.io/mount-ctrl"><img style="display: inline! important" src="https://img.shields.io/badge/doc-current-red.svg"></img></a> |
| M1 Control System | m1-ctrl | <a href="https://github.com/rconan/m1-ctrl"><img style="display: inline! important" src="https://img.shields.io/badge/src-main-green.svg"></img></a> | <a href="https://rconan.github.io/m1-ctrl"><img style="display: inline! important" src="https://img.shields.io/badge/doc-current-red.svg"></img></a> |

In addition to the crates for the models, there are other crates that provide specific interfaces:

| Model | Crate | Github | Doc |
| ----- | ----- | ------ | --- |
| Dos Interface | dosio | <a href="https://github.com/rconan/dosio"><img style="display: inline! important" src="https://img.shields.io/badge/src-main-green.svg"></img></a> | <a href="https://rconan.github.io/dosio"><img style="display: inline! important" src="https://img.shields.io/badge/doc-current-green.svg"></img></a> |
| Simulink Interface | simulink-rs | <a href="https://github.com/rconan/simulink-rs"><img style="display: inline! important" src="https://img.shields.io/badge/src-main-green.svg"></img></a> | <a href="https://rconan.github.io/simulink-rs"><img style="display: inline! important" src="https://img.shields.io/badge/doc-current-green.svg"></img></a> |
