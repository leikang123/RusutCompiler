use threadpool::ThreadPool;

fn main() {
    // 线程池出创建3条线程处理
    let pool = ThreadPool::new(3);
     for i in 1..5{
         pool.execute(move || {
         println!("number is {} thread_1",i);
         });
     }

     for i in 1..5 {
         pool.execute(move||{
             println!("Number is {} thread_2",i);
         })
     }
     for i in 1..5{
         pool.execute(move || {
             println!("number is {} thread_3",i)
         });
     }
     pool.join();
}
