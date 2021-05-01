use std::collections::HashMap;

use crate::model::preference::Preference;

pub fn plurality_winner(permutation: Vec<Preference>) {
    let mut winners: HashMap<u8, u64> = HashMap::new();

    for pref in permutation {
        let (winner, count ) = pref.winner_count();
        if winners.contains_key(&winner) {
            winners.insert(winner, winners[&winner] + count);
        } else {
            winners.insert(winner, count);
        }
    }
    
    let winner = winners
        .iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .map(|(k, _)| k)
        .unwrap();

    println!("{} with {}", winner, winners[&winner]);
}
