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

pub fn part_one() {
    let r = solve(false);
    println!("{r}");
}

pub fn part_two() {
    let r = solve(true);
    println!("{r}");
}

fn solve(j_wild: bool) -> i64 {
    let contents = std::fs::read_to_string("resources/inputs/day_7.txt").unwrap();

    let lines: Vec<&str> = contents.split('\n').collect();
    let mut hands: Vec<Hand> = vec![];

    for l in lines {
        hands.push(Hand::from_string(l, j_wild).expect("Invalid hand format"));
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

const CARD_ORDERING_WILD: [Card; 13] = [
    Card::J,
    Card::Two,
    Card::Three,
    Card::Four,
    Card::Five,
    Card::Six,
    Card::Seven,
    Card::Eight,
    Card::Nine,
    Card::Ten,
    Card::Q,
    Card::K,
    Card::A,
];

const CARD_ORDERING_STANDARD: [Card; 13] = [
    Card::Two,
    Card::Three,
    Card::Four,
    Card::Five,
    Card::Six,
    Card::Seven,
    Card::Eight,
    Card::Nine,
    Card::Ten,
    Card::J,
    Card::Q,
    Card::K,
    Card::A,
];

#[derive(Eq)]
struct Hand {
    cards: [Card; 5],
    strength: HandStrength,

    bet: i64,

    // used to handle dynamic card orderings
    card_ordering: Vec<Card>,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.strength == other.strength {
            for i in 0..self.cards.len() {
                let c = self.cards[i];
                let o = other.cards[i];

                let c_index = self
                    .card_ordering
                    .iter()
                    .position(|&i| i == c)
                    .expect("Missing card in custom ordering.");
                let o_index = self
                    .card_ordering
                    .iter()
                    .position(|&i| i == o)
                    .expect("Missing card in custom ordering.");

                if c_index < o_index {
                    return Ordering::Less;
                } else if c_index > o_index {
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
    fn from_string(input: &str, j_wild: bool) -> Result<Hand, String> {
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

        if j_wild {
            return Ok(Hand::new_wild(cards, bet));
        } else {
            return Ok(Hand::new_standard(cards, bet));
        }
    }

    fn new_wild(cards: [Card; 5], bet: i64) -> Hand {
        let mut ret = Hand {
            cards,
            strength: HandStrength::FiveKind,
            bet,
            card_ordering: CARD_ORDERING_WILD.to_vec(),
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

    fn new_standard(cards: [Card; 5], bet: i64) -> Hand {
        let mut ret = Hand {
            cards,
            strength: HandStrength::FiveKind,
            bet,
            card_ordering: CARD_ORDERING_STANDARD.to_vec(),
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
    let hand = Hand::new_wild([Card::A, Card::A, Card::A, Card::A, Card::A], 0);
    assert_eq!(hand.strength, HandStrength::FiveKind);

    let hand = Hand::new_wild([Card::A, Card::A, Card::A, Card::A, Card::K], 0);
    assert_eq!(hand.strength, HandStrength::FourKind);

    let hand = Hand::new_wild([Card::A, Card::A, Card::A, Card::K, Card::K], 0);
    assert_eq!(hand.strength, HandStrength::FullHouse);

    let hand = Hand::new_wild(
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

    let hand = Hand::new_wild([Card::A, Card::Q, Card::Q, Card::Q, Card::Ten], 0);
    assert_eq!(hand.strength, HandStrength::ThreeKind);

    let hand = Hand::new_wild([Card::A, Card::A, Card::Q, Card::Q, Card::K], 0);
    assert_eq!(hand.strength, HandStrength::TwoPair);

    let hand = Hand::new_wild([Card::A, Card::A, Card::Eight, Card::Two, Card::K], 0);
    assert_eq!(hand.strength, HandStrength::OnePair);

    let hand = Hand::new_wild([Card::A, Card::Q, Card::K, Card::Eight, Card::Ten], 0);
    assert_eq!(hand.strength, HandStrength::HighCard)
}

// Tests only the hands that are possible with wilds
// Two pairs impossible with wilds. It would always be three of kind.
// High card isn't possible with wild. It would always be a pair.
#[test]
fn build_hand_wild() {
    let hand = Hand::new_wild([Card::A, Card::A, Card::A, Card::A, Card::J], 0);
    assert_eq!(hand.strength, HandStrength::FiveKind);

    let hand = Hand::new_wild([Card::A, Card::A, Card::A, Card::J, Card::K], 0);
    assert_eq!(hand.strength, HandStrength::FourKind);

    let hand = Hand::new_wild([Card::A, Card::A, Card::A, Card::K, Card::J], 0);
    assert_eq!(hand.strength, HandStrength::FourKind);

    let hand = Hand::new_wild(
        [Card::Eight, Card::Seven, Card::Eight, Card::Seven, Card::J],
        0,
    );
    assert_eq!(hand.strength, HandStrength::FullHouse);

    let hand = Hand::new_wild([Card::A, Card::Two, Card::Q, Card::Q, Card::J], 0);
    assert_eq!(hand.strength, HandStrength::ThreeKind);

    let hand = Hand::new_wild([Card::A, Card::K, Card::Q, Card::J, Card::Ten], 0);
    assert_eq!(hand.strength, HandStrength::OnePair);

    let hand = Hand::new_wild([Card::Three, Card::Two, Card::Ten, Card::Three, Card::K], 0);
    assert_eq!(hand.strength, HandStrength::OnePair);

    let hand = Hand::new_wild([Card::Ten, Card::Five, Card::Five, Card::J, Card::Five], 0);
    assert_eq!(hand.strength, HandStrength::FourKind);

    let hand = Hand::new_wild([Card::K, Card::K, Card::Six, Card::Seven, Card::Seven], 0);
    assert_eq!(hand.strength, HandStrength::TwoPair);

    let hand = Hand::new_wild([Card::J, Card::J, Card::J, Card::J, Card::J], 0);
    assert_eq!(hand.strength, HandStrength::FiveKind);

    let hand = Hand::new_wild([Card::J, Card::J, Card::J, Card::J, Card::Q], 0);
    assert_eq!(hand.strength, HandStrength::FiveKind);

    let hand = Hand::new_wild([Card::J, Card::Two, Card::Two, Card::Three, Card::Three], 0);
    assert_eq!(hand.strength, HandStrength::FullHouse);

    let hand = Hand::new_wild([Card::J, Card::Two, Card::Two, Card::Three, Card::Four], 0);
    assert_eq!(hand.strength, HandStrength::ThreeKind);

    let hand = Hand::new_wild([Card::K, Card::K, Card::J, Card::Five, Card::Four], 0);
    assert_eq!(hand.strength, HandStrength::ThreeKind);

    let hand = Hand::new_wild([Card::K, Card::K, Card::Six, Card::Six, Card::K], 0);
    assert_eq!(hand.strength, HandStrength::FullHouse);

    let hand = Hand::new_wild([Card::K, Card::Two, Card::K, Card::K, Card::Two], 0);
    assert_eq!(hand.strength, HandStrength::FullHouse);

    let hand = Hand::new_wild([Card::Q, Card::Two, Card::K, Card::J, Card::J], 0);
    assert_eq!(hand.strength, HandStrength::ThreeKind);
}

#[test]
fn compare_hands() {
    let first = Hand::new_wild([Card::A, Card::A, Card::A, Card::A, Card::A], 0);
    let second = Hand::new_wild([Card::A, Card::A, Card::A, Card::A, Card::K], 0);
    assert!(first > second);

    let first = Hand::new_wild([Card::A, Card::A, Card::A, Card::J, Card::Q], 0);
    let second = Hand::new_wild([Card::A, Card::A, Card::A, Card::A, Card::K], 0);
    assert!(first < second);

    let first = Hand::new_wild([Card::A, Card::A, Card::A, Card::K, Card::A], 0);
    let second = Hand::new_wild([Card::A, Card::A, Card::A, Card::A, Card::Q], 0);
    assert!(first < second);

    let first = Hand::new_wild([Card::A, Card::A, Card::A, Card::Ten, Card::A], 0);
    let second = Hand::new_wild([Card::A, Card::A, Card::A, Card::A, Card::Q], 0);
    assert!(first < second);

    let first = Hand::new_wild([Card::Two, Card::A, Card::A, Card::A, Card::A], 0);
    let second = Hand::new_wild([Card::A, Card::A, Card::A, Card::A, Card::Q], 0);
    assert!(first < second);

    let first = Hand::new_wild(
        [
            Card::Three,
            Card::Three,
            Card::Three,
            Card::Three,
            Card::Two,
        ],
        0,
    );
    let second = Hand::new_wild([Card::Two, Card::A, Card::A, Card::A, Card::A], 0);
    assert!(first > second);
}

#[test]
fn hand_parsing() {
    let first = Hand::from_string("2JKQA 100 ", false).expect("Invalid hand format");
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
            hands.push(Hand::from_string(l, true).expect("Invalid hand format"));
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
            hands.push(Hand::from_string(l, true).expect("Invalid hand format"));
        }
    }

    hands.sort();

    assert_eq!(hands[0].bet, 1);
    assert_eq!(hands[1].bet, 2);
    assert_eq!(hands[2].bet, 3);
    assert_eq!(hands[3].bet, 5);
}
