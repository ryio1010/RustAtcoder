use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        S: String
    }
    let mut map : HashMap<char,usize> = HashMap::new();

    for s in S.chars() {
        *map.entry(s).or_insert(0) += 1;
    }

    for (k,v) in map.iter() {
        if *v == 1 {
            println!("{}",k);
            return;
        }
    }
    println!("-1");
}
