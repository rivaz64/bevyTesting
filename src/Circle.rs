use bevy::prelude::*;

#[derive(Component)]
struct Circle;



#[derive(Bundle)]
struct CircleBundle{
    marker : Circle,
    sprite : SpriteBundle
}

impl Default for CircleBundle{
    fn default() -> CircleBundle{
        CircleBundle{
            marker : Circle,
            sprite : Default::default()
        }
        
    }
}

fn spawn_cirlce(
    commands: &mut Commands, 
    asset_server: &Res<AssetServer>,
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

//fn add_circle(
//    mut commands: Commands, 
//    asset_server: Res<AssetServer>,
//    mut clicked : EventReader<MouseClicked>,
//    q_windows: Query<&Window, With<PrimaryWindow>>
//){
//    let width = q_windows.single().width()/2.0;
//    let height = q_windows.single().height()/2.0;
//    if let Some(mut position) = q_windows.single().cursor_position(){
//        for click in clicked.iter(){
//            position.y = -position.y;
//            spawn_cirlce(&mut commands,&asset_server,position+Vec2{x:-width,y:height});
//        }
//    }
//}