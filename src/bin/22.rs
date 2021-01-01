use aoc2020::utils;
use std::collections::{VecDeque, HashSet};

fn parse(data: &Vec<String>) -> (Vec<usize>, Vec<usize>) {
    let mut player1: Vec<usize> = vec![];
    let mut player2: Vec<usize> = vec![];

    let mut lines = data.iter();
    lines.next(); // Skip player 1 line

    loop {
        let line = lines.next().unwrap();

        if line.starts_with("Player") {
            break;
        }

        player1.push(line.parse::<usize>().unwrap());
    }

    loop {
        let next_line = lines.next();

        if next_line.is_none() {
            break;
        }

        let line = next_line.unwrap();
        player2.push(line.parse::<usize>().unwrap());
    }

    (player1, player2)
}

type GameState = (VecDeque<usize>, VecDeque<usize>);
type GameLog = HashSet<GameState>;

// Return true if player1 won, false otherwise
// Returns the winning deck as well
fn part2_play_game(mut state: GameState) -> (bool, VecDeque<usize>) {
    let mut game_log: GameLog = HashSet::new();

    while state.0.len() > 0 && state.1.len() > 0 {
        if game_log.contains(&state) {
            // Player 1 wins
            return (true, state.0);
        }
        game_log.insert(state.clone());

        let player1_card = state.0.pop_front().unwrap();
        let player2_card = state.1.pop_front().unwrap();

        let player1_won: bool;

        if player1_card <= state.0.len() && player2_card <= state.1.len() {
            // Recurse!
            let recurse_player1_deck: VecDeque<usize> = state.0.iter().take(player1_card).map(|c| c.clone()).collect();
            let recurse_player2_deck: VecDeque<usize> = state.1.iter().take(player2_card).map(|c| c.clone()).collect();
            player1_won = part2_play_game((recurse_player1_deck, recurse_player2_deck)).0;
        } else {
            player1_won = player1_card > player2_card;
        }

        if player1_won {
            state.0.push_back(player1_card);
            state.0.push_back(player2_card);
        } else {
            state.1.push_back(player2_card);
            state.1.push_back(player1_card);
        }
    }

    return if state.0.len() > 0 {
        (true, state.0)
    } else {
        (false, state.1)
    };
}

fn part2(player1: VecDeque<usize>, player2: VecDeque<usize>) -> usize {
    let (_, winning_deck) = part2_play_game((player1, player2));
    calculate_score(&winning_deck)
}

fn calculate_score(deck: &VecDeque<usize>) -> usize {
    deck.iter().rev().enumerate().map(|(i, v)| (i + 1) * v).sum()
}

fn part1(mut player1: VecDeque<usize>, mut player2: VecDeque<usize>) -> usize {
    while player1.len() > 0 && player2.len() > 0 {
        let player1_card = player1.pop_front().unwrap();
        let player2_card = player2.pop_front().unwrap();

        if player1_card > player2_card {
            player1.push_back(player1_card);
            player1.push_back(player2_card);
        } else {
            player2.push_back(player2_card);
            player2.push_back(player1_card);
        }
    }

    return if player1.len() > 0 {
        calculate_score(&player1)
    } else {
        calculate_score(&player2)
    };
}

fn main() {
    let data: Vec<String> = utils::read_lines("./input_data/22.txt");
    let (player1, player2) = parse(&data);

    println!("Part 1: {}", part1(VecDeque::from(player1.clone()), VecDeque::from(player2.clone())));
    println!("Part 2: {}", part2(VecDeque::from(player1), VecDeque::from(player2)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let data: Vec<String> = vec![
            "Player 1:",
            "9",
            "2",
            "6",
            "3",
            "1",
            "Player 2:",
            "5",
            "8",
            "4",
            "7",
            "10"
        ].iter().map(|l| l.to_string()).collect();

        let (player1, player2) = parse(&data);
        assert_eq!(player1, vec![9, 2, 6, 3, 1]);
        assert_eq!(player2, vec![5, 8, 4, 7, 10]);

        assert_eq!(part1(VecDeque::from(player1.clone()), VecDeque::from(player2.clone())), 306);
        assert_eq!(part2(VecDeque::from(player1), VecDeque::from(player2)), 291);
    }
}