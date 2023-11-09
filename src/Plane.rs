use bevy::prelude::*;

#[derive(Component)]
struct Plane;

#[derive(Bundle)]
struct PlaneBundle{
    marker : Plane,
    spatial : SpatialBundle
}

impl Default for PlaneBundle{
    fn default() -> PlaneBundle{
        PlaneBundle{
            marker : Plane,
            spatial :  Default::default()
        }
    }
}