use anyhow::Result;
use arbitrage_bot::main_loop;

fn main() -> Result<()> {
    actix::System::with_tokio_rt(|| {
        tokio::runtime::Builder::new_multi_thread()
            .worker_threads(num_cpus::get())
            .enable_all()
            .build()
            .unwrap()
    })
    .block_on(main_loop())
}
