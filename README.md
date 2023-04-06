# orientation

This is a demonstration of real-time visualization of the attitude of a [BNO055](https://crates.io/crates/bno055) IMU across a wireless network to a Bevy app using the [meadow](https://github.com/quietlychris/meadow) middleware. The IMU is reporting the orientation at a rate of 10 Hz. 

<p align="center"><img src="assets/orientation.gif" alt="screenshot" width="50%"/></p>

The display code can be run using
```sh
$ cd display
$ cargo run
```
which needs to happen prior to running the publisher on the remote side. 

For cross-compiling the BNO055 IMU interface code for the Odroid-C4 SBC:
```sh
$ cd remote
$ cross build --release # We have a Cross.toml file specifying the aarch64-unknown-linux-gnu
$ scp target/aarch64-unknown-linux-gnu/release/remote user@sbc-ip:~
```
The `interface` executable should now present on the remote machine may need to be run as superuser at the start in order to access the IMU over I2C. In addition, the visualization app on the development computer should be running before starting the IMU interface code; the meadow Node has a small grace period before the connection will time out, but will eventually error out if connection is not established relatively quickly. 

## Resources

Also, consider checking out the following: 
- [meadow](https://github.com/quietlychris/meadow): The middleware being used to transmit state messages
- [Bevy](https://bevyengine.org): The ECS game engine used to create the orientation visualizer
- [Turtlesim](https://github.com/quietlychris/turtlesim): A ROS2 turtlesim clone built using the meadow middleware and Bevy

## License

This project is licensed under the Mozilla Public License, version 2.0 (MPL-2.0)

