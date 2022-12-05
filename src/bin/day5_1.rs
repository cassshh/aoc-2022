fn main() {
    // let str = include_str!("./input/day5.test.txt");
    let str = include_str!("./input/day5.txt");

    let (cargo, actions) = str.split_once("\n\n").unwrap();

    let mut stacks_of_crates: Vec<Vec<char>> = Vec::new();

    cargo.lines().rev().skip(1).for_each(|cargo| {
        let mut idx = 0;
        let mut crane_idx = 0;

        while idx < cargo.len() {
            if stacks_of_crates.len() <= crane_idx {
                stacks_of_crates.push(Vec::new());
            }

            if cargo[idx..].starts_with('[') {
                let char = cargo.chars().nth(idx + 1).unwrap();
                stacks_of_crates[crane_idx].push(char);
            }

            idx += 4;
            crane_idx += 1;
        }
    });

    actions.lines().for_each(|action| {
        let mut numbers = action
            .split_whitespace()
            .flat_map(|item| item.parse::<usize>())
            .into_iter();
        let amount: usize = numbers.next().unwrap();
        let from_crane_idx: usize = numbers.next().unwrap();
        let to_crane_idx: usize = numbers.next().unwrap();

        for _ in 0..amount {
            let crate_char = stacks_of_crates[from_crane_idx - 1].pop().unwrap();
            stacks_of_crates[to_crane_idx - 1].push(crate_char);
        }
    });

    let top_crates: Vec<char> = stacks_of_crates
        .iter_mut()
        .flat_map(|stack| stack.pop())
        .collect();

    println!(
        "Top packages: {:?}",
        top_crates.into_iter().collect::<String>()
    );
}
