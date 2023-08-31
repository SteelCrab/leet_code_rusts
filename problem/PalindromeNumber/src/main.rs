//9. Palindrome Number

fn main() {
    println!("{}",is_palindrome(-121));
    println!("{}",expert(-121));
}

fn is_palindrome(x: i32) -> bool{  
    //받아온 매개변수를 문자열로 변환
    let st: String = x.to_string();
    
    //벡터값 생성
    let mut digits  = Vec::new();
    //for로 각  벡터값 삽입
    //chars():char 문자열을 매개변수로 반환하여 벡터에 삽입
    for c in st.chars() {
           digits.push(c); 
   }
   
    //벡터값을 복제하여 슬라이스를 역순으로 정렬
    //reverse():슬라이스의 요소 순서를 제자리에 반대로 바꿈
    let mut digits_reverse= digits.clone();
    digits_reverse.reverse();

    //iter() : 반환된 반복자  인덱스를 반환
    //enumerate():현재 반복 횟수를 제공하는 반복자를 생성
   for (i,digit) in digits.iter().enumerate() {
        if &digits_reverse[i] != digit {
            return false;
        }
   }
   true
}


fn expert(x:i32) -> bool {
//to_string() : 문자열로 반환
//chars():char 문자열 슬라이스로 반복자 반환
//rev():반복자의 방향을 반대로 바꿉니다. 
//collect():반복 가능한 모든 것을 취하여 관련성 있는 결과반환
    let rev:String = x.to_string().chars().rev().collect();
    //다시 문자열로 반환하여 다르면 false로 반환
    return rev == x.to_string()

}
