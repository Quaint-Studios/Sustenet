//! Handles reading and writing data of varying types.

const std = @import("std");
const ArrayList = std.ArrayList;
const Packet = @This();

buffer: ArrayList(u8),
readable_buffer: []u8,
read_pos: i32 = 0,

/// Creates an empty packet without an ID.
pub fn new(allocator: std.mem.Allocator) Packet {
    const packet = Packet{};
    packet.init(allocator);
    return Packet{};
}

/// Creates an empty packet with an ID. Used for sending data.
pub fn newWithId(allocator: std.mem.Allocator, _id: i32) Packet {
    const packet = new(allocator);

    packet.writeInt(_id);
    return packet;
}

/// Creates a packet and sets data to prepare it for reading. Used for receiving data.
pub fn newWithData(allocator: std.mem.Allocator, data: []u8) Packet {
    const packet = new(allocator);

    packet.setBytes(data);
    return packet;
}

//#region Packet Functions
/// Sets the packet's content and prepares it to be read.
pub fn setBytes(self: *Packet, allocator: std.mem.Allocator, data: []u8) void {
    self.buffer = ArrayList(u8).init(allocator);
    self.read_pos = 0;
    self.writeBytes(data);
    self.readable_buffer = self.buffer.toOwnedSlice();
}

//#endregion

//#region Write Functions
pub fn writeByte(self: *Packet, data: u8) void {
    self.buffer.append(data);
}

pub fn writeBytes(self: *Packet, data: []u8) void {
    // for (data) |byte| {
    //     self.buffer.append(byte);
    // }

    self.buffer.appendSlice(data);
}

pub fn writeShort(self: *Packet, data: i16) void {
    const bytes: [2]u8 = @intCast(data);
    self.buffer.appendSlice(&bytes);
}

pub fn writeUshort(self: *Packet, data: u16) void {
    const bytes: [2]u8 = @intCast(data);
    self.buffer.appendSlice(&bytes);
}

pub fn writeInt(self: *Packet, data: i32) void {
    const bytes: [4]u8 = @intCast(data);
    self.buffer.appendSlice(&bytes);
}
//#endregion

//#region Memory Functions
/// Initializes the packet.
pub fn init(allocator: std.mem.Allocator) Packet {
    return Packet{
        .buffer = ArrayList(u8).init(allocator),
    };
}

/// Deinitializes the packet.
pub fn deinit(self: *Packet) void {
    self.buffer.deinit();
}
//#endregion
