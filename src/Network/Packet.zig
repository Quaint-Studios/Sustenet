//! Handles reading and writing data of varying types.

const std = @import("std");
const ArrayList = std.ArrayList;
const Packet = @This();

buffer: ArrayList(u8),
/// Private
readable_buffer: ?[]u8,
/// Private
read_pos: u32 = 0,

/// Creates an empty packet without an ID.
pub fn new(allocator: std.mem.Allocator) Packet {
    return Packet{
        .buffer = ArrayList(u8).init(allocator),
        .readable_buffer = null,
    };
}

/// Creates an empty packet with an ID. Used for sending data.
pub fn newWithId(allocator: std.mem.Allocator, _id: i32) !Packet {
    const packet = new(allocator);

    try packet.writeInt(_id);
    return packet;
}

/// Creates a packet and sets data to prepare it for reading. Used for receiving data.
pub fn newWithData(allocator: std.mem.Allocator, data: []u8) !Packet {
    const packet = new(allocator);

    try packet.setBytes(data);
    return packet;
}

//#region Packet Functions
/// Sets the packet's content and prepares it to be read.
pub fn setBytes(self: *Packet, allocator: std.mem.Allocator, data: []u8) !void {
    self.buffer = ArrayList(u8).init(allocator);
    self.read_pos = 0;
    try self.writeBytes(data);
    self.readable_buffer = try self.buffer.toOwnedSlice();
}

/// Insert length of the packet's content at the start of the buffer.
pub fn writeLength(self: *Packet) void {
    self.buffer.insertSlice(0, std.mem.toBytes(self.buffer.items.len));
}

/// Inserts an integer at the start of the buffer.
pub fn insertInt(self: *Packet, data: i32) void {
    self.buffer.insertSlice(0, std.mem.toBytes(data));
}

/// The length of the packet's content.
pub fn length(self: *Packet) usize {
    return self.buffer.items.len;
}

/// Returns the length of unread data in the packet.
pub fn unreadLength(self: *Packet) usize {
    return self.length() - self.read_pos;
}

/// Resets the packet. Defaults to true, reset the whole packet. False resets the last read int.
pub fn reset(
    self: *Packet,
    /// Determines if the whole packet should be reset.
    fullReset: ?bool,
) void {
    if (fullReset orelse true) {
        self.buffer.clearAndFree();
        self.readable_buffer = null;
        self.read_pos = 0;
    } else {
        // Ensure read_pos doesn't go below zero.
        if (self.read_pos >= 4) {
            self.read_pos -= 4; // "Unread" the last read int.
        } else {
            self.read_pos = 0;
        }
    }
}
//#endregion

//#region Write Functions
pub fn writeByte(self: *Packet, data: u8) !void {
    try self.buffer.append(data);
}

pub fn writeBytes(self: *Packet, data: []u8) !void {
    try self.buffer.appendSlice(data);
}

pub fn writeShort(self: *Packet, data: i16) !void {
    const bytes: [2]u8 = @intCast(data);
    try self.buffer.appendSlice(&bytes);
}

pub fn writeUshort(self: *Packet, data: u16) !void {
    const bytes: [2]u8 = @intCast(data);
    try self.buffer.appendSlice(&bytes);
}

pub fn writeInt(self: *Packet, data: i32) !void {
    const bytes: [4]u8 = @intCast(data);
    self.buffer.appendSlice(&bytes);
}
//#endregion

//#region Read Functions
// TODO: Implement read functions.
pub fn readInt(_: *Packet) i32 {
    return 0;
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
    self.buffer.clearAndFree();
    self.buffer.deinit();
    self.readable_buffer = null;
    self.read_pos = 0;
}
//#endregion
