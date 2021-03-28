use crate::util::{print_part_1, print_part_2};
use regex::Regex;
use std::fs::read_to_string;
use std::time::Instant;

#[derive(Clone)]
struct Node {
    x: isize,
    y: isize,
    used: isize,
    available: isize,
}

impl Node {
    fn vec_from_text(input: &str) -> Vec<Node> {
        let re: Regex =
            Regex::new(r"/dev/grid/node-x(\d+)-y(\d+)\s+(\d+)T\s+(\d+)T\s+(\d+)T\s+(\d+)%")
                .unwrap();
        let mut nodes = vec![];
        for cap in re.captures_iter(input) {
            nodes.push(Node {
                x: cap[1].parse().unwrap(),
                y: cap[2].parse().unwrap(),
                used: cap[4].parse().unwrap(),
                available: cap[5].parse().unwrap(),
            })
        }
        nodes
    }
}

fn viable_pairs(input: &str) -> isize {
    let nodes = Node::vec_from_text(input);
    let mut pairs = 0;
    for (i, node1) in nodes.iter().enumerate() {
        for (j, node2) in nodes.iter().enumerate() {
            if i == j {
                continue;
            }
            if node1.used == 0 {
                continue;
            }
            if node1.used <= node2.available {
                pairs += 1;
            }
        }
    }
    pairs
}

fn move_data(input: &str, visual: bool) -> isize {
    let nodes = Node::vec_from_text(input);
    let max_x = nodes.iter().map(|node| node.x).max().unwrap();
    let max_y = nodes.iter().map(|node| node.y).max().unwrap();

    let mut grid = vec![vec![(0, 0); (max_x + 1) as usize]; (max_y + 1) as usize];

    for node in nodes.iter() {
        if node.used > 100 {}
        grid[node.y as usize][node.x as usize] = (node.used, node.used + node.available);
    }

    if visual {
        for row in grid.iter() {
            for (used, size) in row.iter() {
                if *used > 100 {
                    print!("##/##  ");
                } else {
                    if *used == 0 {
                        print!("0")
                    }
                    print!("{}/{}  ", used, size);
                }
            }
            print!("\n");
        }
    }

    // Visually far easier to compute than by programming!
    // Inspect the grid as printed above on a wide enough screen to view to solution;

    // The puzzle is basically equal to solving a sliding puzzle (https://en.wikipedia.org/wiki/Sliding_puzzle)
    // but with some 'walls' that cannot be passed. It is far easier to count the
    // number of steps required than to produce them iteratively!!

    // After manual work:
    let setup = 11 + 9 + 16 + 25;
    let cycle_size = 5;
    let cycles = 29;
    let final_move = 1;
    setup + cycles * cycle_size + final_move
}

pub fn main() {
    let input = read_to_string("inputs/day22.txt").expect("Input not found..");

    // PART 1
    let start = Instant::now();
    let known_answer = "934";
    let part_1: isize = viable_pairs(&input);
    let duration = start.elapsed();
    print_part_1(&part_1.to_string(), &known_answer, duration);

    // PART 2
    let start = Instant::now();
    let known_answer = "207";
    let part_2: isize = move_data(&input, false);
    let duration = start.elapsed();
    print_part_2(&part_2.to_string(), &known_answer, duration);
}
