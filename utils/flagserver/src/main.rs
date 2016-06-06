extern crate wire;
extern crate ctfcommon;

mod database;

use std::collections::HashMap;
use std::thread::spawn;
use std::sync::Arc;

use wire::SizeLimit;
use ctfcommon::request::CTFRequest;
use ctfcommon::request::UserFlagPair;
use ctfcommon::response::CTFResponse;
use ctfcommon::constants::{SERVER_WRITE_LIMIT, SERVER_READ_LIMIT, PORT, ADDRESS};

use database::CtfDb;

type FlagKeys = HashMap<usize, String>;

fn main() {
    // Load verified flags from CSV
    let static_flagset = match database::load_flags() {
        Ok(map) => map,
        Err(e) => panic!("Could not load .csv flag database! {}", e),
    };
    let num_flags = static_flagset.len();
    let atomic_flagset = Arc::new(static_flagset);
    println!("{} Flags loaded!", &num_flags);

    // Create reference to the firebase database
    let scoreboard = CtfDb::new();
    let atomic_board = Arc::new(scoreboard);

    // Create TCP Listener
    let (listener, _) = match wire::listen_tcp((ADDRESS, PORT)) {
        Ok(io_pair) => io_pair,
        _ => panic!("Could not start a server on {}:{}!", ADDRESS, PORT),
    };
    let read_limit  = SizeLimit::Bounded(SERVER_READ_LIMIT);
    let write_limit = SizeLimit::Bounded(SERVER_WRITE_LIMIT);

    // Main TCP Loop
    for (connection, _) in listener.into_blocking_iter() {
        // Spawn a new thread for each connection that we get.
        let flags  = atomic_flagset.clone();
        let scores = atomic_board.clone();

        spawn(move || {
            // Upgrade the connection to read `u64` and write `(u64, u64)`.
            let (i, mut o) = wire::upgrade_tcp(connection, read_limit, write_limit).unwrap();

            // For each request that we read from the network...
            for request in i.into_blocking_iter() {
                // Route requests to functions
                let response = match request {
                    CTFRequest::FlagOffer(user_flag) => verify_flag(user_flag, &flags, &scores),
                    CTFRequest::Leaderboard(min, max) => get_top_players(min, max, &scores),
                    _ => CTFResponse::Err
                };
                // Send Response
                o.send(&response).ok();
            }
        });
    }
}

fn verify_flag(req: UserFlagPair, flags: &FlagKeys, scoreboard: &CtfDb) -> CTFResponse {
    // Check if flag is in db. Does not check if user already got flag.
    // TODO: Put thread to sleep if flag is wrong, to prevent brute force?
    let correct = match flags.get(&req.id) {
        Some(flag) => *flag == req.flag,
        None => false,
    };

    let taken = scoreboard.check_taken(req.id);

    if correct && taken.is_none() {
        scoreboard.reward_flag(&req.user, req.id);
        scoreboard.change_score(&req.user, 1);
    }

    CTFResponse::FlagVerified(req.id, correct, taken)
}

fn get_top_players(min: usize, max: usize, scoreboard: &CtfDb) -> CTFResponse {
    CTFResponse::Leaderboard(scoreboard.get_top_players(max))
}
