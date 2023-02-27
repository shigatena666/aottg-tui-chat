// Import the networking module from a separate file
mod networking;
mod aottg;

// Import necessary modules
use networking::udp::UdpClient;
use networking::client::Client;

fn main() {

    // Define the IP address and port of the server to connect to
    let master_address = "135.125.239.180";
    let master_port = 5055;

    // Format the IP address and port into a single string to use as the server address
    let server_addr = format!("{}:{}", master_address, master_port);

    // Create a new instance of the UdpClient and handle any errors
    let udp_client = UdpClient::new().expect("Failed to create UDP client");

    // Send a query to the server and handle the response or any errors
    match udp_client.send_query(server_addr, &networking::udp::AOTTG_SERVER_LIST_QUERY) {
        Ok(response_bytes) => {
            println!("Received {} bytes from {}:{}", response_bytes.len(), master_address, master_port);
            println!("Response bytes: {:?}", &response_bytes[..]);
        }
        Err(e) => eprintln!("Failed to send query: {}", e),
    };
}