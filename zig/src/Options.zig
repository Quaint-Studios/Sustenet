const std = @import("std");
const sustenet = @import("root").sustenet;
const Constants = sustenet.utils.Constants;

const Options = @This();

const Option = struct {
    name: []const u8,
    description: []const u8,
};

const AllOptions: [5]Option = [5]Option{
    Option{ .name = "help", .description = "This is the help you've been asking for." },
    Option{ .name = "v|version", .description = "Prints the version of the program" },
    Option{ .name = "c|client", .description = "starts a client and waits for connect() to be triggered." },
    Option{ .name = "cs|cluster", .description = "starts a cluster server and uses the config file to connect to a master server." },
    Option{ .name = "ms|master", .description = "Runs the program in cluster mode" },
};

pub fn showHelp() void {
    for (AllOptions) |option| {
        std.debug.print("\t{s}- {s}:{s} {s}\n", .{ Constants.TERMINAL_GREEN, option.name, Constants.TERMINAL_DEFAULT, option.description });
    }
}
