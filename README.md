# orientation

This is a demonstration of real-time visualization of the attitude of a [BNO055](https://crates.io/crates/bno055) IMU across a wireless network to a Bevy app using the [Bissel](https://github.com/quietlychris/bissel) middleware. The IMU is reporting the orientation at a rate of 10 Hz. 

<p align="center"><img src="assets/orientation.gif" alt="screenshot" width="50%"/></p>

To enable quick builds, compile the `orientation` module using dynamic linking
```sh
$ cd orientation
$ RUST_LOG=OFF cargo run --features bevy/dynamic
```

For cross-compiling the BNO055 IMU interface code for the Odroid-C4 SBC:
```sh
$ cd interface
$ cross build --release --target aarch64-unknown-linux-gnu
$ scp target/aarch64-unknown-linux-gnu/release/interface user@sbc-ip:~
```
The `interface` executable should now present on the remote machine may need to be run as superuser at the start in order to access the IMU over I2C. In addition, the visualization app on the development computer should be running before starting the IMU interface code; the Bissel Node has a small grace period before the connection will time out, but will eventually error out if connection is not established relatively quickly. 

Also consider taking a look at: 
- [Bissel](https://github.com/quietlychris/bissel): The middleware being used to transmit state messages
- [Bevy](https://bevyengine.org): The ECS game engine used to create the orientation visualizer
- [Turtlesim](https://github.com/quietlychris/turtlesim): A ROS2 turtlesim clone built using the Bissel middleware and Bevy

## License

This project is licensed under the Mozilla Public LIcense, version 2.0 (MPL-2.0)

