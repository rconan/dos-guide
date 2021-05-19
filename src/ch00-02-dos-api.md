## DOS API

The DOS API is defined in the Rust crate [dosio](https://github.com/rconan/dosio). 
`dosio` consists in the enum `IO` with as many variants as there are inputs and outputs to *DOS* and in the trait `Dos`. 
Every component that needs to be included in a DOS end-to-end model, must implement the `inputs` and `outputs` methods of the `Dos` trait as well as implementing the [`Iterator`](https://doc.rust-lang.org/std/iter/index.html#implementing-iterator) trait.
The inputs data to the component is passed as an argument to the `inputs` method and the component outputs is returned by the `outputs` method.
Both inputs and outputs data are contained into variants of the `IO` enum.
The state of the component is updated with the `step` method of the `Dos` trait which in turns calls the `next` method of the `Iterator` trait implemented by the component.

```rust
{{#rustdoc_include ../examples/ch01-02-dos-api/src/main.rs}}
```
