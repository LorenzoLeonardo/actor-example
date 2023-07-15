use async_trait::async_trait;
use atticus::Actor;

use super::{Car, CarRequest, CarResponse};

pub struct Suv;

#[async_trait]
impl Car for Suv {
    async fn park(&self) -> CarResponse {
        String::from("SUV: park")
    }
    async fn drive(&self) -> CarResponse {
        String::from("SUV: drive")
    }
    async fn brake(&self) -> CarResponse {
        String::from("SUV: brake")
    }
    async fn reverse(&self) -> CarResponse {
        String::from("SUV: reverse")
    }
}

#[async_trait]
impl Actor for Suv {
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

impl Suv {
    pub fn new() -> Self {
        Self
    }
}
