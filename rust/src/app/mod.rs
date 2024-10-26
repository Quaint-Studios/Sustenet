pub use self::app::App;

mod app;
pub mod client;
pub mod cluster;
pub mod master;

pub(super) mod tests {
    #[test]
    fn test_client() {
        println!("Testing client module");
    }

    #[test]
    fn test_cluster() {
        println!("Testing cluster module");
    }

    #[test]
    fn test_master() {
        println!("Testing master module");
    }

}