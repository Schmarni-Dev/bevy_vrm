use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_shader_mtoon::{MtoonMainCamera, MtoonSun};
use bevy_vrm::{loader::Vrm, BoneName, HumanoidBones, VrmBundle, VrmPlugin};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(AssetPlugin {
                file_path: "../../assets".to_string(),
                ..default()
            }),
            VrmPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, move_leg)
        .run();
}

const MODELS: [&str; 3] = ["catbot.vrm", "cool_loops.vrm", "suzuha.vrm"];
const PATH: &str = MODELS[2];

fn setup(
    mut commands: Commands,
    mut config: ResMut<GizmoConfigStore>,
    asset_server: Res<AssetServer>,
) {
    let (config, _) = config.config_mut::<DefaultGizmoConfigGroup>();
    config.depth_bias = -1.0;

    let mut transform = Transform::from_xyz(0.0, -1.0, -4.0);
    transform.rotate_y(PI);

    commands.spawn(VrmBundle {
        vrm: asset_server.load(PATH.to_string()),
        scene_bundle: SceneBundle {
            transform,
            ..default()
        },
        ..default()
    });

    commands.spawn((Camera3dBundle::default(), MtoonMainCamera));

    commands.spawn((
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                shadows_enabled: true,
                illuminance: 1000.0,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 5.0, -5.0),
            ..default()
        },
        MtoonSun,
    ));
}

fn move_leg(
    mut transforms: Query<&mut Transform>,
    time: Res<Time>,
    vrm: Query<&HumanoidBones, With<Handle<Vrm>>>,
) {
    for humanoid in vrm.iter() {
        let leg = match humanoid.0.get(&BoneName::RightUpperLeg) {
            Some(leg) => leg,
            None => continue,
        };

        if let Ok(mut transform) = transforms.get_mut(*leg) {
            let sin = time.elapsed_seconds().sin();
            transform.rotation = Quat::from_rotation_x(sin);
        }
    }
}

// fn draw_spring_bones(
//     mut gizmos: Gizmos,
//     spring_bones: Query<&SpringBones>,
//     transforms: Query<&GlobalTransform>,
// ) {
//     for spring_bones in spring_bones.iter() {
//         for spring_bone in spring_bones.0.iter() {
//             for bone_entity in spring_bone.bones.iter() {
//                 let position = transforms.get(*bone_entity).unwrap().translation();
//                 gizmos.sphere(
//                     position,
//                     Quat::default(),
//                     spring_bone.hit_radius,
//                     Color::rgb(10.0 / spring_bone.stiffiness, 0.2, 0.2),
//                 );
//             }
//         }
//     }
// }
