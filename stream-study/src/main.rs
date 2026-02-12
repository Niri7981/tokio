use async_stream::stream;
use tokio::time::{self, Duration, Instant};
use tokio_stream::StreamExt;

async fn get_price_stream() -> impl tokio_stream::Stream<Item = f64> {
    stream! {
        let mut next_time = Instant::now();
        loop {
            // 1. 这里的要求是：必须精准心跳，不能有累积误差
            time::sleep_until(next_time).await;

            // 2. 生成模拟价格
            let price = 140.0 + (rand::random::<f64>() * 20.0);
            yield price; // 把价格塞进水管

            // 3. 这里的要求是：设定下一次起跳时间
            next_time += Duration::from_millis(500);
        }
    }
}

#[tokio::main]
async fn main() {
    println!("📡 机器人已启动，目标：监控高价信号...");

    let raw_stream = get_price_stream().await;
    let mut high_price_stream = raw_stream.filter(|p| *p > 155.0);
    tokio::pin!(high_price_stream);

    loop {
        tokio::select! {
           Some(price) = high_price_stream.next()=>{
               println!("🔥 警报！价格突破 155: ${:.2}", price);
           }
        _ = tokio::signal::ctrl_c() => {
               println!("\n😴 收到指令，停止监控。早点休息，兄弟！");
               break;
             }
        }
    }
}
