impl Solution {
    pub fn kth_character(k: i32) -> char {
        let mut result: String = String::from("a");

        for _ in 0..k {
            if result.len() < k as usize {
                //all_next_words
                let next_words = result
                    .chars()
                    .map(|x| ((x as u8) + 1) as char)
                    .collect::<String>();
                //push a word
                result = result + &next_words;
                println!("{}", result);
            } else {
                break;
            }
        }

        let result = result.chars().nth(k as usize - 1).unwrap();
        return result;
    }
}
struct Solution;
fn main() {
    let ch = Solution::kth_character(5);
    println!("ch:{}", ch);
}
