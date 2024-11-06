// TODO Implement

// List of possible errors
#[derive(Debug)]
pub enum PacketError {
    ReadError,
}
impl From<PacketError> for String {
    fn from(error: PacketError) -> String {
        match error {
            PacketError::ReadError => "Failed to read packet.".to_string(),
        }
    }
}

pub struct Packet {
    buffer: Vec<u8>,
    readable_buffer: Option<Vec<u8>>,
    read_pos: usize,
}

impl Packet {
    /// Creates an empty packet without an ID.
    pub fn new() -> Self {
        Packet {
            buffer: Vec::new(),
            readable_buffer: None,
            read_pos: 0,
        }
    }

    /// Creates an empty packet with an ID. Used for sending data.
    pub fn new_with_id(id: i32) -> Self {
        let mut packet = Packet::new();
        packet.write_int(id);
        packet
    }

    /// Creates a packet and sets data to prepare it for reading. Used for receiving data.
    pub fn new_with_data(data: Vec<u8>) -> Self {
        let mut packet = Packet::new();
        packet.set_bytes(data);
        packet
    }

    // region: Packet Functions
    /// Sets the packet's content and prepares it to be read.
    pub fn set_bytes(&mut self, data: Vec<u8>) {
        self.buffer.clear();
        self.read_pos = 0;
        self.write_bytes(data);
        self.readable_buffer = Some(self.buffer.clone());
    }

    /// Insert length of the packet's content at the start of the buffer.
    pub fn write_length(&mut self) {
        let length = self.buffer.len() as u32;
        let length_bytes = length.to_be_bytes();
        self.buffer.splice(0..0, length_bytes.iter().cloned());
    }

    /// Inserts an integer at the start of the buffer.
    pub fn insert_int(&mut self, data: i32) {
        let data_bytes = data.to_be_bytes();
        self.buffer.splice(0..0, data_bytes.iter().cloned());
    }

    /// Returns the length of unread data in the packet.
    pub fn unread_length(&self) -> usize {
        self.buffer.len() - self.read_pos
    }

    /// Resets the packet. Defaults to true, reset the whole packet. False resets the last read int.
    pub fn reset(&mut self, full_reset: bool) {
        if full_reset {
            self.buffer.clear();
            self.readable_buffer = None;
            self.read_pos = 0;
        } else {
            self.read_pos = self.read_pos.saturating_sub(4);
        }
    }
    // endregion

    // region: Write Functions
    pub fn write_byte(&mut self, data: u8) {
        self.buffer.push(data)
    }

    pub fn write_bytes(&mut self, data: Vec<u8>) {
        self.buffer.extend(data);
    }

    pub fn write_short(&mut self, data: i16) {
        self.buffer.extend(&data.to_be_bytes());
    }

    pub fn write_ushort(&mut self, data: u16) {
        self.buffer.extend(&data.to_be_bytes());
    }

    pub fn write_int(&mut self, data: i32) {
        self.buffer.extend(&data.to_be_bytes());
    }

    pub fn write_uint(&mut self, data: u32) {
        self.buffer.extend(&data.to_be_bytes());
    }

    pub fn write_long(&mut self, data: i64) {
        self.buffer.extend(&data.to_be_bytes());
    }

    pub fn write_float(&mut self, data: f32) {
        self.buffer.extend(&data.to_be_bytes());
    }

    pub fn write_double(&mut self, data: f64) {
        self.buffer.extend(&data.to_be_bytes());
    }

    pub fn write_bool(&mut self, data: bool) {
        self.buffer.push(data as u8);
    }

    pub fn write_string(&mut self, data: String) {
        self.write_int(data.len() as i32);
        self.write_bytes(data.into_bytes());
    }
    // endregion

    // region: Read Functions
    /// Reads a byte from the packet.
    ///
    /// * `move_read_pos` - If the buffer's read position should be incremented. Defaults to true.
    ///
    /// Returns the byte that was read.
    pub fn read_byte(&mut self, move_read_pos: Option<bool>) -> Result<u8, PacketError> {
        // Check if there are bytes to read.
        if self.buffer.len() < self.read_pos + 1 {
            return Err(PacketError::ReadError);
        }

        let data = self.buffer[self.read_pos]; // Get the byte at the current read_pos.

        if move_read_pos.unwrap_or(true) {
            self.read_pos += 1;
        }

        Ok(data)
    }

