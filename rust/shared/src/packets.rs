pub mod master {
    #[repr(u8)]
    pub enum FromUnknown {
        /// Sends a list of names and IPs to whoever requested it.
        RequestClusters,
        /// Just a way to gracefully disconnect the client.
        JoinCluster,

        /// They send the name of the cluster's key to the Master Server.
        /// If the key doesn't exist, the server will do nothing but
        /// stay silent. If it does exist, it will send a generated
        /// passphrase that's encrypted with AES.
        BecomeCluster,
        /// When they send the decrypted key back to the Master Server.
        AnswerCluster,
    }
    #[repr(u8)]
    pub enum ToUnknown {
        /// Sends a list of cluster servers containing their name, ip, and port.
        SendClusters,

        /// Generates a passphrase that's encrypted with AES and sends
        /// it waiting for it to be sent back. It's stored in their name.
        VerifyCluster,
        /// Once validated, the cluster is moved to the cluster list and
        /// notifies them that they're now a cluster.
        CreateCluster,
        
        // Cluster things go here.
    }
}

pub mod cluster {
    pub enum FromClient {
        /// Sends a list of names and IPs to whoever requested it.
        RequestClusters,
        /// Gracefully disconnect the client and connects to the new cluster.
        JoinCluster,
        /// Gracefully disconnect the client.
        LeaveCluster,

        /// Only works if the server doesn't have a domain config. Sends the pub key.
        /// Ran if the cache key doesn't match.
        RequestKey,
        /// Encrypts the password and sends it.
        SendPassword,

        /// Moves the player's position.
        Move
    }

    pub enum ToClient {
        /// Sends a list of cluster servers containing their name, ip, and port.
        SendClusters,
        /// Disconnects the client from the cluster.
        DisconnectCluster,
        /// Disconnects the client from the cluster.
        LeaveCluster,

        // Sends the cached version of the key.
        VersionOfKey,
        /// Sends the public key to the client. This only works if "domain_pub_key"
        /// is not set in the Config.
        SendPubKey,
        /// This sends back the status to the user. It'll have a status code.
        /// 20 = 200, 44 = 404, 40 = 400, 50 = 500.
        /// If 20, it will send the user their ID (this was assigned on initial connection).
        Authenticate,

        /// Sends the player's new position.
        Move
    }
}
