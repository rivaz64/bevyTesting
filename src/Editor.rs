use bevy::prelude::*;
include!("Circle.rs");
include!("InputHandles.rs");
fn editor_inputs(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut clicked : EventReader<MouseEvent>)
{
    for click in clicked.iter(){
        if(!click.isHold){
            spawn_object(&mut commands, & asset_server,click.pos);
        }
    } 
}

fn spawn_object(
    commands: &mut Commands, 
    asset_server: & Res<AssetServer>,
    pos : Vec2
){
    commands.spawn(CircleBundle{
        sprite: SpriteBundle{
            transform: Transform{
                translation: Vec3 { x: pos.x, y: pos.y, z: (0.0) },
                rotation:Quat::from_rotation_x(0.0), 
                scale: Vec3 { x: (36.0/128.0), y: (36.0/128.0), z: (1.0) }},
            texture: asset_server.load("Circle.png"),
            ..Default::default()
        },
        ..Default::default()
    });
}