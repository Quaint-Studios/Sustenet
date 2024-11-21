use sustenet::events::{Event, MasterServerPackets};

#[tokio::main]
async fn main() {
    println!(
        "{} {} {}",
        MasterServerPackets::PassphraseRequest as u16,
        MasterServerPackets::InitializeCluster as u16,
        MasterServerPackets::ClusterServerList as u16
    );
}
