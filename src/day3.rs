use std::collections::HashMap;

struct PathSegment {
    direction: char,
    magnitude: usize,
}

impl PathSegment {
    fn parse(source: &str) -> PathSegment {
        PathSegment {
            direction: source.chars().nth(0).unwrap(),
            magnitude: source
                .chars()
                .skip(1)
                .collect::<String>()
                .parse::<usize>()
                .unwrap(),
        }
    }
}

fn direction_to_xy_deltas(direction: &char) -> (isize, isize) {
    match direction {
        'U' => (0, -1),
        'D' => (0, 1),
        'R' => (1, 0),
        'L' => (-1, 0),
        _ => panic!("unknown direction {}", direction),
    }
}

fn distance_from_origin((x, y): (isize, isize)) -> usize {
    (x.abs() + y.abs()) as usize
}

fn distance_along_wire((target_x, target_y): (isize, isize), wire_path: &str) -> usize {
    let mut x = 0;
    let mut y = 0;
    let mut distance = 0;

    for segment_str in wire_path.split(",") {
        let segment = PathSegment::parse(&segment_str);
        let (x_delta, y_delta) = direction_to_xy_deltas(&segment.direction);

        for _count in 0..segment.magnitude {
            if x == target_x && y == target_y {
                return distance;
            }

            distance += 1;
            x += x_delta;
            y += y_delta;
        }
    }

    distance
}

#[derive(Clone, PartialEq, Eq, Copy, Hash, Debug)]
enum Cell {
    Origin,
    Occupied(usize),
    Overlap,
}

#[derive(Debug)]
struct Grid<'a> {
    cells: HashMap<(isize, isize), Cell>,
    wire_paths: &'a Vec<&'a str>,
}

impl<'a> Grid<'a> {
    fn new(wire_paths: &'a Vec<&'a str>) -> Grid {
        let mut cells = HashMap::new();
        cells.insert((0, 0), Cell::Origin);
        let mut grid = Grid { cells, wire_paths };

        for (wire_index, wire_path) in wire_paths.iter().enumerate() {
            let mut x = 0;
            let mut y = 0;

            for segment_str in wire_path.split(",") {
                let segment = PathSegment::parse(&segment_str);
                let (x_delta, y_delta) = direction_to_xy_deltas(&segment.direction);

                for _count in 0..segment.magnitude {
                    grid.set_cell_occupied(x, y, wire_index);
                    x += x_delta;
                    y += y_delta;
                }
            }
        }

        grid
    }

    fn set_cell_occupied(&mut self, x: isize, y: isize, wire_index: usize) {
        let new_value = self
            .cells
            .get(&(x, y))
            .map_or(Cell::Occupied(wire_index), |value| match value {
                Cell::Occupied(last_index) => {
                    if *last_index != wire_index {
                        Cell::Overlap
                    } else {
                        Cell::Occupied(*last_index)
                    }
                }
                Cell::Origin => Cell::Origin,
                Cell::Overlap => Cell::Overlap,
            });

        self.cells.insert((x, y), new_value);
    }

    fn get_overlapping_points(&self) -> Vec<(isize, isize)> {
        let mut results = Vec::new();

        for (coordinates, cell) in self.cells.iter() {
            if let Cell::Overlap = cell {
                results.push(*coordinates);
            }
        }

        results
    }

    fn get_closest_overlap_point(&self) -> (isize, isize) {
        let overlapping_points = self.get_overlapping_points();
        let (closest_x, closest_y) = overlapping_points
            .iter()
            .min_by(|a, b| distance_from_origin(**a).cmp(&distance_from_origin(**b)))
            .expect("no closest point?!");

        (*closest_x, *closest_y)
    }

    fn get_sum_distance_along_path(&self, point: (isize, isize)) -> usize {
        assert!(
            self.cells.get(&point) == Some(&Cell::Overlap),
            "point is not an overlap"
        );

        self.wire_paths
            .iter()
            .map(|w| distance_along_wire(point, w))
            .sum()
    }

    fn get_first_path_intersection(&self) -> (isize, isize) {
        *self
            .get_overlapping_points()
            .iter()
            .min_by(|a, b| {
                self.get_sum_distance_along_path(**a)
                    .cmp(&self.get_sum_distance_along_path(**b))
            })
            .unwrap()
    }
}

pub struct DayThree;

impl crate::PuzzleSolver for DayThree {
    fn description(&self) -> &'static str {
        "Day 3: Crossed Wires"
    }

    fn solve(&self, input: &str) {
        let lines = &input.lines().collect();
        let grid = Grid::new(&lines);
        let (target_x, target_y) = grid.get_closest_overlap_point();
        let closest_distance = distance_from_origin((target_x, target_y));
        println!(
            "Part 1: Closest intersection distance: {}",
            closest_distance
        );

        let first_wire_intersection_distance =
            grid.get_sum_distance_along_path(grid.get_first_path_intersection());
        println!(
            "Part 2: Distance along paths: {}",
            first_wire_intersection_distance
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_closest_overlap() {
        let wire_paths = vec![
            "R75,D30,R83,U83,L12,D49,R71,U7,L72",
            "U62,R66,U55,R34,D71,R55,D58,R83",
        ];

        let test_grid = Grid::new(&wire_paths);
        assert_eq!(
            distance_from_origin(test_grid.get_closest_overlap_point()),
            159
        );
    }

    #[test]
    fn test_closest_overlap_two() {
        let wire_paths = vec![
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
            "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
        ];

        let test_grid = Grid::new(&wire_paths);
        assert_eq!(
            distance_from_origin(test_grid.get_closest_overlap_point()),
            135
        );
    }

    #[test]
    fn test_closest_overlap_three() {
        let wire_paths = vec!["R8,U5,L5,D3", "U7,R6,D4,L4"];
        let test_grid = Grid::new(&wire_paths);
        assert_eq!(
            distance_from_origin(test_grid.get_closest_overlap_point()),
            6
        );
    }

    #[test]
    fn test_path_walking() {
        let wire_paths = vec!["R8,U5,L5,D3", "U7,R6,D4,L4"];
        let test_grid = Grid::new(&wire_paths);
        let first_distance =
            test_grid.get_sum_distance_along_path(test_grid.get_first_path_intersection());
        assert_eq!(first_distance, 30);
    }

    #[test]
    fn test_path_walking_two() {
        let wire_paths = vec![
            "R75,D30,R83,U83,L12,D49,R71,U7,L72",
            "U62,R66,U55,R34,D71,R55,D58,R83",
        ];
        let test_grid = Grid::new(&wire_paths);
        let first_distance =
            test_grid.get_sum_distance_along_path(test_grid.get_first_path_intersection());
        assert_eq!(first_distance, 610);
    }

    #[test]
    fn test_path_walking_three() {
        let wire_paths = vec![
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
            "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
        ];
        let test_grid = Grid::new(&wire_paths);
        let first_distance =
            test_grid.get_sum_distance_along_path(test_grid.get_first_path_intersection());
        assert_eq!(first_distance, 410);
    }
}
