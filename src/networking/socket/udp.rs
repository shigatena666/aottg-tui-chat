// Import necessary modules
use std::io;
use std::net::UdpSocket;
use super::client::Client;

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
    fn send_query(&self, server_addr: String, query: &[u8]) -> io::Result<usize> {

        // Send the query to the server address and handle any errors
        let query = self.socket.send_to(query, server_addr)?;
        Ok(query)
    }

    fn receive(&self) -> io::Result<Vec<u8>> {

        // Receive the response from the server and handle any errors
        let mut buf = [0u8; 1024];
        let (amt, _) = self.socket.recv_from(&mut buf)?;
        Ok(buf[..amt].to_vec())
    }
}