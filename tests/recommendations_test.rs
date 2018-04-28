extern crate rucommender;

use std::collections::HashMap;
use std::fs::File;

#[test]
fn it_computes_recommendations_for_dummy_example() {
    let file = File::open("tests/fixtures/dummy/implicit-ratings.csv").unwrap();
    let actual = rucommender::recommendations(file).unwrap();
    let mut expected = HashMap::<u32, HashMap<u32, f64>>::new();
    let empty_recs = HashMap::<u32, f64>::new();
    
    expected.insert(1, empty_recs.clone());
    expected.insert(2, empty_recs.clone());
    expected.insert(3, empty_recs.clone());
    expected.insert(4, empty_recs.clone());

    expected.get_mut(&1).unwrap().insert(100, 2.0);
    expected.get_mut(&1).unwrap().insert(101, 2.0);
    expected.get_mut(&1).unwrap().insert(103, 2.0);

    expected.get_mut(&2).unwrap().insert(100, 2.0);
    expected.get_mut(&2).unwrap().insert(101, 2.0);
    expected.get_mut(&2).unwrap().insert(102, 2.0);
    expected.get_mut(&2).unwrap().insert(103, 1.0);
    expected.get_mut(&2).unwrap().insert(104, 1.0);

    expected.get_mut(&3).unwrap().insert(100, 1.0);
    expected.get_mut(&3).unwrap().insert(101, 1.0);
    expected.get_mut(&3).unwrap().insert(103, 1.0);

    assert_eq!(expected, actual);
}
