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
[package]
name = "ch01-02-dos-api"
version = "0.1.0"
authors = ["Rod Conan <rconan@gmto.org>"]
edition = "2018"

[dependencies]
```
In the *main.rs* file, we add the simple integrator model represented by the `Integrate` structure and implement the `Iterator`  trait for the `Integrate` structure:
```rust,ignore
#[derive(Default)]
pub struct Integrate {
    /// integrator gain
    gain: f64,
    /// output vector
    y: Vec<f64>,
    /// input vector
    u: Vec<f64>,
}
impl Integrate {
    pub fn new(gain: f64, n_data: usize) -> Self {
        Self {
            gain,
            y: vec![0f64; n_data],
            ..Default::default()
        }
    }
    pub fn output(&self) -> Option<Vec<f64>> {
        Some(self.y.clone())
    }
}
impl Iterator for Integrate {
    type Item = ();
    fn next(&mut self) -> Option<Self::Item> {
        let gain = self.gain;
        self.y.iter_mut().zip(self.u.iter()).for_each(|(a, v)| {
            *a += *v * gain;
        });
        Some(())
    }
}

# use dosio::{io::jar, DOSIOSError, Dos, IO};
# 
# impl Dos for Integrate {
#     type Input = Vec<f64>;
#     type Output = Vec<f64>;
#     fn inputs(&mut self, data: Option<Vec<IO<Self::Input>>>) -> Result<&mut Self, DOSIOSError> {
#         self.u = data
#             // Identifying M2 42 segment rigid body motion IO variant
#             .and_then(|data| {
#                 data.into_iter().find_map(|io| match io {
#                     IO::MCM2RB6D { data: value } => value,
#                     _ => None,
#                 })
#             })
#             // Extracting M2 14 Rx and Ry rigid body motions
#             .map(|x| {
#                 x.chunks(6)
#                     .flat_map(|x| x[3..5].to_vec())
#                     .collect::<Vec<f64>>()
#             })
#             // Throwing an error if IO::MCM2RB6D is missing
#             .ok_or_else(|| DOSIOSError::Inputs("Integrate MCM2RB6D IO input error".into()))?;
#         Ok(self)
#     }
#     fn outputs(&mut self) -> Option<Vec<IO<Self::Output>>> {
#         self.output()
#             // Assigning M2 segment Rx and Ry to rigid body motion vector
#             .map(|y| {
#                 y.chunks(2)
#                     .flat_map(|y| vec![0f64, 0f64, 0f64, y[0], y[1], 0f64])
#                     .collect::<Vec<f64>>()
#             })
#             // Embedding the rigid body motions into the corresponding IO variant
#             .map(|x| vec![jar::MCM2RB6D::io_with(x)])
#     }
# }
# 
# fn main() -> Result<(), Box<dyn std::error::Error>> {
#     // Integrator controller
#     let mut integrator = Integrate::new(0.5, 14);
#     // M2 rigid body motions to DOS input
#     let mut m2_rbm = vec![vec![0f64; 6]; 7];
#     m2_rbm[0][3] = 1f64; // Segment #1: Rx
#     let dos_in = vec![jar::MCM2RB6D::io_with(
#         m2_rbm.into_iter().flatten().collect::<Vec<f64>>(),
#     )];
#     // DOS stepping
#     let dos_out = integrator.inputs(Some(dos_in))?.step()?.outputs();
#     // or equivalently:
#     // let dos_out = integrator.in_step_out(Some(dos_in))?;
#     println!("DOS output:");
#     println!("{:#?}", dos_out);
#     Ok(())
# }
```
Next, we add the crate `dosio` as a project dependency:
```toml
[package]
name = "ch01-02-dos-api"
version = "0.1.0"
authors = ["Rod Conan <rconan@gmto.org>"]
edition = "2018"

[dependencies]
dosio = { git = "https://github.com/rconan/dosio.git", branch = "main" }

```
and we implement the `Dos` trait for `Integrate`:
```rust,ignore
# #[derive(Default)]
# pub struct Integrate {
#     /// integrator gain
#     gain: f64,
#     /// output vector
#     y: Vec<f64>,
#     /// input vector
#     u: Vec<f64>,
# }
# impl Integrate {
#     pub fn new(gain: f64, n_data: usize) -> Self {
#         Self {
#             gain,
#             y: vec![0f64; n_data],
#             ..Default::default()
#         }
#     }
#     pub fn output(&self) -> Option<Vec<f64>> {
#         Some(self.y.clone())
#     }
# }
# impl Iterator for Integrate {
#     type Item = ();
#     fn next(&mut self) -> Option<Self::Item> {
#         let gain = self.gain;
#         self.y.iter_mut().zip(self.u.iter()).for_each(|(a, v)| {
#             *a += *v * gain;
#         });
#         Some(())
#     }
# }
# 
use dosio::{io::jar, DOSIOSError, Dos, IO};

