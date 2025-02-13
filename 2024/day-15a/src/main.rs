use anyhow::{anyhow, Context, Result};
use std::io;

fn main() -> Result<()> {
    let mut warehouse: Vec<Vec<char>> = Vec::new();
    let mut lines = io::stdin().lines();
    loop {
        let line = lines.next().context("Failed to read line")??;
        if line.trim().is_empty() {
            break;
        }
        warehouse.push(line.chars().collect());
    }
    let mut moves = Vec::new();
    for line in lines {
        line?.trim().chars().for_each(|c| moves.push(c));
    }

    // dbg!(&moves);

    play_moves(&mut warehouse, &moves)?;
    // print_warehouse(&warehouse);

    let mut total = 0;
    warehouse
        .iter()
        .enumerate()
        .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, &c)| (i, j, c)))
        .filter(|(_i, _j, c)| *c == 'O')
        .for_each(|(i, j, _c)| total += 100 * i + j);
    println!("Total: {}", total);

    Ok(())
}

#[allow(dead_code)]
fn print_warehouse(warehouse: &Vec<Vec<char>>) {
    for i in 0..warehouse.len() {
        for j in 0..warehouse[i].len() {
            print!("{}", warehouse[i][j]);
        }
        println!();
    }
    println!("----------------------------------------------------------------------");
}

fn play_moves(warehouse: &mut Vec<Vec<char>>, moves: &[char]) -> Result<()> {
    let robot = warehouse
        .iter()
        .enumerate()
        .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, &c)| (i, j, c)))
        .find(|(_i, _j, c)| *c == '@')
        .context("Robot not found")?;
    let mut robot = (robot.0, robot.1);
    // dbg!(robot);

    for &m in moves {
        // print_warehouse(&warehouse);
        // dbg!(m);
        // dbg!(robot);

        let orient: (isize, isize) = match m {
            '<' => (0, -1),
            '^' => (-1, 0),
            '>' => (0, 1),
            'v' => (1, 0),
            _ => return Err(anyhow!("Invalid move")),
        };

        let next_coords = (robot.0 as isize + orient.0, robot.1 as isize + orient.1);
        let next_coords = (next_coords.0 as usize, next_coords.1 as usize);

        let next = warehouse[next_coords.0][next_coords.1];
        match next {
            // Wall -> no-op
            '#' => {}
            'O' => {
                //   If we're given a stack of boxes, move them to the space after the last box and move robot to the space the first box was in.
                //   If we're not given a stack of boxes, there must be a wall so this is a no-op!
                let boxes = get_box_stack(&warehouse, next_coords, orient);
                // dbg!(&boxes);
                if !boxes.is_empty() {
                    let last_box = boxes.last().unwrap();
                    warehouse[(last_box.0 as isize + orient.0) as usize]
                        [(last_box.1 as isize + orient.1) as usize] = 'O';
                    let first_box = boxes.first().unwrap();
                    warehouse[robot.0][robot.1] = '.';
                    robot = *first_box;
                    warehouse[robot.0][robot.1] = '@';
                }
            }
            _ => {
                warehouse[robot.0][robot.1] = '.';
                robot = next_coords;
                warehouse[robot.0][robot.1] = '@';
            }
        }
    }

    Ok(())
}

// Return "stack" of boxes in the given orientation that need to move.
//   * Assume start is a box
//   * Go in given direction until we hit something that's not a box
//   * If it's an empty space, return our list of boxes
//   * If it's a box, add it and continue looking
//   * If it's not: it's a wall, return no box stack
fn get_box_stack(
    warehouse: &Vec<Vec<char>>,
    start: (usize, usize),
    orient: (isize, isize),
) -> Vec<(usize, usize)> {
    let mut boxes = vec![start];

    // dbg!(orient);
    let mut next = (start.0 as isize, start.1 as isize);
    loop {
        next = (next.0 + orient.0, next.1 + orient.1);
        let next = (next.0 as usize, next.1 as usize);

        // dbg!(&warehouse[next.0][next.1]);
        match warehouse[next.0][next.1] {
            'O' => boxes.push(next),
            c => {
                // Wall
                if c == '#' {
                    boxes.clear();
                }
                break;
            }
        }
    }
    boxes
}
