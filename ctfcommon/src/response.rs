#[derive(RustcDecodable, RustcEncodable)]
pub enum CTFResponse {
    FlagVerified(usize, bool, Option<String>),
    Leaderboard(Vec<(i32, String)>),
    Err,
}
