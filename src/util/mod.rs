pub mod winner;

use std::fs::File;
use csv::Reader;
use crate::model::{preference::Preference, record::Record};

pub fn collect_records(reader: &mut Reader<File>) -> Vec<Preference>{
    let mut collection: Vec<Preference> = Vec::new();
    reader.deserialize().for_each(|row: Result<Record, _> | {
        if let Ok(record) = row {
            collection.push(record.into())
        }
    });
    collection
}

pub fn parse_schedule() -> Reader<File> {
    csv::ReaderBuilder::new()
        .delimiter(b',')
        .from_path("pref-sched.csv")
        .unwrap()
}