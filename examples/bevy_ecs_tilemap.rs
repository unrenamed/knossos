use bevy::{prelude::*, utils::HashMap};
use bevy_ecs_tilemap::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_knossos::{
    maze::{self, Cell},
    KnossosPlugin,
};

const MAZE_SIZE: u32 = 32;
fn main() {
    let maze = maze::OrthogonalMazeBuilder::new()
        .algorithm(Box::new(maze::RecursiveBacktracking))
        .width(MAZE_SIZE as usize)
        .height(MAZE_SIZE as usize)
        .build()
        .unwrap();

    App::new()
        .insert_resource(maze)
        .add_plugins((DefaultPlugins, TilemapPlugin))
        .add_plugins((KnossosPlugin, WorldInspectorPlugin::new()))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, maze: Res<maze::OrthogonalMaze>, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera2d,
        OrthographicProjection {
            scaling_mode: bevy::render::camera::ScalingMode::AutoMin {
                min_width: 64. * MAZE_SIZE as f32,
                min_height: 64. * MAZE_SIZE as f32,
            },
            ..OrthographicProjection::default_2d()
        },
        Name::new("Camera"),
    ));

    let texture_handle: Handle<Image> = asset_server.load("kenney_topdown_shooter.png");

    let map_size = TilemapSize {
        x: MAZE_SIZE,
        y: MAZE_SIZE,
    };

    // Create a tilemap entity a little early.
    // We want this entity early because we need to tell each tile which tilemap entity
    // it is associated with. This is done with the TilemapId component on each tile.
    // Eventually, we will insert the `TilemapBundle` bundle on the entity, which
    // will contain various necessary components, such as `TileStorage`.
    let tilemap_entity = commands.spawn_empty().id();

    // To begin creating the map we will need a `TileStorage` component.
    // This component is a grid of tile entities and is used to help keep track of individual
    // tiles in the world. If you have multiple layers of tiles you would have a tilemap entity
    // per layer, each with their own `TileStorage` component.
    let mut tile_storage = TileStorage::empty(map_size);

    let maze_cache: HashMap<(usize, usize), &maze::Cell> = maze.iter().collect();

    for ((x, y), cell) in maze.iter() {
        let index = cell_to_index(cell.to_bits_str().as_str(), (x, y), &maze_cache);

        let tile_pos = TilePos {
            x: x as u32,
            y: map_size.y - (y as u32) - 1,
        };
        let tile_entity = commands
            .spawn((
                TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: index,
                    ..default()
                },
                cell.clone(),
            ))
            .id();
        tile_storage.set(&tile_pos, tile_entity);
    }

    let tile_size = TilemapTileSize { x: 64.0, y: 64.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        ..default()
    });
}

// given a corner direction like 1,1,
// move on each axis and check the direction of the opposite
// axis. (ex: move on x, then check the y direction) to see
// if there's a wall, which is then used to determine if there
// should be a wall piece in the corner of the tile
// ex: a left wall with a top-right corner block.
fn check_corner(
    pos: (usize, usize),
    corner: IVec2,
    cache: &HashMap<(usize, usize), &maze::Cell>,
) -> bool {
    pos.0
        .checked_add_signed(corner.x as isize)
        .and_then(|new_x| cache.get(&(new_x, pos.1)))
        .map(|cell| {
            !cell.contains(match corner.y {
                1 => Cell::SOUTH,
                -1 => Cell::NORTH,
                _ => unreachable!(""),
            })
        })
        .is_some_and(|exists| exists)
        || pos
            .1
            .checked_add_signed(corner.y as isize)
            .and_then(|v| cache.get(&(pos.0, v)))
            .map(|cell| {
                !cell.contains(match corner.x {
                    1 => Cell::EAST,
                    -1 => Cell::WEST,
                    _ => unreachable!(""),
                })
            })
            .is_some_and(|v| v)
}

