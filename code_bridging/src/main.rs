mod clients; // 声明有 clients 文件夹

use tokio::runtime::Builder;
use tokio::time::{Duration, sleep};

// 定义一个全项目通用的 Result 快捷方式
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
fn main() {
    let runtime = Builder::new_multi_thread()
        .worker_threads(1)
        .enable_all()
        .build()
        .unwrap();
    let mut handles = Vec::with_capacity(10);

    for i in 0..10 {
        handles.push(runtime.spawn(my_bg_task(i)));
    }

    std::thread::sleep(Duration::from_millis(750));
    println!("Finished time-consuming task");

    for handle in handles {
        runtime.block_on(handle).unwrap();
    }
}
async fn my_bg_task(i: u64) {
    // By subtracting, the tasks with larger values of i sleep for a
    // shorter duration.
    let millis = 1000 - 50 * i;
    println!("Task {} sleeping for {} ms.", i, millis);

    sleep(Duration::from_millis(millis)).await;

    println!("Task {} stopping.", i);
}
