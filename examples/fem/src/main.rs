use dosio::{io::jar, Dos};
use fem::{dos::DiscreteStateSpace, FEM};
use simple_logger::SimpleLogger;
use std::path::Path;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    SimpleLogger::new().init()?;
    // ANCHOR: fem
    let fem_data_path = Path::new("data").join("20210225_1447_MT_mount_v202102_ASM_wind2");
    let sampling_rate = 1e3;
    let m1_rbm_io = jar::OSSM1Lcl::io();
    let m2_rbm_io = jar::MCM2RB6D::io();
    let mut fem = {
        let fem = FEM::from_pickle(fem_data_path.join("modal_state_space_model_2ndOrder.73.pkl"))?;
        DiscreteStateSpace::from(fem)
    }
    .sampling(sampling_rate)
    .proportional_damping(2. / 100.)
    .max_eigen_frequency(75.0)
    .inputs(vec![jar::OSSM1Lcl6F::io()])
    .outputs(vec![m1_rbm_io.clone(), m2_rbm_io.clone()])
    .build()?;
    println!(
        "Dynamic discrete FEM: {} 2x2 state space model",
        fem.state_space.len()
    );
    // ANCHOR_END: fem

    let m1_wind_loads = {
        let mut u = vec![0f64; 42];
        u.chunks_mut(6).for_each(|u| u[0] = 100f64);
        jar::OSSM1Lcl6F::io_with(u)
    };

    // ANCHOR: fem_dynamic
    // vector norm
    let norm = |x: Option<Vec<f64>>| -> f64 {
        x.as_ref()
            .unwrap()
            .iter()
            .fold(0., |s, &x| s + x * x)
            .sqrt()
    };

    let now = Instant::now();
    println!("{:^03} {:^8} {:^8}", "#", "M1", "M2");
    for k in 0..20 {
        let y = fem.in_step_out(Some(vec![m1_wind_loads.clone()]))?.unwrap();
        let m1_rbm = norm((&y[m1_rbm_io.clone()]).into());
        let m2_rbm = norm((&y[m2_rbm_io.clone()]).into());
        println!("{:<03} {:<8.3e} {:<8.3e}", k, m1_rbm, m2_rbm);
    }
    println!("Elapsed time: {:}ms", now.elapsed().as_millis());
    // ANCHOR_END: fem_dynamic

    Ok(())
}
