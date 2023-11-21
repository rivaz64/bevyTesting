use bevy::prelude::*;

struct Collicion{
    point : Vec3,
    normal : Vec3,
    depth : f32
}

type Overlap = fn(Transform,Transform) -> bool;
type CollicionDynamic = fn(Transform,Transform) -> Option<(Collicion,Collicion)>;
type CollicionStatic = fn(Transform,Transform) -> Option<Collicion>;
type CallbackDynamic = fn(&mut Transform,&mut Transform,Collicion,Collicion);
type CallbackStatic = fn(&mut Transform,Collicion);
type CollicionDynamicCallback = fn(&mut Transform,&mut Transform,CallbackDynamic);
type CollicionStaticCallback = fn(Transform,Transform,CallbackStatic);

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
        Some((Collicion{normal: -normal,point:sphere1.translation+normal*sphere1.scale.x,depth:depth},
            Collicion{normal: normal,point:sphere2.translation-normal*sphere2.scale.x,depth:depth}))
    }
}

fn collicion_sphere_sphere_(
    sphere1: &mut Transform, 
    sphere2: &mut Transform, 
    callback: fn(&mut Transform,&mut Transform,Collicion,Collicion)
)
{
    let size = (sphere1.scale.x+sphere2.scale.x)*0.5;
    let distance_sqrd = (sphere2.translation-sphere1.translation).length_squared();
    if distance_sqrd<size*size{
        let normal = (sphere2.translation-sphere1.translation).normalize();
        let depth = size-distance_sqrd.sqrt();
        callback(sphere1,sphere2,Collicion{normal: -normal,point:sphere1.translation+normal*sphere1.scale.x,depth:depth},
            Collicion{normal: normal,point:sphere2.translation-normal*sphere2.scale.x,depth:depth});
    }
}

fn overlap_sphere_plane(sphere:  Transform, plane:  Transform)->bool{
    sphere.translation.dot(plane.up())-plane.translation.dot(plane.up())<sphere.scale.x*0.5
}

/*fn collicion_sphere_plane(sphere:  Transform, plane:  Transform)->Option<Collicion>
{
    let plane_normal = plane.up();
    let distance = sphere.translation.dot(plane_normal)-plane.translation.dot(plane_normal);
    if !(distance<sphere.scale.x*0.5){
        None
    }
    else{
        Some(Collicion{normal: plane_normal,point:sphere.translation-plane_normal*distance,depth:radius-distance})
    }
}*/

fn collicion_sphere_plane_(
    sphere: &mut Transform, 
    plane:  Transform,
    callback: fn(&mut Transform,Collicion)
)
{
    let plane_normal = plane.up();
    let distance = sphere.translation.dot(plane_normal)-plane.translation.dot(plane_normal);
    let radius = sphere.scale.x*0.5;
    if distance<radius{
        callback(sphere,Collicion{normal: plane_normal,point:sphere.translation-plane_normal*distance,depth:radius-distance});
    }
}
