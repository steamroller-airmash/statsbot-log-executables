
extern crate statslib;

use statslib::*;

use std::io::{BufReader, BufRead, stdin};
use std::collections::{HashMap, VecDeque};

const ANON_NAME: &'static str = "ANON_PLAYER";
const ANON_ID  : i64          = -1;

fn anonymise_user(backlog: &mut VecDeque<String>, id: i64) {
    for record_str in backlog.iter_mut().rev() {
        let val = {
            let mut record = parse(record_str).unwrap();

            let mut to_anon = false;
            if record.entries.contains_key("id") {
                let id_val = record.entries.get_mut("id").unwrap();
                if *id_val == RecordValue::Int(id) {
                    *id_val = RecordValue::Int(ANON_ID);
                    to_anon = true;
                }
            }
            if to_anon && record.entries.contains_key("name") {
                *record.entries.get_mut("name").unwrap() =
                    RecordValue::Str(ANON_NAME);
            }
            if record.entries.contains_key("from") {
                let from = record.entries.get_mut("from").unwrap();
                if *from == RecordValue::Int(id) {
                    *from = RecordValue::Int(ANON_ID);
                }
            }
            if record.entries.contains_key("to") {
                let to = record.entries.get_mut("to").unwrap();
                if *to == RecordValue::Int(id) {
                    *to = RecordValue::Int(ANON_ID);
                }
            }
            if record.entries.contains_key("killer") {
                let killer = record.entries.get_mut("killer").unwrap();
                if *killer == RecordValue::Int(id) {
                    *killer = RecordValue::Int(ANON_ID);
                }
            }
            
            write_record(&record)
        };
        *record_str = val;
    }
}

fn is_player_new(record: &str) -> bool {
    record.starts_with("[PLAYER_NEW")
}

fn main() {
    let mut backlog: VecDeque<String> = VecDeque::new();
    let mut anon_users: HashMap<i64, ()> = HashMap::new();
    let mut users: HashMap<i64, ()> = HashMap::new();
    let mut join_order: VecDeque<i64> = VecDeque::new();
    
    for line in BufReader::new(stdin()).lines() {
        let ln = line.unwrap();
        let result = parse(&ln);

        if result.is_err() {
            continue;
        }
        let record = result.unwrap();



        if record.tag == "ANONYMISE" {
            let id = match record.entries["id"] {
                RecordValue::Int(val) => val,
                _ => {
                    eprintln!("Invalid record: {}", ln);
                    continue;
                }
            };

            anon_users.insert(id, ());
            continue;
        }

        if record.tag == "PLAYER_NEW" {
            let id = match record.entries["id"] {
                RecordValue::Int(val) => val,
                _ => {
                    eprintln!("Invalid record: {}", ln);
                    continue;
                }
            };

            users.insert(id, ());
            join_order.push_back(id);
        }
        
        if record.tag == "PLAYER_LEAVE" {
            let id = match record.entries["id"] {
                RecordValue::Int(val) => val,
                _ => {
                    eprintln!("Invalid record: {}", ln);
                    continue;
                }
            };

            backlog.push_back(write_record(&record).to_string());

            if anon_users.contains_key(&id) {
                anonymise_user(&mut backlog, id);
            }

            while backlog.len() > 0 {
                // Scope to escape borrow
                {
                    let record = backlog.front().unwrap();

                    if is_player_new(record) {
                        let record = parse(&record).unwrap();

                        let id = match record.entries["id"] {
                            RecordValue::Int(val) => val,
                            _ => {
                                eprintln!("Invalid record: {}", ln);
                                continue;
                            }
                        };

                        if id != *join_order.front().unwrap() {
                            break;
                        }

                        join_order.pop_front();
                    }

                    println!("{}", record);
                }

                backlog.pop_front();
            }
        }
        else {
            backlog.push_back(write_record(&record).to_string());
        }
    }

}
