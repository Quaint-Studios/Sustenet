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
    pub enum ToUnknown {
        /// Sends a list of cluster servers containing their name, ip, and port.
        SendClusters,

        /// Generates a passphrase that's encrypted with AES and sends
        /// it waiting for it to be sent back. It's stored in their name.
        VerifyCluster,
        /// Once validated, the cluster is moved to the cluster list and
        /// notifies them that they're now a cluster.
        CreateCluster,
    }

    pub enum FromCluster {}
    pub enum ToCluster {}
}
