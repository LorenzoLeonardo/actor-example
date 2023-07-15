pub mod sedan;
pub mod suv;

use async_trait::async_trait;

pub enum CarRequest {
    Park,
    Drive,
    Brake,
    Reverse,
}

pub type CarResponse = String;

#[async_trait]
pub trait Car {
    async fn park(&self) -> CarResponse;
    async fn drive(&self) -> CarResponse;
    async fn brake(&self) -> CarResponse;
    async fn reverse(&self) -> CarResponse;
}
