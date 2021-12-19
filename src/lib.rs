mod test;

use wasm_bindgen::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;

static mut GRID: Option<Grid> = None;
static mut TILES: Option<Vec<Tile>> = None;
static mut CURRENT_TILE: Option<Tile> = None;
static mut UNIQUE_TILESET: Option<Vec<Tile>> = None;

#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Color { PURPLE, BLUE, GRAY, RED }

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct Tile {
    pub left: Color,
    pub top: Color,
    pub right: Color,
    pub bottom: Color
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
    fn rotate(&self, times: u8) -> Tile {
        match times {
            0 => return self.clone(),
            n => return Tile::rotate(
                &Tile::new(self.bottom.clone(), self.left.clone(), self.top.clone(), self.right.clone()),
                n-1)
        }
    }
    fn is_duplicate_many(tile_1: &Tile, tiles: &Vec<Tile>) -> bool {
        for tile in tiles {
            if Tile::is_duplicate(tile_1, tile) {
                return true;
            }
        }
        return false;
    }
    fn is_duplicate(tile_1: &Tile, tile_2: &Tile) -> bool {
        return Tile::check_duplicate_rotation(&tile_1.rotate( 0), tile_2) ||
               Tile::check_duplicate_rotation(&tile_1.rotate( 1), tile_2) ||
               Tile::check_duplicate_rotation(&tile_1.rotate( 2), tile_2) ||
               Tile::check_duplicate_rotation(&tile_1.rotate( 3), tile_2);
    }
    fn check_duplicate_rotation(tile_1: &Tile, tile_2: &Tile) -> bool {
        return tile_1.left == tile_2.left &&
               tile_1.top == tile_2.top &&
               tile_1.right == tile_2.right &&
               tile_1.bottom == tile_2.bottom;
    }
    fn is_dichromatic_bowtie(tile: &Tile) -> bool {
        return tile.left == tile.right && tile.top == tile.bottom && tile.left != tile.top;
    }
}

struct Grid {
    tiles: Vec<Option<Tile>>,
    width: usize,
    subgrid: Subgrid
}

