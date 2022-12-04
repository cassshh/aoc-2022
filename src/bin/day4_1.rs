fn main() {
    // let str = include_str!("./input/day4.test.txt");
    let str = include_str!("./input/day4.txt");

    let full_overlapping_tasks = str
        .lines()
        .filter(|pair| {
            let (elf_1, elf_2) = pair.split_once(',').expect("must exist");

            let (elf_1_start, elf_1_end) = elf_1.split_once('-').expect("must exist").into();
            let (elf_2_start, elf_2_end) = elf_2.split_once('-').expect("must exist");

            let elf_1_start_num = elf_1_start.parse::<u32>().unwrap();
            let elf_1_end_num = elf_1_end.parse::<u32>().unwrap();
            let elf_2_start_num = elf_2_start.parse::<u32>().unwrap();
            let elf_2_end_num = elf_2_end.parse::<u32>().unwrap();

            (elf_1_start_num >= elf_2_start_num && elf_1_end_num <= elf_2_end_num)
                || (elf_2_start_num >= elf_1_start_num && elf_2_end_num <= elf_1_end_num)
        })
        .count();

    println!(
        "Fully overlapping assignment pairs: {:?}",
        full_overlapping_tasks
    );
}
