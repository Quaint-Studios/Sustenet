//! Handles reading and writing data of varying types.

const std = @import("std");
const ArrayList = std.ArrayList;
const Packet = @This();

buffer: ArrayList(u8),
/// Private
readable_buffer: ?[]u8,
/// Private
read_pos: u32 = 0,

const Errors = error{
    ReadError,
};

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
    var packet = new(allocator);

    try packet.setBytes(allocator, data);
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

// /// The length of the packet's content.
// pub fn length(self: *Packet) usize {
//     return self.buffer.items.len;
// }

/// Returns the length of unread data in the packet.
pub fn unreadLength(self: *Packet) usize {
    return self.buffer.items.len - self.read_pos;
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
// NOTE: This could all be simplified with a single comptime function
// that takes a type, length, and move_read_pos.
//
// i32 is literally just 32/8 = a 4 byte shift.

/// Reads a byte from the packet.
///
/// Returns the byte that was read.
pub fn readByte(
    self: *Packet,
    /// If the buffer's read position should be incremented. Defaults to true.
    move_read_pos: ?bool,
) !u8 {
    // If there are still bytes left unread.
    if (self.buffer.items.len > self.read_pos) {
        const data = self.readable_buffer.?[self.read_pos]; // Get the byte at the current readPos.

        if (move_read_pos orelse true) self.read_pos += 1;

        return data;
    }

    return error.ReadError;
}

/// Reads a range of bytes from the packet.
///
/// Returns the range of bytes that were read.
pub fn readBytes(
    self: *Packet,
    /// The length of the array to read.
    length: u32,
    /// If the buffer's read position should be incremented by the length. Defaults to true.
    move_read_pos: ?bool,
) ![]u8 {
    // If there are still bytes left unread.
    if (self.buffer.items.len > self.read_pos) {
        const data = self.readable_buffer.?[self.read_pos .. self.read_pos + length];

        if (move_read_pos orelse true) self.read_pos += length;

        return data;
    }

    return error.ReadError;
}

/// Reads a short from the packet.
///
/// Returns the short that was read.
pub fn readShort(
    self: *Packet,
    /// If the buffer's read position should be incremented by 2. Defaults to true.
    move_read_pos: ?bool,
) !i16 {
    // If there are still bytes left unread.
    if (self.buffer.items.len > self.read_pos) {
        const data = self.readable_buffer.?[self.read_pos .. self.read_pos + 2];
        const short: i16 = @intCast(data);

        if (move_read_pos orelse true) self.read_pos += 2;

        return short;
    }

    return error.ReadError;
}

/// Reads an ushort from the packet.
///
/// Returns the ushort that was read.
pub fn readUShort(
    self: *Packet,
    /// If the buffer's read position should be incremented by 2. Defaults to true.
    move_read_pos: ?bool,
) !u16 {
    // If there are still bytes left unread.
    if (self.buffer.items.len > self.read_pos) {
        const data = self.readable_buffer.?[self.read_pos .. self.read_pos + 2];
        const ushort: u16 = @as(u16, data);

        if (move_read_pos orelse true) self.read_pos += 2;

        return ushort;
    }

    return error.ReadError;
}

/// Reads an int from the packet.
///
/// Returns the int that was read.
pub fn readInt(
    self: *Packet,
    /// If the buffer's read position should be incremented by 4. Defaults to true.
    move_read_pos: ?bool,
) !i32 {
    // If there are still bytes left unread.
    if (self.buffer.items.len > self.read_pos) {
        const data = self.readable_buffer.?[self.read_pos .. self.read_pos + 4];

        var fbs = std.io.fixedBufferStream(data);
        var reader = fbs.reader();
        const int = try reader.readInt(i32, .big);

        if (move_read_pos orelse true) self.read_pos += 4;

        return int;
    }

    return error.ReadError;
}

/// Reads a uint from the packet.
///
/// Returns the uint that was read.
pub fn readUInt(
    self: *Packet,
    /// If the buffer's read position should be incremented by 4. Defaults to true.
    move_read_pos: ?bool,
) !u32 {
    // If there are still bytes left unread.
    if (self.buffer.items.len > self.read_pos) {
        const data = self.readable_buffer.?[self.read_pos .. self.read_pos + 4];

        var fbs = std.io.fixedBufferStream(data);
        var reader = fbs.reader();
        const uint = try reader.readInt(u32, .big);

        if (move_read_pos orelse true) self.read_pos += 4;

        return uint;
    }

    return error.ReadError;
}

/// Reads a long from the packet.
///
/// Returns the long that was read.
pub fn readLong(
    self: *Packet,
    /// If the buffer's read position should be incremented by 8. Defaults to true.
    move_read_pos: ?bool,
) !i64 {
    // If there are still bytes left unread.
    if (self.buffer.items.len > self.read_pos) {
        const data = self.readable_buffer.?[self.read_pos .. self.read_pos + 8];

        var fbs = std.io.fixedBufferStream(data);
        var reader = fbs.reader();
        const long: i64 = try reader.readInt(i64, .big);

        if (move_read_pos orelse true) self.read_pos += 8;

        return long;
    }

    return error.ReadError;
}

/// Reads a ulong from the packet.
///
/// Returns the ulong that was read.
pub fn readULong(
    self: *Packet,
    /// If the buffer's read position should be incremented by 8. Defaults to true.
    move_read_pos: ?bool,
) !u64 {
    // If there are still bytes left unread.
    if (self.buffer.items.len > self.read_pos) {
        const data = self.readable_buffer.?[self.read_pos .. self.read_pos + 8];
        const ulong: u64 = @intCast(data);

        if (move_read_pos orelse true) self.read_pos += 8;

        return ulong;
    }

    return error.ReadError;
}

/// Reads a float from the packet.
///
/// Returns the float that was read.
pub fn readFloat(
    self: *Packet,
    /// If the buffer's read position should be incremented by 4. Defaults to true.
    move_read_pos: ?bool,
) !f32 {
    // If there are still bytes left unread.
    if (self.buffer.items.len > self.read_pos) {
        const data = self.readable_buffer.?[self.read_pos .. self.read_pos + 4];
        const float: f32 = @intCast(data);

        if (move_read_pos orelse true) self.read_pos += 4;

        return float;
    }

    return error.ReadError;
}

/// Reads a double from the packet.
///
/// Returns the double that was read.
pub fn readDouble(
    self: *Packet,
    /// If the buffer's read position should be incremented by 8. Defaults to true.
    move_read_pos: ?bool,
) !f64 {
    // If there are still bytes left unread.
    if (self.buffer.items.len > self.read_pos) {
        const data = self.readable_buffer.?[self.read_pos .. self.read_pos + 8];
        const double: f64 = @intCast(data);

        if (move_read_pos orelse true) self.read_pos += 8;

        return double;
    }

    return error.ReadError;
}

/// Reads a string from the packet.
///
/// Returns the string that was read.
pub fn readString(
    self: *Packet,
    /// If the buffer's read position should be incremented by the string's length. Defaults to true.
    move_read_pos: ?bool,
) ![]const u8 {
    // If there are still bytes left unread.
    if (self.buffer.items.len > self.read_pos) {
        const length = self.readInt();
        const data = self.readable_buffer.?[self.read_pos .. self.read_pos + length];

        if (move_read_pos orelse true) self.read_pos += length;

        return data;
    }

    return error.ReadError;
}
//#endregion

//#region Memory Functions
/// Deinitializes the packet.
pub fn deinit(self: *Packet) void {
    self.buffer.clearAndFree();
    self.buffer.deinit();
    self.readable_buffer = null;
    self.read_pos = 0;
}
//#endregion
