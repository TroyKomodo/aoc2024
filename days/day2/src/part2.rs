use utils::*;

fn fast(numbers: &mut [i64]) -> bool {
    if numbers.len() < 2 {
        return true;
    }

    let mut increasing = numbers[0] < numbers[1];
    let mut failed = false;

    fn passes(first: i64, second: i64, increasing: bool) -> bool {
        (first - second).abs() <= 3 && first != second && increasing == (first < second)
    }

    for i in 0..numbers.len() - 1 {
        let first = numbers[i];
        let second = numbers[i + 1];

        let success = passes(first, second, increasing)
            && (i != 0
                || numbers
                    .get(i + 2)
                    .is_none_or(|third| passes(second, *third, increasing)));

        if !success {
            if failed {
                return false;
            }

            failed = true;

            let prev = i.checked_sub(1).map(|i| numbers[i]);
            let third = numbers.get(i + 2).copied();

            match (prev, third) {
                (Some(prev), Some(third)) => {
                    let new_increasing = if i == 1 { prev < second } else { increasing };

                    if passes(prev, second, new_increasing) && passes(second, third, new_increasing)
                    {
                        // try to skip first
                        numbers[i] = prev;
                        increasing = new_increasing;
                    } else if passes(first, third, increasing) {
                        // try to skip second
                        numbers[i + 1] = first;
                    } else {
                        return false;
                    }
                }
                (None, Some(third)) => {
                    if passes(second, third, second < third)
                        && numbers
                            .get(i + 3)
                            .map_or(true, |forth| passes(third, *forth, second < third))
                    {
                        // try to skip first
                        increasing = second < third;
                    } else if passes(first, third, first < third)
                        && numbers
                            .get(i + 3)
                            .map_or(true, |forth| passes(third, *forth, first < third))
                    {
                        // try to skip second
                        numbers[i + 1] = first;
                        increasing = first < third;
                    } else {
                        return false;
                    }
                }
                _ => return true,
            }
        }
    }

    true
}

fn main() {
    let input = init!();

    let safe = input
        .lines()
        .filter(|line| {
            let mut numbers = line
                .split_whitespace()
                .map(|s| parse::<i64>(s))
                .collect::<Vec<_>>();

            fast(&mut numbers)
        })
        .count();

    info!("safe: {}", safe);
}
