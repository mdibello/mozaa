mod test;

use wasm_bindgen::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashSet;
use std::collections::VecDeque;
use serde::{Serialize, Deserialize};
use serde_json;

static mut STATE: State = State {
    GRID: None,
    TILES: None,
    CURRENT_TILE: None,
    UNIQUE_TILESET: None,
    NODES: Vec::new(),
    SCORES: Vec::new(),
    HISTORY: Vec::new(),
    TILES_CREATED: 0
};

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct State {
    GRID: Option<Grid>,
    TILES: Option<Vec<Tile>>,
    CURRENT_TILE: Option<Tile>,
    UNIQUE_TILESET: Option<Vec<Tile>>,
    NODES: Vec<Node>,
    SCORES: Vec<usize>,
    HISTORY: Vec<Turn>,
    TILES_CREATED: u64
}

#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub enum Color { PURPLE, BLUE, GRAY, RED }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Node {
    pub id: usize,
    pub color: Color,
    pub edges: Vec<usize>,
    pub tile_id: u64,
}

impl Node {
    fn new(id: usize, color: Color, tile_id: u64) -> Node {
        return Node { id: id, color: color, edges: Vec::new(), tile_id: tile_id }
    }
    fn add_edge(&mut self, id: usize) -> () {
        self.edges.push(id);
    }
}

