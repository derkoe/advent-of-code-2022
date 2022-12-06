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
        let result = line.chars().nth(2).unwrap();

        // Calculate the score for the round
        let player = match (opponent, result) {
            ('A', 'X') => 'Z',
            ('B', 'X') => 'X',
            ('C', 'X') => 'Y',
            ('A', 'Y') => 'X',
            ('B', 'Y') => 'Y',
            ('C', 'Y') => 'Z',
            ('A', 'Z') => 'Y',
            ('B', 'Z') => 'Z',
            ('C', 'Z') => 'X',
            _ => 'A' 
        };


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
