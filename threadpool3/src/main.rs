use futures::executor::block_on;
use futures::join;
use futures::DataStruct;
async fn lds() -> DataStruct{

}
async fn la(d_s:DataStruct){

}
async fn lr(){

}
async fn ldsaa(){
    let d_s = lds().await;
    la(d_s).await
}
async fn async_main(){
    let futures1 = ldsaa();
    let futures2 = lr();
    join!(futures1,futures2);

}

fn main() {
    block_on(async_main());
}
