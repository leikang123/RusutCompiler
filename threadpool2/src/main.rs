use futures::executor::block_on;
async fn hello_async(){
    println!("hello async")
}
fn main() {
    let x = hello_async();
    block_on(x);
}
