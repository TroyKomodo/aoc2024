use itertools::Itertools;
use utils::*;

const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];

fn check_xmas(input: &Vec<Vec<char>>, y: usize, x: usize, dx: i32, dy: i32) -> bool {
    let height = input.len();
    let width = input[0].len();

    let max_y = y as i32 + dy * (XMAS.len() - 1) as i32;
    let max_x = x as i32 + dx * (XMAS.len() - 1) as i32;

    if max_y < 0 || max_x < 0 || max_y >= height as i32 || max_x >= width as i32 {
        return false;
    }

    for i in 0..XMAS.len() {
        let y = y as i32 + i as i32 * dy;
        let x = x as i32 + i as i32 * dx;

        if input[y as usize][x as usize] != XMAS[i] {
            return false;
        }
    }

    true
}

fn main() {
    let input = init!();

    let mut sum = 0;

    let input = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == 'X' {
                let directions = [
                    (0, 1),
                    (-1, 1),
                    (1, 1),
                    (0, -1),
                    (-1, -1),
                    (1, -1),
                    (1, 0),
                    (-1, 0),
                ];

                for (dx, dy) in directions {
                    if check_xmas(&input, y, x, dx, dy) {
                        sum += 1;
                    }
                }
            }
        }
    }

    println!("{}", sum);
}
