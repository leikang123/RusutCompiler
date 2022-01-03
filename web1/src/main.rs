fn main() {
    let test_string = String::from("Hello world");
    // 函数print调用变量test_string
    print(test_string);
    let srt_a = 12;
    asd(srt_a);
}
// 定义函数带变量名input
fn print(input:String) {
    println!("{}",input);
}
fn asd(s:i32){
    println!("{}",s);
}