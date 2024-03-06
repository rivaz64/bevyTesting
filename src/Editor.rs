include!("circle.rs");
include!("plane.rs");
include!("movement.rs");
include!("grid.rs");
include!("input_handles.rs");
include!("grid_drawer.rs");
fn editor_inputs(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut clicked : EventReader<MouseEvent>,
    keys: Res<Input<KeyCode>>)
{
    for click in clicked.iter(){
        spawn_object(&mut commands, & asset_server,click.pos, & keys);
        /*if !click.isHold {
            spawn_object(&mut commands, & asset_server,click.pos, & keys);
        }
        else{

        }*/
    } 
}

fn spawn_object(
    commands: &mut Commands, 
    asset_server: & Res<AssetServer>,
    pos : Vec2,
    keys: & Res<Input<KeyCode>>,
){
    if keys.pressed(KeyCode::Key1) {
        commands.spawn((
            CircleBundle{
                spatial: SpatialBundle{
                    visibility : Visibility::Visible,
                    transform: Transform{
                        translation: Vec3 { x: pos.x, y: pos.y, z: (0.0) },
                        rotation:Quat::from_rotation_x(0.0), 
                        scale: Vec3 { x: (6.0), y: (6.0), z: (1.0) }},
                    ..Default::default()
                },
                ..Default::default()
            },
            Particle{
                prev_translation: Vec3 { x: pos.x, y: pos.y, z: (0.0) },
                ..Default::default()
            }
        )
        ).with_children(|parent| {
            parent.spawn(SpriteBundle{
                transform: Transform{
                    translation: Vec3 { x: (0.0), y: (0.0), z: (0.0) },
                    rotation:Quat::from_rotation_x(0.0), 
                    scale: Vec3 { x: (1.0/128.0), y: (1.0/128.0), z: (1.0) }},
                texture: asset_server.load("Circle.png"),
                ..Default::default()
            });
        });
    }
    if keys.pressed(KeyCode::Key2) {
        commands.spawn(PlaneBundle{
            spatial: SpatialBundle{
                visibility : Visibility::Visible,
                transform: Transform{
                    translation: Vec3 { x: pos.x, y: pos.y, z: (0.0) },
                    rotation:Quat::from_rotation_x(0.0), 
                    scale: Vec3 { x: (1.0), y: (1.0), z: (1.0) }},
                ..Default::default()
            },
            ..Default::default()
        }).with_children(|parent| {
            parent.spawn(SpriteBundle{
                transform: Transform{
                    translation: Vec3 { x: (0.0), y: (0.0), z: (0.0) },
                    rotation:Quat::from_rotation_x(0.0), 
                    scale: Vec3 { x: (1296.0), y: (1.0), z: (1.0) }},
                ..Default::default()
            });
        });
    }
    if keys.pressed(KeyCode::Key3) {
        let x = 6;
        let y = 6;
        let xf = 6.0;
        let yf = 6.0;
        commands.spawn(
            (
            GridDrawer{extent_x:6,extent_y:6},
            SpatialBundle{
                visibility : Visibility::Visible,
                transform: Transform{
                    translation: Vec3 { x: pos.x, y: pos.y, z: (0.0) },
                    rotation:Quat::from_rotation_x(0.0), 
                    scale: Vec3 { x: (6.0), y: (6.0), z: (1.0) }},
                ..Default::default()
            }),
            //Grid2d::<4>::new(6,6,collicion_sphere_sphere_,collicion_resolution_test)
        ).with_children(|parent| {
            for i in 0..(x+1){
                parent.spawn(SpriteBundle{
                    transform: Transform{
                        translation: Vec3 { x: (i as f32)-xf/2.0, y: (0.0), z: (0.0) },
                        rotation:Quat::from_rotation_x(0.0), 
                        scale: Vec3 { x: (1.0/6.0), y: yf, z: (1.0) }},
                    ..Default::default()
                });
            }
            for i in 0..(y+1){
                parent.spawn(SpriteBundle{
                    transform: Transform{
                        translation: Vec3 { x: (0.0), y: (i as f32)-yf/2.0, z: (0.0) },
                        rotation:Quat::from_rotation_x(0.0), 
                        scale: Vec3 { x: xf, y: (1.0/6.0), z: (1.0) }},
                    ..Default::default()
                });
            }
        });
    }
}