extern crate csv;
extern crate serde;

use std::collections::HashMap;
use std::error::Error;

use similarity::{UserId, ItemId, UserItemsMap, Similarities};

type RecommenderScore = f64;

type Recommendations = HashMap<ItemId, RecommenderScore>;
pub type RecommendationMap = HashMap<UserId,Recommendations>;

#[derive(Debug, Deserialize)]
struct Record {
    user_id: UserId,
    item_id: ItemId
}

pub fn compute_recommendations(user_items: &UserItemsMap, similarities: &Similarities) -> Result<RecommendationMap, Box<Error>> {
    
    let mut all_recommendations = RecommendationMap::new();

    for (current_user_id, similar_users) in similarities {
        let mut current_user_recommendations = Recommendations::new();        

        for (similar_user_id, similarity_score) in similar_users {
            // FIXME: might legitimately be not entries here, don't panic if there aren't!
            let items_for_similar_user = user_items.get(&similar_user_id).expect("cannot find items for this similar user");

            for item_id in items_for_similar_user {
                current_user_recommendations.entry(*item_id).or_insert(0.0);
                *current_user_recommendations.get_mut(item_id).expect("similarity") += similarity_score;
            }
        }
        all_recommendations.insert(*current_user_id, current_user_recommendations);
    }

    Ok(all_recommendations)
}
