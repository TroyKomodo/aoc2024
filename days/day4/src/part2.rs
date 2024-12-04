use itertools::Itertools;
use utils::*;

fn main() {
    let input = init!();

    let mut sum = 0;

    let input = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let height = input.len();
    let width = input[0].len();

    let mut xmas_positions = input
        .iter()
        .map(|line| line.iter().map(|_| false).collect_vec())
        .collect_vec();

    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == 'A' {
                let check_down = y + 1 < height;
                let check_up = y >= 1;
                let check_right = x + 1 < width;
                let check_left = x >= 1;

                if !check_up || !check_down || !check_left || !check_right {
                    continue;
                }

                let top_left = input[y - 1][x - 1];
                let top_right = input[y - 1][x + 1];
                let bottom_left = input[y + 1][x - 1];
                let bottom_right = input[y + 1][x + 1];

                if matches!((top_left, bottom_right), ('M', 'S') | ('S', 'M'))
                    && matches!((top_right, bottom_left), ('M', 'S') | ('S', 'M'))
                {
                    sum += 1;
                    xmas_positions[y][x] = true;
                    xmas_positions[y - 1][x - 1] = true;
                    xmas_positions[y - 1][x + 1] = true;
                    xmas_positions[y + 1][x - 1] = true;
                    xmas_positions[y + 1][x + 1] = true;
                }
            }
        }
    }

    for (y, line) in xmas_positions.iter().enumerate() {
        println!(
            "{}",
            line.iter()
                .enumerate()
                .map(|(x, c)| if *c { input[y][x] } else { '.' })
                .collect::<String>()
        );
    }

    println!("{}", sum);
}
