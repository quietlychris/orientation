use bevy::prelude::*;
use meadow::{Host as MeadowHost, Node as MeadowNode, *};

#[derive(Debug, Component)]
pub struct Host(pub MeadowHost);
#[derive(Debug, Component)]
pub struct Node<T: Message>(pub MeadowNode<Subscription, T>);

#[derive(Component, Default)]
struct Cube;

pub const MEADOW_ADDR: &str = "192.168.8.105:25000";

fn main() {
    App::new()
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 5.0f32,
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(meadow_host)
        .add_startup_system(imu_recv_node)
        .add_system(rotate_cube)
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}

fn setup(
    mut commands: Commands,
    //asset_server: Res<AssetServer>
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // We could load a .gltf file from a CAD program
    //commands.spawn_scene(asset_server.load("models/FlightHelmet/FlightHelmet.gltf#Scene0"));

    let rectangle: shape::Box = shape::Box {
        min_x: -1.5,
        max_x: 1.5,
        min_y: -1.5,
        max_y: 1.5,
        min_z: -0.1,
        max_z: 0.1,
    };

    // Physically-based rendering object
    // This is what will show up in our simulator!
    // Yes, it's technically a rectanglular prism, but that would mean a longer variable name ¯\_(ツ)_/¯
    let cube = PbrBundle {
        mesh: meshes.add(Mesh::from(rectangle)),
        material: materials.add(StandardMaterial {
            base_color: Color::hex("cc0000").unwrap(),
            // vary key PBR parameters on a grid of spheres to show the effect
            metallic: 0.1,
            perceptual_roughness: 0.1,
            ..Default::default()
        }),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    };

    commands.spawn_bundle(cube).insert(Cube);

    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(6.0, 0.0, 0.0)
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0)),
        ..Default::default()
    });

    // ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 5.0,
    });
}

fn rotate_cube(
    // time: Res<Time>,
    mut query: Query<(&mut Cube, &mut Transform, &mut GlobalTransform)>,
    mut imu_node_query: Query<&mut Node<Quat>>,
) {
    // Since we're directly reading/writing the GlobalTransform, we don't need
    // the timestep; otherwise, this would be useful
    // let delta = time.delta_seconds();
    let (_cube, _transform, mut global_transform) = query.single_mut();
    let imu_recv_node = imu_node_query.single_mut();

    if let Ok(quat) = imu_recv_node.0.get_subscribed_data() {
        global_transform.rotation = quat;
    }
}

fn meadow_host(mut commands: Commands) {
    // Setup our meadow host on WiFi
    // The network interface of your WiFi adapter may be different; use `ifconfig` to check
    let meadow_host: MeadowHost = HostConfig::default()
        .with_udp_config(Some(host::UdpConfig::default("wlp5s0")))
        .build()
        .expect("Couldn't create a Host");

    let mut host = Host(meadow_host);
    host.0.start().unwrap();

    commands.spawn().insert(host);
}

// Create a node for managing the IMU
fn imu_recv_node(mut commands: Commands) {
    // Sleep for a second while setting up to allow the Host to fully get setup
    std::thread::sleep(std::time::Duration::from_millis(500));
    let meadow_node = NodeConfig::<Quat>::new("IMU_SUBSCRIPTION")
        .topic("quaternion")
        .build()
        .unwrap()
        .subscribe(std::time::Duration::from_millis(50)) // Run subscriber at 20 Hz
        .unwrap();
    // Wrap our meadow node in the NewType pattern for Bevy
    let imu_node = Node(meadow_node);
    // Each node establishes a TCP connection with central host
    println!("IMU_RECV connected");

    commands.spawn().insert(imu_node);
}
