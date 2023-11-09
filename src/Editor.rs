use bevy::prelude::*;
include!("Circle.rs");
include!("Plane.rs");
include!("InputHandles.rs");
fn editor_inputs(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut clicked : EventReader<MouseEvent>,
    keys: Res<Input<KeyCode>>)
{
    for click in clicked.iter(){
        if(!click.isHold){
            spawn_object(&mut commands, & asset_server,click.pos, & keys);
        }
    } 
}

fn spawn_object(
    commands: &mut Commands, 
    asset_server: & Res<AssetServer>,
    pos : Vec2,
    keys: & Res<Input<KeyCode>>,
){
    if keys.pressed(KeyCode::Key1){
        commands.spawn(CircleBundle{
            spatial: SpatialBundle{
                visibility : Visibility::Visible,
                transform: Transform{
                    translation: Vec3 { x: pos.x, y: pos.y, z: (0.0) },
                    rotation:Quat::from_rotation_x(0.0), 
                    scale: Vec3 { x: (36.0), y: (36.0), z: (1.0) }},
                ..Default::default()
            },
            ..Default::default()
        }).with_children(|parent| {
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
                    scale: Vec3 { x: (216.0), y: (1.0), z: (1.0) }},
                ..Default::default()
            });
        });
    }
    
}