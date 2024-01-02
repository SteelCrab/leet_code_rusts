fn main() {

    let v = vec![3,5,10,7,5,3,5,5,4,8,1];
    let s = String::from("aaabbbabbbb");
    
    let sum = min_cost(s,v);

    println!("결과 : {}",sum);
}

pub fn min_cost(colors:String, needed_time:Vec<i32>) -> i32 {
  
    let mut l = 0;
 

    let mut sum = 0;
    let str_vec:Vec<char> = colors.chars().collect();

    for r in 1..str_vec.len(){

    //현재 l,r의 배열상태 출력=
      println!("l[{}]:{}:{} , r[{}]:{}:{}",l,str_vec[l],needed_time[l],r,str_vec[r],needed_time[r]);
   
      //같은 문자일 경우 
        if str_vec[l] == str_vec[r] {
          //str_vec의해당  인덱스  needed_time을 sum에 저장
          if needed_time[l] >  needed_time[r]{
            sum += needed_time[r];
            println!("+r sum : {}",sum);
            }else{
            sum += needed_time[l];
            println!("+l sum : {}",sum);
            l = r;
            }
        }else{
          l = r;
        }
        println!("{},{}",l,r);
        println!();
    }

sum
}
