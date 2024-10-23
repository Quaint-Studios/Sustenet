use crate::utils::constants;

struct Option<'a> {
    name: &'a str,
    description: &'a str,
}

const ALL_OPTIONS: [Option; 5] = [
    Option {
        name: "help",
        description: "This is the help you've been asking for.",
    },
    Option {
        name: "v|version",
        description: "Prints the version of the program",
    },
    Option {
        name: "c|client",
        description: "starts a client and waits for connect() to be triggered.",
    },
    Option {
        name: "cs|cluster",
        description:
            "starts a cluster server and uses the config file to connect to a master server.",
    },
    Option {
        name: "ms|master",
        description: "Runs the program in cluster mode",
    },
];

pub fn show_help() {
    for option in ALL_OPTIONS {
        println!(
            "\t{}- {}:{} {}",
            constants::TERMINAL_GREEN,
            option.name,
            constants::TERMINAL_DEFAULT,
            option.description
        );
    }
}
