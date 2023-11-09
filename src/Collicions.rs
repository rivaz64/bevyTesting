use bevy::prelude::*;
fn collicionPointCircle(point: Vec3, circle:  Transform)->bool
{
    let size = circle.scale.x*0.5;
    (point-circle.translation).length_squared()<size*size
}

fn collicionCircleCircle(circle1:  Transform, circle2:  Transform)->bool
{
    let size = (circle1.scale.x+circle2.scale.x)*0.5;
    (circle2.translation-circle1.translation).length_squared()<size*size
}

fn collicionPlaneCirlce(circle:  Transform, plane:  Transform)->bool{
    circle.translation.dot(plane.forward())-plane.translation.length()<circle.scale.x*0.5
}
