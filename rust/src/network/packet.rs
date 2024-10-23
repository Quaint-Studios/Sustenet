// TODO Implement

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

    //#region Packet Functions
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
    //#endregion

    //#region Write Functions
    pub fn write_byte(&mut self, data: u8) {
        self.buffer.push(data)
    }

    pub fn write_bytes(&mut self, data: Vec<u8>) {
        self.buffer.extend(data);
    }

    pub fn write_short(&mut self, data: i16) {
        let bytes = data.to_be_bytes();
        self.buffer.extend(&bytes);
    }

    pub fn write_ushort(&mut self, data: u16) {
        let bytes = data.to_be_bytes();
        self.buffer.extend(&bytes);
    }

    pub fn write_int(&mut self, data: i32) {
        let bytes = data.to_be_bytes();
        self.buffer.extend(&bytes);
    }
    //#endregion

    //#region Read Functions
    // TODO Implement
    //endregion

    //#region Memory Functions
    /// Deinitializes the packet.
    pub fn deinit(&mut self) {
        self.buffer.clear();
        self.readable_buffer = None;
        self.read_pos = 0;
    }
    //#endregion
}
