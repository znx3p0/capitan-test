use std::sync::atomic::AtomicU32;

use anyhow::Result as Res;
use async_trait::async_trait;
use capitan_lib::services::prelude::*;
use capitan_lib::services::{IsolatedService, SharedReactor, SharedService};

pub struct MasterService(pub SharedReactor<ChildService>, pub usize);
#[async_trait]
impl IsolatedService for MasterService {
    async fn init(&mut self) -> Res<()> {
        Ok(())
    }

    async fn main(&mut self) -> Res<()> {
        println!("{}", self.1);
        tokio::time::sleep(std::time::Duration::from_millis(self.1 as u64 * 10)).await;
        Err(anyhow::anyhow!("some error"))
    }

    async fn repeat(&mut self) -> Res<()> {
        println!("repeating");
        Ok(())
    }

    async fn catch(&mut self, error: anyhow::Error) -> Res<()> {
        println!("catching {:?}", error);
        Err(anyhow::anyhow!("couldn't catch"))
    }

    async fn abort(&mut self) -> Res<()> {
        println!("aborting");
        Ok(())
    }
}

pub struct ChildService(AtomicU32);
#[async_trait]
impl SharedService for ChildService {
    async fn init(&self) -> Res<()> {
        println!("starting child service");
        Ok(())
    }

    async fn main(&self) -> Res<()> {
        self.0.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        println!("{:?}", self.0);
        Ok(())
    }

    async fn repeat(&self) -> Res<()> {
        Ok(())
    }

    async fn catch(&self, _: anyhow::Error) -> Res<()> {
        Ok(())
    }

    async fn abort(&self) -> Res<()> {
        Ok(())
    }
}
