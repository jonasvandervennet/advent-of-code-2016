use crate::util::{print_part_1, print_part_2};
use std::collections::{HashMap, HashSet};
use std::time::Instant;

fn get_number_of_one_bits(mut decimal: usize) -> usize {
    let mut counter = 0;
    while decimal > 0 {
        if decimal % 2 == 1 {
            counter += 1
        }
        decimal /= 2;
    }
    counter
}

fn is_wall(pos: (usize, usize), design_number: usize) -> bool {
    let x = pos.0;
    let y = pos.1;
    let value = x * x + 3 * x + 2 * x * y + y + y * y + design_number;
    get_number_of_one_bits(value) % 2 == 1
}

fn get_neighbours(position: (usize, usize), design_number: usize) -> Vec<(usize, usize)> {
    let src_x = position.0;
    let src_y = position.1;

    let mut possible_neighbours = vec![(src_x + 1, src_y), (src_x, src_y + 1)];
    if src_x > 0 {
        possible_neighbours.push((src_x - 1, src_y))
    };
    if src_y > 0 {
        possible_neighbours.push((src_x, src_y - 1))
    };

    possible_neighbours
        .into_iter()
        .filter(|&x| !is_wall(x, design_number))
        .collect()
}

fn find_distance_to_destination(
    design_number: usize,
    destination: (usize, usize),
    part: usize,
) -> usize {
    let source = (1, 1);
    let mut distance_map = HashMap::new();
    let mut vertices_todo = HashSet::new();
    let mut vertices_done = HashSet::new();
    vertices_todo.insert(source);
    distance_map.insert(source, 0);

    while !vertices_todo.is_empty() {
        let next_permanent = vertices_todo.iter().min().unwrap().to_owned();
        if part == 1 && next_permanent == destination {
            return distance_map[&next_permanent];
        };

        vertices_todo.remove(&next_permanent);
        vertices_done.insert(next_permanent);

        for &neighbour in get_neighbours(next_permanent, design_number).iter() {
            if vertices_done.contains(&neighbour) {
                continue;
            }
            // manhattan distance to neighbour is always 1
            let new_distance = distance_map[&next_permanent] + 1;
            if part == 2 && new_distance > 50 {
                continue;
            }
            distance_map
                .entry(neighbour)
                .and_modify(|e| {
                    if new_distance < *e {
                        *e = new_distance
                    }
                })
                .or_insert(new_distance);
            vertices_todo.insert(neighbour);
        }
    }
    if part == 2 {
        return distance_map.iter().count();
    }
    panic!("Could not reach destination!");
}

pub fn main() {
    let input = 1352;

    // PART 1
    let start = Instant::now();
    let known_answer = "90";
    let part_1: usize = find_distance_to_destination(input, (31, 39), 1);
    let duration = start.elapsed();
    print_part_1(&part_1.to_string(), &known_answer, duration);

    // PART 2
    let start = Instant::now();
    let known_answer = "135";
    let part_2: usize = find_distance_to_destination(input, (31, 39), 2);
    let duration = start.elapsed();
    print_part_2(&part_2.to_string(), &known_answer, duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input = 10;
        let answer: usize = find_distance_to_destination(input, (7, 4), 1);
        assert_eq!(answer, 11);
    }
}
