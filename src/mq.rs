use lapin::{options::*, types::FieldTable, BasicProperties, Connection, ConnectionProperties};
use tokio_amqp::*;
const QUEUE_NAME: &str = "petstore";

pub struct RabbitMQ {
   
 }
 
 impl RabbitMQ {

async fn add_pet(message: String) -> Result<()> {
    let conn = Connection::connect(
        "amqp://guest:guest@localhost:5672",
        ConnectionProperties::default(),
    )
    .await?;

    let channel = conn.create_channel().await?;
    let queue = channel.queue_declare(QUEUE_NAME, QueueDeclareOptions::default(), FieldTable::default()).await?;

    channel
        .basic_publish(
            "add_pet",
            queue.name().as_str(),
            BasicPublishOptions::default(),
            message.into_bytes(),
            BasicProperties::default(),
        )
        .await?;

    // add log of published message
    log::info!("add_pet:: Published message: {}", message);

    conn.close(0, "").await?;
    Ok(())
}

async fn update_pet(message: String) -> Result<()> {
    let conn = Connection::connect(
        "amqp://guest:guest@localhost:5672",
        ConnectionProperties::default(),
    )
    .await?;

    let channel = conn.create_channel().await?;
    let queue = channel.queue_declare(QUEUE_NAME, QueueDeclareOptions::default(), FieldTable::default()).await?;

    channel
        .basic_publish(
            "update_pet",
            queue.name().as_str(),
            BasicPublishOptions::default(),
            message.into_bytes(),
            BasicProperties::default(),
        )
        .await?;

    // add log of published message
    log::info!("update_pet:: Published message: {}", message);

    conn.close(0, "").await?;
    Ok(())
	}// end fn

    // add delete_pet
    async fn delete_pet(message: String) -> Result<()> {
    let conn = Connection::connect(
        "amqp://guest:guest@localhost:5672",
        ConnectionProperties::default(),
    ) 
    .await?;

    let channel = conn.create_channel().await?;
    let queue = channel.queue_declare(QUEUE_NAME, QueueDeclareOptions::default(), FieldTable::default()).await?;

    channel
        .basic_publish(
            "delete_pet",
            queue.name().as_str(),
            BasicPublishOptions::default(),
            message.into_bytes(),
            BasicProperties::default(),
        )
        .await?;

    // add log of published message
    log::info!("delete_pet:: Published message: {}", message);
    
    conn.close(0, "").await?;
    Ok(())
}// end fn


}// end impl