impl Dos for Integrate {
    type Input = Vec<f64>;
    type Output = Vec<f64>;
    fn inputs(&mut self, data: Option<Vec<IO<Self::Input>>>) -> Result<&mut Self, DOSIOSError> {
        self.u = data
            // Identifying M2 42 segment rigid body motion IO variant
            .and_then(|data| {
                data.into_iter().find_map(|io| match io {
                    IO::MCM2RB6D { data: value } => value,
                    _ => None,
                })
            })
            // Extracting M2 14 Rx and Ry rigid body motions
            .map(|x| {
                x.chunks(6)
                    .flat_map(|x| x[3..5].to_vec())
                    .collect::<Vec<f64>>()
            })
            // Throwing an error if IO::MCM2RB6D is missing
            .ok_or_else(|| DOSIOSError::Inputs("Integrate MCM2RB6D IO input error".into()))?;
        Ok(self)
    }
    fn outputs(&mut self) -> Option<Vec<IO<Self::Output>>> {
        self.output()
            // Assigning M2 segment Rx and Ry to rigid body motion vector
            .map(|y| {
                y.chunks(2)
                    .flat_map(|y| vec![0f64, 0f64, 0f64, y[0], y[1], 0f64])
                    .collect::<Vec<f64>>()
            })
            // Embedding the rigid body motions into the corresponding IO variant
            .map(|x| vec![jar::MCM2RB6D::io_with(x)])
    }
}
# 
# fn main() -> Result<(), Box<dyn std::error::Error>> {
#     // Integrator controller
#     let mut integrator = Integrate::new(0.5, 14);
#     // M2 rigid body motions to DOS input
#     let mut m2_rbm = vec![vec![0f64; 6]; 7];
#     m2_rbm[0][3] = 1f64; // Segment #1: Rx
#     let dos_in = vec![jar::MCM2RB6D::io_with(
#         m2_rbm.into_iter().flatten().collect::<Vec<f64>>(),
#     )];
#     // DOS stepping
#     let dos_out = integrator.inputs(Some(dos_in))?.step()?.outputs();
#     // or equivalently:
#     // let dos_out = integrator.in_step_out(Some(dos_in))?;
#     println!("DOS output:");
#     println!("{:#?}", dos_out);
#     Ok(())
# }
```
Now we can write the main function:
```rust,ignore
# #[derive(Default)]
# pub struct Integrate {
#     /// integrator gain
#     gain: f64,
#     /// output vector
#     y: Vec<f64>,
#     /// input vector
#     u: Vec<f64>,
# }
# impl Integrate {
#     pub fn new(gain: f64, n_data: usize) -> Self {
#         Self {
#             gain,
#             y: vec![0f64; n_data],
#             ..Default::default()
#         }
#     }
#     pub fn output(&self) -> Option<Vec<f64>> {
#         Some(self.y.clone())
#     }
# }
# impl Iterator for Integrate {
#     type Item = ();
#     fn next(&mut self) -> Option<Self::Item> {
#         let gain = self.gain;
#         self.y.iter_mut().zip(self.u.iter()).for_each(|(a, v)| {
#             *a += *v * gain;
#         });
#         Some(())
#     }
# }
# 
# use dosio::{io::jar, DOSIOSError, Dos, IO};
# 
# impl Dos for Integrate {
#     type Input = Vec<f64>;
#     type Output = Vec<f64>;
#     fn inputs(&mut self, data: Option<Vec<IO<Self::Input>>>) -> Result<&mut Self, DOSIOSError> {
#         self.u = data
#             // Identifying M2 42 segment rigid body motion IO variant
#             .and_then(|data| {
#                 data.into_iter().find_map(|io| match io {
#                     IO::MCM2RB6D { data: value } => value,
#                     _ => None,
#                 })
#             })
#             // Extracting M2 14 Rx and Ry rigid body motions
#             .map(|x| {
#                 x.chunks(6)
#                     .flat_map(|x| x[3..5].to_vec())
#                     .collect::<Vec<f64>>()
#             })
#             // Throwing an error if IO::MCM2RB6D is missing
#             .ok_or_else(|| DOSIOSError::Inputs("Integrate MCM2RB6D IO input error".into()))?;
#         Ok(self)
#     }
#     fn outputs(&mut self) -> Option<Vec<IO<Self::Output>>> {
#         self.output()
#             // Assigning M2 segment Rx and Ry to rigid body motion vector
#             .map(|y| {
#                 y.chunks(2)
#                     .flat_map(|y| vec![0f64, 0f64, 0f64, y[0], y[1], 0f64])
#                     .collect::<Vec<f64>>()
#             })
#             // Embedding the rigid body motions into the corresponding IO variant
#             .map(|x| vec![jar::MCM2RB6D::io_with(x)])
#     }
# }
# 
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Integrator controller
    let mut integrator = Integrate::new(0.5, 14);
    // M2 rigid body motions to DOS input
    let mut m2_rbm = vec![vec![0f64; 6]; 7];
    m2_rbm[0][3] = 1f64; // Segment #1: Rx
    let dos_in = vec![jar::MCM2RB6D::io_with(
        m2_rbm.into_iter().flatten().collect::<Vec<f64>>(),
    )];
    // DOS stepping
    let dos_out = integrator.inputs(Some(dos_in))?.step()?.outputs();
    // or equivalently:
    // let dos_out = integrator.in_step_out(Some(dos_in))?;
    println!("DOS output:");
    println!("{:#?}", dos_out);
    Ok(())
}
```
