use std::rc::Rc;
use std::cell::{RefCell,Ref,RefMut};
fn main() {
    let x = Rc::new(6);
    println!("{:p},count after construting x:{}",x,Rc::strong_count(&x));
    
    let y = x.clone();
    println!("{:p},count after construting y:{}",y,Rc::strong_count(&x));
    
    println!("count after y:{}",Rc::strong_count(&x));

    let z = Rc::clone(&x);
    println!("{:p},count after construting z:{}",z,Rc::strong_count(&x));

    println!("count after z:{}",Rc::strong_count(&x));
    // RefCell<T>
    let v:RefCell<Vec<i32>> = RefCell::new(vec![1,2,3,4]);
    println!("{:?}",v.borrow());
    v.borrow_mut().push(6);
    println!("{:?}",v);
    let v2 :RefCell<Vec<i32>> = RefCell::new(vec![2,3,4,5,6]);
    let v_brrow_1:Ref<Vec<i32>> = v2.borrow();
    println!("{:?}",v_brrow_1);
    //let v_brrow_2:Ref<Vec<i32>>  = v2.borrow();
    //println!("{:?}",v_brrow_2);
    //let v_brrow_3:Ref<Vec<i32>> = v2.borrow();
    //println!("{:?}",v_brrow_3);
    let mut v_brrow_4:RefMut<Vec<i32>> =v2.borrow_mut();
    v_brrow_4.push(13);
    println!("{:?}",v_brrow_4);
    
    

}
