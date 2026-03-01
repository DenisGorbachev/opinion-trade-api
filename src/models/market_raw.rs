// TODO: Add the remaining fields
#[derive(serde::Serialize, serde::Deserialize, Eq, PartialEq, Default, Hash, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MarketRaw {
    pub market_id: u64,
    pub market_title: String,
    pub rules: String,
}
