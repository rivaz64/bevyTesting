mod InputHandles;
mod Editor;

use bevy::prelude::*;



include!("Editor.rs");

fn inputTest(
    mut clicked : EventReader<MouseEvent>)
{
    for click in clicked.iter(){
        let mut ans = "".to_string();
        if(click.isHold){
            ans.push_str("holding at ");
        }
        else{
            ans.push_str("click at ");
        }
        println!("{}", ans+&click.pos.to_string());
    } 
}

fn add_cam(mut commands: Commands){
    commands.spawn(Camera2dBundle::default());
}

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, add_cam)
    .add_systems(Update,(mouse_pressed,mouse_handle,editor_inputs,inputTest))
    .add_event::<MouseEvent>()
    .run();
}
