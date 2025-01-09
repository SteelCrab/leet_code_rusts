pub mod problem;

fn main() {
    println!("Run `cargo test` to execute tests.");
    println!("{:?}",problem::counting_words_with_a_given_prefix::Solution::prefix_count_user_str(
        vec![
            "pay".to_string(),
            "attention".to_string(),
            "practice".to_string(),
            "attend".to_string()
        ],
        "at".to_string()
    ));
}

