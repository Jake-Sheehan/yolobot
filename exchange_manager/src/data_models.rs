use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TickerData {
    pub symbol: String,
    pub bid: f32,
    pub bid_qty: f32,
    pub ask: f32,
    pub ask_qty: f32,
    pub last: f32,
    pub volume: f32,
    pub vwap: f32,
    pub low: f32,
    pub high: f32,
    pub change: f32,
    pub change_pct: f32,
}

impl TickerData {
    pub fn new(symbol: String) -> Self {
        TickerData {
            symbol,
            bid: 0.0,
            bid_qty: 0.0,
            ask: 0.0,
            ask_qty: 0.0,
            last: 0.0,
            volume: 0.0,
            vwap: 0.0,
            low: 0.0,
            high: 0.0,
            change: 0.0,
            change_pct: 0.0,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubscribeResult {
    channel: String,
    snapshot: bool,
    symbol: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubscribeResponse {
    pub method: String,
    pub result: SubscribeResult,
    pub success: bool,
    pub time_in: String,
    pub time_out: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TickerResponse {
    r#type: String,
    data: Vec<TickerData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HeartBeatResponse {}

#[derive(Serialize, Deserialize, Debug)]
pub struct StatusData {
    api_version: String,
    connection_id: u64,
    system: String,
    version: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StatusResponse {
    channel: String,
    data: Vec<StatusData>,
    r#type: String,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "channel")]
pub enum StreamResponse {
    status {
        data: Vec<StatusData>,
        r#type: String,
    },
    heartbeat {},
    ticker {
        r#type: String,
        data: Vec<TickerData>,
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Response {
    Subscribe(SubscribeResponse),
    Stream(StreamResponse),
}
