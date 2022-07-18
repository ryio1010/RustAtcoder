use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        S: String,
        a:usize,
        b:usize,
    }
    let mut str : Vec<char> = S.chars().collect();
    let tmp : Vec<char> = S.chars().collect();

    str[a-1] = tmp[b-1];
    str[b-1] = tmp[a-1];

    let ans:String = str.iter().collect();

    println!("{}", ans);
}
