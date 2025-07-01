use mqtt::client::{new_client, Client, EventLoop, Receiver, Event};
use mqtt::packet::{QoS, ConnectProperties, PublishProperties, SubscribeProperties};
use mqtt::topic::{TopicFilter, TopicName};

#[tokio::main]
async fn main() {
    let (client, event_loop, receiver) = new_client();

    tokio::select! {
        _ = connection_runner(event_loop) => {
            // Connection runner finished
        }
        _ = receive(receiver) => {
            // Receiver finished
        }
        _ = program(client) => {
            // Program finished
        }
    }

}

async fn program(client: Client) {
    // Connect to the MQTT broker and wait for the connection to complete
    let connect_properties = ConnectProperties::default();
    let ct = client.connect(connect_properties).await.unwrap();
    ct.await.unwrap();

    // Subscribe to a topic and wait for the subscription to complete
    let subscribe_properties = SubscribeProperties::default();
    let ct = client.subscribe(TopicFilter::from("test/topic"), QoS::AtLeastOnce, subscribe_properties).await.unwrap();
    match ct.await {
        Ok(_) => println!("Subscribed to topic successfully"),
        Err(e) => eprintln!("Failed to subscribe: {:?}", e),
    }

    loop {
        // Publish a message to the topic (with no regard for the acknowledgement)
        let publish_properties = PublishProperties::default();
        client.publish(TopicName::from("test/topic"), QoS::AtLeastOnce, "Hello, MQTT!".into(), publish_properties).await.unwrap();

        // Sleep for a while before publishing again
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }
}

async fn connection_runner(mut event_loop: EventLoop) {
    loop {
        match event_loop.poll().await {
            Event::Connected => {
                println!("Connected to MQTT broker");
            }
            Event::Disconnected => {
                println!("Disconnected from MQTT broker");
            }
            // Handle other events as needed
        }
    }

}

async fn receive(mut receiver: Receiver) {
    loop {
        while let Some((publish, ack_token)) = receiver.recv().await {
            // NOTE: If you don't want manual ack, simply ignore the ack_token by using a _, and it
            // will be acked automatically on drop.
            // No need for "manual ack" setting on the client.


            // NOTE: Delegate any of this to another task if you like.
            println!("Received publish");

            if let Some(token) = ack_token {
                // Await the ack token to ensure the publish is acknowledged
                token.ack().await.unwrap();
                println!("Publish acknowledged");
            } else {
                println!("Publish does not require acknowledgment (QoS 0)");
            }
        }
    }
}