#![feature(slice_patterns)]
#![feature(match_default_bindings)]
extern crate problem_54;

use problem_54::build_hands;


fn main() {
    let one_thousand_card_games = build_hands("p054_poker.txt");
    let mut wins = 0;
    for mut players in one_thousand_card_games {
        if players.0.heads_up_poker(players.1) {
            wins += 1;
        }
    }
    println!("Wins = {}", wins);
}