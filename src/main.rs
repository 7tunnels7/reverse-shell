use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use std::thread;

use ssh2::Session;

fn handle_client(mut stream: TcpStream, channel: Arc<Mutex<ssh2::Channel>>) {
    let mut buf = [0; 4096];

    let mut writer = stream.try_clone().unwrap();
    let reader = Arc::clone(&channel);

    let reader_thread = thread::spawn(move || {
        let mut reader = reader.lock().unwrap();
        loop {
            let nbytes = reader.read(&mut buf).unwrap();
            if nbytes == 0 {
                break;
            }
            writer.write_all(&buf[..nbytes]).unwrap();
        }
    });

    loop {
        let nbytes = stream.read(&mut buf).unwrap();
        if nbytes == 0 {
            break;
        }
        let mut channel = channel.lock().unwrap();
        channel.write_all(&buf[..nbytes]).unwrap();
    }

    reader_thread.join().unwrap();
}

fn main() {
    let remote_host = "127.0.0.1";
    let remote_port = 22;

    //SSH auth only
    // let remote_user = "admin";
    // let remote_password = "admin";

    let local_host = "localhost";
    let local_port = 8081;
    let remote_port_forwarded = 9000;

    let listener = TcpListener::bind((local_host, local_port)).unwrap();
    println!("Listening on {}:{}", local_host, local_port);

    let tcp = TcpStream::connect((remote_host, remote_port)).unwrap();
    let mut session = Session::new().unwrap();
    session.set_tcp_stream(tcp.try_clone().unwrap());
    session.handshake().unwrap();
    // session.userauth_password(remote_user, remote_password).unwrap();

    let channel = Arc::new(Mutex::new(session.channel_session().unwrap()));
    channel.lock().unwrap().request_pty("dumb", None, None).unwrap();
    channel.lock().unwrap().shell().unwrap();

    for incoming_stream in listener.incoming() {
        match incoming_stream {
            Ok(stream) => {
                let channel = Arc::clone(&channel);
                thread::spawn(move || {
                    handle_client(stream, channel);
                });
            }
            Err(e) => {
                eprintln!("Failed to accept connection: {}", e);
            }
        }
    }
}