    /// Reads a range of bytes from the packet.
    /// 
    /// * `length` - The length of the array to read.
    /// * `move_read_pos` - If the buffer's read position should be incremented. Defaults to true.
    /// 
    /// Returns the rarnge of bytes that were read.
    pub fn read_bytes(&mut self, length: usize, move_read_pos: Option<bool>) -> Result<Vec<u8>, PacketError> {
        // Check if there are bytes to read.
        if self.buffer.len() < self.read_pos + length {
            return Err(PacketError::ReadError);
        }

        let data = self.buffer[self.read_pos..self.read_pos + length].to_vec();

        if move_read_pos.unwrap_or(true) {
            self.read_pos += length;
        }

        Ok(data)
    }

    /// Reads a short from the packet.
    ///
    /// * `move_read_pos` - If the buffer's read position should be incremented. Defaults to true.
    /// Returns the short that was read.
    pub fn read_short(&mut self, move_read_pos: Option<bool>) -> Result<i16, PacketError> {
        let data = self.read_bytes(2, move_read_pos)?;
        Ok(i16::from_be_bytes(data.try_into().unwrap()))
    }

    /// Reads an unsigned short from the packet.
    /// 
    /// * `move_read_pos` - If the buffer's read position should be incremented. Defaults to true.
    /// Returns the unsigned short that was read.
    pub fn read_ushort(&mut self, move_read_pos: Option<bool>) -> Result<u16, PacketError> {
        let data = self.read_bytes(2, move_read_pos)?;
        Ok(u16::from_be_bytes(data.try_into().unwrap()))
    }

    /// Reads an integer from the packet.
    /// 
    /// * `move_read_pos` - If the buffer's read position should be incremented. Defaults to true.
    /// Returns the integer that was read.
    pub fn read_int(&mut self, move_read_pos: Option<bool>) -> Result<i32, PacketError> {
        let data = self.read_bytes(4, move_read_pos)?;
        Ok(i32::from_be_bytes(data.try_into().unwrap()))
    }

    /// Reads a unsigned integer from the packet.
    /// 
    /// * `move_read_pos` - If the buffer's read position should be incremented. Defaults to true.
    /// Returns the unsigned integer that was read.
    pub fn read_uint(&mut self, move_read_pos: Option<bool>) -> Result<u32, PacketError> {
        let data = self.read_bytes(4, move_read_pos)?;
        Ok(u32::from_be_bytes(data.try_into().unwrap()))
    }

    /// Reads a long from the packet.
    /// 
    /// * `move_read_pos` - If the buffer's read position should be incremented. Defaults to true.
    /// Returns the long that was read.
    pub fn read_long(&mut self, move_read_pos: Option<bool>) -> Result<i64, PacketError> {
        let data = self.read_bytes(8, move_read_pos)?;
        Ok(i64::from_be_bytes(data.try_into().unwrap()))
    }


    /// Reads a float from the packet.
    /// 
    /// * `move_read_pos` - If the buffer's read position should be incremented. Defaults to true.
    /// Returns the float that was read.
    pub fn read_float(&mut self, move_read_pos: Option<bool>) -> Result<f32, PacketError> {
        let data = self.read_bytes(4, move_read_pos)?;
        Ok(f32::from_be_bytes(data.try_into().unwrap()))
    }

    /// Reads a double from the packet.
    /// 
    /// * `move_read_pos` - If the buffer's read position should be incremented. Defaults to true.
    /// Returns the double that was read.
    pub fn read_double(&mut self, move_read_pos: Option<bool>) -> Result<f64, PacketError> {
        let data = self.read_bytes(8, move_read_pos)?;
        Ok(f64::from_be_bytes(data.try_into().unwrap()))
    }

    /// Reads a boolean from the packet.
    /// 
    /// * `move_read_pos` - If the buffer's read position should be incremented. Defaults to true.
    /// Returns the boolean that was read.
    pub fn read_bool(&mut self, move_read_pos: Option<bool>) -> Result<bool, PacketError> {
        let data = self.read_byte(move_read_pos)?;
        Ok(data != 0)
    }

    /// Reads a string from the packet.
    /// 
    /// * `move_read_pos` - If the buffer's read position should be incremented. Defaults to true.
    /// Returns the string that was read.
    pub fn read_string(&mut self, move_read_pos: Option<bool>) -> Result<String, PacketError> {
        let length = self.read_int(move_read_pos)? as usize;
        let data = self.read_bytes(length, move_read_pos)?;
        Ok(String::from_utf8(data).unwrap())
    }
    // endregion

    // region: Memory Functions
    /// Deinitializes the packet.
    pub fn deinit(&mut self) {
        self.buffer.clear();
        self.readable_buffer = None;
        self.read_pos = 0;
    }
    // endregion
}
