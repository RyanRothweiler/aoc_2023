#![allow(unused_variables, dead_code, unused_mut, unused_imports)]

use std::cmp::Ordering;
use std::collections::HashMap;

mod hand;

pub fn run() {
    let x = part_one();
    println!("{x}");
}

fn part_one() -> i64{
    let contents =
        std::fs::read_to_string("resources/day_7/day_7_input.txt").expect("Invalid file");

    let lines: Vec<&str> = contents.split('\n').collect();
    let mut hands: Vec<Hand> = vec![];

    for l in lines {
        hands.push(Hand::from_string(l).expect("Invalid hand format"));
    }

    hands.sort();
    let mut accum: i64 = 0;
    for i in 0..hands.len() {
        accum += (i as i64 + 1) * hands[i].bet;
    }
    accum
}

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash, Ord, PartialOrd)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    J,
    Q,
    K,
    A,
}

impl Card {
    fn from_char(input: char) -> Result<Card, String> {
        match input {
            'A' => return Ok(Card::A),
            'K' => return Ok(Card::K),
            'Q' => return Ok(Card::Q),
            'J' => return Ok(Card::J),
            'T' => return Ok(Card::Ten),
            '9' => return Ok(Card::Nine),
            '8' => return Ok(Card::Eight),
            '7' => return Ok(Card::Seven),
            '6' => return Ok(Card::Six),
            '5' => return Ok(Card::Five),
            '4' => return Ok(Card::Four),
            '3' => return Ok(Card::Three),
            '2' => return Ok(Card::Two),
            _ => {
                return Err("Invalid card character".to_string());
            }
        };
    }
}

#[derive(Eq, PartialEq, PartialOrd, Debug, Copy, Clone)]
enum HandStrength {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

#[derive(Eq)]
struct Hand {
    cards: [Card; 5],
    strength: HandStrength,

    bet: i64,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.strength == other.strength {
            for i in 0..self.cards.len() {
                let c = self.cards[i];
                let o = other.cards[i];

                if self.cards[i] < other.cards[i] {
                    return Ordering::Less;
                } else if self.cards[i] > other.cards[i] {
                    return Ordering::Greater;
                }
            }

            // if we get here then all cards are equal, so its exactly the same hand
            return Ordering::Equal;
        }

        if self.strength < other.strength {
            return Ordering::Less;
        }

        return Ordering::Greater;
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.strength == other.strength
    }
}

impl Hand {
    //format is  "AAQKJ 1234"
    fn from_string(input: &str) -> Result<Hand, String> {
        let parts: Vec<&str> = input.trim().split(' ').collect();
        if parts.len() != 2 {
            return Err("Invalid hand format.".to_string());
        }

        // get cards array
        let mut cards: [Card; 5] = [Card::A; 5];
        let chars: Vec<char> = parts[0].chars().collect();
        for i in 0..chars.len() {
            cards[i] = match Card::from_char(chars[i]) {
                Ok(v) => v,
                Err(e) => {
                    return Err(e);
                }
            }
        }

        // get bet
        let bet: i64 = match parts[1].trim().parse() {
            Ok(v) => v,
            Err(e) => {
                return Err(e.to_string());
            }
        };

        return Ok(Hand::new(cards, bet));
    }

    fn new(cards: [Card; 5], bet: i64) -> Hand {
        let mut ret = Hand {
            cards,
            strength: HandStrength::FiveKind,
            bet,
        };

        let mut cards_count: HashMap<Card, i32> = HashMap::new();

        for c in cards {
            let count: &mut i32 = cards_count.entry(c).or_insert(0);
            *count += 1;
        }

        // Five of a kind
        let mut five = false;
        let mut four = false;
        let mut three = false;
        let mut two = false;
        let mut two_count = 0;
        for (key, value) in &cards_count {
            if *value == 5 {
                five = true;
            }
            if *value == 4 {
                four = true;
            }
            if *value == 3 {
                three = true;
            }
            if *value == 2 {
                two = true;
                two_count += 1;
            }
        }

        if five {
            ret.strength = HandStrength::FiveKind;
        } else if four {
            ret.strength = HandStrength::FourKind;
        } else if three && two {
            ret.strength = HandStrength::FullHouse;
        } else if three {
            ret.strength = HandStrength::ThreeKind;
        } else if two_count == 2 {
            ret.strength = HandStrength::TwoPair;
        } else if two {
            ret.strength = HandStrength::OnePair;
        } else {
            ret.strength = HandStrength::HighCard;
        }

        return ret;
    }
}

