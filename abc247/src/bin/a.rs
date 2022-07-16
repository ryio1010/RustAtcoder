use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
    }
    let ans = &s[0..3];
    println!("0{}",ans);
}
