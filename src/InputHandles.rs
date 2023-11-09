use bevy::{prelude::*,window::PrimaryWindow};
use std::time::{Instant, Duration};

#[derive(Component)]
struct MousePressed(Instant);

#[derive(Event)]
struct MouseEvent{
    isHold:bool,
    pos:Vec2,
}

fn mouse_pressed(
    mut commands: Commands,
    input: Res<Input<MouseButton>>
){
    if input.just_pressed(MouseButton::Right){
        commands.spawn(MousePressed(Instant::now()));
    }
}

fn mouse_handle(
    mut commands: Commands,
    input: Res<Input<MouseButton>>,
    mut query : Query<(Entity, &MousePressed)>,
    mut mouseEvents : EventWriter<MouseEvent>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
){
    match q_windows.single().cursor_position(){
        Some(mut mousePos)=>{
            mousePos.x-=q_windows.single().width()*0.5;
            mousePos.y=q_windows.single().height()*0.5-mousePos.y;
            let now = Instant::now();
            for (ent,&ref pressed) in query.iter_mut(){
                let timePressed = pressed.0;
                if(now.duration_since(timePressed)<Duration::from_millis(200)){
                    for &released in input.get_just_released(){
                        mouseEvents.send(MouseEvent{isHold:false,pos:mousePos});
                        commands.entity(ent).despawn();
                    }
                }
                else{
                    mouseEvents.send(MouseEvent{isHold:true,pos:mousePos});
                    for &released in input.get_just_released(){
                        commands.entity(ent).despawn();
                    }
                }
            }
            
        }
        None =>{}
    }
    
}




