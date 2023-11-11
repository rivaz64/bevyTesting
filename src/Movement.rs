use bevy::prelude::*;
use bevy::time::prelude;
#[derive(Component)]
struct Particle{
    prevTranslation : Vec3,
    acceleration : Vec3
}

impl Default for Particle{
    fn default() -> Particle{
        Particle{
            prevTranslation : Vec3::ZERO,
            acceleration : Vec3::ZERO
        }
    }
}

fn movement(
    time: Res<Time>,
    mut particles : Query<(&mut Transform,&mut Particle)>
){
    for (mut transform,mut particle) in particles.iter_mut(){
        
        let vel = transform.translation-particle.prevTranslation;
        particle.prevTranslation=transform.translation;
        let delta = time.delta_seconds();
        println!("{}",(vel*delta+particle.acceleration*delta*delta*0.5).to_string());
        transform.translation += vel+particle.acceleration*0.002;
        //particle.acceleration=Vec3::ZERO;
        particle.acceleration=Vec3{x:0.0,y:-9.81,z:0.0};
    }
}