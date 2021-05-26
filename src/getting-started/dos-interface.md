## DOS Interface

The DOS Interface is defined in the Rust crate [dosio](https://github.com/rconan/dosio). 
[`dosio`](https://rconan.github.io/dosio) consists in the enum `IO` with as many variants as there are inputs and outputs to *DOS* and in the trait `Dos`. 
Every component that needs to be included in a DOS end-to-end model, must implement the `inputs` and `outputs` methods of the `Dos` trait as well as implementing the [`Iterator`](https://doc.rust-lang.org/std/iter/index.html#implementing-iterator) trait.
The inputs data to the component is passed as an argument to the `inputs` method and the component outputs is returned by the `outputs` method.
Both inputs and outputs data are contained into variants of the `IO` enum.
The state of the component is updated with the `step` method of the `Dos` trait which in turns calls the `next` method of the `Iterator` trait implemented by the component.

In the following example, a simple integrator controller is integrated with DOS using the interface defined in the crate `dosio`. 

### Example

A new project is created with Cargo:

```console
$ cargo init
```

The Cargo command creates a *Cargo.toml* file
and a *main.rs* file in the *src* directory.
The *Cargo.toml* contains the package meta-data and an optional list of the project dependencies, which is empty for now:
```toml
{{#include examples/Cargo.toml:original}}
```
In the *main.rs* file, we add the simple integrator model represented by the `Integrate` structure and implement the `Iterator`  trait for the `Integrate` structure:
```rust,ignore
{{#rustdoc_include examples/src/main.rs:integrator}}
```
Next, we add the crate `dosio` as a project dependency:
```toml
{{#include examples/Cargo.toml:all}}
```
and we implement the `Dos` trait for `Integrate`:
```rust,ignore
{{#rustdoc_include examples/src/main.rs:dos}}
```
Now we can write the main function:
```rust,ignore
{{#rustdoc_include examples/src/main.rs:main}}
```
