use std::{cmp::Ordering, collections::HashMap};

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    if hands.len() == 1 {
        return hands.to_vec();
    }

    let mut winning_hands_vec: Vec<&Hand> = Vec::new();

    let mut hands_parsed: Vec<Hand> = hands
        .iter()
        .map(|s| Hand::from_str(s).expect("Failed to parse hand"))
        .collect();

    hands_parsed.sort_by(|a, b| {
        a.highest_hand()
            .partial_cmp(&b.highest_hand())
            .unwrap_or(Ordering::Equal)
    });

    let highest_hand = hands_parsed
        .last()
        .expect("Parsed hands should have at least one");
    // println!("highest_hand in {:?} = {:?}", hands, highest_hand.str);

    winning_hands_vec.push(highest_hand);

    for i in 1..hands.len() {
        if hands_parsed[i].highest_hand() == highest_hand.highest_hand() {
            winning_hands_vec.push(&hands_parsed[i])
        }
    }

    if winning_hands_vec.len() == 1 {
        return winning_hands_vec.iter().map(|h| h.str).collect();
    }

    winning_hands_vec.sort_by(|a, b| {
        a.highest_card()
            .partial_cmp(&b.highest_card())
            .unwrap_or(Ordering::Equal)
    });

    // println!("ordered by highest card {:?}", winning_hands_vec);

    winning_hands_vec
        .iter()
        .filter(|h| h.highest_card().rank == winning_hands_vec.last().unwrap().highest_card().rank)
        .map(|h| h.str)
        .collect()
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
enum Suite {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

impl Suite {
    fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "H" => Ok(Suite::Hearts),
            "D" => Ok(Suite::Diamonds),
            "C" => Ok(Suite::Clubs),
            "S" => Ok(Suite::Spades),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Hash)]
enum Rank {
    Ace = 14,
    King = 13,
    Queen = 12,
    Jack = 11,
    Ten = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
}

impl Rank {
    fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "A" => Ok(Rank::Ace),
            "K" => Ok(Rank::King),
            "Q" => Ok(Rank::Queen),
            "J" => Ok(Rank::Jack),
            "10" => Ok(Rank::Ten),
            "9" => Ok(Rank::Nine),
            "8" => Ok(Rank::Eight),
            "7" => Ok(Rank::Seven),
            "6" => Ok(Rank::Six),
            "5" => Ok(Rank::Five),
            "4" => Ok(Rank::Four),
            "3" => Ok(Rank::Three),
            "2" => Ok(Rank::Two),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
struct Card {
    rank: Rank,
    suite: Suite,
}

impl Card {
    fn from_str(s: &str) -> Result<Self, ()> {
        let (first_chars, last_char) = s.split_at(s.len() - 1);

        let suite = Suite::from_str(last_char)?;
        let rank = Rank::from_str(first_chars)?;

        Ok(Self { rank, suite })
    }
}

impl Default for Card {
    fn default() -> Self {
        Self {
            rank: Rank::Two,
            suite: Suite::Spades,
        }
    }
}

// struct Cards([Card; 5]);

// impl Cards {
//     fn highest_card(self) -> Card {
//         *self.0.last().expect("")
//     }
// }
//
#[derive(Debug, PartialEq, PartialOrd)]
enum HandRank {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    RoyalFlush,
}

#[derive(Debug)]
struct Hand<'a> {
    str: &'a str,
    cards: [Card; 5],
}

impl<'a> Hand<'a> {
    fn from_str(s: &'a str) -> Result<Self, ()> {
        let mut cards: [Card; 5] = [Card::default(); 5];

        for (i, c) in s.split(" ").enumerate() {
            cards[i] = Card::from_str(c).expect("Failed to parse card");
            if i == 4 {
                break;
            }
        }
        cards.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));

        Ok(Hand { str: s, cards })
    }

    fn highest_hand(&'a self) -> HandRank {
        if self.is_royal_flush() {
            HandRank::RoyalFlush
        } else if self.is_straight_flush() {
            HandRank::StraightFlush
        } else if self.is_four_of_a_kind() {
            HandRank::FourOfAKind
        } else if self.is_full_house() {
            HandRank::FullHouse
        } else if self.is_flush() {
            HandRank::Flush
        } else if self.is_straight() {
            HandRank::Straight
        } else if self.is_three_of_a_kind() {
            HandRank::ThreeOfAKind
        } else if self.is_two_pair() {
            HandRank::TwoPair
        } else if self.is_pair() {
            HandRank::Pair
        } else {
            HandRank::HighCard
        }
    }

    fn highest_card(&'a self) -> Card {
        *self.cards.last().expect("Hand should have cards")
    }

    fn get_ranks(&'a self) -> HashMap<Rank, i32> {
        let mut card_map: HashMap<Rank, i32> = HashMap::new();

        for c in self.cards {
            card_map.entry(c.rank).and_modify(|e| *e += 1).or_insert(1);
        }

        card_map
    }

    fn is_flush(&'a self) -> bool {
        let first_suite = self.cards.first().expect("Hand should have cards").suite;

        self.cards.iter().all(|c| c.suite == first_suite)
    }

    fn is_straight(&'a self) -> bool {
        if self.highest_card().rank < Rank::Five {
            return false;
        }

        if self.cards.map(|c| c.rank) == [Rank::Two, Rank::Three, Rank::Four, Rank::Five, Rank::Ace]
        {
            return true;
        }

        for i in 0..4 {
            if self.cards[i].rank as u32 + 1 != self.cards[i + 1].rank as u32 {
                return false;
            }
        }
        true
    }

    fn is_straight_flush(&'a self) -> bool {
        self.is_straight() && self.is_flush()
    }

    fn is_royal_flush(&'a self) -> bool {
        self.is_straight() && self.is_flush() && self.highest_card().rank == Rank::Ace
    }

    fn is_full_house(&'a self) -> bool {
        let card_map = self.get_ranks();

        if card_map.len() != 2 {
            return false;
        }

        card_map.iter().map(|(_, count)| count).any(|c| *c == 3)
    }

    fn is_two_pair(&'a self) -> bool {
        let card_map = self.get_ranks();

        card_map.len() == 3
    }

    fn is_pair(&'a self) -> bool {
        let card_map = self.get_ranks();

        card_map.len() == 4
    }

    fn is_four_of_a_kind(&'a self) -> bool {
        let card_map = self.get_ranks();

        if card_map.len() != 2 {
            return false;
        }

        card_map.iter().map(|(_, count)| count).any(|c| *c == 4)
    }

    fn is_three_of_a_kind(&'a self) -> bool {
        let card_map = self.get_ranks();

        if card_map.len() != 3 {
            return false;
        }

        card_map.iter().map(|(_, count)| count).any(|c| *c == 3)
    }
}

