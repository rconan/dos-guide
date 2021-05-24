# Installation

CEO will compile only on a machine with a NVIDIA GPU installed and with the CUDA API present. 
CEO installation steps are given [here](https://github.com/rconan/CEO).

CEO bindings crate `crseo`, needs to be added to the dependencies list in the *Cargo.toml* file of your project, the dosio feature is required in order to compile the *Dos* interface:
```toml
crseo = crseo = { git = "https://github.com/rconan/crseo.git", branch = "main", features = ["dosio"] }
```
