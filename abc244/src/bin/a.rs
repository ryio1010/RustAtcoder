use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        _n : usize,
        mut s : String,
    }
    
    println!("{}",s.pop().unwrap());
}
