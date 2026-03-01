#[allow(unused_imports)]
use MarketStatusFilter::*;
use strum::Display;

#[derive(serde::Serialize, serde::Deserialize, Display, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Copy, Debug)]
#[serde(rename_all = "camelCase")]
pub enum MarketStatusFilter {
    Activated,
    Resolved,
}

impl MarketStatusFilter {}
