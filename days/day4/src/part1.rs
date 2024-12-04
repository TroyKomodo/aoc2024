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
            if *c == 'X' {
                // Check all 8 directions for XMAS
                const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];

                let check_down = y + XMAS.len() <= height;
                let check_up = y >= XMAS.len() - 1;
                let check_right = x + XMAS.len() <= width;
                let check_left = x >= XMAS.len() - 1;

                if check_down {
                    // Check down
                    let mut check = true;
                    for i in 0..XMAS.len() {
                        if input[y + i][x] != XMAS[i] {
                            check = false;
                            break;
                        }
                    }

                    if check {
                        sum += 1;
                        for i in 0..XMAS.len() {
                            xmas_positions[y + i][x] = true;
                        }
                    }

                    if check_left {
                        // Check down-left
                        let mut check = true;
                        for i in 0..XMAS.len() {
                            if input[y + i][x - i] != XMAS[i] {
                                check = false;
                                break;
                            }
                        }

                        if check {
                            sum += 1;
                            for i in 0..XMAS.len() {
                                xmas_positions[y + i][x - i] = true;
                            }
                        }
                    }

                    if check_right {
                        // Check down-right
                        let mut check = true;
                        for i in 0..XMAS.len() {
                            if input[y + i][x + i] != XMAS[i] {
                                check = false;
                                break;
                            }
                        }

                        if check {
                            sum += 1;
                            for i in 0..XMAS.len() {
                                xmas_positions[y + i][x + i] = true;
                            }
                        }
                    }
                }

                if check_up {
                    // Check up
                    let mut check = true;
                    for i in 0..XMAS.len() {
                        if input[y - i][x] != XMAS[i] {
                            check = false;
                            break;
                        }
                    }

                    if check {
                        sum += 1;
                        for i in 0..XMAS.len() {
                            xmas_positions[y - i][x] = true;
                        }
                    }

                    if check_left {
                        // Check up-left
                        let mut check = true;
                        for i in 0..XMAS.len() {
                            if input[y - i][x - i] != XMAS[i] {
                                check = false;
                                break;
                            }
                        }

                        if check {
                            sum += 1;
                            for i in 0..XMAS.len() {
                                xmas_positions[y - i][x - i] = true;
                            }
                        }
                    }

                    if check_right {
                        // Check up-right
                        let mut check = true;
                        for i in 0..XMAS.len() {
                            if input[y - i][x + i] != XMAS[i] {
                                check = false;
                                break;
                            }
                        }

                        if check {
                            sum += 1;
                            for i in 0..XMAS.len() {
                                xmas_positions[y - i][x + i] = true;
                            }
                        }
                    }
                }

                if check_right {
                    // Check right
                    let mut check = true;
                    for i in 0..XMAS.len() {
                        if input[y][x + i] != XMAS[i] {
                            check = false;
                            break;
                        }
                    }

                    if check {
                        sum += 1;
                        for i in 0..XMAS.len() {
                            xmas_positions[y][x + i] = true;
                        }
                    }
                }

                if check_left {
                    // Check left
                    let mut check = true;
                    for i in 0..XMAS.len() {
                        if input[y][x - i] != XMAS[i] {
                            check = false;
                            break;
                        }
                    }

                    if check {
                        sum += 1;
                        for i in 0..XMAS.len() {
                            xmas_positions[y][x - i] = true;
                        }
                    }
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
