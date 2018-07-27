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
use std::fs::File;
use std::io::{BufRead, BufReader};


type Suit = u32;
type Rank = u32;
type Ranks = Vec<Rank>;
type Shapes = Vec<u32>;

#[derive(Debug)]
pub struct Properties {
    hand_type: Poker,
    values: Ranks,
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
        self.rank == other.rank
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
    let ch = c.to_string();
    "23456789TJQKA".split("").position(|i| i == ch).unwrap() as u32 + 2
}

fn to_suit(c: char) -> Suit {
    let ch = c.to_string();
    "CDHS".split("").position(|i| i == ch).unwrap() as u32
}



#[derive(Debug, Clone, Hash)]
pub struct PokerHand {
    hand: Vec<Card>,
}

impl PokerHand {
    pub fn new(cards: Vec<Card>) -> PokerHand {
        let mut cs = cards;
        cs.sort(); 
        PokerHand { hand: cs }
    }

    fn shape(&mut self) -> Vec<(u32, u32)> {
        let mut map = HashMap::new();
        for c in &self.hand {
            *map.entry(c.rank).or_insert(0) += 1;
        }
        let mut v_map = map.iter().collect::<Vec<(&u32, &u32)>>();
        v_map.sort_by(|&x, &y| if x.1 != y.1 { x.1.cmp(y.1) } else { x.0.cmp(y.0) });
        v_map.iter().cloned().map(|(&x, &y)| (x, y)).rev().collect()
    }
    
    pub fn hand_ranks(&mut self) -> Ranks {
        let ranks = self.shape();
        ranks.iter().map(|x| x.0 ).collect()        
    } 

    pub fn derive_hand(&mut self) -> Vec<usize> {
        let mut map = HashMap::new();
        for c in &self.hand {
            map.entry(c.rank)
               .or_insert(Vec::new())
               .push(c.rank);
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
        let low_straight_card = self.hand_ranks()
                                    .into_iter()
                                    .last()
                                    .unwrap();
        let hh = &self.hand[1..];
        let zipped_cards = self.hand.iter()
                                    .zip(hh.iter())
                                    .collect::<Vec<_>>();
        let mut aa = Vec::new();
        for x in zipped_cards {
            aa.push((x.1.rank - x.0.rank) as i32);
        }
        match aa.as_slice() {
            [1,1,1,1]   => Some(low_straight_card), // low card in straight
            [1,1,1,10]  => Some(1),                 // starting Ace in straight
            _           => None,                    // not a straight
        }
    }
    
    pub fn scoring_properties(&mut self) -> Poker {
        match (self.derive_hand().as_slice(), self.is_same_suit(), self.is_consecutive()) {
        ([_, _, _, _, _], false, None)                              => Poker::HighCard,   
        ([2, _, _, _], false, None)                                 => Poker::OnePair, 
        ([2, 2, _], false, None)                                    => Poker::TwoPair, 
        ([3 , _, _], false, None)                                   => Poker::ThreeOfAKind, 
        ([_, _, _, _, _], false, Some(_))                           => Poker::Straight, 
        ([_, _, _, _, _], true, None)                               => Poker::Flush,
        ([3, _], false, None)                                       => Poker::FullHouse,
        ([4 ,_], false, None)                                       => Poker::FourOfAKind,
        ([_, _, _, _, _], true, Some(r)) if r < Some(10).unwrap()   => Poker::StraightFlush,
        ([_, _, _, _, _], true, Some(r)) if r == Some(10).unwrap()  => Poker::RoyalFlush,
        _                                                           => Poker::MisDeal,
        }
    }
    
    pub fn heads_up_poker(&mut self, mut other: PokerHand) -> bool {
        let this_player = Properties { hand_type: self.scoring_properties(), values: self.hand_ranks() } ;
        let other_player = Properties { hand_type: other.scoring_properties(), values: other.hand_ranks() };
        if this_player.hand_type > other_player.hand_type {
            return true;
        }
        if this_player.hand_type == other_player.hand_type {
            if this_player.values > other_player.values {
                return true;
            }
        }
        false
    }
}

pub fn build_hands(file_name: &str) -> Vec<(PokerHand,PokerHand)> {
    let file = BufReader::new(File::open(file_name).unwrap());
    let lines: Vec<String> = file.lines().map(|line| { line.unwrap() }).collect();
  
    let mut cards = Vec::new();
    for l in &lines {
        cards.push(l.split(" ")
            .map(|x| Card::from(x))
            .collect::<Vec<_>>());
    }
    let poker_hands = cards.iter()
                          .map(|x| x.split_at(5))
                          .collect::<Vec<_>>();
    
    let mut player_cards = Vec::new();
    for x in poker_hands {
        player_cards.push((PokerHand::new(x.0.to_vec()), 
                           PokerHand::new(x.1.to_vec())));
    }
    player_cards    
}




#[cfg(test)]
mod tests {

