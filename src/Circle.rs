use bevy::prelude::*;

#[derive(Component)]
struct Circle;

#[derive(Bundle)]
struct CircleBundle{
    marker : Circle,
    spatial : SpatialBundle
}

impl Default for CircleBundle{
    fn default() -> CircleBundle{
        CircleBundle{
            marker : Circle,
            spatial :  Default::default()
        }
    }
}