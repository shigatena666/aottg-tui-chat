// Import necessary modules
use std::io;

pub trait Client {

    // Define a method to send a query to a server and receive a response
    fn send_query(&self, server_addr: String, query: &[u8]) -> io::Result<usize>;
    fn receive(&self) -> io::Result<Vec<u8>>;
}
