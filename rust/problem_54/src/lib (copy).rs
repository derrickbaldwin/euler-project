/*

Problem 54:

In the card game poker, a hand consists of five cards and 
are ranked, from lowest to highest, in the following way:

    High Card: Highest value card.
    One Pair: Two cards of the same value.
    Two Pairs: Two different pairs.
    Three of a Kind: Three cards of the same value.
    Straight: All cards are consecutive values.
    Flush: All cards of the same suit.
    Full House: Three of a kind and a pair.
    Four of a Kind: Four cards of the same value.
    Straight Flush: All cards are consecutive values of same suit.
    Royal Flush: Ten, Jack, Queen, King, Ace, in same suit.

The cards are valued in the order:
2, 3, 4, 5, 6, 7, 8, 9, 10, Jack, Queen, King, Ace.

If two players have the same ranked hands then the rank made 
up of the highest value wins; for example, a pair of eights 
beats a pair of fives (see example 1 below). But if two ranks 
tie, for example, both players have a pair of queens, then 
highest cards in each hand are compared (see example 4 below); 
if the highest cards tie then the next highest cards are 
compared, and so on.

Consider the following five hands dealt to two players:
Hand	 	Player 1	 	Player 2	 	Winner
1	 	5H 5C 6S 7S KD
Pair of Fives
	 	2C 3S 8S 8D TD
Pair of Eights
	 	Player 2
2	 	5D 8C 9S JS AC
Highest card Ace
	 	2C 5C 7D 8S QH
Highest card Queen
	 	Player 1
3	 	2D 9C AS AH AC
Three Aces
	 	3D 6D 7D TD QD
Flush with Diamonds
	 	Player 2
4	 	4D 6S 9H QH QC
Pair of Queens
Highest card Nine
	 	3D 6D 7H QD QS
Pair of Queens
Highest card Seven
	 	Player 1
5	 	2H 2D 4C 4D 4S
Full House
With Three Fours
	 	3C 3D 3S 9S 9D
Full House
with Three Threes
	 	Player 1

The file, poker.txt, contains one-thousand random hands dealt 
to two players. Each line of the file contains ten cards 
(separated by a single space): the first five are Player 1's 
cards and the last five are Player 2's cards. You can assume 
that all hands are valid (no invalid characters or repeated 
cards), each player's hand is in no specific order, and in 
each hand there is a clear winner.

How many hands does Player 1 win?
*/
#![feature(slice_patterns)]
#![feature(match_default_bindings)]


use std::fmt;
use std::collections::HashSet;
use std::collections::HashMap;



#[derive(Debug, Eq, PartialEq, Copy, Clone, Ord, PartialOrd, Hash)]
pub enum Suit {
    C, D, H, S
}

impl Suit {
    fn show(&self) -> String {
        match *self {
            Suit::C => String::from("C"),
            Suit::D => String::from("D"),
            Suit::H => String::from("H"),
            Suit::S => String::from("S"),
        }
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.show())
    }
}


#[derive(Debug, Eq, PartialEq, Copy, Clone, Ord, PartialOrd, Hash)]
pub enum Rank {
    Number(u32), 
    T,
    J,
    Q,
    K,
    A
}

impl Rank {
    pub fn show(&self) -> String {
        match *self {
            Rank::Number(n) => n.to_string(),
            Rank::T         => String::from("T"),
            Rank::J         => String::from("J"),
            Rank::Q         => String::from("Q"),
            Rank::K         => String::from("K"),
            Rank::A         => String::from("A")  
        }
    } 
    pub fn value(&self) -> u32 {
        match *self {
            Rank::Number(n) => n,
            Rank::T        => 10,
            Rank::J        => 11,
            Rank::Q        => 12,
            Rank::K        => 13,
            Rank::A        => 14,
        }
    }
} 

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.show())
    }
}

#[derive(Debug, Eq, Copy, Clone, Ord, PartialOrd, Hash)]
pub struct Card {
    pub rank: Rank, 
    pub suit: Suit, 
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}{}]", self.rank, self.suit)
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Card) -> bool {
        self.rank.value() == other.rank.value()
    }
}

impl<'a> From<&'a str> for Card {
    fn from(s: &str) -> Self {
        let parts = s.chars().collect::<Vec<_>>();
        Card {
            rank: to_rank(parts[0]),
            suit: to_suit(parts[1]),
        }
    }
}

fn to_rank(c: char) -> Rank {
    match c {
        '2'...'9' => Rank::Number(c.to_digit(10).unwrap()),
        'T' => Rank::T,
        'J' => Rank::J,
        'Q' => Rank::Q,
        'K' => Rank::K,
        'A' => Rank::A,
        _   => panic!("Not a Ranked Card!")
    }
}

fn to_suit(c: char) -> Suit {
    match c {
        'C' => Suit::C,
        'D' => Suit::D,
        'H' => Suit::H,
        'S' => Suit::S,
        _   => panic!("Not a Card Suit")

    }
}



