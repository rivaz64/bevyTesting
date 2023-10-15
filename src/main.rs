mod InputHandles;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

include!("Circle.rs");
include!("InputHandles.rs");

fn inputTest(
    mut clicked : EventReader<MouseClick>,
    mut handle : EventReader<MouseDown>)
{
    for click in clicked.iter(){
        println!("clicked");
    } 
    for handl in handle.iter(){
        println!("handle");
    } 
}

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    //.add_systems(Startup, add_cam)
    .add_systems(Update,(mouse_pressed,mouse_handle,inputTest))
    .add_event::<MouseClick>()
    .add_event::<MouseDown>()
    .run();
}
