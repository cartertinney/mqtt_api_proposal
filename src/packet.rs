// DISCUSS: Is this necessary? Should Publish be Message?
pub struct Publish {}

pub enum QoS {
    AtMostOnce,
    AtLeastOnce,
    ExactlyOnce,
}


pub struct ConnectProperties {}
impl Default for ConnectProperties {
    fn default() -> Self {
        ConnectProperties {}
    }
}

pub struct DisconnectProperties {}
impl Default for DisconnectProperties {
    fn default() -> Self {
        DisconnectProperties {}
    }
}

pub struct PublishProperties {}
impl Default for PublishProperties {
    fn default() -> Self {
        PublishProperties {}
    }
}

pub struct SubscribeProperties {}
impl Default for SubscribeProperties {
    fn default() -> Self {
        SubscribeProperties {}
    }
}

pub struct UnsubscribeProperties {}
impl Default for UnsubscribeProperties {
    fn default() -> Self {
        UnsubscribeProperties {}
    }
}

pub struct AuthProperties {}
impl Default for AuthProperties {
    fn default() -> Self {
        AuthProperties {}
    }
}