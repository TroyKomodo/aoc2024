use utils::*;

fn main() {
    let input = init!();

    let numbers = input.lines().map(|line| {
        line.split_whitespace().map(|s| parse::<i64>(s)).collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    let safe = numbers.iter().filter(|numbers| {
        let mut increasing = None;

        for chunk in numbers.windows(2) {
            if increasing.is_none() {
                increasing = Some(chunk[0] < chunk[1]);
            } else if increasing.unwrap() != (chunk[0] < chunk[1]) {
                return false;
            }

            if (chunk[0] - chunk[1]).abs() > 3 || chunk[0] == chunk[1] {
                return false;
            }
        }

        true
    }).count();

    info!("safe: {}", safe);
}
