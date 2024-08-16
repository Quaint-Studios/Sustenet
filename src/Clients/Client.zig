//! A standard client that connects to a server.

const std = @import("std");

const BaseClient = @import("root").sustenet.transport.BaseClient;

super: BaseClient,

const Client = @This();

pub fn new(port: u16) Client {
    const client = Client{
        .super = BaseClient.new(port),
    };
    return client;
}

pub fn connect(self: *Client) !void {
    try self.super.connect();
}
