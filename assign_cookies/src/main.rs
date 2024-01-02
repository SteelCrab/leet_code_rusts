struct Solution;

fn main() {
    let g = vec![1,2,3];
    let s = vec![1,1];
    let result = Solution::find_content_children(g, s);
    println!("{}",result);
}
impl Solution {
    pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
    g.sort(); // 욕심 지수 배열을 정렬
    s.sort(); // 쿠키 크기 배열을 정렬

    let mut child = 0;
    let mut cookie = 0;

    while child < g.len() && cookie < s.len() {
        if g[child] <= s[cookie] {
            // 현재 쿠키가 현재 아이를 만족시킬 수 있다면
            child += 1; // 다음 아이로 넘어감
        }
        cookie += 1; // 다음 쿠키로 넘어감
    }

    child as i32 // 만족한 아이들의 수
}

pub fn find_content_children_2(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
    g.sort(); // 욕심 지수 배열을 정렬
    s.sort(); // 쿠키 크기 배열을 정렬

    let mut count=0;

    while !g.is_empty() && !s.is_empty() {
        //가장 큰 욕심지수가 큰 아이를 선택
       let x = g.pop().unwrap();
       
       //아이의 욕심 지수가 쿠키 사이즈보다 더 클경우 다음 아이로 넘어감
       if x  > *s.last().unwrap(){
        continue;
       }
    // 아이의 욕심 지수가 현재 가장 큰 쿠키로 만족될 수 있다면, 이 쿠키를 먹임
       s.pop();
       count+=1;
    }

    count
}


}

