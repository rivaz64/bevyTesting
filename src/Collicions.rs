use bevy::prelude::*;

struct Collicion{
    point : Vec3,
    normal : Vec3,
    depth : f32
}

fn overlap_point_sphere(point: Vec3, sphere:  Transform)->bool
{
    let size = sphere.scale.x*0.5;
    (point-sphere.translation).length_squared()<size*size
}

fn overlap_sphere_sphere(sphere1:  Transform, sphere2:  Transform)->bool
{
    let size = (sphere1.scale.x+sphere2.scale.x)*0.5;
    (sphere2.translation-sphere1.translation).length_squared()<size*size
}

fn collicion_sphere_sphere(sphere1:  Transform, sphere2:  Transform)->Option<(Collicion,Collicion)>
{
    let size = (sphere1.scale.x+sphere2.scale.x)*0.5;
    let distance_sqrd = (sphere2.translation-sphere1.translation).length_squared();
    if !(distance_sqrd<size*size){
        None
    }
    else{
        let normal = (sphere2.translation-sphere1.translation).normalize();
        let depth = size-distance_sqrd.sqrt();
        Some((Collicion{normal: normal,point:sphere1.translation+normal*sphere1.scale.x,depth:depth},
            Collicion{normal: -normal,point:sphere2.translation-normal*sphere2.scale.x,depth:depth}))
    }
}

fn overlap_sphere_plane(sphere:  Transform, plane:  Transform)->bool{
    (sphere.translation.dot(plane.up())-plane.translation.dot(plane.up())).abs()<sphere.scale.x*0.5
}
