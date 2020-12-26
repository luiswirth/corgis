use bevy::prelude::*;

pub struct MyAssets {
    pub corgi_material: Handle<StandardMaterial>,
    pub corgi_mesh: Handle<Mesh>,
}

pub fn load_assets(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = Mesh::from(shape::Quad {
        size: Vec2::new(2.0, 2.0),
        flip: false,
    });

    let color = Color::rgb_linear(1.0, 0.0, 0.0);
    let material = StandardMaterial::from(color);

    let my_assets = MyAssets {
        corgi_mesh: meshes.add(mesh),
        corgi_material: materials.add(material),
    };

    commands.insert_resource(my_assets);
}
