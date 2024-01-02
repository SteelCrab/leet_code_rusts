# The Rust Programming Language

[![Rust Community](https://img.shields.io/badge/Rust_Community%20-Join_us-brightgreen?style=plastic&logo=rust)](https://www.rust-lang.org/community)


## leet_code_rusts
solving the letcode problem

## problems
`addsun`

`frogJump`

`PalindromeNumber`

 ## PalindromeNumber functions 
 * `fn is_palindrome`
        [to_string],[chars],[push],[clone],[reverse],[iter],[enumerate]
   
* `fn expert`
        [to_string] [chars] [rev] [collect]
    
[to_string]: https://doc.rust-lang.org/std/string/trait.ToString.html#tymethod.to_string
[chars]: https://doc.rust-lang.org/std/primitive.str.html#method.chars
[push]: https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html#method.push
[clone]: https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html#method.push
[reverse]: https://doc.rust-lang.org/std/cmp/struct.Reverse.html
[iter]: https://doc.rust-lang.org/std/primitive.slice.html#method.iter
[enumerate]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.enumerate
[rev]: https://doc.rust-lang.org/std/iter/struct.Rev.html
[collect]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect


## Convert an Array Into a 2D Array With Conditions
[2610. Convert an Array Into a 2D Array With Conditions]: https://leetcode.com/problems/convert-an-array-into-a-2d-array-with-conditions/
### 문제 해결 
* 입력 배열 nums의 각 숫자의 발생 횟수를 추적하기 위해 해쉬 맵 사용
* 배열을 순회하면서  각 숫자를 이전에 나타나지 않을 경우 첫 번째 사용 가능한 행으로 배치 
* 각 행은  중복되는 않는 정수만으로 포함하게 됨
* 모든 요소는 2차원 배열에 배치를 계속 함
use std::collections::HashMap;

 ``` Rust
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
          
            if *count == result.len(){
                result.push(Vec::new());
            }
            //
                //num이 추가될 행을 가리킴 
                result[*count].push(n);
                //count는 num의 발생 횟수에 대한 가변 참조이므로, 이를 1만큼 증가 
                *count+=1;
        }

       
        result
    }
    
    }

    ``` 


