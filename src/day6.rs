use std::collections::{HashMap, HashSet};

const ORIGIN_IDENTIFIER: &'static str = "YOU";
const GOAL_IDENTIFIER: &'static str = "SAN";

#[derive(PartialEq, Eq, Clone, Debug)]
struct Orbit<'a> {
    id: &'a str,
    parent_id: Option<&'a str>,
    orbits: HashSet<&'a str>,
}

fn parse_map(map: &str) -> HashMap<&str, Orbit> {
    let mut orbits = HashMap::new();

    let orbit_pairs: Vec<(&str, &str)> = map
        .lines()
        .map(|l| {
            let mut parts = l.trim().split(")");
            let pair = (parts.next().unwrap(), parts.next().unwrap());
            assert!(parts.next().is_none());
            pair
        })
        .collect();

    for pair in &orbit_pairs {
        let parent = pair.0;
        let child: &str = pair.1;

        if !orbits.contains_key(parent) {
            orbits.insert(
                parent,
                Orbit {
                    id: parent,
                    parent_id: None,
                    orbits: HashSet::new(),
                },
            );
        }

        if !orbits.contains_key(child) {
            orbits.insert(
                child,
                Orbit {
                    id: child,
                    parent_id: Some(parent),
                    orbits: HashSet::new(),
                },
            );
        }
    }

    for pair in &orbit_pairs {
        let parent = pair.0;
        let child = pair.1;

        let parent_orbit = orbits.get_mut(parent).unwrap();
        parent_orbit.orbits.insert(child);

        let child_orbit = orbits.get_mut(child).unwrap();
        child_orbit.parent_id = Some(parent);
    }

    orbits
}

fn depth_in_tree(orbits: &HashMap<&str, Orbit>, id: &str) -> usize {
    let mut depth: usize = 0;
    let mut current_orbit = orbits.get(id).unwrap();

    loop {
        let parent = current_orbit.parent_id;

        match parent {
            Some(id) => {
                depth += 1;
                current_orbit = orbits.get(id).unwrap();
            }
            None => break,
        }
    }

    depth.saturating_sub(1)
}

fn is_descended_from(orbits: &HashMap<&str, Orbit>, parent: &str, descendant: &str) -> bool {
    let parent_orbit = orbits.get(parent).unwrap();

    if parent_orbit.orbits.contains(descendant) {
        return true;
    } else if parent_orbit.orbits.len() == 0 {
        return false;
    } else {
        for child in &parent_orbit.orbits {
            if is_descended_from(orbits, child, descendant) {
                return true;
            }
        }
    }

    false
}

fn minimum_distance_to(orbits: &HashMap<&str, Orbit>, origin: &str, goal: &str) -> usize {
    let mut distance = 0usize;

    let mut common_ancestor = origin;
    while !is_descended_from(orbits, common_ancestor, goal) {
        common_ancestor = orbits.get(common_ancestor).unwrap().parent_id.unwrap();
        distance += 1;
    }

    let distance_to_goal = depth_in_tree(orbits, goal) - depth_in_tree(orbits, common_ancestor);

    distance + distance_to_goal
}

pub struct DaySix;

impl crate::PuzzleSolver for DaySix {
    fn description(&self) -> &'static str {
        "Day Six: Universal Orbit Map"
    }

    fn solve(&self, input: &str) {
        let orbits = parse_map(input);

        // Direct orbits are len(orbits) - 1 (for the COM)
        let direct_orbits = orbits.len() - 1;

        let mut indirect_orbit_count = 0usize;

        for id in orbits.keys() {
            let depth = depth_in_tree(&orbits, id);
            indirect_orbit_count += depth;
        }

        println!(
            "Direct orbit count: {}; indirect: {}; sum: {}",
            direct_orbits,
            indirect_orbit_count,
            direct_orbits + indirect_orbit_count
        );

        let origin_parent = orbits.get(ORIGIN_IDENTIFIER).unwrap().parent_id.unwrap();
        let goal_parent = orbits.get(GOAL_IDENTIFIER).unwrap().parent_id.unwrap();

        println!(
            "{}",
            minimum_distance_to(&orbits, origin_parent, goal_parent)
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! static_hashset {
        [$($items:literal),*] => { [$($items),*].iter().cloned().collect() }
    }

    #[test]
    fn test_parse_map() {
        let input = "COM)B
            B)C
            C)D
            D)E
            E)F
            B)G
            G)H
            D)I
            E)J
            J)K
            K)L";

        let mut expected = HashMap::new();
        expected.insert(
            "COM",
            Orbit {
                id: "COM",
                parent_id: None,
                orbits: static_hashset!["B"],
            },
        );

        expected.insert(
            "B",
            Orbit {
                id: "B",
                parent_id: Some("COM"),
                orbits: static_hashset!["C", "G"],
            },
        );

        expected.insert(
            "G",
            Orbit {
                id: "G",
                parent_id: Some("B"),
                orbits: static_hashset!["H"],
            },
        );

        expected.insert(
            "H",
            Orbit {
                id: "H",
                parent_id: Some("G"),
                orbits: static_hashset![],
            },
        );

        expected.insert(
            "C",
            Orbit {
                id: "C",
                parent_id: Some("B"),
                orbits: static_hashset!["D"],
            },
        );

        expected.insert(
            "D",
            Orbit {
                id: "D",
                parent_id: Some("C"),
                orbits: static_hashset!["I", "E"],
            },
        );

        expected.insert(
            "I",
            Orbit {
                id: "I",
                parent_id: Some("D"),
                orbits: static_hashset![],
            },
        );

        expected.insert(
            "E",
            Orbit {
                id: "E",
                parent_id: Some("D"),
                orbits: static_hashset!["F", "J"],
            },
        );

        expected.insert(
            "F",
            Orbit {
                id: "F",
                parent_id: Some("E"),
                orbits: static_hashset![],
            },
        );

        expected.insert(
            "J",
            Orbit {
                id: "J",
                parent_id: Some("E"),
                orbits: static_hashset!["K"],
            },
        );

        expected.insert(
            "K",
            Orbit {
                id: "K",
                parent_id: Some("J"),
                orbits: static_hashset!["L"],
            },
        );

        expected.insert(
            "L",
            Orbit {
                id: "L",
                parent_id: Some("K"),
                orbits: static_hashset![],
            },
        );

        let parsed = parse_map(&input);

        assert_eq!(
            parsed, expected,
            "\nleft: {:#?}\nright: {:#?}",
            parsed, expected
        );
    }

    #[test]
    fn test_counting() {
        let input = "COM)B
            B)C
            C)D
            D)E
            E)F
            B)G
            G)H
            D)I
            E)J
            J)K
            K)L";

        let orbits = parse_map(&input);
        let direct_orbits = orbits.len() - 1;

        let mut indirect_orbit_count = 0usize;

        for id in orbits.keys() {
            indirect_orbit_count += depth_in_tree(&orbits, id);
        }

        assert_eq!(depth_in_tree(&orbits, "L"), 6);
        assert_eq!(depth_in_tree(&orbits, "D"), 2);
        assert_eq!(direct_orbits + indirect_orbit_count, 42);
    }
}
