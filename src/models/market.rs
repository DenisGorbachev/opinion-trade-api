use crate::MarketRaw;
use thiserror::Error;

// TODO: Add the remaining fields
#[derive(serde::Serialize, serde::Deserialize, Eq, PartialEq, Default, Hash, Clone, Debug)]
#[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize))]
pub struct Market {
    pub id: u64,
    pub title: String,
    pub rules: String,
}

#[allow(clippy::infallible_try_from)]
impl TryFrom<MarketRaw> for Market {
    type Error = TryFromMarketRawForMarketError;

    fn try_from(_raw: MarketRaw) -> Result<Self, Self::Error> {
        todo!()
    }
}

#[derive(Error, Debug)]
pub enum TryFromMarketRawForMarketError {}
