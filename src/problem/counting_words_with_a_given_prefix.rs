pub struct Solution;

impl Solution {
    pub fn prefix_count(words: Vec<String>, pref: String) -> i32 {
        let mut res = 0;
        for s in &words {
            if s.starts_with(&pref) {
                res += 1;
            }
        }
        res 
    }


    pub fn prefix_count_user(words: Vec<String>, pref: String) -> i32 {
        words.iter().filter(|&s| s.starts_with(&pref)).count() as i32
    }
    
    pub fn prefix_count_user_str(words: Vec<String>, pref: String) -> Vec<String>{
        words.iter().filter(|&s|s.starts_with(&pref))
        .map(|s|s.to_string()).collect()
    }
  
}
