use futures_lite::stream::StreamExt;
use lapin::{Connection, ConnectionProperties, message::Delivery, options::*, types::FieldTable};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to RabbitMQ server
    let conn =
        Connection::connect("amqp://localhost:5672", ConnectionProperties::default()).await?;

    // Create a channel
    let channel = conn.create_channel().await?;

    // Declare the queue
    channel
        .queue_declare(
            "hello",
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await?;

    // Start consuming messages
    let mut consumer = channel
        .basic_consume(
            "hello",
            "my_consumer",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    println!(" [*] Waiting for messages. To exit press CTRL+C");

    // Process messages as they arrive
    while let Some(delivery) = consumer.next().await {
        let delivery: Delivery = delivery?;
        let msg = String::from_utf8_lossy(&delivery.data);
        println!(" [x] Received {}", msg);

        // Acknowledge the message
        delivery.ack(BasicAckOptions::default()).await?;
    }

    Ok(())
}
