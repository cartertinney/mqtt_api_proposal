use bytes::Bytes;

use crate::packet::{Publish, QoS, PublishProperties, SubscribeProperties, UnsubscribeProperties, ConnectProperties, DisconnectProperties, AuthProperties};
use crate::token::{CompletionToken, AckToken};
use crate::error::ClientError;
use crate::topic::{TopicName, TopicFilter};


/// DISCUSS: What should this module and factory function be called?
/// The three components are the client collectively - so what should the outbound struct (currently called the Client) be?
/// Should it be MqttSender or something? Or are we fine with the duplicate semantic?
/// Alternatively, maybe we break up connect/disconnect/auth into a separate fourth component?

/// Creates the three components needed to run the MQTT client
pub fn new_client() -> (Client, EventLoop, Receiver) {
    unimplemented!()
}


/// Sends outgoing data.
/// Must be clonable.
pub struct Client {
}

// DISCUSS: Should the client provide preformed Publish, Connect, Subscribe, etc.? This would be strange with PKID,
// which can't be assigned yet. But if you don't assemble it, there is a strange semantic with receiving fully formed Publishes.
// Should the received Publishes be framed as Messages instead?

// DISCUSS: What type should topic be? (See discussion in error.rs)
// DISCUSS: What type should payload be?

impl Client {
    pub async fn connect(&self, properties: ConnectProperties) -> Result<CompletionToken, ClientError> {
        unimplemented!()
    }

    pub async fn disconnect(&self, properties: DisconnectProperties) -> Result<CompletionToken, ClientError> {
        unimplemented!()
    }

    pub async fn auth(&self, properties: AuthProperties) -> Result<CompletionToken, ClientError> {
        unimplemented!()
    }

    pub async fn publish(&self, topic_name: TopicName, qos: QoS, payload: Bytes, properties: PublishProperties) -> Result<CompletionToken, ClientError> {
        unimplemented!()
    }

    pub async fn subscribe(&self, topic_filter: TopicFilter, qos: QoS, properties: SubscribeProperties) -> Result<CompletionToken, ClientError> {
        unimplemented!()
    }

    pub async fn unsubscribe(&self, topic_filter: TopicFilter, properties: UnsubscribeProperties) -> Result<CompletionToken, ClientError> {
        unimplemented!()
    }
}



/// DISCUSS: Should this be called Connection instead? I think that's semantically clearer, but, it precludes us
/// from providing Events for certain things that aren't connection related, e.g. outgoing publish, etc, although
/// it's unclear if those things are even valuable.
pub struct EventLoop {}

impl EventLoop {
    /// Polls for an event from the event loop.
    /// As long as the event loop is being polled, the MQTT client will continue to run.
    pub async fn poll(&mut self) -> Event {
        unimplemented!()
    }
}

// DISCUSS: How should disconnect be handled? DesiredDisconnect vs UnexpectedDisconnect? Or leave it up to the user to stitch that
// together based on incoming data. Lean towards the latter so we don't project semantics.
// Should it be packet based, e.g. CONNACK to keep it as simple as possible?

pub enum Event {
    Connected,  // NOTE: These enums will need to have values in them where appropriate, e.g. CONNACK
    Disconnected,
    // other stuff
}


// DISCUSS: There probably ought to be two methods here - recv() and recv_manual_ack().
// In higher level dispatching scenarios, this would allow the use of this same structure.
// COUNTERPOINT: No, just always use the API with the AckToken and rely on the drop functionality for autoack.

pub struct Receiver {}
impl Receiver {
    /// Receive an incoming Publish, and any AckToken that may be associated with it.
    /// 
    /// AckToken will only be present if the Publish has a QoS of 1 or 2.
    /// 
    /// Receiving None indicates that the client has been dropped, and no more messages will be received.
    pub async fn recv(&self) -> Option<(Publish, Option<AckToken>)> {
        unimplemented!()
    }
}