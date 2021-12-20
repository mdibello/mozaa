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
    let tile = Tile::new(Color::GRAY, Color::GRAY, Color::GRAY, Color::GRAY);
    place_tile(&Coordinate::new(33, 32), &tile);
    recalculate_subgrid(Coordinate::new(33, 32));
    place_tile(&Coordinate::new(34, 32), &tile);
    recalculate_subgrid(Coordinate::new(34, 32));
    place_tile(&Coordinate::new(35, 32), &tile);
    recalculate_subgrid(Coordinate::new(35, 32));
    std::assert_eq!(get_subgrid(), Subgrid {
        start: Coordinate { x: 27, y: 27 },
        end: Coordinate { x: 37, y: 37 },
        max_dimensions: Coordinate { x: 35, y: 32 },
        min_dimensions: Coordinate { x: 32, y: 32 }
    });
}

#[test]
fn test_subgrid_calculation_2() {
    initialize();
    let tile = Tile::new(Color::GRAY, Color::GRAY, Color::GRAY, Color::GRAY);
    place_tile(&Coordinate::new(31, 32), &tile);
    recalculate_subgrid(Coordinate::new(31, 32));
    place_tile(&Coordinate::new(30, 32), &tile);
    recalculate_subgrid(Coordinate::new(30, 32));
    place_tile(&Coordinate::new(29, 32), &tile);
    recalculate_subgrid(Coordinate::new(29, 32));
    std::assert_eq!(get_subgrid(), Subgrid {
        start: Coordinate { x: 27, y: 27 },
        end: Coordinate { x: 37, y: 37 },
        max_dimensions: Coordinate { x: 32, y: 32 },
        min_dimensions: Coordinate { x: 29, y: 32 }
    });
}