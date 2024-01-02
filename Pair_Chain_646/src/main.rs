fn main() {
   // println!("Hello, world!");
    let a  = vec![vec![1, 2], vec![2,3],vec![8,5]];
    //print!("{}",a.len());
    //print!("{}",a[2][1]);
    
    println!("{}",chain(a)); 
}

pub fn chain(pairs: Vec<Vec<i32>>) -> i32 {    
   use std::collections::HashMap;  
    let mut m:HashMap<i32,i32> = HashMap::new();
    let mut temp:Vec<i32> = Vec::new();


    for (i,col) in pairs.iter().enumerate(){
           // println!("Col[row={}]={:?}", i, col);
        for (j ,num) in col.iter().enumerate(){
           // println!("J = {}",j);
           // println!("num = {}",num); 
            println!("{}", pairs[i][j] );
           // temp.insert(col[0],col[1]);
            temp.push(col[0]);temp.push(col[1]); 
            println!("{:?}",temp);

            :w

            if temp[1] == *num && i > 0 && j==0 {
                println!("painek");    
            }else{    
                m.insert(col[0],col[1]);
                
            }
            temp.clear();
        }     
    }
        println!("{:?}",m);
        let leng = m.len();
        leng as i32 
}

        /*
        for n as i32 in ..2{
            println!(pairs[node][n]);
            if (n == 1){pairs[node][n]}
        }

    }*/


