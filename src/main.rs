use avian2d::prelude::*;
use bevy::{
    camera::Viewport,
    color::palettes::{
        basic::WHITE,
        css::{GREEN, RED},
    },
    math::{VectorSpace, ops::powf},
    prelude::*,
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            MeshPickingPlugin,
            PhysicsPlugins::default(),
            PhysicsDebugPlugin,
        ))
        .insert_resource(Gravity(Vec2::ZERO))
        .add_systems(Startup, setup)
        .add_systems(Update, draw_cursor)
        .run();
}

fn draw_cursor(
    camera_query: Single<(&Camera, &GlobalTransform)>,
    window: Single<&Window>,
    mut gizmos: Gizmos,
) {
    let (camera, transform) = *camera_query;

    if let Some(cursor_position) = window.cursor_position()
        && let Ok(world_pos) = camera.viewport_to_world_2d(transform, cursor_position)
    {
        gizmos.circle_2d(world_pos, 10., WHITE);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Single<&Window>,
) {
    let window_size = window.resolution.physical_size().as_vec2();

    // initialise centered, non-window-filling viewport
    commands.spawn((
        Camera2d,
        Camera {
            viewport: Some(Viewport {
                physical_position: (window_size * 0.125).as_uvec2(),
                physical_size: (window_size * 0.75).as_uvec2(),
                ..default()
            }),
            ..default()
        },
    ));

    //Create a UI explaining shit
    commands.spawn((
        Text::new("mouse will follow the cursor dipshit"),
        Node {
            position_type: PositionType::Absolute,
            top: px(12),
            left: px(12),
            ..default()
        },
    ));

    // add a couple of playing cards
    let rectangle_mesh = Rectangle::new(50.0, 70.0);
    commands
        .spawn((
            RigidBody::Dynamic,
            Collider::rectangle(50.0, 70.0),
            LinearVelocity::ZERO,
            Mesh2d(meshes.add(rectangle_mesh)),
            MeshMaterial2d(materials.add(asset_server.load("A_spade.png"))),
        ))
        .observe(|event: On<Pointer<Over>>| println!("over!"))
        .observe(on_drag);
}

fn on_drag(
    mut event: On<Pointer<Drag>>,
    mut collider_query: Query<(&Transform, &mut LinearVelocity)>,
    window: Single<&Window>,
    camera_query: Single<(&Camera, &GlobalTransform)>,
) {
    let entity = event.entity;
    println!("{entity:?} was dragged");

    // get the camera (for cursor position) & get its transform (to convert this to world space)
    let (camera, camera_transform) = *camera_query;

    // get the collider's transform (to locate it relative to cursor) & and mut vel in order to affect it!
    let Ok((collider_transform, mut collider_linear_velocity)) = collider_query.get_mut(entity)
    else {
        return;
    };

    // get cursor position in screen space and convert to world space
    if let Some(cursor_screen_pos) = window.cursor_position()
        && let Ok(cursor_world_pos) =
            camera.viewport_to_world_2d(camera_transform, cursor_screen_pos)
    {
        let difference = cursor_world_pos - collider_transform.translation.truncate();

        // move object in that direction (to cursor)!
        collider_linear_velocity.0 = difference;
    }
}
