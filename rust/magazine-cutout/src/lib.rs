// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::{collections::HashMap, hash::Hash};

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut magazine_map = HashMap::new();
    for m in magazine {
        magazine_map
            .entry(m)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    let mut note_map = HashMap::new();
    for n in note {
        note_map
            .entry(n)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    for n in note_map {
        let avaiable = match  magazine_map.get(n.0) {
            Some(v) => *v >= n.1 ,
            None => false
        };
        if !avaiable {
            return false;
        }
    }
    true
}
