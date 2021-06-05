# Wind Loading

The [windloading](https://github.com/rconan/windloading) crate loads the time series of forces and moments computed with the CFD simulations and allocates them to different `IO` enum variants of the *Dos* interface. 
The wind loads can be applied on the following components of the FEM:
 - the top-end,
 - the M2 ASMs top-end,
 - the M2 segments,
 - the M2 ASMs reference bodies,
 - the trusses,
 - the M1 segments,
 - the M1 cells,
 - the GIR,
 - the C-Ring.
