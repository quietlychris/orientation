use bevy::prelude::*;
use bissel::{Host as BisselHost, Node as BisselNode, *};

#[derive(Debug, Component)]
pub struct Host(pub BisselHost);
#[derive(Debug, Component)]
pub struct Node<T: Message>(pub BisselNode<Subscription, T>);

#[derive(Component, Default)]
struct Cube;

pub const BISSEL_ADDR: &str = "192.168.8.105:25000";

fn main() {
    App::new()
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 5.0f32,
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(bissel_host)
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

    if let Ok(Some(quat)) = imu_recv_node.0.get_subscribed_data() {
        global_transform.rotation = quat;
    }
}

fn bissel_host(mut commands: Commands) {
    // Setup our Bissel host on WiFi
    // The network interface of your WiFi adapter may be different; use `ifconfig` to check
    let bissel_host: BisselHost = HostConfig::new("wlp3s0")
        .socket_num(25_000) // Port 25000 is the default address
        .store_filename("store") // sled DBs allow persistence across reboots
        .build()
        .expect("Couldn't create a Host");

    let mut host = Host(bissel_host);
    host.0.start().unwrap();

    commands.spawn().insert(host);
}

// Create a node for managing the IMU
fn imu_recv_node(mut commands: Commands) {
    // Sleep for a second while setting up to allow the Host to fully get setup
    std::thread::sleep(std::time::Duration::from_millis(500));
    let addr = BISSEL_ADDR.parse::<std::net::SocketAddr>().unwrap();
    let bissel_node = NodeConfig::<Quat>::new("IMU_SUBSCRIPTION")
        .topic("quaternion")
        .host_addr(addr)
        .build()
        .unwrap()
        .subscribe(std::time::Duration::from_millis(50)) // Run subscriber at 20 Hz
        .unwrap();
    // Wrap our Bissel node in the NewType pattern for Bevy
    let imu_node = Node(bissel_node);
    // Each node establishes a TCP connection with central host
    println!("IMU_RECV connected");

    commands.spawn().insert(imu_node);
}
