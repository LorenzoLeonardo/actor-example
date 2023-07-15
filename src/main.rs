mod car;

use atticus::Requestor;
use car::sedan::Sedan;
use car::suv::Suv;
use car::CarRequest;
use car::CarResponse;

enum CarKind {
    Sedan(Sedan),
    Suv(Suv),
}

pub type CarRequestResponse = Requestor<CarRequest, CarResponse>;

impl CarKind {
    pub fn spawn(self) -> CarRequestResponse {
        match self {
            CarKind::Sedan(car) => {
                let handle = atticus::run_actor(car, 1);
                handle.requestor
            }
            CarKind::Suv(car) => {
                let handle = atticus::run_actor(car, 1);
                handle.requestor
            }
        }
    }
}

async fn run_car(car: CarKind) {
    let actor_handle = car.spawn();

    let response = actor_handle.request(CarRequest::Park).await.unwrap();
    println!("{:?}", response);

    let response = actor_handle.request(CarRequest::Drive).await.unwrap();
    println!("{:?}", response);

    let response = actor_handle.request(CarRequest::Brake).await.unwrap();
    println!("{:?}", response);

    let response = actor_handle.request(CarRequest::Reverse).await.unwrap();
    println!("{:?}", response);
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    run_car(CarKind::Sedan(Sedan::new())).await;
    run_car(CarKind::Suv(Suv::new())).await;
}
