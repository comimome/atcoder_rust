use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
    }
    let ans: Vec<&str> = s.split('.').collect();
    println!("{}",ans.last().unwrap());
}
