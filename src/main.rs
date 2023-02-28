// Import the networking module from a separate file
mod networking;
mod aottg;
mod utils;

// Import necessary modules
use networking::socket::udp::UdpClient;
use networking::socket::client::Client;
use networking::serialization::packetserializer::PacketSerializer;

// Define a constant representing a query to send to the server
pub const AOTTG_SERVER_LIST_QUERY: [u8; 44] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

fn main() {

    // Define the IP address and port of the server to connect to
    let master_address = "135.125.239.180";
    let master_port = 5055;

    // Format the IP address and port into a single string to use as the server address
    let server_addr = format!("{}:{}", master_address, master_port);

    // Create a new instance of the UdpClient and handle any errors
    let udp_client = UdpClient::new().expect("Failed to create UDP client");

    let packet_serializer = PacketSerializer::new();

    // Send a query to the server and handle the response or any errors
    match udp_client.send_query(server_addr, packet_serializer.sequence.get_ref()) {
        Ok(bytes_count) => {
            println!("Sent {} bytes to {}:{}", bytes_count, master_address, master_port);
            println!("Sent bytes: {:?}", &packet_serializer.sequence.get_ref()[..]);
        }
        Err(e) => eprintln!("Failed to send query: {}", e),
    };
    
    // Receive the response from the server and handle the result or any errors
    match udp_client.receive() {
        Ok(response_bytes) => {
            println!("Received {} bytes from {}:{}", &response_bytes.len(), master_address, master_port);
            println!("Response bytes: {:?}", &response_bytes[..]);
        }
        Err(e) => eprintln!("Failed to receive data: {}", e),
    };
}