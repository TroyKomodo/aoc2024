use utils::*;

fn main() {
    let input = init!();

    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    input.lines().into_iter().for_each(|line| {
        let mut split = line.split_whitespace();

        let first = parse::<i64>(split.next().unwrap());
        let second = parse::<i64>(split.next().unwrap());

        list1.push(first);
        list2.push(second);
    });

    list1.sort();
    list2.sort();   

    let mut dist = 0;

    for (a, b) in list1.iter().zip(list2.iter()) {
        dist += (a - b).abs();
    }

    println!("dist: {}", dist);
}
