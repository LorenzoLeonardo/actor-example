use async_trait::async_trait;
use atticus::Actor;

use super::{Car, CarRequest, CarResponse};

pub struct Sedan;

#[async_trait]
impl Car for Sedan {
    async fn park(&self) -> CarResponse {
        String::from("Sedan: park")
    }
    async fn drive(&self) -> CarResponse {
        String::from("Sedan: drive")
    }
    async fn brake(&self) -> CarResponse {
        String::from("Sedan: brake")
    }
    async fn reverse(&self) -> CarResponse {
        String::from("Sedan: reverse")
    }
}

#[async_trait]
impl Actor for Sedan {
    type Request = CarRequest;
    type Response = String;

    async fn handle(&mut self, message: Self::Request) -> Option<Self::Response> {
        match message {
            CarRequest::Park => Some(self.park().await),
            CarRequest::Drive => Some(self.drive().await),
            CarRequest::Brake => Some(self.brake().await),
            CarRequest::Reverse => Some(self.reverse().await),
        }
    }
}

impl Sedan {
    pub fn new() -> Self {
        Self
    }
}
