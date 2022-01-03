use std::fmt::format;

fn main() {
    let x = "leikang";
    let y = format!("my forvatis is {:?}",x);
    println!("{}",y);
    let x2 = "shanghai";
    let y2 = "large too";
    let z = format!("{} {}",x2,y2);
    println!("z={}",z);
    
    let x3 = 9;
    let y3 = "hello";
    let z3 = format!("{} {}",x3,y3);
    print!("z3:{}",z3);
    let x4 = format!("my name is {}",2);
    println!("x4:{}",x4);
    let x5 = format!("{0},{0},{0},{1},{2},{2},{3}","leikang","shanghai","nanjing",4);
    println!("x5:{:?}",x5);
    let x6 = format!("my name is {name},{age},{size}",name ="leikang",age =12,size=23);
    println!("x6:{}",x6);
    let x7 = format!("wang is {0},age is {1},name is {uname},demp is {2}","gulao",13,uname = "xiaoli");
    println!("x7:{}",x7);
    
    
}
    
    
