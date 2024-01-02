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


# convert_an_array_into_a_2d_array_with_conditions
[2610. Convert an Array Into a 2D Array With Conditions]: https://leetcode.com/problems/convert-an-array-into-a-2d-array-with-conditions/
문제 해결 
* 입력 배열 nums의 각 숫자의 발생 횟수를 추적하기 위해 해쉬 맵 사용
* 배열을 순회하면서  각 숫자를 이전에 나타나지 않을 경우 첫 번째 사용 가능한 행으로 배치 
* 각 행은  중복되는 않는 정수만으로 포함하게 됨
* 모든 요소는 2차원 배열에 배치를 계속 함


