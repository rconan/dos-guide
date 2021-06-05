# Finite Element Model

The finite element model (FEM) is a second order coupled set of differential equations that models the dynamic behavior of the telescope mechanical structure subject to forces and moments applied to a discrete set of nodes on the telescope.
The FEM used in the DOS has been decomposed into its eigen modes, each mode representing a unique dynamic resonance of the telescope at a particular eigen frequency.
In the space of the eigen modes, the FEM is reduced to a set of second order **decoupled** differential equations

The FEM is modeled with the crate [fem](https://github.com/rconan/fem) allowing to import the data exported from a FEM solver and to convert the data into a discrete dynamic model with a *Dos* interface.
