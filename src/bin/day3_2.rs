fn main() {
    // let str = include_str!("./input/day3.test.txt");
    let str = include_str!("./input/day3.txt");

    let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    let rucksacks: Vec<&str> = str.split("\n").collect();
    let chunks: Vec<Vec<&str>> = rucksacks.chunks(3).map(|group| group.into()).collect();
    let prio_sum: usize = chunks
        .iter()
        .map(|group| {
            let first = group.get(0).unwrap();
            let second = group.get(1).unwrap();
            let third = group.get(2).unwrap();
            let duplicate = first
                .chars()
                .find(|char| second.contains(*char) && third.contains(*char));

            alphabet.find(duplicate.unwrap()).unwrap() + 1
        })
        .sum();

    println!("Total prio: {:?}", prio_sum);
}
