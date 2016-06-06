#[derive(RustcDecodable, RustcEncodable)]
pub enum CTFRequest {
    FlagOffer(UserFlagPair),
    Leaderboard(usize, usize),
    Ping,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct UserFlagPair {
    pub id: usize,
    pub flag: String,
    pub user: String,
}

impl CTFRequest {
    pub fn verify_flag(id: usize, flag: String, user: String) -> Self {
        CTFRequest::FlagOffer(UserFlagPair::new(id, flag, user))
    }

    pub fn view_leaderboard(start: usize, stop: usize) -> Self {
        CTFRequest::Leaderboard(start, stop)
    }
}

impl UserFlagPair {
    pub fn new(id: usize, flag: String, user: String) -> Self {
        UserFlagPair {
            id: id,
            flag: flag,
            user: user,
        }
    }
}
