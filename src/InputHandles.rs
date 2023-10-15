use bevy::prelude::*;
use std::time::{Instant, Duration};

#[derive(Component)]
struct MousePressed(Instant);

#[derive(Event)]
struct MouseClick();

#[derive(Event)]
struct MouseDown();

fn mouse_pressed(
    mut commands: Commands,
    input: Res<Input<MouseButton>>
){
    for &MouseButton in input.get_just_pressed(){
        commands.spawn(MousePressed(Instant::now()));
    }
}

fn mouse_handle(
    mut commands: Commands,
    input: Res<Input<MouseButton>>,
    mut query : Query<(Entity, &MousePressed)>,
    mut clicked : EventWriter<MouseClick>,
    mut handle : EventWriter<MouseDown>
){
    let now = Instant::now();
    for (ent,&ref pressed) in query.iter_mut(){
        let timePressed = pressed.0;
        if(now.duration_since(timePressed)<Duration::from_millis(100)){
            for &released in input.get_just_released(){
                clicked.send(MouseClick());
                commands.entity(ent).despawn();
            }
        }
        else{
            handle.send(MouseDown());
            for &released in input.get_just_released(){
                commands.entity(ent).despawn();
            }
        }
    }
}




