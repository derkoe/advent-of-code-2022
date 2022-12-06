use std::fs;

fn main() {
    // Parse the input
    let input = fs::read_to_string("input.txt")
        .expect("Failed to read input file");
    let mut lines = input.lines();

    // Initialize the score
    let mut score = 0;

    // Play each round of the tournament
    while let Some(line) = lines.next() {
        let opponent = line.chars().next().unwrap();
        let player = line.chars().nth(2).unwrap();

        println!("Opponent: {}, Player: {}", opponent, player);

        // Calculate the score for the round
        let round_score = match (opponent, player) {
            ('A', 'X') | ('B', 'Y') | ('C', 'Z') => 3,
            ('A', 'Z') | ('B', 'X') | ('C', 'Y') => 0,
            _ => 6 
        };

        // Update the total score if the player shape is valid
        if player == 'X' || player == 'Y' || player == 'Z' {
            score += match player {
                'X' => 1 + round_score,
                'Y' => 2 + round_score,
                'Z' => 3 + round_score,
                _ => panic!("Invalid player shape")
            };
        }
    }

    // Print the result
    println!("Total score: {}", score);
}