    use Card;
    use PokerHand;
    
    #[test]
    fn jack_beats_eights() {
        let c1 = Card::from("8H");
        let c2 = Card::from("JS");
        assert_eq!(c1 < c2, true);
    }

    #[test]
    fn ace_beats_king() {
        let c1 = Card::from("AD");
        let c2 = Card::from("KC");
        assert_eq!(c1 > c2, true);
    }

    #[test]
    fn equal_nines() {
        let c1 = Card::from("9H");
        let c2 = Card::from("9S");
        assert_eq!(c1.eq(&c2), true);
    }

    #[test]
    fn equal_queens() {
        let c1 = Card::from("QC");
        let c2 = Card::from("QH");
        assert_eq!(c1.eq(&c2), true);
    }

    #[test]
    fn high_card() {
        let mut hand1 = PokerHand::new(vec!(Card::from("5D"), Card::from("8C"), Card::from("9S"),Card::from("JS"), Card::from("AC")));
        let mut hand2 = PokerHand::new(vec!(Card::from("2C"), Card::from("5C"), Card::from("7D"),Card::from("8S"), Card::from("QH")));
        assert_eq!(hand1.heads_up_poker(hand2), true);
    }

    #[test]
    fn best_pair() {
        let mut hand1 = PokerHand::new(vec!(Card::from("5H"), Card::from("5C"), Card::from("6S"),Card::from("7S"), Card::from("KD")));
        let mut hand2 = PokerHand::new(vec!(Card::from("2C"), Card::from("3S"), Card::from("8S"),Card::from("8D"), Card::from("TD")));
        assert_eq!(hand1.heads_up_poker(hand2), false);
    }

    #[test]
    fn three_aces_vs_flush() {
        let mut hand1 = PokerHand::new(vec!(Card::from("2D"), Card::from("9C"), Card::from("AS"),Card::from("AH"), Card::from("AC")));
        let mut hand2 = PokerHand::new(vec!(Card::from("3D"), Card::from("6D"), Card::from("7D"),Card::from("TD"), Card::from("QD")));
        assert_eq!(hand1.heads_up_poker(hand2), false);
    }

    #[test]
    fn equal_pairs_with_high_card() {
        let mut hand1 = PokerHand::new(vec!(Card::from("4D"), Card::from("6S"), Card::from("9H"),Card::from("QH"), Card::from("QC")));
        let mut hand2 = PokerHand::new(vec!(Card::from("3D"), Card::from("6D"), Card::from("7H"),Card::from("QD"), Card::from("QS")));
        assert_eq!(hand1.heads_up_poker(hand2), true);
    }

    #[test]
    fn best_full_house() {
        let mut hand1 = PokerHand::new(vec!(Card::from("2H"), Card::from("2D"), Card::from("4C"),Card::from("4D"), Card::from("4S")));
        let mut hand2 = PokerHand::new(vec!(Card::from("3C"), Card::from("3D"), Card::from("3S"),Card::from("9S"), Card::from("9D")));
        assert_eq!(hand1.heads_up_poker(hand2), true);
        
    }

    #[test]
    fn best_straight() {
        let mut hand1 = PokerHand::new(vec!(Card::from("AH"), Card::from("2C"), Card::from("3S"),Card::from("4S"), Card::from("5D")));
        let mut hand2 = PokerHand::new(vec!(Card::from("8C"), Card::from("9S"), Card::from("TS"),Card::from("JD"), Card::from("QD")));
        assert_eq!(hand1.heads_up_poker(hand2), false);
        
    }

    #[test]
    fn straight_flush_vs_straight() {
        let mut hand1 = PokerHand::new(vec!(Card::from("9H"), Card::from("TH"), Card::from("JH"),Card::from("QH"), Card::from("KH")));
        let mut hand2 = PokerHand::new(vec!(Card::from("TC"), Card::from("JS"), Card::from("QS"),Card::from("KD"), Card::from("AD")));
        assert_eq!(hand1.heads_up_poker(hand2), true);
        
    }

    #[test]
    fn straight_flush_vs_royal_flush() {
        let mut hand1 = PokerHand::new(vec!(Card::from("9H"), Card::from("TH"), Card::from("JH"),Card::from("QH"), Card::from("KH")));
        let mut hand2 = PokerHand::new(vec!(Card::from("TC"), Card::from("JC"), Card::from("QC"),Card::from("KC"), Card::from("AC")));
        assert_eq!(hand1.heads_up_poker(hand2), false);
        
    }
}
