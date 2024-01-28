#![allow(
    unused_variables,
    dead_code,
    unused_mut,
    unused_imports,
    unused_assignments
)]

use std::cmp::Ordering;
use std::collections::HashMap;

mod hand;

pub fn run() {
    // TODO this will give answer to part_two. Need to fix that to handle both/
    let x = part_one();
    println!("{x}");
}

fn part_one() -> i64 {
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
    J,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
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

        let mut jack_count = 0;
        for c in cards {
            if c == Card::J {
                jack_count += 1;
            }

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

            // jack counts are special cased, so that they don't interact with eachother
            if *key == Card::J {
                continue;
            }

            if *value == 5 - jack_count {
                five = true;
            }
            if *value == 4 - jack_count {
                four = true;
            }
            if *value == 3 - jack_count {
                three = true;
            }
            if *value == 2 - jack_count {
                two = true;
                two_count += 1;
            }
        }

        //handle cases of all jacks.
        if jack_count == 5 {
            five = true;
        }
        if jack_count == 4 {
            four = true;
        }
        if jack_count == 3 {
            three = true;
        }
        if jack_count == 2 {
            two = true;
            two_count += 1;
        }

        // check easy ones first
        if five {
            ret.strength = HandStrength::FiveKind;
            return ret;
        } else if four {
            ret.strength = HandStrength::FourKind;
            return ret;
        }

        // full house
        for (key, value) in &cards_count {
            // found tripplet
            if *value + jack_count == 3 {

                // check for a pair without jacks
                for (key_inner, value_inner) in &cards_count {
                    if *key_inner != Card::J && *key_inner != *key && *value_inner == 2 {
                        ret.strength = HandStrength::FullHouse;
                        return ret;
                    }
                }
            }
        }

        // Check for more counts now that we know we don't have full house
        if three {
            ret.strength = HandStrength::ThreeKind;
            return ret;
        }

        // Two pair isn't possible if there is a jack. It would always be three of kind
        if jack_count == 0 && two_count == 2 {
            ret.strength = HandStrength::TwoPair;
            return ret;
        }

        if two {
            ret.strength = HandStrength::OnePair;
            return ret;
        }

        ret.strength = HandStrength::HighCard;
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

    let hand = Hand::new([Card::A, Card::Q, Card::K, Card::Eight, Card::Ten], 0);
    assert_eq!(hand.strength, HandStrength::HighCard)
}

// Tests only the hands that are possible with wilds
// Two pairs impossible with wilds. It would always be three of kind.
// High card isn't possible with wild. It would always be a pair.
#[test]
fn build_hand_wild() {
    let hand = Hand::new([Card::A, Card::A, Card::A, Card::A, Card::J], 0);
    assert_eq!(hand.strength, HandStrength::FiveKind);

    let hand = Hand::new([Card::A, Card::A, Card::A, Card::J, Card::K], 0);
    assert_eq!(hand.strength, HandStrength::FourKind);

    let hand = Hand::new([Card::A, Card::A, Card::A, Card::K, Card::J], 0);
    assert_eq!(hand.strength, HandStrength::FourKind);

    let hand = Hand::new(
        [Card::Eight, Card::Seven, Card::Eight, Card::Seven, Card::J],
        0,
    );
    assert_eq!(hand.strength, HandStrength::FullHouse);

    let hand = Hand::new([Card::A, Card::Two, Card::Q, Card::Q, Card::J], 0);
    assert_eq!(hand.strength, HandStrength::ThreeKind);

    let hand = Hand::new([Card::A, Card::K, Card::Q, Card::J, Card::Ten], 0);
    assert_eq!(hand.strength, HandStrength::OnePair);

    let hand = Hand::new([Card::Three, Card::Two, Card::Ten, Card::Three, Card::K], 0);
    assert_eq!(hand.strength, HandStrength::OnePair);

    let hand = Hand::new([Card::Ten, Card::Five, Card::Five, Card::J, Card::Five], 0);
    assert_eq!(hand.strength, HandStrength::FourKind);

    let hand = Hand::new([Card::K, Card::K, Card::Six, Card::Seven, Card::Seven], 0);
    assert_eq!(hand.strength, HandStrength::TwoPair);

    let hand = Hand::new([Card::J, Card::J, Card::J, Card::J, Card::J], 0);
    assert_eq!(hand.strength, HandStrength::FiveKind);

    let hand = Hand::new([Card::J, Card::J, Card::J, Card::J, Card::Q], 0);
    assert_eq!(hand.strength, HandStrength::FiveKind);

    let hand = Hand::new([Card::J, Card::Two, Card::Two, Card::Three, Card::Three], 0);
    assert_eq!(hand.strength, HandStrength::FullHouse);

    let hand = Hand::new([Card::J, Card::Two, Card::Two, Card::Three, Card::Four], 0);
    assert_eq!(hand.strength, HandStrength::ThreeKind);

    let hand = Hand::new([Card::K, Card::K, Card::J, Card::Five, Card::Four], 0);
    assert_eq!(hand.strength, HandStrength::ThreeKind);

    let hand = Hand::new([Card::K, Card::K, Card::Six, Card::Six, Card::K], 0);
    assert_eq!(hand.strength, HandStrength::FullHouse);

    let hand = Hand::new([Card::K, Card::Two, Card::K, Card::K, Card::Two], 0);
    assert_eq!(hand.strength, HandStrength::FullHouse);

    let hand = Hand::new([Card::Q, Card::Two, Card::K, Card::J, Card::J], 0);
    assert_eq!(hand.strength, HandStrength::ThreeKind);
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
fn hand_ranking() {
    let contents = std::fs::read_to_string("resources/day_7/day_7_sample.txt").unwrap();

    let lines: Vec<&str> = contents.split('\n').collect();
    let mut hands: Vec<Hand> = vec![];

    for l in lines {
        if l.len() > 0 {
            println!("{l}");
            hands.push(Hand::from_string(l).expect("Invalid hand format"));
        }
    }

    hands.sort();

    assert_eq!(hands[0].bet, 765);
    assert_eq!(hands[1].bet, 28);
    assert_eq!(hands[2].bet, 684);
    assert_eq!(hands[3].bet, 483);
    assert_eq!(hands[4].bet, 220);

    let mut accum: i64 = 0;
    for i in 0..hands.len() {
        accum += (i as i64 + 1) * hands[i].bet;
    }
    assert_eq!(accum, 5905);
}

#[test]
fn sample_hard() {
    let contents = std::fs::read_to_string("resources/day_7/day_7_sample_hard.txt").unwrap();

    let lines: Vec<&str> = contents.split('\n').collect();
    let mut hands: Vec<Hand> = vec![];

    for l in lines {
        if l.len() > 0 {
            println!("{l}");
            hands.push(Hand::from_string(l).expect("Invalid hand format"));
        }
    }

    hands.sort();

    assert_eq!(hands[0].bet, 1);
    assert_eq!(hands[1].bet, 2);
    assert_eq!(hands[2].bet, 3);
    assert_eq!(hands[3].bet, 5);
}


#[test]
fn part_two_answer() {
    assert_eq!(part_one(), 253907829);
}

// Disabled because data is setup for part_two.
/*
#[test]
fn part_one_answer() {
assert_eq!(part_one(), 253205868);
}
*/
