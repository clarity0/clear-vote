use util::{collect_records, parse_schedule, winner::plurality_winner};

mod model;
mod util;

fn main() {
    let mut reader = parse_schedule();
	let preference_list = collect_records(&mut reader);
	plurality_winner(preference_list);
}