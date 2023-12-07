use itertools::Itertools;
use std::ops::Deref;

#[derive(Debug)]
struct Game {
    handtype: HandType,
    handscore: (u32, u32, u32, u32, u32),
    bet: u32,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
enum HandType {
    FiveOfKind = 7,
    FourOfKind = 6,
    FullHouse = 5,
    ThreeOfKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

fn identify_hand(hand: &str) -> HandType {
    let card_multiples = hand.chars().counts();
    let multiple_str = card_multiples.values().sorted().join("");

    match multiple_str.deref() {
        "5" => HandType::FiveOfKind,
        "14" => HandType::FourOfKind,
        "23" => HandType::FullHouse,
        "113" => HandType::ThreeOfKind,
        "122" => HandType::TwoPair,
        "1112" => HandType::OnePair,
        "11111" => HandType::HighCard,
        err => panic!("This can't happen on {}", err),
    }
}

fn score_hand(hand: &str) -> (u32, u32, u32, u32, u32) {
    hand.chars()
        .map(|card| match card {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            value => value.to_digit(10).unwrap(),
        })
        .collect_tuple::<(u32, u32, u32, u32, u32)>()
        .unwrap()
}

pub fn process(_input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let lines: Vec<(&str, u32)> = _input
        .lines()
        .map(|line| {
            let (card, bid) = line.trim().split_once(" ").unwrap();
            (card, bid.parse::<u32>().unwrap())
        })
        .collect();

    let mut games: Vec<Game> = vec![];

    for (hand, bid) in lines {
        let known_hand = identify_hand(hand);
        let score = score_hand(hand);

        games.push(Game {
            handtype: known_hand,
            handscore: score,
            bet: bid,
        })
    }

    let sorted = games
        .iter()
        .sorted_by_key(|game| (game.handtype, game.handscore))
        .enumerate()
        .map(|(idx, game)| (idx as u32 + 1) * game.bet)
        .sum::<u32>();

    Ok(sorted.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483";

        let result = process(input).unwrap();
        let answer = "6440".to_string();
        assert_eq!(result, answer);
    }
}
