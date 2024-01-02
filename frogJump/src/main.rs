fn main() {

    let stone = [0,1,2,3,4,8,9,11]; 
     println!("{}",  can_crossi1(stone.to_vec())); 
}
//해쉬맵 사용
use std::collections::{HashMap, HashSet};
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn can_cross(stones: Vec<i32>) -> bool {
        // Create a HashMap to store possible jump distances for each stone position
        let mut dp: HashMap<i32, HashSet<i32>> = HashMap::new();
        
        // Initialize jump set for each stone
        //각가의 stone을 반복을 수행
        for &stone in &stones {
            dp.insert(stone, HashSet::new());
        }
        
        // Initialize the first stone with jump distance 0
        //점프 거리가 0인 첫 번째 stone 초기화
        dp.get_mut(&0).unwrap().insert(0);

        // Iterate through each stone 
        //각각의 stone을 반복통함
        for &stone in &stones {
            // 변경 가능한 차용 없이 반복하도록 jump_set clone
            if let Some(jump_set) = dp.get(&stone).cloned() {
               
                for jump in jump_set {
                    // Try jump distance: jump - 1, jump, jump + 1
                    for &jump_distance in &[jump - 1, jump, jump + 1] {
                        // 점프가 유효하고 목적지 stone이 존재하는지 확인합니다
                        if jump_distance > 0 && dp.contains_key(&(stone + jump_distance)) {
                            // 목적지 스톤의 점프 세트에 유효한 점프 거리 삽입
                            dp.entry(stone + jump_distance)
                                .or_insert(HashSet::new())
                                .insert(jump_distance);
                        }
                    }
                }
            }
        }

        // 유효한 경로를 나타내는 마지막 스톤의 점프 세트가 비어 있지 않은지 확인합니다
        !dp[&stones.last().unwrap()].is_empty()
    }
}
