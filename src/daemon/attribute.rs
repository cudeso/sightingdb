use chrono::{DateTime, Utc, NaiveDateTime};
use std::fmt;
use serde::{Deserialize, Serialize};
use chrono::serde::ts_seconds;
use std::collections::BTreeMap;

#[derive(Serialize,Deserialize)]
pub struct Attribute {
    pub value: String,
    #[serde(with = "ts_seconds")]
    pub first_seen: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    pub last_seen: DateTime<Utc>,
    pub count: u128,
    pub tags: String,
    pub ttl: u128,
    // #[serde(skip)]
    pub stats: BTreeMap<i64, u128>, // i64 because DateTime.timestamp() returns i64 :'(; We track count by time. 
}

impl Attribute {
    pub fn new(value: &str) -> Attribute {
        Attribute {
            value: String::from(value), // FIXME: change to Vec<u8>
            first_seen: DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(0, 0), Utc),
            last_seen: DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(0, 0), Utc),
            count: 0,
            tags: String::from(""),
            ttl: 0,
            stats: BTreeMap::new(),
        }
    }
    pub fn make_stats(&mut self, time: DateTime<Utc>) {
        let mut rounded_time = time.timestamp() - time.timestamp()%3600;
        if self.stats.contains_key(&rounded_time) {
            let mut cnt: u128;
            cnt = *self.stats.get(&rounded_time).unwrap();
            cnt += 1;
            self.stats.insert(rounded_time, cnt);
        } else {
            self.stats.insert(rounded_time, 1);            
        }
    }
    pub fn make_stats_from_timestamp(&mut self, timestamp: i64) {
        let mut rounded_time = timestamp - timestamp%3600;
        if self.stats.contains_key(&rounded_time) {
            let mut cnt: u128;
            cnt = *self.stats.get(&rounded_time).unwrap();
            cnt += 1;
            self.stats.insert(rounded_time, cnt);
        } else {
            self.stats.insert(rounded_time, 1);            
        }
    }
    pub fn count(&mut self) -> u128 {
        return self.count;
    }
    pub fn incr(&mut self) {
        if self.first_seen.timestamp() == 0 {
            self.first_seen = Utc::now();
        }
        self.last_seen = Utc::now();

        self.make_stats(self.last_seen);
        
        self.count += 1;
    }
    pub fn incr_from_timestamp(&mut self, timestamp: i64) {
        if self.first_seen.timestamp() == 0 {
            self.first_seen = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(timestamp, 0), Utc);
        }
        if timestamp < self.first_seen.timestamp() {
            self.first_seen = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(timestamp, 0), Utc);
        }
        if timestamp > self.last_seen.timestamp() {
            self.last_seen = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(timestamp, 0), Utc);
        }
        self.make_stats_from_timestamp(timestamp);
        self.count += 1;
    }    
}

impl fmt::Debug for Attribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Attribute {{ value: {}, first_seen: {:?}, last_seen: {:?}, count: {}, tags: {:?}, ttl: {:?}}}",
               self.value, self.first_seen, self.last_seen, self.count, self.tags, self.ttl)
    }
}
