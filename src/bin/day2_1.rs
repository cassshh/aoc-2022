fn main() {
    // let str = include_str!("./input/day2.test.txt");
    let str = include_str!("./input/day2.txt");

    let score: u32 = str
        .split("\n")
        .map(|game| game.replace(" ", ""))
        .map(|game| {
            let mut game_points: u32 = 0;

            let match_points = match game.as_str() {
                "AY" | "BZ" | "CX" => 6, // Win
                "AX" | "BY" | "CZ" => 3, // Draw
                "AZ" | "BX" | "CY" => 0, // Lose
                _ => panic!("Not possible"),
            };
            game_points += match_points;

            let hand_points = match game.chars().last().unwrap().to_string().as_str() {
                "X" => 1,
                "Y" => 2,
                "Z" => 3,
                _ => panic!("Not possible"),
            };
            game_points += hand_points;

            game_points
        })
        .sum();

    println!("Score: {:?}", score);
}
