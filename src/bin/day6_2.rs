use std::collections::HashSet;

fn main() {
    // let str = include_str!("./input/day6.test.txt");
    let str = include_str!("./input/day6.txt");

    let distinct = 14;

    let mut idx = 0;
    let mut set = HashSet::new();

    while idx < (str.len() - distinct + 1) {
        set.clear();

        let segment = &str[idx..idx + distinct];
        segment.chars().for_each(|c| {
            set.insert(c);
        });

        if set.len() == distinct {
            break;
        }

        idx += 1;
    }

    println!("Message start position: {:?}", idx + distinct);
}
