pub enum Packets {
    /// Handles packets related to messaging and chat.
    Messaging(Messaging),
    /// Handles packets related to movement and player actions.
    Player(Player),
    /// Handles packets related to connection management.
    Connection(Connection),
    /// Cluster setup packets
    ClusterSetup(ClusterSetup),
    /// Diagnostics packets
    Diagnostics(Diagnostics),
}

#[repr(u8)]
/// Handles packets related to messaging and chat.
pub enum Messaging {
    /// Send a message to the server.
    SendGlobalMessage = 200,
    /// Send a message to a specific player.
    SendPrivateMessage,
    /// Send a message to the party.
    SendPartyMessage,
    /// Send a local message.
    SendLocalMessage,
}

#[repr(u8)]
/// Handles packets related to movement and player actions.
/// Should eventually be migrated to an external crate.
pub enum Player {
    /// Move the player to a new position.
    Move = 210,
    /// Teleport the player to a new position.
    Teleport,
    /// Change the player's name.
    ChangeName,
    /// Update the player's health.
    UpdateHealth,
    /// Update the player's inventory.
    UpdateInventory,
    /// Sends the entire inventory to the client.
    RequestInventory,
}

#[repr(u8)]
pub enum Connection {
    /// The client is requesting to connect to the server.
    Connect = 240,
    /// The client is disconnecting from the server.
    Disconnect,
    /// Authenticate the client with the server.
    Authenticate,
}

#[repr(u8)]
/// Used to set up the cluster server.
pub enum ClusterSetup {
    /// They send the name of the cluster's key to the Master Server.
    /// If the key doesn't exist, the server will do nothing but
    /// stay silent. If it does exist, it will send a generated
    /// passphrase that's encrypted with AES.
    Init = 245,
    /// When they send the decrypted key back to the Master Server.
    AnswerSecret,
}

#[repr(u8)]
/// Used to request information about the server.
pub enum Diagnostics {
    /// Requests information about a server's type.
    CheckServerType = 250,
    /// Requests information about a server's version.
    CheckServerVersion,
    /// Requests information about a server's uptime.
    CheckServerUptime,
    /// Requests information about how many players are connected to a server.
    CheckServerPlayerCount,
}

#[cfg(test)]
pub mod tests {
    use super::Packets;

    #[test]
    fn test_enum_size() {
        assert_eq!(std::mem::size_of::<Packets>(), 1);
    }
}

// pub mod master {
//     #[repr(u8)]
//     pub enum FromUnknown {
//         /// Sends a list of names and IPs to whoever requested it.
//         RequestClusters,
//         /// Just a way to gracefully disconnect the client.
//         JoinCluster,

//         /// They send the name of the cluster's key to the Master Server.
//         /// If the key doesn't exist, the server will do nothing but
//         /// stay silent. If it does exist, it will send a generated
//         /// passphrase that's encrypted with AES.
//         BecomeCluster,
//         /// When they send the decrypted key back to the Master Server.
//         AnswerCluster,
//     }
//     #[repr(u8)]
//     pub enum ToUnknown {
//         /// Sends a list of cluster servers containing their name, ip, and port.
//         SendClusters,

//         /// Generates a passphrase that's encrypted with AES and sends
//         /// it waiting for it to be sent back. It's stored in their name.
//         VerifyCluster,
//         /// Once validated, the cluster is moved to the cluster list and
//         /// notifies them that they're now a cluster.
//         CreateCluster,
//         // Cluster things go here.
//     }
// }

// pub mod cluster {
//     pub enum FromClient {
//         /// Sends a list of names and IPs to whoever requested it.
//         RequestClusters,
//         /// Gracefully disconnect the client and connects to the new cluster.
//         JoinCluster,
//         /// Gracefully disconnect the client.
//         LeaveCluster,

//         /// Only works if the server doesn't have a domain config. Sends the pub key.
//         /// Ran if the cache key doesn't match.
//         RequestKey,
//         /// Encrypts the password and sends it.
//         SendPassword,

//         /// Moves the player's position.
//         Move,
//     }

//     pub enum ToClient {
//         /// Sends a list of cluster servers containing their name, ip, and port.
//         SendClusters,
//         /// Disconnects the client from the cluster.
//         DisconnectCluster,
//         /// Disconnects the client from the cluster.
//         LeaveCluster,

//         // Sends the cached version of the key.
//         VersionOfKey,
//         /// Sends the public key to the client. This only works if "domain_pub_key"
//         /// is not set in the Config.
//         SendPubKey,
//         /// This sends back the status to the user. It'll have a status code.
//         /// 20 = 200, 44 = 404, 40 = 400, 50 = 500.
//         /// If 20, it will send the user their ID (this was assigned on initial connection).
//         Authenticate,

//         /// Sends the player's new position.
//         Move,
//     }
// }
