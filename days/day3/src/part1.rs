use utils::*;

macro_rules! parse_char {
    ($input:expr, $char:literal) => {{
        if $input.peek() != Some(&$char) {
            continue;
        }

        $input.next();
    }};
}

macro_rules! parse_number {
    ($input:expr) => {{
        let mut number = None;

        for _ in 0..3 {
            let Some(next) = $input.peek() else {
                break;
            };

            if next.is_digit(10) {
                number = Some(number.unwrap_or(0u32) * 10 + next.to_digit(10).unwrap());
                $input.next();
            } else {
                break;
            }
        }

        number
    }};
}

fn main() {
    let input = init!();

    let mut sum = 0;

    let mut input = input.chars().peekable();

    while let Some(next) = input.next() {
        if next == 'm' {
            parse_char!(input, 'u');
            parse_char!(input, 'l');
            parse_char!(input, '(');

            let Some(first) = parse_number!(input) else {
                continue;
            };

            parse_char!(input, ',');

            let Some(second) = parse_number!(input) else {
                continue;
            };

            parse_char!(input, ')');

            sum += first * second;
        }
    }

    println!("{}", sum);
}
