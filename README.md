# Rucommender
Recommendation system written in Rust

## Overview

An implementation in Rust of a collaborative filtering recommendations algorithm with a user-user similarity based on implicit ratings on the same items (e.g. both users clicked on the same item).

**STATUS:** This is project I'm using to learn some Rust and to reinforce my knowledge of recommender systems.

## Getting Started

### Pre-requisites

Rust and Cargo

### Setting up

1. Fork and clone (or download)
2. `cargo build`

### Usage

#### Inputs

1. CSV of implicit ratings by users on items, e.g.
    ```csv
    user_id,item_id
    1,100
    1,101
    2,100
    2,102
    ```

#### Outputs
Depends on command used:

* user->user similarity maps
* user->(item, score) recommendation maps

#### Examples

To spit out some similarities for a set of activities to make sure everything is working:
`cargo run --bin similarities < tests/fixtures/dummy/implicit-ratings.csv`

To spit out some recommendations for a set of activities to make sure everything is working:
`cargo run --bin recommendations < tests/fixtures/dummy/implicit-ratings.csv`

#### Tests

`cargo test`

## Getting involved

### Community

[Ruccommender has a mailing list](https://groups.google.com/forum/#!forum/rucommender/new). Feel free to join it and ask any questions you may have.

### Contributing

Contributions welcome.
How?
Err.
Fork and PR?
I don't know I've not done much of this before.
