pub mod problem; // solutions 폴더를 모듈로 선언


#[cfg(test)]
mod tests {
    use super::problem::counting_words_with_a_given_prefix::Solution;

    #[test]
    fn test_prefix_count() {
        assert_eq!(
            Solution::prefix_count(
                vec![
                    "pay".to_string(),
                    "attention".to_string(),
                    "practice".to_string(),
                    "attend".to_string()
                ],
                "at".to_string()
            ),
            2
        );

        assert_eq!(
            Solution::prefix_count_user(
                vec![
                    "pay".to_string(),
                    "attention".to_string(),
                    "practice".to_string(),
                    "attend".to_string()
                ],
                "at".to_string()
            ),
            2
        );
    }
}
