
extern crate statslib;

use statslib::*;

use std::io::{BufReader, BufRead, stdin};
use std::collections::HashMap;
use std::mem;

fn main() {
    let mut players: HashMap<i64, ()> = HashMap::new();
    let mut is_login = false;
    let mut newplayers: HashMap<i64, ()> = HashMap::new();
    let mut msgbuffer: Vec<String> = vec![];

	for line in BufReader::new(stdin()).lines() {
		let ln = line.unwrap();
		let result = parse(&ln);

		if result.is_err() {
			    eprintln!("Invalid Record: {}", ln);
			continue;
		}
		let record = result.unwrap();

        if is_login && !(record.tag == "PLAYER_NEW" || record.tag == "PLAYER_LEVEL") {
            {
                 let mut leftplayers = players.clone();

                for (player, &()) in newplayers.iter() {
                    leftplayers.remove(player);
                }

                let mut it1 = leftplayers.iter().map(|(player, _)| {
                    format!("[PLAYER_LEAVE,id:{}]", *player).to_string()
                });

                let tmpbuf = mem::replace(&mut msgbuffer, vec![]);

                let it2 = tmpbuf.into_iter().filter(|x| {
                    let record = parse(&x).unwrap();

                    if record.tag == "PLAYER_NEW" || record.tag == "PLAYER_LEVEL" {
                        let id = match record.entries["id"] {
                            RecordValue::Int(val) => val,
                            _ => panic!()
                        };

                        let result = !players.contains_key(&id);

                        result
                    }
                    else {
                        true
                    }
                });

                
                for msg in it1 {
                    println!("{}", msg);
                }
                for msg in it2 {
                    println!("{}", msg);
                }
            }
            players = mem::replace(&mut newplayers, HashMap::new());
            is_login = false;
        }

		if record.tag == "LOGIN" {
            is_login = true;
        }
        else if record.tag == "PLAYER_NEW" {
            let id = match record.entries["id"] {
                RecordValue::Int(val) => val,
                _ => {
                    eprintln!("Invalid record: {}", ln);
                    continue;
                }
            };

            if is_login {
                newplayers.insert(id, ());
            }
            else {
                players.insert(id, ());
            }
        }
        else if record.tag == "PLAYER_LEAVE" {
            let id = match record.entries["id"] {
                RecordValue::Int(val) => val,
                _ => {
                    eprintln!("Invalid record: {}", ln);
                    continue;
                }
            };

            players.remove(&id);
        }

        if !is_login {
            println!("{}", ln);
        }
        else {
            msgbuffer.push(ln.to_string());
        }
	}
}