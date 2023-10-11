use bevy::{gltf::Gltf, prelude::*};

pub struct ArmsPlugin;

impl Plugin for ArmsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, (setup_scene_once_loaded, load_arms))
            .add_systems(PostUpdate, follow_arms);
    }
}

#[derive(Resource)]
pub struct Animations(pub Vec<Handle<AnimationClip>>);

#[derive(Component)]
pub struct Arms;

#[derive(Resource)]
struct ArmsScene {
    handle: Handle<Gltf>,
    is_loaded: bool,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(Animations(vec![
        asset_server.load("arms_pistol.glb#Idle"),
        asset_server.load("arms_pistol.glb#Take"),
        asset_server.load("arms_pistol.glb#Hide"),
        asset_server.load("arms_pistol.glb#Shoot"),
        asset_server.load("arms_pistol.glb#Reload"),
        asset_server.load("arms_pistol.glb#RunStart"),
        asset_server.load("arms_pistol.glb#Run"),
        asset_server.load("arms_pistol.glb#RunEnd"),
    ]));

    commands.insert_resource(ArmsScene {
        handle: asset_server.load("arms_pistol.glb"),
        is_loaded: false,
    });
}

fn load_arms(
    mut commands: Commands,
    mut arms_scene: ResMut<ArmsScene>,
    gltf_assets: Res<Assets<Gltf>>,
) {
    if arms_scene.is_loaded {
        return;
    }

    let gltf = gltf_assets.get(&arms_scene.handle);

    const SCALE: f32 = 1.5;

    if let Some(gltf) = gltf {
        let scene = gltf.scenes.first().unwrap().clone();

        for node in scene.iter_fields().collect::<Vec<_>>() {
            println!("Node: {:?}", node);
        }

        commands.spawn((
            SceneBundle {
                scene,
                transform: Transform {
                    translation: Vec3::new(-25.0, -3.75, -29.0),
                    scale: Vec3::splat(SCALE),
                    ..Default::default()
                },
                ..default()
            },
            Arms,
            AnimationPlayer::default(),
        ));

        arms_scene.is_loaded = true;
    }
}

fn setup_scene_once_loaded(
    animations: Res<Animations>,
    mut players: Query<&mut AnimationPlayer, (With<Arms>, Added<AnimationPlayer>)>,
) {
    for mut player in players.iter_mut() {
        player.play(animations.0[1].clone_weak()).repeat();
    }
}

fn follow_arms(
    mut arms_query: Query<&mut Transform, With<Arms>>,
    logical_query: Query<&Transform, (With<crate::LogicalPlayer>, Without<Arms>)>,
    render_query: Query<&Transform, (With<crate::RenderPlayer>, Without<Arms>)>,
) {
    let logical_transform = logical_query.single();
    let render_transform = render_query.single();
    for mut arms_transform in arms_query.iter_mut() {
        arms_transform.translation = logical_transform.translation;
        arms_transform.rotation = render_transform.rotation;

        // NOTE: TEST
        // arms_transform.rotation = Quat::from_rotation_z(std::f32::consts::PI)
        //     * arms_transform.rotation
        //     * Quat::from_rotation_z(std::f32::consts::PI);
    }
}
