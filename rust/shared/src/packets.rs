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
pub enum Connection {
    /// The client is requesting to connect to the server.
    /// 
    /// 1. From Client to Server: CMD + Len&VersionNumber
    /// 
    /// TODO: Run the check version function.
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
    /// 
    /// 1. From Cluster to Master: CMD + Len&VersionNumber + Len&Key Name
    /// 2. From Master to Cluster: CMD + Encrypted Passphrase
    /// TODO: Then you need to temporarity store them in a DashMap outside of clusters.
    /// 
    /// If the key doesn't exist, do nothing.
    Init = 245,
    /// When they send the decrypted key back to the Master Server.
    /// 
    /// 1. From Cluster to Master: CMD + Decrypted Passphrase + IP + Port + Max connections + Len&Name
    /// 2. From Master to Cluster: CMD
    /// 
    /// If it fails, say nothing.
    AnswerSecret,
}

#[repr(u8)]
#[derive(Debug, Clone)]
/// Used to request information about the server.
pub enum Diagnostics {
    /// Requests information about a server's type.
    CheckServerType = 250,
    /// Requests information about a server's uptime.
    CheckServerUptime,
    /// Requests information about how many players are connected to a server.
    CheckServerPlayerCount,
}

#[cfg(test)]
pub mod tests {
    use crate::packets::{ ClusterSetup, Connection, Diagnostics, Messaging };

    #[test]
    fn test_enum_size() {
        assert_eq!(std::mem::size_of::<Messaging>(), 1);
        assert_eq!(std::mem::size_of::<Connection>(), 1);
        assert_eq!(std::mem::size_of::<ClusterSetup>(), 1);
        assert_eq!(std::mem::size_of::<Diagnostics>(), 1);
    }

    #[test]
    /// This test checks that all enum values are unique.
    /// The values should range between 200 and 255.
    /// That's the range for reserved commands for Sustenet.
    fn test_enum_unique_values() {
        use std::collections::HashSet;

        macro_rules! enum_values {
            ($enum:ty, [$( $variant:path ),* $(,)?]) => {
                vec![$($variant as u8),*]
            };
        }

        let all_enums = [
            enum_values!(Messaging, [
                Messaging::SendGlobalMessage,
                Messaging::SendPrivateMessage,
                Messaging::SendPartyMessage,
                Messaging::SendLocalMessage,
            ]),
            enum_values!(Connection, [
                Connection::Connect,
                Connection::Disconnect,
                Connection::Authenticate,
            ]),
            enum_values!(ClusterSetup, [
                ClusterSetup::Init,
                ClusterSetup::AnswerSecret,
            ]),
            enum_values!(Diagnostics, [
                Diagnostics::CheckServerType,
                Diagnostics::CheckServerUptime,
                Diagnostics::CheckServerPlayerCount,
            ]),
        ].concat();

        let mut set = HashSet::new();
        for val in all_enums {
            assert!(set.insert(val), "Duplicate value found: {val}");
        }

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
