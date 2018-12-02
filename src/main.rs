extern crate appnetcore;
extern crate termion;

use std::collections::HashMap;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

use appnetcore::reader::{check_app_commands, check_comm_commands};
use appnetcore::reader::{CommCommand,AppCommand};
use appnetcore::reader::PacketReaderServer;
use appnetcore::network::read_packets;

use appnetcore::writer::PacketWriter;


use appnetcore::connstate::SocketReadAddress;

use std::time::{SystemTime, UNIX_EPOCH};

use std::io::Read;


fn get_current_time() -> u64 {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    let in_ms = since_the_epoch.as_secs() * 1000 +
        since_the_epoch.subsec_nanos() as u64 / 1_000_000;
    in_ms
}


const MS_PER_UPDATE: f64 = 60.0;

fn main() {
    // -
    // - Bind to port XYZ for listening... This becomes the "ClientHandle" for self
    // - Set up reader, just like server, for receiving incoming commands
    // -
    // -
    println!("Initialization...");

    let listen_address = SocketReadAddress{
        read_host: String::from("localhost"),
        _read_port: 10001
    };

    // States.
    let mut client_state: HashMap<String,SocketReadAddress> = HashMap::new();

    let (tx,command_rx): (Sender<Box<CommCommand + Send>>,
                          Receiver<Box<CommCommand + Send>>) = mpsc::channel();

    let (app_tx,app_command_rx): (Sender<Box<AppCommand + Send>>,
                                  Receiver<Box<AppCommand + Send>>) = mpsc::channel();

    let pri = PacketReaderServer::with_senders(tx, app_tx);

    // Initialize our packet reader
    let _rthread = read_packets(pri, &listen_address);

    println!("Initialized.");
    println!("Connecting...");

    let packet_writer = PacketWriter::with_destination(
        "127.0.0.1","10001",
        "testclient","testpass",
        "127.0.0.1","10000");

    packet_writer.send_connection_request();
    eprintln!("Connected.");
    packet_writer.send_connection_request();
    eprintln!("Connected.");
    packet_writer.send_connection_request();
    eprintln!("Connected.");
    packet_writer.send_connection_request();
    eprintln!("Connected.");
    packet_writer.send_connection_request();
    eprintln!("Connected.");
    let mut previous = get_current_time();
    let mut lag: f64 = 0.0;
    let mut stdin_reader = termion::async_stdin();
    let mut data_read = String::new();

    loop {
        let current = get_current_time();
        let elapsed = current - previous;
        previous = current;

        lag = lag + elapsed as f64;

        while lag >= MS_PER_UPDATE {
            // Process connection commands
            let _ = check_comm_commands(&command_rx, &mut client_state);
            let _ = check_app_commands(&app_command_rx);

            // Process input
            let numbytes = stdin_reader.read_to_string(&mut data_read).unwrap();
            if numbytes > 0 {
//                eprintln!("Number read: {}", numbytes);
//                eprintln!("String read: {}", line_read);
                eprint!("{}",data_read);
                packet_writer.send_text_message(12, &data_read);
                data_read.clear();
            }
            // Process game state

            // Crank engine
        }
        // render(lag / MS_PER_UPDATE) // Not useful for connection state but will be with graphics
    }
}
