#[macro_use]
extern crate serde_derive;

mod recommendations;
mod similarity;

use std::error::Error;

use similarity::{compute_similarities, Similarities, UserItemsMap, ItemUsersMap};

use recommendations::{compute_recommendations, RecommendationMap};

fn extract_data<R: std::io::Read>(ratings_source: R) -> Result<(UserItemsMap, ItemUsersMap), Box<Error>> {
    similarity::read_csv(ratings_source)
}

pub fn similarities<R: std::io::Read>(ratings_source: R) -> Result<Similarities, Box<Error>> {
    let (user_items, item_users) = match extract_data(ratings_source) {
        Ok((user_items, item_users)) => { (user_items, item_users) },
        Err(err) => return Err(err)
    };

    compute_similarities(&user_items, &item_users)
}

pub fn recommendations<R: std::io::Read>(ratings_source: R) -> Result<RecommendationMap, Box<Error>> {

    let (user_items, item_users) = match extract_data(ratings_source) {
        Ok((user_items, item_users)) => { (user_items, item_users) },
        Err(err) => return Err(err)
    };


    let sims = match compute_similarities(&user_items, &item_users) {
        Ok(sims) => { sims },
        Err(err) => return Err(err)
    };

    compute_recommendations(&user_items, &sims)
}

