use system_benchmarks::system::tokio::{build_nfqueue_system, run_system};

#[tokio::main]
async fn main() {
    run_system(build_nfqueue_system).await
}