// #[test]
// fn test_hand_is_straight() {
//     let h1 = Hand::from_str("4S 5H 6C 8D 7H").unwrap();
//     assert!(h1.is_straight());

//     let h2 = Hand::from_str("AS 5H 4C 3D 2H").unwrap();
//     assert!(h2.is_straight());

//     let h3 = Hand::from_str("AS QH KC 10D JH").unwrap();
//     assert!(h3.is_straight());
// }

// #[test]
// fn test_hand_is_flush() {
//     let h1 = Hand::from_str("4S 5S 6S 8S 7S").unwrap();
//     assert!(h1.is_flush());

//     let h2 = Hand::from_str("4C 5S 6S 8S 7S").unwrap();
//     assert!(!h2.is_flush());
// }

// #[test]
// fn test_hand_is_straight_flush() {
//     let h1 = Hand::from_str("4S 5S 6S 8S 7S").unwrap();
//     assert!(h1.is_straight_flush());

//     let h2 = Hand::from_str("4S 5C 6S 8S 7S").unwrap();
//     assert!(!h2.is_straight_flush());
// }

// #[test]
// fn test_hand_is_royal_flush() {
//     let h1 = Hand::from_str("JH 10H KH AH QH").unwrap();
//     assert!(h1.is_royal_flush());

//     let h2 = Hand::from_str("4S 5C 6S 8S 7S").unwrap();
//     assert!(!h2.is_royal_flush());
// }

// #[test]
// fn test_hand_is_full_house() {
//     let h1 = Hand::from_str("3H 5H 5D 3C 3D").unwrap();
//     assert!(h1.is_full_house());

//     let h2 = Hand::from_str("4S 5C 6S 8S 7S").unwrap();
//     assert!(!h2.is_full_house());
// }

// #[test]
// fn test_hand_is_two_pair() {
//     let h1 = Hand::from_str("3H 5H 5D 3C 2D").unwrap();
//     assert!(h1.is_two_pair());

//     let h2 = Hand::from_str("3H 5H 5D 3C 3D").unwrap();
//     assert!(!h2.is_two_pair());
// }

// #[test]
// fn test_hand_is_pair() {
//     let h1 = Hand::from_str("3H 6H 5D 3C 2D").unwrap();
//     assert!(h1.is_pair());

//     let h2 = Hand::from_str("3H 5H 5D 3C 3D").unwrap();
//     assert!(!h2.is_pair());

//     let h3 = Hand::from_str("3H 5H 5D 3C 2D").unwrap();
//     assert!(!h3.is_pair());
// }

// #[test]
// fn test_hand_is_four() {
//     let h1 = Hand::from_str("3H 3S 3D 3C 2D").unwrap();
//     assert!(h1.is_four_of_a_kind());

//     let h2 = Hand::from_str("3H 5H 5D 3C 3D").unwrap();
//     assert!(!h2.is_four_of_a_kind());

//     let h3 = Hand::from_str("3H 5H 5D 3C 2D").unwrap();
//     assert!(!h3.is_four_of_a_kind());
// }

// #[test]
// fn test_hand_is_three() {
//     let h1 = Hand::from_str("3H 3S 3D 7C 2D").unwrap();
//     assert!(h1.is_three_of_a_kind());

//     let h2 = Hand::from_str("3H 5H 5D 3C 3D").unwrap();
//     assert!(!h2.is_three_of_a_kind());

//     let h3 = Hand::from_str("3H 5H 5D 3C 2D").unwrap();
//     assert!(!h3.is_three_of_a_kind());
// }
