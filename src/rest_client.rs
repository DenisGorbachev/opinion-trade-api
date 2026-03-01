use crate::{Market, MarketPageRequest};
use derive_more::From;
use derive_new::new;
use futures::Stream;
use futures::stream::empty;
use thiserror::Error;

#[derive(new, From, Clone, Debug)]
pub struct RestClient {
    #[new(into)]
    pub key: String,
}

impl RestClient {
    pub fn market_pages(&self, _request: &mut MarketPageRequest) -> impl Stream<Item = Vec<Market>> {
        // TODO
        empty()
    }

    pub async fn market_page(&self, _request: &MarketPageRequest) -> Result<Vec<Market>, MarketPageError> {
        todo!()
    }
}

impl From<&str> for RestClient {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

#[derive(Error, Debug)]
pub enum MarketPageError {}
