## Installation

CEO compiles only on a machine with a NVIDIA GPU installed and with the CUDA API present. 
CEO installation steps are given [here](https://github.com/rconan/CEO).

CEO bindings crate `crseo`, needs to be added to the dependencies list in the *Cargo.toml* file of your project, both the `dosio` crate and the `dos` feature of `crseo` are required in order to compile the *Dos* interface:
```toml
dosio = { git = "https://github.com/rconan/dosio.git", branch = "main" }
# should be the git url, but with the git repo, CEO does not compile? 
crseo = { path = "/home/rconan/projects/crseo" , features = ["dos"]}
#crseo = { git = "https://github.com/rconan/crseo.git", branch = "main", features = ["dos"] }
```