#[test]
fn build_hand() {
    let hand = Hand::new([Card::A, Card::A, Card::A, Card::A, Card::A], 0);
    assert_eq!(hand.strength, HandStrength::FiveKind);

    let hand = Hand::new([Card::A, Card::A, Card::A, Card::A, Card::K], 0);
    assert_eq!(hand.strength, HandStrength::FourKind);

    let hand = Hand::new([Card::A, Card::A, Card::A, Card::K, Card::K], 0);
    assert_eq!(hand.strength, HandStrength::FullHouse);

    let hand = Hand::new(
        [
            Card::Eight,
            Card::Seven,
            Card::Eight,
            Card::Seven,
            Card::Eight,
        ],
        0,
    );
    assert_eq!(hand.strength, HandStrength::FullHouse);

    let hand = Hand::new([Card::A, Card::Q, Card::Q, Card::Q, Card::Ten], 0);
    assert_eq!(hand.strength, HandStrength::ThreeKind);

    let hand = Hand::new([Card::A, Card::A, Card::Q, Card::Q, Card::K], 0);
    assert_eq!(hand.strength, HandStrength::TwoPair);

    let hand = Hand::new([Card::A, Card::A, Card::Eight, Card::Two, Card::K], 0);
    assert_eq!(hand.strength, HandStrength::OnePair);

    let hand = Hand::new([Card::A, Card::Q, Card::K, Card::J, Card::Ten], 0);
    assert_eq!(hand.strength, HandStrength::HighCard)
}

#[test]
fn compare_hands() {
    let first = Hand::new([Card::A, Card::A, Card::A, Card::A, Card::A], 0);
    let second = Hand::new([Card::A, Card::A, Card::A, Card::A, Card::K], 0);
    assert!(first > second);

    let first = Hand::new([Card::A, Card::A, Card::A, Card::J, Card::Q], 0);
    let second = Hand::new([Card::A, Card::A, Card::A, Card::A, Card::K], 0);
    assert!(first < second);

    let first = Hand::new([Card::A, Card::A, Card::A, Card::K, Card::A], 0);
    let second = Hand::new([Card::A, Card::A, Card::A, Card::A, Card::Q], 0);
    assert!(first < second);

    let first = Hand::new([Card::A, Card::A, Card::A, Card::Ten, Card::A], 0);
    let second = Hand::new([Card::A, Card::A, Card::A, Card::A, Card::Q], 0);
    assert!(first < second);

    let first = Hand::new([Card::Two, Card::A, Card::A, Card::A, Card::A], 0);
    let second = Hand::new([Card::A, Card::A, Card::A, Card::A, Card::Q], 0);
    assert!(first < second);

    let first = Hand::new(
        [
            Card::Three,
            Card::Three,
            Card::Three,
            Card::Three,
            Card::Two,
        ],
        0,
    );
    let second = Hand::new([Card::Two, Card::A, Card::A, Card::A, Card::A], 0);
    assert!(first > second);
}

#[test]
fn hand_parsing() {
    let first = Hand::from_string("2JKQA 100 ").expect("Invalid hand format");
    assert_eq!(first.cards[0], Card::Two);
    assert_eq!(first.cards[1], Card::J);
    assert_eq!(first.cards[2], Card::K);
    assert_eq!(first.cards[3], Card::Q);
    assert_eq!(first.cards[4], Card::A);
    assert_eq!(first.bet, 100);
}

#[test]
fn part_one_answer() {
    assert_eq!(part_one(), 253205868);
}
