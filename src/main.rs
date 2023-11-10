mod InputHandles;
mod Editor;
mod Collicions;

use bevy::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin,quick::ResourceInspectorPlugin};
use bevy_inspector_egui::prelude::*;
include!("Editor.rs");
include!("Collicions.rs");

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

fn collicion_test(
    circles : Query<&mut Transform,(With<Circle>,Without<Plane>)>,
    planes : Query<&mut Transform,(With<Plane>,Without<Circle>)>,
)
{
    /*let mut combinations = query.iter_combinations_mut();
    while let Some([mut a, mut b]) = combinations.fetch_next() {
        let circle1 = a.into_inner();
        let circle2 = b.into_inner();
        if collicionCircleCircle(*circle1, *circle2){
            println!("collicion");
        }
    }*/
    for cirlce in circles.iter(){
        for plane in planes.iter(){
            if collicionPlaneCirlce(*cirlce,*plane){
                println!("collicion");
            }
        }
    }
}

fn add_cam(mut commands: Commands){
    commands.spawn(Camera2dBundle::default());
}

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(EguiPlugin)
    .add_plugins(WorldInspectorPlugin::new())
    .add_systems(Startup, add_cam)
    .add_systems(Update,(mouse_pressed,mouse_handle,editor_inputs,collicion_test))
    .add_event::<MouseEvent>()
    .run();
}
