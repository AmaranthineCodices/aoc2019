use std::collections::HashMap;

#[derive(Clone, PartialEq, Eq, Copy, Hash, Debug)]
enum Cell {
    Origin,
    Occupied(usize),
    Overlap,
}

#[derive(Debug)]
struct Grid {
    cells: HashMap<(isize, isize), Cell>,
}

impl Grid {
    fn new() -> Grid {
        let mut cells = HashMap::new();
        cells.insert((0, 0), Cell::Origin);

        Grid { cells }
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

    fn get_closest_overlap_distance(&self) -> isize {
        let overlapping_points = self.get_overlapping_points();
        let (closest_x, closest_y) = overlapping_points
            .iter()
            .min_by(|a, b| manhattan_distance(**a, (0, 0)).cmp(&manhattan_distance(**b, (0, 0))))
            .expect("no closest point?!");

        println!("{}, {}", closest_x, closest_y);

        manhattan_distance((*closest_x, *closest_y), (0, 0))
    }
}

fn parse_grid(wire_paths: Vec<&str>) -> Grid {
    let mut grid = Grid::new();

    for (wire_index, wire_path) in wire_paths.iter().enumerate() {
        let mut x = 0;
        let mut y = 0;

        for path_segment in wire_path.split(",") {
            let direction = path_segment.chars().nth(0).unwrap();
            let magnitude = path_segment
                .chars()
                .skip(1)
                .collect::<String>()
                .parse::<isize>()
                .unwrap();

            let (x_delta, y_delta) = match direction {
                'U' => (0, -1),
                'D' => (0, 1),
                'R' => (1, 0),
                'L' => (-1, 0),
                _ => panic!("unknown direction {}", direction),
            };
            for _count in 0..magnitude {
                grid.set_cell_occupied(x, y, wire_index);
                x += x_delta;
                y += y_delta;
            }
        }
    }

    grid
}

fn manhattan_distance((a_x, a_y): (isize, isize), (b_x, b_y): (isize, isize)) -> isize {
    (a_x - b_x).abs() + (a_y - b_y).abs()
}

pub struct DayThree;

impl crate::PuzzleSolver for DayThree {
    fn description(&self) -> &'static str {
        "Day 3: Crossed Wires"
    }

    fn solve(&self, input: &str) {
        let grid = parse_grid(input.lines().collect());
        let closest_distance = grid.get_closest_overlap_distance();
        println!("{}", closest_distance);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid() {
        let test_grid = parse_grid(vec![
            "R75,D30,R83,U83,L12,D49,R71,U7,L72",
            "U62,R66,U55,R34,D71,R55,D58,R83",
        ]);
        assert_eq!(test_grid.get_closest_overlap_distance(), 159);
    }

    #[test]
    fn test_grid_two() {
        let test_grid = parse_grid(vec![
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
            "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
        ]);
        assert_eq!(test_grid.get_closest_overlap_distance(), 135);
    }

    #[test]
    fn test_grid_three() {
        let test_grid = parse_grid(vec!["R8,U5,L5,D3", "U7,R6,D4,L4"]);
        assert_eq!(test_grid.get_closest_overlap_distance(), 6);
    }
}
