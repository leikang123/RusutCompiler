pub fn add(x:i32,y:i32) ->i32{
    return x+y+1
}
pub fn sub(x:i32,y:i32) -> i32{
    return x- y
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_tset() {
        assert_eq!(add(3,4),8);
    }
    #[test]
    fn sub_tset(){
        assert_ne!(sub(8,9),-1);
    }
}
