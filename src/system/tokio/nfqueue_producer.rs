use tracing::{trace, warn, error};
use crate::{
    system::tokio::orchestrator::spawn_packet_orchestrator,
    messages::tcp_packet::Packet,
};
use std::{
    io,
    process::exit,
};

pub fn nfqueue_producer() -> io::Result<()> {
    use nfq::{Queue, Verdict};

    let mut queue = Queue::open()?;
    queue.bind(0)?;

    let orchestrator = spawn_packet_orchestrator();
    std::thread::spawn(move || {
        loop {
            let capture = queue.recv();
            match capture {
                Ok(mut msg) => {
                    let packet = if let Some(packet) = Packet::new(msg.get_payload()) {
                        trace!(captured_length = packet.ip_buffer().len(), "captured packet");
                        super::update_capture(packet.ip_buffer().len());
                        packet
                    } else {
                        msg.set_verdict(Verdict::Accept);
                        let _ = queue.verdict(msg);
                        continue;
                    };
                    msg.set_verdict(Verdict::Accept);
                    let _ = queue.verdict(msg);

                    loop {
                        match orchestrator.send(packet) {
                            Ok(()) => {
                                trace!("sent packet for processing");
                                break;
                            }
                            Err(_) => {
                                error!("orchestrator channel closed abruptly");
                                exit(-1);
                            }
                        }
                    }
                }
                Err(err) => {
                    warn!(error = display(err), "failed capture packet from socket");
                }
            }
        }
    });
    Ok(())
}