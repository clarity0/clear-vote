use serde::Deserialize;
use super::preference::Preference;

#[derive(Debug, Deserialize)]
pub struct Record {
    #[serde(rename = "Permutation")]
    pub permutation: String,
    #[serde(rename = "Number")]
    pub number: u64,
}

impl Into<Preference> for Record {
    fn into(self) -> Preference {
        Preference {
            permutation: self.permutation
                .split(',')
                .map(|n| n.parse().expect("parse error"))
                .collect(),
            number: self.number,
        }
    }
}