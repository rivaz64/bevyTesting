

#[derive(Reflect)]
#[derive(Component)]
struct GridDrawer{
    extent_x : usize,
    extent_y : usize,
}

#[derive(Bundle)]
struct GridDrawerBundle{
    grid_drawer : GridDrawer,
    spatial : SpatialBundle
}

impl Default for GridDrawerBundle{
    fn default() -> GridDrawerBundle{
        GridDrawerBundle{
            grid_drawer : GridDrawer{extent_x:1,extent_y:1},
            spatial :  Default::default()
        }
    }
}

impl GridDrawer{
    fn create_grid(
        & self,
        ent: Entity,
        commands: &mut Commands,
    ){
        let x = self.extent_x;
        let y = self.extent_y;
        for i in 0..(x+1){
            let new_child = commands.spawn(SpriteBundle{
                transform: Transform{
                    translation: Vec3 { x: (i as f32)/*-(x as f32)/2.0*/, y: (y as f32)/2.0, z: (0.0) },
                    rotation:Quat::from_rotation_x(0.0), 
                    scale: Vec3 { x: (1.0/6.0), y: (y as f32), z: (1.0) }},
                ..Default::default()
            }).id();
            commands.entity(ent).push_children(&[new_child]);
        }
        for i in 0..(y+1){
            let new_child = commands.spawn(SpriteBundle{
                transform: Transform{
                    translation: Vec3 { x: (x as f32)/2.0, y: (i as f32)/*-(y as f32)/2.0*/, z: (0.0) },
                    rotation:Quat::from_rotation_x(0.0), 
                    scale: Vec3 { x: (x as f32), y: (1.0/6.0), z: (1.0) }},
                ..Default::default()
            }).id();
            commands.entity(ent).push_children(&[new_child]);
        }
    }
}