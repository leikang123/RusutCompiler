pub fn as_a(x:i32) ->bool{
    return x>1
}
pub fn add(x:i32,y:i32) ->i32{
    return x+y
}
pub fn sub(x:i32,y:i32) -> i32{
    return x-y
}
pub fn mulit(x:i32,y:i32) ->i32{
    return x *y
}
pub fn dev(x:i32,y:i32) ->i32{
    return x /y
    
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn as_a_test(){
        assert!(as_a(3));
    }
    #[test]
    fn add_test() {
        assert_eq!(add(2,3),5);
    }
    #[test]
    fn add_test_2(){
        assert_ne!(add(1,2),3);
    }
    #[test]
    
    fn sub_test(){
        assert_eq!(sub(12,4),8);
    }
    #[test]
    fn mulit_test(){
        assert_eq!(mulit(4,5),20);
    }
    #[test]
    #[ignore]
    fn dev_test(){
        assert_eq!(dev(12,3),4);
    }

}