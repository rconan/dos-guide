// ANCHOR: integrator
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

// ANCHOR_END: integrator
// ANCHOR: dos
use dosio::{io::jar, DOSIOSError, Dos, IO};

impl Dos for Integrate {
    type Input = Vec<f64>;
    type Output = Vec<f64>;
    fn inputs(&mut self, data: Option<Vec<IO<Self::Input>>>) -> Result<&mut Self, DOSIOSError> {
        self.u = data
            .map(|data| {
                data.into_iter().find_map(|io| match io {
                    IO::MCM2RB6D { data: value } => value,
                    _ => None,
                })
            })
            .flatten()
            .map(|x| {
                x.chunks(6)
                    .flat_map(|x| x[3..5].to_vec())
                    .collect::<Vec<f64>>()
            })
            .ok_or_else(|| DOSIOSError::Inputs("Integrate MCM2RB6D IO input error".into()))?;
        Ok(self)
    }
    fn outputs(&mut self) -> Option<Vec<IO<Self::Output>>> {
        self.output()
            .map(|y| {
                y.chunks(2)
                    .flat_map(|y| {
                        let mut data = vec![0f64; 6];
                        data[3] = y[0];
                        data[4] = y[1];
                        data
                    })
                    .collect::<Vec<f64>>()
            })
            .map(|x| vec![jar::MCM2RB6D::io_with(x)])
    }
}
// ANCHOR_END: dos

// ANCHOR: main
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
// ANCHOR_END: main
