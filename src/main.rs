use arrow2::array::{Array, MutableArray};
use arrow_format::flight::{data::FlightData, service::flight_service_client::FlightServiceClient};
use futures::stream::StreamExt;
use std::sync::Arc;

trait MyTrait {}

#[tokio::main]
async fn main() {
    let mut client = FlightServiceClient::connect("http://localhost:50051")
        .await
        .unwrap();

    // does not work
    let stream = futures::stream::iter::<Vec<Option<Arc<dyn Array>>>>(vec![]);

    // also does not work...
    let stream =
        futures::stream::iter::<Vec<Option<Arc<dyn MyTrait + Send + Sync + 'static>>>>(vec![]);

    // but this works... what makes it that dyn error the lifetime of tokio spawn here?
    let stream = futures::stream::iter::<Vec<Option<Arc<()>>>>(vec![]);

    let fut = async move {
        let mapped_stream = stream.map(|_| FlightData {
            flight_descriptor: None,
            data_header: vec![],
            app_metadata: vec![],
            data_body: vec![],
        });

        let result = client.do_put(mapped_stream).await;
        println!("{:#?}", result);
    };

    tokio::spawn(fut);
}
