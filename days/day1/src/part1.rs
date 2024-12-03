use utils::*;

fn main() {
    let input = init!();

    let (mut list1, mut list2) = input
        .lines()
        .map(|line| {
            let mut split = line.split_whitespace();

            let first = parse::<i64>(split.next().unwrap());
            let second = parse::<i64>(split.next().unwrap());

            (first, second)
        })
        .unzip::<_, _, Vec<_>, Vec<_>>();

    list1.sort();
    list2.sort();

    let mut dist = 0;

    for (a, b) in list1.iter().zip(list2.iter()) {
        dist += (a - b).abs();
    }

    info!("result: {}", dist);
}
