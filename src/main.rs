use serde::{Deserialize};

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct MessageFloat {
    pub heartbeat_interval: f64,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct MessageInt {
    pub heartbeat_interval: u64,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum MessageWrapper<T> {
    Message(T),
}

fn main() {
    let data = std::fs::read("data.bin").unwrap();

    // parsing message is fine if the type is exact
    let msg: MessageInt = rmp_serde::from_slice(&data).unwrap();

    // parsing message is fine if the type is not exact
    let msg: MessageFloat = rmp_serde::from_slice(&data).unwrap();

    // parsing nested message is fine if the type is exact
    let msg: MessageWrapper<MessageInt> = rmp_serde::from_slice(&data).unwrap();

    // parsing nested message fails if the type is not exact
    let msg: MessageWrapper<MessageFloat> = rmp_serde::from_slice(&data).unwrap();
}
