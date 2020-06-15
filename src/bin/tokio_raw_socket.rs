use system_benchmarks::system::tokio::{build_raw_socket_system, run_system};

#[tokio::main]
async fn main() {
    run_system(build_raw_socket_system).await
}