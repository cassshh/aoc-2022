fn main() {
    // let str = include_str!("./input/day1.test.txt");
    let str = include_str!("./input/day1.txt");

    let elf_cals: Option<u32> = str
        .split("\n\n")
        .map(|elf| elf.split("\n").flat_map(|cal| cal.parse::<u32>()).sum())
        .max();

    println!("Top total cals: {:?}", elf_cals);
}
