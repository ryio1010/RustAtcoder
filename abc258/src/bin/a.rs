use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        K:usize
    }
    let ans1 = 21 + K/60;
    let ans2 = K%60;
    println!("{}:{:>02}",ans1,ans2);
}
