extern crate csv;
extern crate serde;

use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::io;

pub type UserId = u32;
pub type ItemId = u32;
pub type Similarity = f64;

pub type UserItemsMap = HashMap<UserId, HashSet<ItemId>>;
pub type ItemUsersMap = HashMap<ItemId, HashSet<UserId>>;
pub type Similarities = HashMap<UserId, HashMap<UserId, Similarity>>;

#[derive(Debug, Deserialize)]
struct Record {
    user_id: UserId,
    item_id: ItemId
}

pub fn read_csv<R: io::Read>(activity_source: R) -> Result<(UserItemsMap, ItemUsersMap), Box<Error>> {
    let mut rdr = csv::Reader::from_reader(activity_source);
    let mut user_items = UserItemsMap::new();
    let mut item_users = ItemUsersMap::new();

    let empty_items = HashSet::<ItemId>::new();
    let empty_users = HashSet::<UserId>::new();

    for result in rdr.deserialize() {
        let record: Record = result?;
        {
            user_items.entry(record.user_id).or_insert(empty_items.clone());
            let current_user_items = user_items.get_mut(&record.user_id).expect("item-score pairs");
            current_user_items.insert(record.item_id);
        }

        {
            item_users.entry(record.item_id).or_insert(empty_users.clone());
            let current_item_users = item_users.get_mut(&record.item_id).expect("set of user ids");
            current_item_users.insert(record.user_id);
        }
    }
    Ok((user_items, item_users))
}

pub fn compute_similarities(user_items: &UserItemsMap, item_users: &ItemUsersMap) -> Result<Similarities, Box<Error>> {
    let mut similarities = Similarities::new();
    for (user_id, current_user_items) in user_items {
        let mut current_user_similarity = HashMap::<ItemId, Similarity>::new();

        for item_id in current_user_items {
            // similar users based on this item
            let candidate_users_for_item = item_users.get(&item_id).expect("set of user ids");

            for candidate_user_id in candidate_users_for_item {
                if candidate_user_id == user_id {
                    continue;
                }
                current_user_similarity.entry(*candidate_user_id).or_insert(0.0);
                *current_user_similarity.get_mut(candidate_user_id).expect("similarity") += 1.0;
            }

        }

        similarities.insert(*user_id, current_user_similarity);
    }
    Ok(similarities)
}
