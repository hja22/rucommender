#[macro_use]
extern crate serde_derive;

mod similarity;

use std::error::Error;

use similarity::{read_csv, compute_similarities, Similarities};

pub fn similarities<R: std::io::Read>(activity_source: R) -> Result<Similarities, Box<Error>> {
    let (user_items, item_users) = match read_csv(activity_source) {
        Ok((user_items, item_users)) => { (user_items, item_users) },
        Err(err) => return Err(err)
    };

    compute_similarities(&user_items, &item_users)
}


