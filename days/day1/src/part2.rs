use std::collections::HashMap;

use utils::*;

fn main() {
    let input = init!();

    let mut list2 = HashMap::<i64, i64>::new();

    let list1 = input.lines().into_iter().map(|line| {
        let mut split = line.split_whitespace();

        let first = parse::<i64>(split.next().unwrap());
        let second = parse::<i64>(split.next().unwrap());

        *list2.entry(second).or_default() += 1;
        first
    }).collect::<Vec<_>>();

    let mut dist = 0;

    for a in list1.iter() {
        dist += list2.get(a).copied().unwrap_or_default() * a;
    }

    info!("result: {}", dist);
}
