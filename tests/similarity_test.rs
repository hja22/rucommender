extern crate rucfuu;

use std::collections::HashMap;
use std::fs::File;

#[test]
fn it_computes_similarities_for_dummy_example() {
    let file = File::open("examples/dummy/activity.csv").unwrap();
    let actual = rucfuu::similarities(file).unwrap();
    let mut expected = HashMap::<u32, HashMap<u32, f64>>::new();
    let empty_sims = HashMap::<u32, f64>::new();
    
    expected.insert(1, empty_sims.clone());
    expected.insert(2, empty_sims.clone());
    expected.insert(3, empty_sims.clone());
    expected.insert(4, empty_sims.clone());

    expected.get_mut(&1).unwrap().insert(2, 2.0);
    expected.get_mut(&2).unwrap().insert(1, 2.0);
    expected.get_mut(&2).unwrap().insert(3, 1.0);
    expected.get_mut(&3).unwrap().insert(2, 1.0);

    assert_eq!(expected, actual);

}
