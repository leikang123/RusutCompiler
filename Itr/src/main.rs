
fn triangle(n:i32){
    let mut sum =0;
    for i in 1.. n+1{
        sum +=i;
    }
    println!("sum:{}",&sum);
}
fn main() {
    triangle(2);
    println!("main:");
    let mut v = vec![1,3,5,7,9,11];
   /** *for i in v {
        println!("i:{}",i)
    }*/
    let  mut ite = v.iter();
    println!("ite:{:?}",ite);
    assert_eq!(ite.next(),Some(&1));
    assert_eq!(ite.next(),Some(&3));
    let  ite2 = v.iter_mut();
    println!("ite2:{:?}",ite2);
    

}
