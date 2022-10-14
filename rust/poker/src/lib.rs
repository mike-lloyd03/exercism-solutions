use std::cmp::Ordering;

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    unimplemented!("Out of {:?}, which hand wins?", hands)
}

// Royal Flush
// Straight Flush
// Four of a Kind
// Full House
// Flush
// Straight
// Three of a Kind
// Two Pair
// Pair
// High Card

#[derive(Copy, Clone, PartialEq, PartialOrd)]
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

#[derive(Copy, Clone, PartialEq, PartialOrd)]
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

#[derive(Copy, Clone, PartialEq, PartialOrd)]
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

struct Hand<'a> {
    str: &'a str,
    cards: [Card; 5],
}

impl<'a> Hand<'a> {
    fn from_str(s: &'a str) -> Result<Self, ()> {
        let mut hand: [Card; 5] = [Card::default(); 5];

        for (i, c) in s.split(" ").enumerate() {
            hand[i] = Card::from_str(c).expect("Failed to parse card");
            if i == 4 {
                break;
            }
        }
        hand.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));

        Ok(Hand {
            str: s,
            cards: hand,
        })
    }
}

impl<'a> Hand<'a> {
    fn highest_card(&'a self) -> Card {
        *self.cards.first().expect("Hand should have cards")
    }

    fn lowest_card(&'a self) -> Card {
        let mut lowest_card = Card::default();

        for c in self.cards {
            if c.rank <= lowest_card.rank {
                lowest_card = c
            }
        }

        lowest_card
    }

    fn is_flush(&'a self) -> bool {
        let first_suite = self.cards.first().expect("Hand should have cards").suite;

        self.cards.iter().all(|c| c.suite == first_suite)
    }

    fn is_straight(&'a self) -> bool {
        if self.highest_card().rank < Rank::Five {
            return false;
        }

        if self.cards.map(|c| c.rank) == [Rank::Ace, Rank::Five, Rank::Four, Rank::Three, Rank::Two]
        {
            return true;
        }

        for i in 0..4 {
            if self.cards[i].rank as u32 - 1 != self.cards[i + 1].rank as u32 {
                return false;
            }
        }
        true
    }
}
