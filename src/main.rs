extern crate appnetcore;

use std::sync::mpsc::{Sender, Receiver, TryRecvError};
use std::sync::mpsc;

use appnetcore::reader::CommCommand;
use appnetcore::reader::PacketReaderServer;
use appnetcore::network::read_packets;

use appnetcore::connstate::SocketReadAddress;

//
// Grabs 1 command off the channel and executes it.
//
fn check_comm_commands(rx: &Receiver<Box<CommCommand + Send>>,
                       /*client_state: & mut HashMap<String,ClientHandle> */) -> Result<Box<CommCommand>, TryRecvError> {
    let received_value = rx.try_recv()?;
    received_value.execute(client_state);
    Ok(received_value)
}

fn get_current_time() -> u64 {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    let in_ms = since_the_epoch.as_secs() * 1000 +
        since_the_epoch.subsec_nanos() as u64 / 1_000_000;
    in_ms
}



fn main() {

    let client_address = SocketReadAddress{
        read_host: String::from("localhost"),
        _read_port: 1234
    };

    // -
    // - Bind to port XYZ for listening... This becomes the "ClientHandle" for self
    // - Set up reader, just like server, for receiving incoming commands
    // -
    // -
    println!("Initialization...");

    // States.
    //let mut client_state: HashMap<String,ClientHandle> = HashMap::new();

    let (tx,command_rx): (Sender<Box<CommCommand + Send>>, Receiver<Box<CommCommand + Send>>) = mpsc::channel();
    let pri = PacketReaderServer::with_sender(tx);

    // Initialize our packet reader
    let _rthread = read_packets(pri);

    println!("Initialized.");


    let mut previous = get_current_time();

    let mut lag: f64 = 0.0;

    loop {
        let current = get_current_time();
        let elapsed = current - previous;
        previous = current;

        lag = lag + elapsed as f64;

        while lag >= MS_PER_UPDATE {
            // Process connection commands
            let _ = check_comm_commands(&command_rx); //, &mut client_state);

            // Process game state

            // Crank engine
        }
        // render(lag / MS_PER_UPDATE) // Not useful for connection state but will be with graphics
    }
}
