use lapin::{BasicProperties, Connection, ConnectionProperties, options::*, types::FieldTable};

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

    // Publish the message
    channel
        .basic_publish(
            "",
            "hello",
            BasicPublishOptions::default(),
            b"Hello World!",
            BasicProperties::default(),
        )
        .await?;

    println!(" [x] Sent 'Hello World!'");

    // Close the connection
    conn.close(0, "Normal shutdown").await?;

    Ok(())
}
