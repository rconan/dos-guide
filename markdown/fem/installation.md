## Installation

Both the crates `dosio` and `fem` needs to be added to the dependencies list in the *Cargo.toml* file of your project, the features *dos* of the `fem` is required to compile the *Dos* interface.
```toml
dosio = { git = "https://github.com/rconan/dosio.git", branch = "main" }
fem = { path = "/home/rconan/projects/fem", features = ["dos"] }
simple_logger = "1.11.0"
```
