pub fn is_pos(x:i32) -> bool{
    x >0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_pos_test() {
        assert!(is_pos(3));
    }
}
