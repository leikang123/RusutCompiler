// 范型结构体的使用案例
struct Room<T>{
    width:T,
    length:T,

}
impl<T> Room<T>{
    fn circ(&self) -> &T{
        return &self.length;
    }
    fn circ2(&self) -> &T {
       return  &self.width;
    }
}
impl Room<i32>{
    fn arec(&self) -> i32{
        return &self.length * &self.width;

    }
}
fn main() {
    let x = Room{
        width:12,
        length:13
    };
    println!("x.width:{},x.length:{}",x.width,x.length);
    println!("x.arec:{}",x.arec());
    
}
