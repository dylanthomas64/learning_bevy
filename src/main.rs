use bevy::{
    camera::Viewport,
    color::palettes::{
        basic::WHITE,
        css::{GREEN, RED},
    },
    math::ops::powf,
    prelude::*,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
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
        Text::new( "mouse will follow the cursor dipshit"),
        Node {
            position_type: PositionType::Absolute,
            top: px(12),
            left: px(12),
            ..default()
        },
    ));


    // add a couple of playing cards

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(50.0, 70.0))),
        MeshMaterial2d(materials.add(asset_server.load("A_spade.png"))),
    ));


}