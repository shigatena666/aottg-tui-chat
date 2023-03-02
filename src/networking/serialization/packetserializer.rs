use crate::utils::streambuffer::StreamBuffer;

// Define a constant representing a query to send to the server
const CHANNEL: u8 = 2;

// Define a constant representing a query to send to the server
pub const AOTTG_HEADER: [u8; 24] = [
    255, 255, 0, 1, 0, 0, 0, 62, 14, 182, 52, 81, 2, 255, 1, 4, 0, 0, 0, 44, 0, 0, 0, 1,
    ];

pub struct PacketSerializer {
    pub sequence: StreamBuffer,
}

impl PacketSerializer {

    // Define a constructor method to create a new PacketSerializer
    pub fn new() -> Self {
        let sequence = Self::init_sequence();
        PacketSerializer { sequence }
    }

    // Serialize a 16-bit integer into a byte array
    pub fn serialize(value: i16, target: &mut [u8], target_offset: &mut usize) {
        target[*target_offset] = (value >> 8) as u8;
        *target_offset += 1;
        target[*target_offset] = value as u8;
        *target_offset += 1;
    }

    // Initialize a sequence to send to the server
    fn init_sequence() -> StreamBuffer {

        // Initialize a byte array with 32 elements
        let mut array = [0u8; 32];

        // Write the maximum transmission unit (MTU) to the byte array
        Self::serialize(1200, &mut array[2..], &mut 2);

        // Write the remaining bytes to the byte array
        array[0] = 0;
        array[4] = 0;
        array[5] = 0;
        array[6] = 128;
        array[7] = 0;
        array[11] = CHANNEL;
        array[15] = 0;
        array[19] = 0;
        array[22] = 19;
        array[23] = 136;
        array[27] = 2;
        array[31] = 2;

        let bytes = array.to_vec();
        let mut header = AOTTG_HEADER.to_vec();
        header.extend(bytes);

        // Convert the byte array to a StreamBuffer
        StreamBuffer::from(header)
    }
}