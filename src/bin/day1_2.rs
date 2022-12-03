fn main() {
    // let str = include_str!("./input/day1.test.txt");
    let str = include_str!("./input/day1.txt");

    let mut elf_cals: Vec<u32> = str
        .split("\n\n")
        .map(|elf| elf.split("\n").flat_map(|cal| cal.parse::<u32>()).sum())
        .collect();

    elf_cals.sort_by(|a, b| b.cmp(a));

    println!(
        "Top three total cals: {:?}",
        elf_cals.iter().take(3).sum::<u32>()
    );
}
