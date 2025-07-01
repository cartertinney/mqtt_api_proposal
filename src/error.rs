
// DISCUSS: Is too large worth preventing? Technically, you're allowed to send 256mb, even though the broker will tell you it
// can reject anything above a certain size. Worth validating? Or should we just let the broker reject it? Technically the only
// time you aren't allowed per spec to send something large is in the SUBACK/UNSUBACK/PUBACK/PUBCOMP/PUBREL/PUBREC flow.
// I think we probably still need the too large error just simply because the 256mb hard limit exists.
//
// I would also say that, it's fairly impractical to expect the application to simply know the max size, given that, we only find it
// out in the CONNACK, and so it requires the user to set up state for the application to track it, which is... odd


// DISCUSS: I prefer this because it keeps the error surface simpler, and has clearer semantics that don't step
// on the toes of the CompletionError.
// DISCUSS: In a real implementation, this (and all other errors) would be a struct, not an enum. Should it also contain T where T is the
// packet type, so you get back the packet data on failure? e.g. ClientError<Publish>? where error.packet() -> Publish?
#[derive(Debug)]
pub enum ClientError {
    DetachedClient,
    TooLarge,
}

/// Indicates the failure of an MQTT operation
#[derive(Debug)]
pub enum CompletionError {
    /// The operation was cancelled by the client.
    Cancelled,
    /// The operation was rejected by the broker.
    Rejected, // probably needs to contain some kind of reason string or code, ideally not one that's strongly typed. See discussion in token.rs
}



// OR:

// DISCUSS: If topic filters / topic names were validated at compile time as above, we could eliminate the entire class of error.
// This would also allow at least CONN/PUB/SUB/UNSUB to share an error (as above). Ack Error is still weird, but see the discussion in token.rs

// pub enum ConnectError {
//     DetachedClient,
//     TooLarge,
// }

// // DISCUSS: connect multiple times?

// pub enum PublishError {
//     DetachedClient,
//     InvalidTopicName,
//     TooLarge,
// }

// pub enum SubscribeError {
//     DetachedClient,
//     InvalidTopicFilter,
//     TooLarge,   // This could happen even without payload due to large user properties
// }

// pub enum UnsubscribeError {
//     DetachedClient,
//     InvalidTopicFilter,
//     TooLarge,           // This could happen even without payload due to large user properties
// }

// pub enum AckError {
//     /// The client is detached and no longer available
//     DetachedClient,
//     TooLarge,           // This could happen even without payload due to large user properties
// }