fn create_node(color: Color, tile_id: u64) -> usize {
    unsafe {
        let id: usize = STATE.NODES.len();
        STATE.NODES.push(Node::new(id, color, tile_id));
        return id;
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Tile {
    pub id: u64,
    pub left: Color,
    pub top: Color,
    pub right: Color,
    pub bottom: Color,
    pub left_id: Option<usize>,
    pub top_id: Option<usize>,
    pub right_id: Option<usize>,
    pub bottom_id: Option<usize>
}

impl Tile {
    fn new(left: Color, top: Color, right: Color, bottom: Color) -> Tile {
        let tile = Tile {
            id: unsafe { STATE.TILES_CREATED },
            left: left,
            top: top,
            right: right,
            bottom: bottom,
            left_id: None,
            top_id: None,
            right_id: None,
            bottom_id: None,
        };
        unsafe { STATE.TILES_CREATED += 1; }
        return tile;
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
    fn get_nodes(&self) -> Vec<usize> {
        return vec![self.left_id.unwrap(), self.top_id.unwrap(), self.right_id.unwrap(), self.bottom_id.unwrap()];
    }
}

#[derive(Serialize, Deserialize)]
struct Grid {
    tiles: Vec<Option<Tile>>,
    width: i32,
    subgrid: Subgrid,
    count: u64
}

impl Grid {
    fn at(&self, coordinate: &Coordinate) -> Option<Tile> {
        return self.tiles[xy_to_index(coordinate) as usize].clone();
    }
    fn place(&mut self, tile: Tile, coordinate: &Coordinate) -> () {
        self.tiles[xy_to_index(coordinate) as usize] = Some(tile);
        self.count += 1;
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub struct Subgrid {
    pub start: Coordinate,
    pub end: Coordinate,
    max_dimensions: Coordinate,
    min_dimensions: Coordinate
}

#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32
}

impl Coordinate {
    fn new(x: i32, y: i32) -> Coordinate {
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
    fn is_out_of_bounds(&self) -> bool {
        return self.x < 0 || self.x > 63 || self.y < 0 || self.y > 63;
    }
}

#[derive(Serialize, Deserialize)]
#[wasm_bindgen]
pub struct Turn {
    player: usize,
    tile: Tile,
    location: Coordinate
}

#[wasm_bindgen]
pub fn initialize() -> () {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    unsafe {
        let grid: Vec<Option<Tile>> = vec![None; 64 * 64];

        STATE.GRID = Some(Grid {
            tiles: grid,
            width: 64,
            subgrid: Subgrid {
                start: Coordinate { x: 30, y: 30 },
                end: Coordinate { x: 34, y: 34 },
                max_dimensions: Coordinate { x: 32, y: 32 },
                min_dimensions: Coordinate { x: 32, y: 32 }
            },
            count: 0
        });

        STATE.NODES = Vec::new();
        STATE.TILES_CREATED = 0;
        
        place_tile(&coordinate(32, 32), &mut Tile::default(), 0);

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

        STATE.UNIQUE_TILESET = Some(tiles.clone());
        shuffle_tiles(&mut tiles);
        STATE.CURRENT_TILE = Some(tiles.pop().unwrap());
        STATE.TILES = Some(tiles);

        STATE.SCORES = vec![0];
    }
}

#[wasm_bindgen]
pub fn is_valid_placement(new_tile: &Tile, coordinate: &Coordinate) -> bool {
    if coordinate.is_out_of_bounds() {
        return false;
    }
    unsafe {
        match &mut STATE.GRID {
            None => panic!("STATE.GRID uninitialized!"),
            Some(grid) => {
                if grid.count == 0 {
                    return true;
                }
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
        match &STATE.GRID {
            None => panic!("STATE.GRID uninitialized"),
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
pub fn place_tile(coordinate: &Coordinate, tile: &mut Tile, player: usize) -> () {
    if is_valid_placement(tile, coordinate) {
        unsafe {
            match &mut STATE.GRID {
                None => panic!("STATE.GRID uninitialized!"),
                Some(grid) => {
                    let left = create_node(tile.left, tile.id);
                    let top = create_node(tile.top, tile.id);
                    let right = create_node(tile.right, tile.id);
                    let bottom = create_node(tile.bottom, tile.id);

                    STATE.NODES[left].add_edge(top);
                    STATE.NODES[left].add_edge(bottom);
                    STATE.NODES[top].add_edge(left);
                    STATE.NODES[top].add_edge(right);
                    STATE.NODES[right].add_edge(top);
                    STATE.NODES[right].add_edge(bottom);
                    STATE.NODES[bottom].add_edge(left);
                    STATE.NODES[bottom].add_edge(right);

                    tile.left_id = Some(left);
                    tile.top_id = Some(top);
                    tile.right_id = Some(right);
                    tile.bottom_id = Some(bottom);

                    grid.place(tile.clone(), coordinate);

                    let left_tile = grid.tiles[xy_to_index(&coordinate.left()) as usize];
                    match left_tile {
                        Some(t) => {
                            match t.right_id {
                                Some(id) => {
                                    STATE.NODES[id].add_edge(tile.left_id.unwrap());
                                    STATE.NODES[tile.left_id.unwrap()].add_edge(id);
                                },
                                None => ()
                            }
                        },
                        None => ()
                    }
                    let top_tile = grid.tiles[xy_to_index(&coordinate.above()) as usize];
                    match top_tile {
                        Some(t) => {
                            match t.bottom_id {
                                Some(id) => {
                                    STATE.NODES[id].add_edge(tile.top_id.unwrap());
                                    STATE.NODES[tile.top_id.unwrap()].add_edge(id);
                                },
                                None => ()
                            }
                        },
                        None => ()
                    }
                    let right_tile = grid.tiles[xy_to_index(&coordinate.right()) as usize];
                    match right_tile {
                        Some(t) => {
                            match t.left_id {
                                Some(id) => {
                                    STATE.NODES[id].add_edge(tile.right_id.unwrap());
                                    STATE.NODES[tile.right_id.unwrap()].add_edge(id);
                                },
                                None => ()
                            }
                        },
                        None => ()
                    }
                    let bottom_tile = grid.tiles[xy_to_index(&coordinate.below()) as usize];
                    match bottom_tile {
                        Some(t) => {
                            match t.top_id {
                                Some(id) => {
                                    STATE.NODES[id].add_edge(tile.bottom_id.unwrap());
                                    STATE.NODES[tile.bottom_id.unwrap()].add_edge(id);
                                },
                                None => ()
                            }
                        },
                        None => ()
                    }

                    STATE.HISTORY.push(
                        Turn { player: player, tile: *tile, location: *coordinate }
                    );

                    calculate_score(*tile);
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
        match &STATE.GRID {
            None => panic!("STATE.GRID uninitialized!"),
            Some(grid) => {
                if coordinate.is_out_of_bounds() {
                    return false;
                }
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
        match &STATE.GRID {
            None => panic!("STATE.GRID uninitialized!"),
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
        match &STATE.CURRENT_TILE {
            None => panic!("STATE.CURRENT_TILE uninitialized!"),
            Some(tile) => return tile.clone(),
        }
    }
}

#[wasm_bindgen]
pub fn draw_new_tile() -> () {
    unsafe {
        match &mut STATE.TILES {
            None => panic!("STATE.TILES uninitialized!"),
            Some(tiles) => {
                match &mut STATE.CURRENT_TILE {
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
        match &mut STATE.GRID {
            None => panic!("STATE.GRID uninitialized!"),
            Some(grid) => {
                grid.subgrid.min_dimensions = Coordinate::new(
                    std::cmp::min(grid.subgrid.min_dimensions.x, coordinate.x),
                    std::cmp::min(grid.subgrid.min_dimensions.y, coordinate.y)
                );
                grid.subgrid.max_dimensions = Coordinate::new(
                    std::cmp::max(grid.subgrid.max_dimensions.x, coordinate.x),
                    std::cmp::max(grid.subgrid.max_dimensions.y, coordinate.y)
                );
                if coordinate.x - 1 < grid.subgrid.start.x || coordinate.y - 1 < grid.subgrid.start.y {
                    if coordinate.x - 1 < grid.subgrid.start.x && grid.subgrid.end.x > grid.subgrid.max_dimensions.x + 1  {
                        grid.subgrid.start.x -= 1;
                        grid.subgrid.end.x -= 1;
                    } else if coordinate.y - 1 < grid.subgrid.start.y && grid.subgrid.end.y > grid.subgrid.max_dimensions.y + 1 {
                        grid.subgrid.start.y -= 1;
                        grid.subgrid.end.y -= 1;
                    } else {
                        grid.subgrid.start.x -= 1;
                        grid.subgrid.start.y -= 1;
                        grid.subgrid.end.x += 1;
                        grid.subgrid.end.y += 1;
                    }
                }
                if coordinate.x + 1 > grid.subgrid.end.x || coordinate.y + 1 > grid.subgrid.end.y {
                    if coordinate.x + 1 > grid.subgrid.end.x && grid.subgrid.start.x < grid.subgrid.min_dimensions.x - 1 {
                        grid.subgrid.start.x += 1;
                        grid.subgrid.end.x += 1;
                    } else if coordinate.y + 1 > grid.subgrid.end.y && grid.subgrid.start.y < grid.subgrid.min_dimensions.y - 1  {
                        grid.subgrid.start.y += 1;
                        grid.subgrid.end.y += 1;
                    } else {
                        grid.subgrid.start.x -= 1;
                        grid.subgrid.start.y -= 1;
                        grid.subgrid.end.x += 1;
                        grid.subgrid.end.y += 1;
                    }
                }
            }
        }
    }
}

#[wasm_bindgen]
pub fn get_subgrid() -> Subgrid {
    unsafe {
        match &STATE.GRID {
            None => panic!("STATE.GRID uninitialized!"),
            Some(grid) => {
                return grid.subgrid.clone();
            }
        }
    }
}

#[wasm_bindgen]
pub fn tiles_remaining() -> usize {
    unsafe {
        match &STATE.TILES {
            None => panic!("STATE.TILES uninitialized!"),
            Some(tiles) => return tiles.len()
        }
    }
}

#[wasm_bindgen]
pub fn coordinate(x: i32, y: i32) -> Coordinate {
    return Coordinate::new(x, y);
}

#[wasm_bindgen]
pub fn select_new_tile() -> () {
    unsafe {
        match &mut STATE.TILES {
            None => panic!("STATE.TILES uninitialized!"),
            Some(tiles) => {
                match &mut STATE.CURRENT_TILE {
                    None => panic!("STATE.CURRENT_TILE uninitialized!"),
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
        match &mut STATE.TILES {
            None => panic!("STATE.TILES uninitialized!"),
            Some(tiles) => {
                match &STATE.UNIQUE_TILESET {
                    None => panic!("UNIQUE_STATE.TILESET uninitialized!"),
                    Some(unique_tileset) => {
                        tiles.append(&mut unique_tileset.clone());
                        shuffle_tiles(tiles);
                        match &STATE.CURRENT_TILE {
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

pub fn calculate_score(tile: Tile) -> () {
    unsafe {
        println!("{:?}", STATE.NODES);
        let mut visited: HashSet<usize> = HashSet::new();
        let mut score: usize = 0;
        for origin in tile.get_nodes() {
            let mut pending: VecDeque<usize> = VecDeque::new();
            let mut tiles: HashSet<u64> = HashSet::new();
            let origin_node = &STATE.NODES[origin];
            println!("ORIGIN: {:?}", origin_node);
            if !visited.contains(&origin) {
                pending.push_back(origin);
                visited.insert(origin);
                tiles.insert(origin_node.tile_id);
            }
            let mut valid = true;
            while !pending.is_empty() {
                println!("{:?}", visited);
                let node_id = pending.pop_front().unwrap();
                let adjacent_ids = &STATE.NODES[node_id].edges;
                if adjacent_ids.len() != 3 {
                    valid = false;
                }
                for &id in adjacent_ids {
                    let node = &STATE.NODES[id];
                    println!("ADJACENT: {:?}", node);
                    if node.color == origin_node.color {
                        if !visited.contains(&id) {
                            visited.insert(id);
                            pending.push_back(id);
                            if valid {
                                tiles.insert(node.tile_id);
                            }
                        }
                    }
                }
            }
            println!("VALID: {}\n", valid);
            if valid {
                score += tiles.len();
                if tiles.len() > 3 {
                    score += tiles.len();
                }
            }
        }
        println!("SCORE: {}", score);
        STATE.SCORES.push(score);
    }
}

#[wasm_bindgen]
pub fn score() -> i32 {
    unsafe {
        return *STATE.SCORES.last().unwrap() as i32;
    }
}

#[wasm_bindgen]
pub fn total_score() -> i32 {
    unsafe {
        return STATE.SCORES.iter().sum::<usize>() as i32;
    }
}

fn shuffle_tiles(tiles: &mut Vec<Tile>) -> () {
    let mut rng = thread_rng();
    tiles.shuffle(&mut rng);
}

fn xy_to_index(c: &Coordinate) -> i32 {
    unsafe {
        match &STATE.GRID {
            None => panic!("STATE.GRID is uninitialized!"),
            Some(g) => return (c.y * g.width) + c.x,
        }
    }
}

#[wasm_bindgen]
pub fn serialize_state() -> String {
    unsafe {
        return serde_json::to_string(&STATE).unwrap();
    }
}

#[wasm_bindgen]
pub fn deserialize_state(payload: String) -> () {
    unsafe {
        STATE = serde_json::from_str(&payload).unwrap();
    }
}
