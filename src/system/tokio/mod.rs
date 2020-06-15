use std::sync::atomic::{AtomicUsize, Ordering};

pub mod orchestrator;
pub mod p2p_parser;
pub mod raw_socket_producer;
pub mod nfqueue_producer;

pub mod prelude {
    pub use super::p2p_parser::spawn_p2p_parser;
    pub use super::raw_socket_producer::raw_socket_producer;
    pub use super::nfqueue_producer::nfqueue_producer;
    pub use super::orchestrator::spawn_packet_orchestrator;
}

pub fn build_raw_socket_system() -> std::io::Result<()> {
    raw_socket_producer::raw_socket_producer()
}

pub fn build_nfqueue_system() -> std::io::Result<()> {
    nfqueue_producer::nfqueue_producer()
}

static CAPTURED_DATA: AtomicUsize = AtomicUsize::new(0);

pub fn update_capture(len: usize) {
    let _ = CAPTURED_DATA.fetch_add(len, Ordering::SeqCst);
}

pub fn get_loaded_data() -> usize {
    CAPTURED_DATA.load(Ordering::SeqCst)
}

pub async fn run_system<F: Fn() -> std::io::Result<()>>(system_builder: F) {
    use tracing::{Level, info, error};
    use std::process::exit;

    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    match system_builder() {
        Ok(_) => {
            info!("system built");
        }
        Err(err) => {
            error!(error = display(err), "failed to build system");
            exit(-1)
        }
    }

    if let Err(err) = tokio::signal::ctrl_c().await {
        error!(signal = display("ctrl_c"), error = display(err), "failed while listening for signal");
        exit(-1);
    }

    info!(bytes = get_loaded_data(), "traced data");
    info!("ctrl-c received");
}