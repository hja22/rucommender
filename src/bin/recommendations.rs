extern crate rucommender;

use std::error::Error;
use std::process;

use rucommender::recommendations;

fn main() {
    let recs = match recommendations(std::io::stdin()) {
        Ok(recs) => {
            recs
        },
        Err(err) => {
            handle_err(err);
            return ();
        }
    };

    println!("{:?}", recs);

    ()
}

fn handle_err(err: Box<Error>) {
    println!("{}", err);
    process::exit(1);
}
