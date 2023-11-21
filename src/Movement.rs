use bevy::prelude::*;
use bevy::time::prelude;
#[derive(Component)]
struct Particle{
    prev_translation : Vec3,
    acceleration : Vec3
}

impl Default for Particle{
    fn default() -> Particle{
        Particle{
            prev_translation : Vec3::ZERO,
            acceleration : Vec3::ZERO
        }
    }
}

fn movement(
    mut particles : Query<(&mut Transform,&mut Particle)>
){
    for (mut transform,mut particle) in particles.iter_mut(){
        
        let vel = transform.translation-particle.prev_translation;
        particle.prev_translation=transform.translation;
        transform.translation += vel+particle.acceleration*0.002;
        //particle.acceleration=Vec3::ZERO;
        particle.acceleration=Vec3{x:0.0,y:-9.81,z:0.0};
    }
}