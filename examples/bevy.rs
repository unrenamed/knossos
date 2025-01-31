use bevy::{prelude::*, utils::HashMap};
use bevy_knossos::{maze::*, Coords, CoordsComponent};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    let maze = OrthogonalMazeBuilder::new()
        .algorithm(Box::new(RecursiveBacktracking))
        .width(5)
        .height(5)
        .build()
        .unwrap();

    App::new()
        .insert_resource(maze)
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, load_assets)
        .add_systems(PostStartup, setup.after(load_assets))
        .run();
}

#[derive(Clone, Debug, Reflect, Resource, Default)]
pub struct TilesHandles {
    map: HashMap<String, Handle<bevy::image::Image>>,
}

fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut tiles = TilesHandles::default();
    let images = vec![
        "tile_0000.png",
        "tile_0001.png",
        "tile_0010.png",
        "tile_0011.png",
        "tile_0100.png",
        "tile_0101.png",
        "tile_0110.png",
        "tile_0111.png",
        "tile_1000.png",
        "tile_1001.png",
        "tile_1010.png",
        "tile_1011.png",
        "tile_1100.png",
        "tile_1101.png",
        "tile_1110.png",
        "tile_1111.png",
    ];

    for image in images {
        let handle = asset_server.load(image);
        tiles.map.insert(image.to_string(), handle);
    }
    commands.insert_resource(tiles);
}

fn setup(mut commands: Commands, maze: Res<OrthogonalMaze>, tiles: Res<TilesHandles>) {
    commands.spawn((Camera2d, Name::new("Camera")));

    for (coords, cell) in maze.iter() {
        let bundle = load_image(&coords, cell, &tiles);
        commands.spawn(bundle);
    }
}

fn load_image(
    coords: &Coords,
    cell: &Cell,
    tiles: &Res<TilesHandles>,
) -> (CoordsComponent, Cell, Sprite, Name, Transform) {
    let cell_sprite = format!("tile_{}.png", cell.to_bits_str());
    let sprite = Sprite::from_image(
        tiles
            .map
            .get(&cell_sprite)
            .expect("All tiles should have been registered")
            .clone(),
    );
    let name = Name::new(format!("({},{}): {}", coords.0, coords.1, cell));
    let position = Transform::from_xyz(coords.0 as f32 * 45., (5 - coords.1) as f32 * 45., 0.)
        .with_scale(Vec3::from_slice(&[5., 5., 0.1]));

    (coords.to_owned().into(), *cell, sprite, name, position)
}