impl Grid {
    fn at(&self, coordinate: &Coordinate) -> Option<Tile> {
        return self.tiles[xy_to_index(coordinate)].clone();
    }
    fn place(&mut self, tile: Tile, coordinate: &Coordinate) -> () {
        self.tiles[xy_to_index(coordinate)] = Some(tile);
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Subgrid {
    pub start: Coordinate,
    pub end: Coordinate
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize
}

impl Coordinate {
    fn new(x: usize, y: usize) -> Coordinate {
        return Coordinate { x: x, y: y };
    }
    fn left(&self) -> Coordinate {
        return Coordinate::new(self.x - 1, self.y);
    }
    fn above(&self) -> Coordinate {
        return Coordinate::new(self.x, self.y - 1);
    }
    fn right(&self) -> Coordinate {
        return Coordinate::new(self.x + 1, self.y);
    }
    fn below(&self) -> Coordinate {
        return Coordinate::new(self.x, self.y + 1);
    }
}

#[wasm_bindgen]
pub fn initialize() -> () {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    unsafe {
        let mut grid: Vec<Option<Tile>> = vec![None; 64 * 64];
        let starting_index = (32 * 64) + 32;
        grid[starting_index] = Some(Tile::default());

        GRID = Some(Grid {
            tiles: grid,
            width: 64,
            subgrid: Subgrid {
                start: Coordinate { x: 30, y: 30 },
                end: Coordinate { x: 34, y: 34 }
            }
        });
        
        place_tile(&coordinate(32, 32), &Tile::default());

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

        UNIQUE_TILESET = Some(tiles.clone());
        shuffle_tiles(&mut tiles);
        CURRENT_TILE = Some(tiles.pop().unwrap());
        TILES = Some(vec![Tile::default()]);
    }
}

#[wasm_bindgen]
pub fn is_valid_placement(new_tile: &Tile, coordinate: &Coordinate) -> bool {
    unsafe {
        match &GRID {
            None => panic!("GRID uninitialized!"),
            Some(grid) => {
                match grid.at(&coordinate) {
                    None => (),
                    Some(_) => {
                        return false;
                    }
                }
                match grid.at(&coordinate.left()) {
                    None => (),
                    Some(tile_to_left) => {
                        if tile_to_left.right != new_tile.left {
                            return false;
                        }
                    }
                }
                match grid.at(&coordinate.above()) {
                    None => (),
                    Some(tile_above) => {
                        if tile_above.bottom != new_tile.top {
                            return false;
                        }
                    }
                }
                match grid.at(&coordinate.right()) {
                    None => (),
                    Some(tile_to_right) => {
                        if tile_to_right.left != new_tile.right {
                            return false;
                        }
                    }
                }
                match grid.at(&coordinate.below()) {
                    None => (),
                    Some(tile_below) => {
                        if tile_below.top != new_tile.bottom {
                            return false;
                        }
                    }
                }
            }
        }
    }
    return true;
}

#[wasm_bindgen]
pub fn get_tile(coordinate: &Coordinate) -> Tile {
    unsafe {
        match &GRID {
            None => panic!("GRID uninitialized"),
            Some(grid) => {
                match grid.at(coordinate) {
                    None => panic!("Tried to get tile that doesn't exist!"),
                    Some(tile) => return tile,
                }
            }
        }
    }
}

#[wasm_bindgen]
pub fn place_tile(coordinate: &Coordinate, tile: &Tile) -> () {
    if is_valid_placement(tile, coordinate) {
        unsafe {
            match &mut GRID {
                None => panic!("GRID uninitialized!"),
                Some(grid) => {
                    grid.place(tile.clone(), coordinate);
                }
            }
        }
    }
}

#[wasm_bindgen]
pub fn rotate_tile(tile: Tile, times: u8) -> Tile {
    return tile.rotate(times);
}

#[wasm_bindgen]
pub fn should_be_displayed(coordinate: &Coordinate) -> bool {
    unsafe {
        match &GRID {
            None => panic!("GRID uninitialized!"),
            Some(grid) => {
                return grid.at(&coordinate).is_some() ||
                       grid.at(&coordinate.left()).is_some() ||
                       grid.at(&coordinate.above()).is_some() ||
                       grid.at(&coordinate.right()).is_some() ||
                       grid.at(&coordinate.below()).is_some();
            }
        }
    }
}

#[wasm_bindgen]
pub fn contains_tile(coordinate: &Coordinate) -> bool {
    unsafe {
        match &GRID {
            None => panic!("GRID uninitialized!"),
            Some(grid) => {
                match grid.at(coordinate) {
                    None => return false,
                    Some(_) => return true
                }
            }
        }
    }
}

#[wasm_bindgen]
pub fn get_current_tile() -> Tile {
    unsafe {
        match &CURRENT_TILE {
            None => panic!("CURRENT_TILE uninitialized!"),
            Some(tile) => return tile.clone(),
        }
    }
}

#[wasm_bindgen]
pub fn draw_new_tile() -> () {
    unsafe {
        match &mut TILES {
            None => panic!("TILES uninitialized!"),
            Some(tiles) => {
                match &mut CURRENT_TILE {
                    None => (),
                    Some(current_tile) => {
                        if tiles.len() > 0 {
                            *current_tile = tiles.pop().unwrap();
                        } else {
                            panic!("Tried to draw new tile when there are none remaining. Need to check first");
                        }
                    }
                }
            }
        }
    }
}

#[wasm_bindgen]
pub fn recalculate_subgrid(coordinate: Coordinate) -> () {
    unsafe {
        match &mut GRID {
            None => panic!("GRID uninitialized!"),
            Some(grid) => {
                if coordinate.x - 1 < grid.subgrid.start.x || coordinate.y - 1 < grid.subgrid.start.y {
                    grid.subgrid.start.x -= 1;
                    grid.subgrid.start.y -= 1;
                }
                if coordinate.x + 1 > grid.subgrid.end.x || coordinate.y + 1 > grid.subgrid.end.y {
                    grid.subgrid.end.x += 1;
                    grid.subgrid.end.y += 1;
                }
            }
        }
    }
}

#[wasm_bindgen]
pub fn get_subgrid() -> Subgrid {
    unsafe {
        match &GRID {
            None => panic!("GRID uninitialized!"),
            Some(grid) => {
                return grid.subgrid.clone();
            }
        }
    }
}

#[wasm_bindgen]
pub fn tiles_remaining() -> usize {
    unsafe {
        match &TILES {
            None => panic!("TILES uninitialized!"),
            Some(tiles) => return tiles.len()
        }
    }
}

#[wasm_bindgen]
pub fn coordinate(x: usize, y: usize) -> Coordinate {
    return Coordinate::new(x, y);
}

#[wasm_bindgen]
pub fn select_new_tile() -> () {
    unsafe {
        match &mut TILES {
            None => panic!("TILES uninitialized!"),
            Some(tiles) => {
                match &mut CURRENT_TILE {
                    None => panic!("CURRENT_TILE uninitialized!"),
                    Some(current_tile) => {
                        tiles.push(current_tile.clone());
                        shuffle_tiles(tiles);
                        *current_tile = tiles.pop().unwrap();
                    }
                }
            }
        }
    }
}

#[wasm_bindgen]
pub fn add_more_tiles() -> () {
    unsafe {
        match &mut TILES {
            None => panic!("TILES uninitialized!"),
            Some(tiles) => {
                match &UNIQUE_TILESET {
                    None => panic!("UNIQUE_TILESET uninitialized!"),
                    Some(unique_tileset) => {
                        tiles.append(&mut unique_tileset.clone());
                        shuffle_tiles(tiles);
                        match &CURRENT_TILE {
                            Some(_) => (),
                            None => {
                                draw_new_tile();
                            }
                        }
                    }
                }
            }
        }
    }
}

#[wasm_bindgen]
pub fn score() -> i32 {
    return 0;
}

fn shuffle_tiles(tiles: &mut Vec<Tile>) -> () {
    let mut rng = thread_rng();
    tiles.shuffle(&mut rng);
}

fn xy_to_index(c: &Coordinate) -> usize {
    unsafe {
        match &GRID {
            None => panic!("GRID is uninitialized!"),
            Some(g) => return (c.y * g.width) + c.x,
        }
    }
}
