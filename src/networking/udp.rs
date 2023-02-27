// Import necessary modules
use std::io;
use std::net::UdpSocket;
use super::client::Client;

// Define a constant representing a query to send to the server
pub const AOTTG_SERVER_LIST_QUERY: [u8; 56] = [
    255, 255, 0, 1, 0, 0, 0, 62, 14, 182, 52, 81, 2, 255, 1, 4, 0, 0, 0, 44, 0, 0, 0, 1, 0, 0, 4, 176, 0,
    0, 128, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 19, 136, 0, 0, 0, 2, 0, 0, 0, 2,
];

// Define a struct representing a UDP client
pub struct UdpClient {
    socket: UdpSocket,
}

// Implement methods for the UdpClient struct
impl UdpClient {

// Define a constructor method to create a new UdpClient
pub fn new() -> io::Result<Self> {
    
        // Bind the UDP socket to an available local address and port
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        Ok(Self { socket })
    }
}

// Implement the Client trait for the UdpClient struct
impl Client for UdpClient {

    // Define a method to send a query to a server and receive a response
    fn send_query(&self, server_addr: String, query: &[u8]) -> io::Result<Vec<u8>> {
        // Send the query to the server address and handle any errors
        self.socket.send_to(query, server_addr)?;

        // Receive the response from the server and handle any errors
        let mut buf = [0u8; 1024];
        let (amt, _) = self.socket.recv_from(&mut buf)?;
        Ok(buf[..amt].to_vec())
    }
}