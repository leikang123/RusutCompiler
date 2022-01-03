use std :: fmt::{Display,Formatter,Result};
trait common{
    // 没有方法体，处于抽象方法，类似于结口一样
   fn arec(&self) ->i32;
   fn perimer(&self) ->i32;
}
struct Room{
    width:i32,
    length:i32
}
struct Circle{
    radis:i32
}
#[derive(Debug)]
struct Point{
    x:i32,
    y:i32
}
impl common for Room {
 fn arec(&self) ->i32{
     return &self.length * &self.length;
 }
 fn perimer(&self) -> i32{
     return &self.length + &self.width;
 }
}
impl common for Circle {
    fn arec(&self) -> i32{
        return self.radis *self.radis*3;
    }
    fn perimer(&self) ->i32{
        return &self.radis*3*2;
    }
}
impl Display for Point{
    fn fmt(&self,f:&mut Formatter<'_>) -> Result{
        return write!(f,"({},{})",self.x,self.y)
    }

}
fn main() {
    let orign = Point{
        x:12,
        y:34
    };
    println!("{}",orign);
    println!("{:?}",orign);
    println!("{:#?}",orign);
    let x = Room {
        width:14,
        length:23
    };
    println!("x.width:{},x.length():{}",x.width,x.length);
    println!("x.arec:{},x.perimer:{}",x.arec(),x.perimer());
    let x2 =Circle{
        radis:4
    };
    println!("x2.radis:{},x2.arec:{},x2.perimer:{}",x2.radis,x2.arec(),x2.perimer());
}
