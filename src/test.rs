#[cfg(test)]

use crate::*;

#[test]
fn test_initialize() {
    initialize();
    assert_eq!(tiles_remaining(), 62);
}

#[test]
fn test_empty_space() {
    initialize();
    std::assert!(!contains_tile(&coordinate(31, 31)));
}

#[test]
fn test_subgrid_calculation() {
    initialize();
    let mut tile = Tile::new(Color::GRAY, Color::GRAY, Color::GRAY, Color::GRAY);
    place_tile(&Coordinate::new(33, 32), &mut tile);
    recalculate_subgrid(Coordinate::new(33, 32));
    place_tile(&Coordinate::new(34, 32), &mut tile);
    recalculate_subgrid(Coordinate::new(34, 32));
    place_tile(&Coordinate::new(35, 32), &mut tile);
    recalculate_subgrid(Coordinate::new(35, 32));
    std::assert_eq!(get_subgrid(), Subgrid {
        start: Coordinate { x: 30, y: 29 },
        end: Coordinate { x: 36, y: 35 },
        max_dimensions: Coordinate { x: 35, y: 32 },
        min_dimensions: Coordinate { x: 32, y: 32 }
    });
}

#[test]
fn test_subgrid_calculation_2() {
    initialize();
    let mut tile = Tile::new(Color::GRAY, Color::GRAY, Color::GRAY, Color::GRAY);
    place_tile(&Coordinate::new(31, 32), &mut tile);
    recalculate_subgrid(Coordinate::new(31, 32));
    place_tile(&Coordinate::new(30, 32), &mut tile);
    recalculate_subgrid(Coordinate::new(30, 32));
    place_tile(&Coordinate::new(29, 32), &mut tile);
    recalculate_subgrid(Coordinate::new(29, 32));
    std::assert_eq!(get_subgrid(), Subgrid {
        start: Coordinate { x: 28, y: 29 },
        end: Coordinate { x: 34, y: 35 },
        max_dimensions: Coordinate { x: 32, y: 32 },
        min_dimensions: Coordinate { x: 29, y: 32 }
    });
}

#[test]
fn test_graph_single_tile() {
    initialize();
    unsafe {
        std::assert_eq!(NODES.len(), 4);
        std::assert_eq!(NODES[0].edges, vec![1,3]);
        std::assert_eq!(NODES[1].edges, vec![0,2]);
        std::assert_eq!(NODES[2].edges, vec![1,3]);
        std::assert_eq!(NODES[3].edges, vec![0,2]);
    }
}

#[test]
fn test_graph_plus_one_tile() {
    initialize();
    let initial_coord = Coordinate::new(32, 32);
    let mut tile = Tile::new(Color::GRAY, Color::GRAY, Color::GRAY, Color::GRAY);
    place_tile(&initial_coord.right(), &mut tile);
    unsafe {
        std::assert_eq!(NODES.len(), 8);
        std::assert_eq!(NODES[0].edges, vec![1,3]);
        std::assert_eq!(NODES[1].edges, vec![0,2]);
        std::assert_eq!(NODES[2].edges, vec![1,3,4]);
        std::assert_eq!(NODES[3].edges, vec![0,2]);
        std::assert_eq!(NODES[4].edges, vec![5,7,2]);
        std::assert_eq!(NODES[5].edges, vec![4,6]);
        std::assert_eq!(NODES[6].edges, vec![5,7]);
        std::assert_eq!(NODES[7].edges, vec![4,6]);
    }
}

#[test]
fn test_graph_plus_three_tiles() {
    initialize();
    let initial_coord = Coordinate::new(32, 32);
    let mut tile = Tile::new(Color::GRAY, Color::GRAY, Color::GRAY, Color::GRAY);
    place_tile(&initial_coord.right(), &mut tile);
    tile = Tile::new(Color::GRAY, Color::RED, Color::GRAY, Color::GRAY);
    place_tile(&initial_coord.below(), &mut tile);
    tile = Tile::new(Color::GRAY, Color::GRAY, Color::GRAY, Color::GRAY);
    place_tile(&(initial_coord.right()).below(), &mut tile);
    unsafe {
        std::assert_eq!(NODES.len(), 16);
        std::assert_eq!(NODES[0].edges, vec![1,3]);
        std::assert_eq!(NODES[1].edges, vec![0,2]);
        std::assert_eq!(NODES[2].edges, vec![1,3,4]);
        std::assert_eq!(NODES[3].edges, vec![0,2,9]);
        std::assert_eq!(NODES[4].edges, vec![5,7,2]);
        std::assert_eq!(NODES[5].edges, vec![4,6]);
        std::assert_eq!(NODES[6].edges, vec![5,7]);
        std::assert_eq!(NODES[7].edges, vec![4,6,13]);
        std::assert_eq!(NODES[8].edges, vec![9,11]);
        std::assert_eq!(NODES[9].edges, vec![8,10,3]);
        std::assert_eq!(NODES[10].edges, vec![9,11,12]);
        std::assert_eq!(NODES[11].edges, vec![8,10]);
        std::assert_eq!(NODES[12].edges, vec![13,15,10]);
        std::assert_eq!(NODES[13].edges, vec![12,14,7]);
        std::assert_eq!(NODES[14].edges, vec![13,15]);
        std::assert_eq!(NODES[15].edges, vec![12,14]);
    }
}

#[test]
fn test_score_0() {
    initialize();
    std::assert_eq!(score(), 0);
}

#[test]
fn test_score_1() {
    initialize();
    let initial_coord = Coordinate::new(32, 32);
    let mut tile = Tile::new(Color::GRAY, Color::GRAY, Color::GRAY, Color::GRAY);
    place_tile(&initial_coord.right(), &mut tile);
    std::assert_eq!(score(), 0);
}

#[test]
fn test_score_2() {
    initialize();
    let initial_coord = Coordinate::new(32, 32);
    let mut tile = Tile::new(Color::GRAY, Color::RED, Color::RED, Color::RED);
    place_tile(&initial_coord.right(), &mut tile);
    std::assert_eq!(score(), 2);
}

#[test]
fn test_score_3() {
    initialize();
    let initial_coord = Coordinate::new(32, 32);
    let mut tile = Tile::new(Color::GRAY, Color::RED, Color::RED, Color::GRAY);
    place_tile(&initial_coord.right(), &mut tile);
    tile = Tile::new(Color::RED, Color::GRAY, Color::RED, Color::RED);
    place_tile(&initial_coord.right().below(), &mut tile);
    std::assert_eq!(score(), 3);
}

#[test]
fn test_score_4() {
    initialize();
    let initial_coord = Coordinate::new(32, 32);
    let mut tile = Tile::new(Color::GRAY, Color::RED, Color::RED, Color::GRAY);
    place_tile(&initial_coord.right(), &mut tile);
    tile = Tile::new(Color::GRAY, Color::GRAY, Color::RED, Color::RED);
    place_tile(&initial_coord.right().below(), &mut tile);
    tile = Tile::new(Color::GRAY, Color::RED, Color::GRAY, Color::RED);
    place_tile(&initial_coord.below(), &mut tile);
    std::assert_eq!(score(), 10);
}

#[test]
fn test_score_5() {
    initialize();
    let initial_coord = Coordinate::new(32, 32);
    let mut tile = Tile::new(Color::GRAY, Color::GRAY, Color::BLUE, Color::BLUE);
    place_tile(&initial_coord.above(), &mut tile);
    std::assert_eq!(score(), 0);
}