#[derive(Debug, Clone, Hash)]
pub struct Hand {
    hand: Vec<Card>,
}

impl Hand {
    pub fn new(cards: Vec<Card>) -> Hand {
        let mut cs = cards;
        cs.sort(); 
        Hand {
            hand: cs,
        }
    }
    
    pub fn shape(&mut self) -> HashMap<u32, Vec<u32>> {
        let mut map = HashMap::new();
        for c in &self.hand {
            map.entry(c.rank.value())
               .or_insert(Vec::new())
               .push(c.rank.value());
        }
        map        
    }

    pub fn hand_values(&mut self) -> Vec<(u32, u32)> {
        let mut map = HashMap::new();
        for c in &self.hand {
            *map.entry(c.rank.value()).or_insert(0) += 1;
        }
        let mut v_map = map.iter().collect::<Vec<(&u32, &u32)>>();
        v_map.sort_by(|&x, &y| if x.1 != y.1 { x.1.cmp(y.1) } else { x.0.cmp(y.0) });
        v_map.iter().cloned().map(|(&x, &y)| (x, y)).rev().collect()
    }

    pub fn score_values(&mut self) -> Vec<u32> {
        let v = self.hand_values();
        v.iter().map(|x| x.0 ).collect()
    } 

    
    pub fn derive_hand(&mut self) -> Vec<usize> {
        let mut map = HashMap::new();
        for c in &self.hand {
            map.entry(c.rank.value())
               .or_insert(Vec::new())
               .push(c.rank.value());
        }
        
        let mut vec = Vec::new();
        for v in map.values() {
            vec.push(v.len());
        }
        vec.sort();
        vec.into_iter().rev().collect::<Vec<usize>>()
    }
    

    pub fn is_same_suit(&mut self) -> bool {
        let mut set = HashSet::new();
        for card in &self.hand {
            set.insert(card.suit);
        }
        set.len() == 1
    }
  

    pub fn is_consecutive(&mut self) -> Option<u32> {
        let low_straight_card = self.score_values()
                                    .into_iter()
                                    .last()
                                    .unwrap();
        let hh = &self.hand[1..];
        let zipped_cards = self.hand.iter()
                                    .zip(hh.iter())
                                    .collect::<Vec<_>>();
        let mut aa = Vec::new();
        for x in zipped_cards {
            aa.push((x.1.rank.value() - x.0.rank.value()) as i32);
        }
        match aa.as_slice() {
            [1,1,1,1]   => Some(low_straight_card), // low card in straight
            [1,1,1,9]   => Some(2),                 // starting Ace in straight
            _           => None,                    // not a straight
        }
    }
    
    pub fn scoring_properties(&mut self) -> (u32, Vec<u32>) {
        match (self.derive_hand().as_slice(), self.is_same_suit(), self.is_consecutive()) {
        ([_, _, _, _, _], false, None)                              => (Poker::HighCard as u32, self.score_values()),  
        ([2, _, _, _], false, None)                                 => (Poker::OnePair as u32, self.score_values()),
        ([2, 2, _], false, None)                                    => (Poker::TwoPair as u32, self.score_values()),
        ([3 , _, _], false, None)                                   => (Poker::ThreeOfAKind as u32, self.score_values()),
        ([_, _, _, _, _], false, Some(_))                           => (Poker::Straight as u32, self.score_values()),
        ([_, _, _, _, _], true, None)                               => (Poker::Flush as u32, self.score_values()),
        ([3, _], false, None)                                       => (Poker::FullHouse as u32, self.score_values()),
        ([4 ,_], false, None)                                       => (Poker::FourOfAKind as u32, self.score_values()),
        ([_, _, _, _, _], true, Some(r)) if r < Some(10).unwrap()   => (Poker::StraightFlush as u32, self.score_values()),
        ([_, _, _, _, _], true, Some(r)) if r == Some(10).unwrap()  => (Poker::RoyalFlush as u32, self.score_values()),
        _                                                           => (Poker::MisDeal as u32,self.score_values()),
        }
    }
}


#[derive(Debug, Eq, PartialEq, Copy, Clone, Ord, PartialOrd, Hash)]
pub enum Poker {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    RoyalFlush,
    MisDeal,
}

pub struct ShowDown {
    player1: Hand,
    player2: hand,
}



#[cfg(test)]
mod tests {

    use Card;

    #[test]
    fn card_ranks1() {
        let c1 = Card::from("8H");
        let c2 = Card::from("JS");
        assert_eq!(c1 < c2, true);
    }

    #[test]
    fn card_ranks2() {
        let c1 = Card::from("AD");
        let c2 = Card::from("KC");
        assert_eq!(c1 > c2, true);
    }

    #[test]
    fn card_ranks3() {
        let c1 = Card::from("9H");
        let c2 = Card::from("9S");
        assert_eq!(c1.eq(&c2), true);
    }

    #[test]
    fn card_ranks4() {
        let c1 = Card::from("QC");
        let c2 = Card::from("QH");
        assert_eq!(c1.eq(&c2), true);
    }


}
