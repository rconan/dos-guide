use dosio::{io::jar::*, io::IO, Dos};
use fem::{dos::DiscreteStateSpace, FEM};
use m1_ctrl as m1;
use mount_ctrl as mount;
//use serde_pickle as pkl;
use simple_logger::SimpleLogger;
use windloading::WindLoads;
//use std::error::Error;
use std::path::Path;
use std::time::Instant;
//use std::{fs::File, io::Error};

struct Timer {
    time: Instant,
}
impl Timer {
    pub fn tic() -> Self {
        Self {
            time: Instant::now(),
        }
    }
    pub fn toc(self) -> f64 {
        self.time.elapsed().as_secs_f64()
    }
    pub fn print_toc(self) {
        println!("... in {:3}s", self.toc());
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    SimpleLogger::new().init().unwrap();
    let fem_data_path = Path::new("data").join("20210225_1447_MT_mount_v202102_ASM_wind2");
    // WIND LOADS
    let tic = Timer::tic();
    // ANCHOR: windloads
    println!("Loading wind loads ...");
    let mut wind_loading = WindLoads::from_pickle(
        fem_data_path.join("b2019_0z_30az_os_7ms.wind_loads_1kHz_100-400s.pkl"),
    )?
    .range(0.0, 20.0)
    .truss()?
    .m2_asm_topend()?
    .m1_segments()?
    .m1_cell()?
    .m2_asm_reference_bodies()?
    .build()?;
    // ANCHOR_END: windloads
    tic.print_toc();

    // MOUNT CONTROL
    // ANCHOR: mount_control
    let mut mnt_drives = mount::drives::Controller::new();
    let mut mnt_ctrl = mount::controller::Controller::new();
    // ANCHOR_END: mount_control

    // M1
    // ANCHOR: m1_control
    let mut m1_hardpoints = m1::hp_load_cells::Controller::new();
    let mut m1_ctrl = m1::cg_controller::Controller::new();
    // ANCHOR_END: m1_control

    // FEM
    // ANCHOR: fem
    let sampling_rate = 1e3;
    let m1_rbm = OSSM1Lcl::io();
    let m2_rbm = MCM2RB6D::io();
    let tic = Timer::tic();
    println!("Building FEM dynamic model...");
    let mut fem = DiscreteStateSpace::from(FEM::from_pickle(
        fem_data_path.join("modal_state_space_model_2ndOrder.73.pkl"),
    )?)
    .dump_eigen_frequencies(fem_data_path.join("eigen_frequencies.pkl"))
    .sampling(sampling_rate)
    .proportional_damping(2. / 100.)
    .max_eigen_frequency(75.0)
    .inputs_from(&wind_loading)
    .inputs_from(&mnt_drives)
    .outputs(vec![m1_rbm, m2_rbm])
    .outputs(vec![
        OSSAzEncoderAngle::io(),
        OSSElEncoderAngle::io(),
        OSSRotEncoderAngle::io(),
    ])
    .outputs(vec![OSSHardpointD::io()])
    .build()?;
    // ANCHOR_END: fem
    tic.print_toc();

    // DATA LOGGING
    /*let mut data = DataLogging::new()
    .sampling_rate(sampling_rate)
    //.key(m1_rbm.clone())
    //.key(m2_rbm.clone())
    .build();*/

    // FEEDBACK LOOP
    println!("Running model ...");
    let tic = Timer::tic();
    // ANCHOR: feedback_loop
    let mut mount_drives_forces = Some(vec![
        OSSAzDriveTorque::io_with(vec![0f64; 12]),
        OSSElDriveTorque::io_with(vec![0f64; 4]),
        OSSRotDriveTorque::io_with(vec![0f64; 4]),
    ]);
    let mut m1_cg_fm: Option<Vec<IO<Vec<f64>>>> = None;
    let mut k = 0;
    while let Some(mut fem_forces) = wind_loading.outputs() {
        // FEM
        mount_drives_forces.as_mut().map(|x| {
            fem_forces.append(x);
        });
        m1_cg_fm.as_ref().map(|x| {
            fem_forces[OSSM1Lcl6F::io()] += &x[0];
            fem_forces[OSSCellLcl6F::io()] -= &x[0];
        });
        let fem_outputs = fem
            .in_step_out(Some(fem_forces))?
            .ok_or("FEM output is empty")?;
        // MOUNT CONTROLLER & DRIVES
        let mount_encoders = &fem_outputs[2..5];
        mount_drives_forces = mnt_ctrl
            .in_step_out(Some(mount_encoders.to_vec()))?
            .map(|mut x| {
                x.extend_from_slice(mount_encoders);
                mnt_drives.in_step_out(Some(x))
            })
            .unwrap()?;
        // M1 HARDPOINT & CG CONTROLLER
        if k % 10 == 0 {
            let mut m1_hp = vec![M1HPCmd::io_with(vec![0f64; 42])];
            m1_hp.extend_from_slice(&[fem_outputs[OSSHardpointD::io::<()>()].clone()]);
            m1_cg_fm = m1_ctrl.in_step_out(m1_hardpoints.in_step_out(Some(m1_hp))?)?;
        }
        k += 1;
    }
    // ANCHOR_END: feedback_loop
    tic.print_toc();

    // DATA LOGGING
    //data.step()?;
    //data.log(&fem_outputs[0])?.log(&fem_outputs[1])?;
    /*
    // OUTPUTS SAVING
    let mut f = File::create(fem_data_path.join("mount_control.data.pkl")).unwrap();
    pkl::to_writer(
        &mut f,
        &[
            data.time_series(m1_rbm),
            data.time_series(m2_rbm),
            //data.time_series(M1HPLC::new()),
            //data.time_series(M1CGFM::new()),
            //data.time_series(OSSAzEncoderAngle::new()),
            //data.time_series(OSSElEncoderAngle::new()),
            //data.time_series(OSSRotEncoderAngle::new()),
            //data.time_series(MountCmd::new()),
        ],
        true,
    )
    .unwrap();
     */
    Ok(())
}
