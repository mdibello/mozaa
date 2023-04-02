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
        std::assert_eq!(NODES[0].edges, vec![1,2,3]);
        std::assert_eq!(NODES[1].edges, vec![0,2,3]);
        std::assert_eq!(NODES[2].edges, vec![0,1,3]);
        std::assert_eq!(NODES[3].edges, vec![0,1,2]);
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
        std::assert_eq!(NODES[0].edges, vec![1,2,3]);
        std::assert_eq!(NODES[1].edges, vec![0,2,3]);
        std::assert_eq!(NODES[2].edges, vec![0,1,3,4]);
        std::assert_eq!(NODES[3].edges, vec![0,1,2]);
        std::assert_eq!(NODES[4].edges, vec![5,6,7,2]);
        std::assert_eq!(NODES[5].edges, vec![4,6,7]);
        std::assert_eq!(NODES[6].edges, vec![4,5,7]);
        std::assert_eq!(NODES[7].edges, vec![4,5,6]);
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
        println!("{:?}", NODES);
        std::assert_eq!(NODES.len(), 16);
        std::assert_eq!(NODES[0].edges, vec![1,2,3]);
        std::assert_eq!(NODES[1].edges, vec![0,2,3]);
        std::assert_eq!(NODES[2].edges, vec![0,1,3,4]);
        std::assert_eq!(NODES[3].edges, vec![0,1,2,9]);
        std::assert_eq!(NODES[4].edges, vec![5,6,7,2]);
        std::assert_eq!(NODES[5].edges, vec![4,6,7]);
        std::assert_eq!(NODES[6].edges, vec![4,5,7]);
        std::assert_eq!(NODES[7].edges, vec![4,5,6,13]);
        std::assert_eq!(NODES[8].edges, vec![9,10,11]);
        std::assert_eq!(NODES[9].edges, vec![8,10,11,3]);
        std::assert_eq!(NODES[10].edges, vec![8,9,11,12]);
        std::assert_eq!(NODES[11].edges, vec![8,9,10]);
        std::assert_eq!(NODES[12].edges, vec![13,14,15,10]);
        std::assert_eq!(NODES[13].edges, vec![12,14,15,7]);
        std::assert_eq!(NODES[14].edges, vec![12,13,15]);
        std::assert_eq!(NODES[15].edges, vec![12,13,14]);
    }
}