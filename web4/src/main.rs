
struct Coum{
    data:String
}
trait Drop{
    fn dsa(& mut self);
}
impl Drop for Coum{
    fn dsa(& mut self) {
        println!("dsa:{}",self.data);

    }
}
fn main() {
    let str1 = Coum {
        data:String::from("leikang is lei")
    };
    let str2 = Coum {
        data:String::from("sdf oiuy xcb")
    };
    println!("str1:{}",str1.data);
    println!("str2:{}",str2.data);
    let x:Box<i32> = Box::new(5);
    //let y =*x;
    let y =&x;
    println!("x:{}",x);
    println!("y:{}",y);

}
