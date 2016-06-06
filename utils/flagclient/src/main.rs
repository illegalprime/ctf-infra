extern crate wire;
extern crate ctfcommon;
extern crate rustc_serialize;
extern crate docopt;

use ctfcommon::request::CTFRequest;
use ctfcommon::response::CTFResponse;
use ctfcommon::constants::{SERVER_WRITE_LIMIT, SERVER_READ_LIMIT, PORT, ADDRESS};
use docopt::Docopt;
use wire::SizeLimit;

mod formatting;

static USAGE: &'static str = "
Usage:
    ctf claim <flag-id> <flag-number> <username> [ -p <port> ]
    ctf leaderboard <start> <end>                [ -p <port> ]

Options:
    -p <port>   TCP port to connect to (optional). The default port is the right one.
";

#[derive(RustcDecodable, Debug)]
struct Args {
    flag_p: Option<u16>,
    arg_flag_id: String,
    arg_flag_number: usize,
    arg_username: String,
    arg_start: usize,
    arg_end: usize,
    cmd_claim: bool,
    cmd_leaderboard: bool,
}

fn main() {
    highscores_request(0, 5, PORT);

    let args: Args = Docopt::new(USAGE)
                        .and_then(|d| d.decode())
                        .unwrap_or_else(|e| e.exit());

    let port = args.flag_p.unwrap_or(PORT);
    let id   = args.arg_flag_number;
    let flag = args.arg_flag_id;
    let user = args.arg_username;


    if args.cmd_claim {
        flag_verify_request(id, flag, user, port);
    } else if args.cmd_leaderboard {

    }
}

fn highscores_request(start: usize, end: usize, port: u16) {
    let req = CTFRequest::view_leaderboard(start, end);

    if let Some(CTFResponse::Leaderboard(scores)) = send(&req, port) {
        println!("{}", formatting::generate_leaderboard(start, &scores));
    }
}

fn flag_verify_request(id: usize, flag: String, user: String, port: u16) {
    // Create a request to check a flag
    // TODO: Get rid of this cloning
    let flag_offer = CTFRequest::verify_flag(id, flag, user.clone());

    // Print statuses!
    if let Some(CTFResponse::FlagVerified(_, verified, taken)) = send(&flag_offer, port) {
        if let Some(usr) = taken {
            if usr == user {
                println!("You already claimed Flag #{}", id);
            } else {
                println!("Sorry, Flag #{} has already been claimed by {}.", id, usr);
            }
        } else if verified {
            println!("Congratulations! Flag #{} has been claimed in your name.", id);
        } else {
            println!("Sorry, the hash provided does not match the one for Flag #{}.", id);
        }
    } else {
        println!("Server could not interpret messages. Contact admins. D:");
    }
}

fn send(req: &CTFRequest, port: u16) -> Option<CTFResponse> {
    // Control max size of incoming and outgoing messages.
    // Values are flipped beacuse we are on the client side.
    let read_limit  = SizeLimit::Bounded(SERVER_WRITE_LIMIT);
    let write_limit = SizeLimit::Bounded(SERVER_READ_LIMIT);

    // Try to connect to the flag server...
    let (i, mut o) = match wire::connect_tcp((ADDRESS, port), read_limit, write_limit) {
        Ok(io_pair) => io_pair,
        _ => panic!("Unable to connect to the flag server at port {}!", port),
    };

    // Send the flag request
    o.send(&req).ok();

    // Close our outgoing pipe. This is necessary because otherwise,
    // the server will keep waiting for the client to send it data and
    // we will deadlock.
    o.close();

    i.into_blocking_iter().next()
}
