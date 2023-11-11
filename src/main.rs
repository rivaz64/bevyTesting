mod InputHandles;
mod Editor;
mod Collicions;
mod Movement;

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

fn collicion_resolution_test(
    sphere1: &mut Transform, 
    sphere2: &mut Transform, 
    collicion1: Collicion,
    collicion2: Collicion,
){
    sphere1.translation += collicion1.normal*collicion1.depth;
    sphere2.translation += collicion2.normal*collicion2.depth;
}

fn collicion_resolution_test2(
    sphere: &mut Transform, 
    collicion: Collicion,
){
    sphere.translation += collicion.normal*collicion.depth;
}

fn collicion_test(
    mut circles : Query<&mut Transform,(With<Circle>,Without<Plane>)>,
    planes : Query<&mut Transform,(With<Plane>,Without<Circle>)>,
)
{
    let mut combinations = circles.iter_combinations_mut();
    while let Some([mut a, mut b]) = combinations.fetch_next() {
        collicion_sphere_sphere_(a.as_mut(), b.as_mut(),collicion_resolution_test);
    }
    for mut cirlce in circles.iter_mut(){
        for plane in planes.iter(){
            collicion_sphere_plane_(cirlce.as_mut(), *plane, collicion_resolution_test2);
        }
    }
}

fn add_cam(mut commands: Commands){
    commands.spawn(Camera2dBundle::default());
}

fn main() {
    //FixedTime = 1;
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(EguiPlugin)
    .add_plugins(WorldInspectorPlugin::new())
    .add_systems(Startup, add_cam)
    .add_systems(Update,(mouse_pressed,mouse_handle,editor_inputs))
    .add_systems(FixedUpdate,(collicion_test,movement))
    .insert_resource(FixedTime::new_from_secs(0.02)) 
    .add_event::<MouseEvent>()
    .run();
}
