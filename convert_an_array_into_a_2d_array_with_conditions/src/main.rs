use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn find_matrix(nums: Vec<i32>) -> Vec<Vec<i32>> {
        //해쉬맵 초기화 
        let mut matrix = HashMap::new();
        //결과
        let mut result  = Vec::new();

        //n은 벡터를 순회하면서  각 숫자를 HashMap에 카운트를 찾거나 없을 경우 0으로 삽입한다.  
        for n in nums{
           
           //특정 키가 값을 가지고 있는 검사 
            //count : matrix의해쉬맵에서 n값을 엔트리로 가져옴 
            //or_result(0) : n의 카가 Hashmap에 존재 하지 않을 경우 0을 추가 하고 존재 할시 값에가변 참조를 반환 
           let count  = matrix.entry(n).or_insert(0); 
           println!("{}",n);
           println!("count : {}",count);
           println!("{:?}의 길이{}",result,result.len());
           println!();
           
           // 만약 카운트가 결과값에 같은 길이일 경우  result에 새로운 행을 추가 
           // count는 해당 n의 발생 횟수의 값
           //ex)1이 두번째로 나타날때  이미 아래에서 카운트를 해서 result의 길이 같이져서 새로운 행을 추가 하게 됨
            if *count == result.len(){
                result.push(Vec::new());
            }
            //
                //num이 추가될 행을 가리킴 
                result[*count].push(n);
                //count는 num의 발생 횟수에 대한 가변 참조이므로, 이를 1만큼 증가 
                *count+=1;
        }

        println!("해쉬 : {:?}",matrix);
        result
    }
    }
fn main() {
    let nums = vec![1, 3, 4, 1, 2, 3, 1];
    let matrix = Solution::find_matrix(nums);
    println!("nums:{:?}",matrix);
}
