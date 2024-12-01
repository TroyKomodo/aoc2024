use std::collections::HashMap;

use utils::*;

fn main() {
    let input = init!();

    let mut list1 = Vec::new();
    let mut list2 = HashMap::<i64, i64>::new();

    input.lines().into_iter().for_each(|line| {
        let mut split = line.split_whitespace();

        let first = parse::<i64>(split.next().unwrap());
        let second = parse::<i64>(split.next().unwrap());

        list1.push(first);
        *list2.entry(second).or_default() += 1;

    });

    list1.sort();

    let mut dist = 0;

    for a in list1.iter() {
        dist += list2.get(a).copied().unwrap_or_default() * a;
    }

    println!("dist: {}", dist);
}
