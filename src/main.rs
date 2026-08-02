use avian2d::prelude::*;
use bevy::{color::palettes::basic::WHITE, prelude::*};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            MeshPickingPlugin,
            PhysicsPlugins::default(),
            PhysicsDebugPlugin,
        ))
        .insert_resource(Gravity(Vec2::ZERO))
        .add_systems(Startup, (setup, create_cards))
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

fn setup(mut commands: Commands) {
    // initialise centered, non-window-filling viewport
    commands.spawn((Camera2d,));

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
}

fn create_cards(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // card 1
    commands
        .spawn((
            Transform::from_xyz(-100.0, 0.0, 0.0),
            RigidBody::Dynamic,
            Collider::rectangle(CARD_W, CARD_H),
            LinearDamping(1.2),
            AngularDamping(1.2),
            Mesh2d(meshes.add(Rectangle::new(CARD_W, CARD_H))),
            MeshMaterial2d(materials.add(asset_server.load("A_spade.png"))),
        ))
        .observe(|event: On<Pointer<DragStart>>, mut commands: Commands| {
            commands.entity(event.entity).insert(Held);
        })
        .observe(|event: On<Pointer<DragEnd>>, mut commands: Commands| {
            commands.entity(event.entity).remove::<Held>();
        });

    // card 2
    commands
        .spawn((
            Transform::from_xyz(0.0, 0.0, 0.0),
            RigidBody::Dynamic,
            Collider::rectangle(CARD_W, CARD_H),
            LinearDamping(1.2),
            AngularDamping(1.2),
            Mesh2d(meshes.add(Rectangle::new(CARD_W, CARD_H))),
            MeshMaterial2d(materials.add(asset_server.load("A_spade.png"))),
        ))
        .observe(|event: On<Pointer<DragStart>>, mut commands: Commands| {
            commands.entity(event.entity).insert(Held);
        })
        .observe(|event: On<Pointer<DragEnd>>, mut commands: Commands| {
            commands.entity(event.entity).remove::<Held>();
        });

    //card 3
    commands
        .spawn((
            Transform::from_xyz(100.0, 0.0, 0.0),
            RigidBody::Dynamic,
            Collider::rectangle(CARD_W, CARD_H),
            LinearDamping(1.2),
            AngularDamping(1.2),
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
    mut collider_query: Query<
        (
            &Transform,
            &mut LinearVelocity,
            &Rotation,
            &mut AngularVelocity,
        ),
        With<Held>,
    >,
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
        for (
            collider_transform,
            mut collider_linear_velocity,
            collider_rotation,
            mut collider_angular_velocity,
        ) in collider_query.iter_mut()
        {
            {
                let linear_difference =
                    cursor_world_pos - collider_transform.translation.truncate();
                // move object in that direction (to cursor)!
                collider_linear_velocity.0 = linear_difference * DRAG_STIFFNESS;

                // return angle back to zero
                // rotation is in QUAT, whereas we just read the rotation directly and convert to radians (+- pi)
                let angular_difference = -collider_rotation.as_radians();
                collider_angular_velocity.0 = angular_difference * DRAG_STIFFNESS;
            }
        }
    }
}
