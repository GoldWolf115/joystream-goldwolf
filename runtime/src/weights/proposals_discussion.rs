//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 2.0.0

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::weights::{constants::RocksDbWeight as DbWeight, Weight};

pub struct WeightInfo;
impl proposals_discussion::WeightInfo for WeightInfo {
    // WARNING! Some components were not used: ["j"]
    fn add_post(i: u32) -> Weight {
        (361_189_000 as Weight)
            .saturating_add((508_000 as Weight).saturating_mul(i as Weight))
            .saturating_add(DbWeight::get().reads(4 as Weight))
            .saturating_add(DbWeight::get().writes(2 as Weight))
    }
    // WARNING! Some components were not used: ["j"]
    fn update_post() -> Weight {
        (231_487_000 as Weight).saturating_add(DbWeight::get().reads(3 as Weight))
    }
    fn change_thread_mode(i: u32) -> Weight {
        (379_400_000 as Weight)
            .saturating_add((1_244_000 as Weight).saturating_mul(i as Weight))
            .saturating_add(DbWeight::get().reads(3 as Weight))
            .saturating_add(DbWeight::get().writes(1 as Weight))
    }
}
