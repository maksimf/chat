extern crate mio;
use mio::*;
use mio::tcp::*;
use std::net::SocketAddr;

struct WebSocketServer;

impl Handler for WebSocketServer {
    type Timeout = usize;
    type Message = ();
}

fn main() {
    let mut event_loop = EventLoop::new().unwrap();
    // Create new instance of the handler struct
    let mut handler = WebSocketServer;

    // use lsof -i :10000 to make sure app is running
    let address = "0.0.0.0:10000".parse::<SocketAddr>().unwrap();
    let server_socket = TcpListener::bind(&address).unwrap();

    event_loop.register(&server_socket,
                        Token(0),
                        EventSet::readable(),
                        PollOpt::edge()).unwrap();

    // provide event loop with a mutable reference to it
    event_loop.run(&mut handler).unwrap();
}