fn cell_to_index(
    cell: &str,
    position: (usize, usize),
    cache: &HashMap<(usize, usize), &maze::Cell>,
) -> TileTextureIndex {
    // wesn
    let index = TileTextureIndex(
        match cell {
            "0000" => 358,
            "0001" => 286,
            "0010" => 312,
            "0011" => 309,
            "0100" => 313,
            "0101" => {
                let has_ne_corner = check_corner(position, IVec2::new(1, -1), &cache);
                if has_ne_corner {
                    307
                } else {
                    314
                }
            }
            "0110" => {
                let has_se_corner = check_corner(position, IVec2::new(1, 1), &cache);
                if has_se_corner {
                    280
                } else {
                    287
                }
            }
            "0111" => {
                let has_ne_corner = check_corner(position, IVec2::new(1, -1), &cache);
                let has_se_corner = check_corner(position, IVec2::new(1, 1), &cache);
                match (has_ne_corner, has_se_corner) {
                    (true, true) => 310,
                    (true, false) => 390,
                    (false, true) => 417,
                    (false, false) => 338,
                }
            }
            "1000" => 285,
            "1001" => {
                let has_nw_corner = check_corner(position, IVec2::new(-1, -1), &cache);
                if has_nw_corner {
                    308
                } else {
                    315
                }
            }
            "1010" => {
                let has_sw_corner = check_corner(position, IVec2::new(-1, 1), &cache);
                if has_sw_corner {
                    281
                } else {
                    288
                }
            }
            "1011" => {
                let has_nw_corner = check_corner(position, IVec2::new(-1, -1), &cache);
                let has_sw_corner = check_corner(position, IVec2::new(-1, 1), &cache);
                match (has_nw_corner, has_sw_corner) {
                    (true, true) => 311,
                    (true, false) => 391,
                    (false, true) => 418,
                    (false, false) => 339,
                }
            }
            "1100" => 282,
            "1101" => {
                let has_ne_corner = check_corner(position, IVec2::new(1, -1), &cache);
                let has_nw_corner = check_corner(position, IVec2::new(-1, -1), &cache);
                match (has_ne_corner, has_nw_corner) {
                    (true, true) => 284,
                    (true, false) => 420,
                    (false, true) => 419,
                    (false, false) => 366,
                }
            }
            "1110" => {
                let has_se_corner = check_corner(position, IVec2::new(1, 1), &cache);
                let has_sw_corner = check_corner(position, IVec2::new(-1, 1), &cache);
                match (has_se_corner, has_sw_corner) {
                    (true, true) => 283,
                    (true, false) => 393,
                    (false, true) => 392,
                    (false, false) => 365,
                }
            }
            "1111" => {
                let has_ne_corner = check_corner(position, IVec2::new(1, -1), &cache);
                let has_nw_corner = check_corner(position, IVec2::new(-1, -1), &cache);
                let has_se_corner = check_corner(position, IVec2::new(1, 1), &cache);
                let has_sw_corner = check_corner(position, IVec2::new(-1, 1), &cache);

                match (has_ne_corner, has_nw_corner, has_se_corner, has_sw_corner) {
                    (true, true, true, true) => 341,
                    (true, true, true, false) => 389,
                    (true, true, false, true) => 388,
                    (true, true, false, false) => 337,
                    (true, false, true, true) => 416,
                    (true, false, true, false) => 363,
                    (true, false, false, true) => 394,
                    (true, false, false, false) => 361,
                    (false, true, true, true) => 415,
                    (false, true, true, false) => 421,
                    (false, true, false, true) => 364,
                    (false, true, false, false) => 362,
                    (false, false, true, true) => 336,
                    (false, false, true, false) => 334,
                    (false, false, false, true) => 335,
                    (false, false, false, false) => 340,
                }
            } // check all corners
            _ => unreachable!("cell can only be 4 bits"),
        } - 1,
    );

    index
}
