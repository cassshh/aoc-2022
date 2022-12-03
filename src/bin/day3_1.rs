fn main() {
    // let str = include_str!("./input/day3.test.txt");
    let str = include_str!("./input/day3.txt");

    let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    let prio_sum: usize = str
        .split("\n")
        .map(|rucksack| {
            let (first, second) = rucksack.split_at(rucksack.len() / 2);
            let duplicate = first.chars().find(|char| second.contains(*char));

            alphabet.find(duplicate.unwrap()).unwrap() + 1
        })
        .sum();

    println!("Total prio: {:?}", prio_sum);
}
