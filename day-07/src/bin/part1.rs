fn main() {
    let input = include_str!("../../input1.txt");
    let answer = process(input);
    println!("The answer is: {answer}");
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}
#[derive(Debug)]
struct Hand {
    hand_type: HandType,
    cards: Vec<char>,
    bid: i32,
}

struct Card {
    key: char,
    value: i32,
}

const CARDS: [Card; 13] = [
    Card { key: '2', value: 2 },
    Card { key: '3', value: 3 },
    Card { key: '4', value: 4 },
    Card { key: '5', value: 5 },
    Card { key: '6', value: 6 },
    Card { key: '7', value: 7 },
    Card { key: '8', value: 8 },
    Card { key: '9', value: 9 },
    Card {
        key: 'T',
        value: 10,
    },
    Card {
        key: 'J',
        value: 11,
    },
    Card {
        key: 'Q',
        value: 12,
    },
    Card {
        key: 'K',
        value: 13,
    },
    Card {
        key: 'A',
        value: 14,
    },
];

fn process(input: &str) -> i32 {
    let mut hands = input
        .lines()
        .map(|line| {
            let cards: Vec<char> = line.split_whitespace().collect::<Vec<&str>>()[0]
                .chars()
                .collect();

            let bid = line.split_whitespace().collect::<Vec<&str>>()[1]
                .parse::<i32>()
                .unwrap();

            let matches = of_a_kind(cards.clone());

            let hand_type = hand_type(matches);

            Hand {
                hand_type,
                bid,
                cards,
            }
        })
        .collect::<Vec<Hand>>();

    hands.sort_by(|a, b| {
        if a.hand_type.eq(&b.hand_type) {
            let mut index = 0;
            for i in 0..4 {
                if a.cards[i] == b.cards[i] {
                    index = i + 1;
                } else {
                    break;
                }
            }
            return rank_card(a.cards[index])
                .partial_cmp(&rank_card(b.cards[index]))
                .unwrap();
        }
        return a.hand_type.partial_cmp(&b.hand_type).unwrap();
    });

    hands
        .iter()
        .enumerate()
        .map(|(index, hand)| (index as i32 + 1) * hand.bid)
        .sum()
}

fn of_a_kind(cards: Vec<char>) -> Vec<i32> {
    cards
        .iter()
        .map(|card| {
            cards
                .iter()
                .filter(|check_card| &card == check_card)
                .count() as i32
        })
        .collect::<Vec<i32>>()
}

fn rank_card(card: char) -> i32 {
    let score: i32 = CARDS
        .iter()
        .find(|card_value| card_value.key == card)
        .map(|card| card.value)
        .unwrap();

    score
}

fn hand_type(match_counts: Vec<i32>) -> HandType {
    if match_counts.contains(&5) {
        return HandType::FiveOfAKind;
    }
    if match_counts.contains(&4) {
        return HandType::FourOfAKind;
    }
    if match_counts.contains(&3) && match_counts.contains(&2) {
        return HandType::FullHouse;
    }
    if match_counts.contains(&3) {
        return HandType::ThreeOfAKind;
    }
    if match_counts.contains(&2) {
        let pair_count: Vec<&i32> = match_counts
            .iter()
            .filter(|count| count == &&2)
            .collect::<Vec<&i32>>();

        if pair_count.len() == 4 {
            return HandType::TwoPair;
        }
        return HandType::OnePair;
    }
    HandType::HighCard
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("mock-1.txt");
        let result = 6440;
        assert_eq!(result, process(input));

        let input = include_str!("../../input1.txt");
        let result = 251216224;
        assert_eq!(result, process(input));
    }

    #[test]
    fn it_finds_matches() {
        let input = vec!['2', '3', '4', '5', '6'];
        let result = vec![1, 1, 1, 1, 1];
        assert_eq!(result, of_a_kind(input));

        let input = vec!['2', '3', '4', '5', '2'];
        let result = vec![2, 1, 1, 1, 2];
        assert_eq!(result, of_a_kind(input));

        let input = vec!['2', '2', '2', '2', '2'];
        let result = vec![5, 5, 5, 5, 5];
        assert_eq!(result, of_a_kind(input));

        let input = vec!['2', '2', '3', '3', '3'];
        let result = vec![2, 2, 3, 3, 3];
        assert_eq!(result, of_a_kind(input));
    }

    #[test]
    fn it_ranks_cards() {
        let input = 'T';
        let result = 10;

        assert_eq!(result, rank_card(input));

        let input = '2';
        let result = 2;

        assert_eq!(result, rank_card(input));
    }
}
// 251209354 was wrong. too low
