use bno055::{BNO055OperationMode, Bno055};
use glam::{f32::*, EulerRot};
use linux_embedded_hal::{Delay, I2cdev};
use meadow::*;
use mint::EulerAngles;

fn main() {
    let dev = I2cdev::new("/dev/i2c-0").unwrap();
    let mut delay = Delay {};
    let mut imu = Bno055::new(dev).with_alternative_address();
    imu.init(&mut delay)
        .expect("An error occurred while building the IMU");

    imu.set_mode(BNO055OperationMode::NDOF, &mut delay)
        .expect("An error occurred while setting the IMU mode");

    // Calibrate the IMU; this doesn't have to be done every time
    // Follow the instructions as linked below, involving slowly rotating the IMU between
    // appx. six different stable positions
    // To run the calibration procedure, uncomment the following code block
    /*
    let mut status = imu.get_calibration_status().unwrap();
    println!("The IMU's calibration status is: {:?}", status);

    // Wait for device to auto-calibrate.
    // Please perform steps necessary for auto-calibration to kick in.
    // Required steps are described in Datasheet section 3.11
    // Page 51, https://www.bosch-sensortec.com/media/boschsensortec/downloads/datasheets/bst-bno055-ds000.pdf (As of 2021-07-02)

    println!("- About to begin BNO055 IMU calibration...");
    while !imu.is_fully_calibrated().unwrap() {
        status = imu.get_calibration_status().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(1000));
        println!("Calibration status: {:?}", status);
    }


    let calib = imu.calibration_profile(&mut delay).unwrap();

    imu.set_calibration_profile(calib, &mut delay).unwrap();
    println!("       - Calibration complete!");
    */
    println!("Connected to IMU! First reading: {:?}", imu.euler_angles());

    let host_addr = "192.168.8.122:25000"
        .parse::<std::net::SocketAddr>()
        .unwrap();

    let imu_node = NodeConfig::<Quat>::new("IMU_TX_NODE")
        .with_udp_config(Some(node::UdpConfig::default().set_host_addr(host_addr)))
        .with_tcp_config(None)
        .topic("quaternion")
        .build()
        .unwrap()
        .activate()
        .unwrap();

    // These are sensor fusion reading using the mint crate that the state will be read into
    // We'll initialize our EulerAngle variable before the loop, but won't assign data yet

    let mut euler: EulerAngles<f32, ()>;
    loop {
        match imu.euler_angles() {
            Ok(val) => {
                euler = val;
                // Convert the Euler angles into proper form for the quaternion
                let (a, b, c) = (
                    euler.a.to_radians(),
                    euler.b.to_radians(),
                    euler.c.to_radians(),
                );
                // Adjust the reference frame to match that of the simulator
                // This is why we're starting with the Euler Angles instead of
                // directly reading the quaternion; it's easier for people to reason about Euler Angles
                // than it is about Quaternions, which are a fairly complex mathematical construct
                // However, since Bevy uses the Quaternion, we'll do the conversion before sending it over
                let quat: Quat = Quat::from_euler(EulerRot::XYZ, -a, b, c);
                // We *don't* want to stop publishing if there's an error
                match imu_node.publish_udp(quat) {
                    Ok(_) => (),
                    Err(e) => {
                        eprintln!("Error: {:?}", e);
                        continue;
                    }
                };

                println!("IMU Euler angles: {:?}", euler);
            }
            Err(e) => {
                eprintln!("{:?}", e);
            }
        }
    }
}
