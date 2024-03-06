use bevy::{prelude::*};
use bevy::time::fixed_timestep::*;
//use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
include!("Editor.rs");
fn input_test(
    mut clicked : EventReader<MouseEvent>)
{
    for click in clicked.iter(){
        let mut ans = "".to_string();
        if click.is_hold{
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
    mut circles : Query<&mut Transform,(With<Circle>,Without<Plane>,Without<Grid2d<4>>)>,
    planes : Query<&mut Transform,(With<Plane>,Without<Circle>,Without<Grid2d<4>>)>,
    mut grids : Query<(&mut Transform,&mut Grid2d<4>),(Without<Plane>,Without<Circle>)>
)
{
    //let g = Grid2d::<4>::new(6,6,collicion_sphere_sphere_,collicion_resolution_test);
        
    /*let mut combinations = circles.iter_combinations_mut();
    while let Some([mut a, mut b]) = combinations.fetch_next() {
        collicion_sphere_sphere_(a.as_mut(), b.as_mut(),collicion_resolution_test);
    }*/
    //println!(grids.);
    for (transform,mut grid) in grids.iter_mut(){
        for mut cirlce in circles.iter_mut(){
            let dif = cirlce.as_ref().translation-transform.translation;
            unsafe{grid.insert((dif.x/transform.scale.x).floor() as usize, (dif.y/transform.scale.y).floor() as usize, &mut*cirlce);}
        }
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

fn frames(time: Res<Time>,){
    println!("{}",time.delta_seconds().to_string())
}
fn update_grid_drawers(
    mut commands: Commands,
    mut grids : Query<(Entity,&Children,&mut Transform,&mut GridDrawer)>,
    mut clicked : EventReader<MouseEvent>,
    keys: Res<Input<KeyCode>>
){
    for (ent,children, mut transform,mut grid) in grids.iter_mut(){
        if grid.extent_x+grid.extent_y+2!=children.len(){
            commands.entity(ent).remove_children(children);
            for child in children{
                commands.entity(*child).despawn();
            }
            grid.create_grid(ent, &mut commands);
        }
        for click in clicked.iter(){
            if keys.pressed(KeyCode::Key4){
                commands.spawn(//Grid2dBundle::<4>
                    (
                        /*grid:*/ Grid2d::<4>::new(grid.extent_x,grid.extent_y,collicion_sphere_sphere_,collicion_resolution_test),
                        /*spatial:*/ SpatialBundle{
                            visibility : Visibility::Visible,
                            transform: *transform,
                            ..Default::default()
                        }
                    )
                ); 
            }
        }
    }
    
}


fn main() {
    //FixedTime = 1;
    App::new()
    .add_plugins(DefaultPlugins)
    //.add_plugins(EguiPlugin)
    //.add_plugins(WorldInspectorPlugin::new()).register_type::<GridDrawer>()
    .add_systems(Startup, add_cam)
    .add_systems(Update,(mouse_pressed,mouse_handle,editor_inputs,/*frames*/))
    //.add_systems(FixedUpdate,(collicion_test,movement,update_grid_drawers))
    //.insert_resource(FixedTime::new_from_secs(0.02))
    .add_event::<MouseEvent>()
    .run();
}
