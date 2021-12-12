use wasm_bindgen::prelude::*;
use matrix::prelude::*;

static mut GRID: Option<Grid> = None;
static mut TILES: Option<Vec<Tile>> = None;

#[derive(Clone, PartialEq)]
enum Color { PURPLE, BLUE, GRAY, RED }

#[derive(Clone)]
struct Tile {
    left: Color,
    top: Color,
    right: Color,
    bottom: Color
}

impl Tile {
    fn new(left: Color, top: Color, right: Color, bottom: Color) -> Tile {
        return Tile {
            left: left,
            top: top,
            right: right,
            bottom: bottom
        }
    }
    fn default() -> Tile {
        return Tile::new(Color::PURPLE, Color::BLUE, Color::GRAY, Color::RED);
    }
    fn rotate(tile: &Tile, times: u8) -> Tile {
        match times {
            0 => return tile.clone(),
            n => return Tile::rotate(
                &Tile::new(tile.bottom.clone(), tile.left.clone(), tile.top.clone(), tile.right.clone()),
                n-1)
        }
    }
    fn is_duplicate_many(tile_1: &Tile, tiles: &Vec<Tile>) -> bool {
        for tile in tiles {
            if !Tile::is_duplicate(tile_1, tile) {
                return false;
            }
        }
        return true;
    }
    fn is_duplicate(tile_1: &Tile, tile_2: &Tile) -> bool {
        return Tile::check_duplicate_rotation(&Tile::rotate(&tile_1, 0), tile_2) &&
               Tile::check_duplicate_rotation(&Tile::rotate(&tile_1, 1), tile_2) &&
               Tile::check_duplicate_rotation(&Tile::rotate(&tile_1, 2), tile_2) &&
               Tile::check_duplicate_rotation(&Tile::rotate(&tile_1, 3), tile_2);
    }
    fn check_duplicate_rotation(tile_1: &Tile, tile_2: &Tile) -> bool {
        return tile_1.left == tile_2.left &&
               tile_1.top == tile_2.top &&
               tile_1.right == tile_2.right &&
               tile_1.bottom == tile_2.bottom;
    }
    fn is_dichromatic_bowtie(tile: &Tile) -> bool {
        return tile.left == tile.right && tile.top == tile.bottom;
    }
}

struct Grid {
    tiles: Vec<Option<Tile>>,
    width: usize,
    subgrid: Subgrid
}

struct Subgrid {
    start: Coordinate,
    end: Coordinate
}

struct Coordinate {
    x: usize,
    y: usize
}

#[wasm_bindgen]
pub fn initialize() -> () {
    unsafe {
        let mut grid: Vec<Option<Tile>> = vec![None; 64 * 64];
        let starting_index = xy_to_index(Coordinate { x: 32, y: 32 });
        grid[starting_index] = Some(Tile::default());

        GRID = Some(Grid {
            tiles: grid,
            width: 64,
            subgrid: Subgrid {
                start: Coordinate { x: 30, y: 30 },
                end: Coordinate { x: 34, y: 34 }
            }
        });

        let colors = vec![Color::PURPLE, Color::BLUE, Color::GRAY, Color::RED];
        let mut tiles: Vec<Tile> = Vec::new();
        for c1 in &colors {
            for c2 in &colors {
                for c3 in &colors {
                    for c4 in &colors {
                        let new_tile = Tile::new(c1.clone(), c2.clone(), c3.clone(), c4.clone());
                        if !Tile::is_duplicate(&new_tile, &Tile::default()) &&
                           !Tile::is_duplicate_many(&new_tile, &tiles) &&
                           !Tile::is_dichromatic_bowtie(&new_tile) {
                                tiles.push(new_tile);
                        }
                    }
                }
            }
        }

        TILES = Some(tiles);
    }
}

#[wasm_bindgen]
pub fn score() -> i32 {
    return 3;
}

fn xy_to_index(c: Coordinate) -> usize {
    unsafe {
        match &GRID {
            None => panic!("GRID is uninitialized!"),
            Some(g) => return (c.y * g.width) + c.x,
        }
    }
}
