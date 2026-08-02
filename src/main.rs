use avian2d::prelude::*;
use bevy::{camera::Viewport, color::palettes::basic::WHITE, prelude::*};

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
        .add_systems(Update, (draw_cursor, follow_cursor))
        .run();
}

const CARD_W: f32 = 50.0;
const CARD_H: f32 = 70.0;
const DRAG_STIFFNESS: f32 = 15.0;

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
        Text::new("pick up the card"),
        Node {
            position_type: PositionType::Absolute,
            top: px(12),
            left: px(12),
            ..default()
        },
    ));

    commands
        .spawn((
            RigidBody::Dynamic,
            Collider::rectangle(CARD_W, CARD_H),
            Mesh2d(meshes.add(Rectangle::new(CARD_W, CARD_H))),
            MeshMaterial2d(materials.add(asset_server.load("A_spade.png"))),
        ))
        .observe(|event: On<Pointer<DragStart>>, mut commands: Commands| {
            commands.entity(event.entity).insert(Held);
        })
        .observe(|event: On<Pointer<DragEnd>>, mut commands: Commands| {
            commands.entity(event.entity).remove::<Held>();
        });
}

#[derive(Component)]
struct Held;

// any item with "Held" should follow the cursor
fn follow_cursor(
    mut collider_query: Query<(&Transform, &mut LinearVelocity), With<Held>>,
    window: Single<&Window>,
    camera_query: Single<(&Camera, &GlobalTransform)>,
) {
    // get the camera (for cursor position) & get its transform (to convert this to world space)
    let (camera, camera_transform) = *camera_query;

    // get cursor position in screen space and convert to world space
    if let Some(cursor_screen_pos) = window.cursor_position()
        && let Ok(cursor_world_pos) =
            camera.viewport_to_world_2d(camera_transform, cursor_screen_pos)
    {
        // collider_query may have many entities being held
        for (collider_transform, mut collider_linear_velocity) in collider_query.iter_mut() {
            {
                let difference = cursor_world_pos - collider_transform.translation.truncate();

                // move object in that direction (to cursor)!
                collider_linear_velocity.0 = difference * DRAG_STIFFNESS;
            }
        }
    } else {
        return;
    };
}
