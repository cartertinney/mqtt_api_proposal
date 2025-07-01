use crate::error::{CompletionError, ClientError};

/// Await this token for a notice of the completion of an MQTT operation.
/// It is not required to await this token, only if you want to be notified of the completion.
/// 
/// Ideally, if cloned, this token can report the notice of completion to all copies.
pub struct CompletionToken {}

impl Future for CompletionToken {
    // should this return some ack data?
    type Output = Result<(), CompletionError>;

    fn poll(self: std::pin::Pin<&mut Self>, _cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        unimplemented!()
    }
}

/// DISCUSS: Should this be generic over the type of ACK? e.g. CompletionToken<PubAck> or CompletionToken<SubAck>?
/// That allows both success and error cases to provide the full ack information, but it makes for more complex typing (not that bad)
/// and creates significant confusion around the AckToken case (more concerning).






// DISCUSS: Should ack be able to send rc and properties in AckToken? If so, how?
// Merge properties? Priority system for conflicts?
// - Ewerton votes yes to the more complex one
// - Carl says keep it simple,


/// Used to acknowledge a specific received publish. Will acknowledge automatically
/// when dropped.
/// 
/// If cloned, the network acknowledgement will only occur once all clones are acknowledged
pub struct AckToken {}

impl AckToken {
    // Returns once the network ack has been queued (*after* any ack ordering necessary)
    //
    // The returned CompletionToken will resolve once the ack handshake is complete.
    // - QoS 1: Once the PUBACK is sent
    // - QoS 2: Once the PUBCOMP is received
    //
    // QoS 0 does not have an ack token, as it does not require any acknowledgement.
    //
    // Consumes itself on call, so it cannot be used again.
    pub async fn ack(self) -> Result<CompletionToken, ClientError> {
        unimplemented!()
    }
}

impl Drop for AckToken {
    fn drop(&mut self) {
        unimplemented!()
    }
}

// IMPLEMENTATION NOTE: If there is an outstanding (i.e. non-acknowledged) AckToken for a
// particular PKID, and another publish with the same PKID and a duplicate flag is received,
// it is safe (and, in fact, required) to discard the new message. Because a PKID cannot be
// reassigned, we know for sure that the new message is a duplicate of the outstanding one,
// because by virtue of the outstanding message AckToken existing, we know that the old
// message has not been acknowledged yet, i.e. the PKID has not been reused. This means that
// the original AckToken can ack the new message, because to the broker they are the same.
//
// If you do not do this, your token from the old message will ack the new message, and then
// the AckToken for the new message will ack a message that doesn't exist, creating problems
// with the broker.