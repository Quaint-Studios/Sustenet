const std = @import("std");
const network = @import("network");
const sustenet = @import("root").sustenet;

const BaseServer = sustenet.transport.BaseServer;
const Constants = sustenet.utils.Constants;

const TcpListener = @This();

thread: ?std.Thread = null,
socket: network.Socket,

const TcpListenerError = error{
    ThreadIsNull,
};

/// Creates a new TCP listener and binds it.
pub fn new(address: network.Address, port: u16) !TcpListener {
    var listener = TcpListener{
        .socket = try network.Socket.create(.ipv4, .tcp),
    };

    try listener.socket.bind(.{ .address = address, .port = port });

    return listener;
}

/// Deinitializes the TCP listener.
pub fn deinit(self: *TcpListener) void {
    errdefer self.socket.close(); // Just in case the thread yield fails.

    self.thread = null;
    self.socket.close();
}
