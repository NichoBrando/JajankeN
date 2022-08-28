use jajanken::jajanken_client::JajankenClient;
use jajanken::SelectMovementRequest;

pub mod jajanken {
    tonic::include_proto!("jajanken");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = JajankenClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(SelectMovementRequest {
        movement: "rock".to_string(),
    });

    client.select_movement(request).await?;

    Ok(())
}
