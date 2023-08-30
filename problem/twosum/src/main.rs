fn main() {
/*
   let a = [1,2,3,4,5];
    let mut b = Vec::new()dd;
    let target = 5;

*/
    let a=[10,20,30,40,50];
    let target = 80;
    let mut nums: Vec<i32> = Vec::new();
    let mut i=0;
    while i < a.len()-1{
        let mut j=i+1;
        println!("start");
        
        while j < a.len(){
        println!("{},{}",&a[i],&a[j]);
        if a[i]+a[j] == target{
            nums.push(a[i]);
            nums.push(a[j]);
            println!("targeted!");
            println!("{:?}",&nums);
        }

        println!("---------");
        j +=1;
        }
        i+=1;
    }
    
}


pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut item : Vec<i32> = Vec::new();
    let mut i=0;
    while i < nums.len()-1 {
        let mut j=i+1;

        while j < nums.len(){
            if nums[i]+nums[j] == target {
            item.push(i);
            item.push(j);
            } j+=1;
        } i+=1;
    }
    item
}
