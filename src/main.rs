use capitan_lib::services::prelude::*;
use capitan_lib::services::{IsolatedReactor, SharedReactor};
use structs::MasterService;

mod structs;

use anyhow::Result as Res;

#[tokio::main]
async fn main() -> Res<()> {
    let mut reactor = IsolatedReactor::new();
    
    for i in 0..100usize {
        let test_service = MasterService(SharedReactor::new(), i);
        reactor.spawn_service(test_service, i).await?;
    }
    reactor.wait_all().await;

    println!("Hello, world!");
    Ok(())
}
