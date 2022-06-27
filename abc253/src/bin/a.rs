use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a:u32,
        b:u32,
        c:u32,
    }
    let mut v:Vec<u32> = vec![a,b,c];
    v.sort();
    let ans = if v[1]==b {"Yes"} else {"No"};
    println!("{}",ans);
}