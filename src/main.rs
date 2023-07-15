mod car;

use atticus::{run_actor, Actor};

use car::sedan::Sedan;
use car::suv::Suv;
use car::{Car, CarRequest};

async fn run_car<T>(car: T)
where
    T: Car + Actor,
    T: Actor<Request = CarRequest>,
    T: Actor<Response = String>,
    <T as Actor>::Request: Send + Sync,
    <T as Actor>::Response: Send + Sync,
{
    let actor_handle = run_actor(car, 1);
    let requestor = actor_handle.requestor;

    let response = requestor.request(CarRequest::Park).await.unwrap();
    println!("{}", response.unwrap());

    let response = requestor.request(CarRequest::Drive).await.unwrap();
    println!("{}", response.unwrap());

    let response = requestor.request(CarRequest::Brake).await.unwrap();
    println!("{}", response.unwrap());

    let response = requestor.request(CarRequest::Reverse).await.unwrap();
    println!("{}", response.unwrap());
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    run_car(Sedan::new()).await;
    run_car(Suv::new()).await;
}
