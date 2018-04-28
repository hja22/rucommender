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

pub fn read_csv<R: io::Read>(ratings_source: R) -> Result<(UserItemsMap, ItemUsersMap), Box<Error>> {
    let mut rdr = csv::Reader::from_reader(ratings_source);
    let mut user_items = UserItemsMap::new();
    let mut item_users = ItemUsersMap::new();

    for result in rdr.deserialize() {
        let record: Record = result?;
        add_item(&mut user_items, &record.user_id, &record.item_id);
        add_user(&mut item_users, &record.item_id, &record.user_id);
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

fn add_item(user_items_map: &mut UserItemsMap, user_id: &UserId, item_id: &ItemId) {
    let empty_items = HashSet::<ItemId>::new();

    user_items_map.entry(*user_id).or_insert(empty_items);
    let current_user_items = user_items_map.get_mut(user_id).expect("cannot get items for given user id");
    current_user_items.insert(*item_id);
}

fn add_user(item_users_map: &mut ItemUsersMap, item_id: &ItemId, user_id: &UserId) {
    let empty_users = HashSet::<UserId>::new();

    item_users_map.entry(*item_id).or_insert(empty_users);
    let current_items_users = item_users_map.get_mut(item_id).expect("cannot get users for given items id");
    current_items_users.insert(*user_id);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_item_to_new_user() {
        let item_id = 101;
        let user_id = 1;

        let mut user_items = UserItemsMap::new();
        add_item(&mut user_items, &user_id, &item_id);

        let mut expected_user_items = UserItemsMap::new();
        let expected_item_set: HashSet<ItemId> = [ item_id ].iter().cloned().collect();
        expected_user_items.insert(user_id, expected_item_set);
        assert_eq!(user_items, expected_user_items);
    }

    #[test]
    fn add_user_to_new_item() {
        let item_id = 101;
        let user_id = 1;

        let mut item_users = ItemUsersMap::new();
        add_item(&mut item_users, &item_id, &user_id);

        let mut expected_item_users = ItemUsersMap::new();
        let expected_user_set: HashSet<UserId> = [ user_id ].iter().cloned().collect();
        expected_item_users.insert(item_id, expected_user_set);
        assert_eq!(item_users, expected_item_users);
    }
}
