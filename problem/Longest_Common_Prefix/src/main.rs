fn main() {
    println!("Hello, world!");
    
    let mut strs :Vec<String> = vec![];
    strs.push(String::from("flower"));
    strs.push(String::from("flowi"));
    strs.push(String::from("flight"));
    let a = longest_common_prefix(strs);
    println!("공통 접두어 : {}",a);
}
pub fn longest_common_prefix(strs: Vec<String>) -> String {
    
    let mut count:Vec<char> = Vec::new();
    let mut con_str:String = String::from("");
    let mut re_str:String = String::from("");
    for item in strs.iter(){
    println!("{}",item); 
        
        let mut temp=0; // 카운트시킴 
                        
        for i in item.chars() {
        println!("{}", i);






            if strs[0].eq(item) || temp == 0  {
                count.push(i); 
               
            }else if  count[temp].eq(&i){
                
                let count_str:String =count.iter().cloned().collect();
                con_str = count_str; 
            }
            
            //if temp < strs[0].len()&& count[temp].eq(&i) {
           
            //con_str.insert(temp, i);
            //println!("con_str : {:?}",con_str);
            
            println!("count : {:?}",count);
            temp += 1;
        } 
    }
    con_str
}


