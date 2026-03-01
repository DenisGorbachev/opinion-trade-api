use crate::MarketStatusFilter;
use bon::Builder;
use std::num::NonZeroU64;

// TODO: Add the remaining fields
#[derive(Builder, Clone, Debug)]
#[derive(Default)]
pub struct MarketPageRequest {
    pub page: Option<NonZeroU64>,
    pub limit: Option<u32>,
    pub status: Option<MarketStatusFilter>,
    pub chain_id: Option<String>,
}
