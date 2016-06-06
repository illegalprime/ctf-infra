extern crate csv;
extern crate firebase;
extern crate rustc_serialize;
extern crate ctfcommon;

use self::csv::Error;
use self::firebase::Firebase;
use self::rustc_serialize::json;
use self::ctfcommon::entities::Player;

use std::path::Path;
use std::collections::HashMap;

mod constants;

pub fn load_flags() -> Result<HashMap<usize, String>, Error> {
    let flags_path = Path::new(constants::FLAGS_PATH);
    let mut reader = try!(csv::Reader::from_file(&flags_path)).has_headers(false);
    let mut db = HashMap::new();

    for row in reader.decode() {
        let (id, flag): (usize, String) = try!(row);
        db.insert(id, flag);
    }
    Ok(db)
}

pub struct CtfDb {
    server: Firebase,
}

// TODO: Major Error Checking!!!
impl CtfDb {
    pub fn new() -> Self {
        CtfDb {
            server: Firebase::authed(constants::FIREBASE_URL, constants::FIREBASE_AUTH),
        }
    }

    pub fn check_taken(&self, id: usize) -> Option<String> {
        let takendb = self.server.at(&format!("/taken/{}", &id));
        let res = takendb.get();

        if res.body == "null" {
            None
        } else {
            Some(res.body[1..(res.body.len() - 1)].to_string())
        }
    }

    pub fn reward_flag(&self, user: &str, id: usize) {
        // Set this id to be taken by 'user'
        // TODO: Check if data update succeeded.
        let data = format!("\"{}\"", user);
        let takendb = self.server.at(&format!("/taken/{}.json", id));
        takendb.set(&data);
    }

    pub fn change_score(&self, user: &str, amount: i32) {
        // TODO: Check if score change succeeded.
        let path = &format!("/scores/{}/score", user);
        let record = self.server.at(path);

        let scoredb = record.get();
        let score: i32 = scoredb.body.parse().unwrap_or(0);

        record.set(& (score + amount).to_string());
    }

    pub fn get_top_players(&self, n: usize) -> Vec<(i32, String)> {
        let scores_db = self.server.at("/scores");
        let topn = scores_db.order_by("\"score\"").limit_to_last(n as u32).get();

        let people: HashMap<String, Player> = json::decode(&topn.body).unwrap();

        // TODO: Use into_inter..
        let mut pairs: Vec<(i32, String)> = people.iter()
                                                  .map(|(k, v)| (v.score.clone(), k.clone()))
                                                  .collect();
        pairs.sort_by(|&(a, _), &(b, _)| b.cmp(&a));
        pairs
    }
}
