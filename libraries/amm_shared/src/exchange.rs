use fadroma::{
    scrt::{HumanAddr, StdResult, Api, CanonicalAddr},
    scrt_addr::{Canonize, Humanize}
};
use crate::token_pair::TokenPair;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Represents the address of an exchange and the pair that it manages
#[derive(Serialize, Deserialize, JsonSchema, Clone, PartialEq, Debug)]
pub struct Exchange<A: Clone> {
    /// The pair that the contract manages.
    pub pair: TokenPair<A>,
    /// Address of the contract that manages the exchange.
    pub address: A,
}

impl Canonize<Exchange<CanonicalAddr>> for Exchange<HumanAddr> {
    fn canonize(&self, api: &impl Api) -> StdResult<Exchange<CanonicalAddr>> {
        Ok(Exchange {
            pair: self.pair.canonize(api)?,
            address: self.address.canonize(api)?,
        })
    }
}

impl Humanize<Exchange<HumanAddr>> for Exchange<CanonicalAddr> {
    fn humanize(&self, api: &impl Api) -> StdResult<Exchange<HumanAddr>> {
        Ok(Exchange {
            pair: self.pair.humanize(api)?,
            address: api.human_address(&self.address)?,
        })
    }
}

#[derive(Serialize, Deserialize, JsonSchema, PartialEq, Debug, Clone)]
pub struct ExchangeSettings<A> {
    pub swap_fee: Fee,
    pub sienna_fee: Fee,
    pub sienna_burner: Option<A>,
}

impl ExchangeSettings<HumanAddr> {
    pub fn canonize(&self, api: &impl Api) -> StdResult<ExchangeSettings<CanonicalAddr>> {
        Ok(ExchangeSettings {
            swap_fee: self.swap_fee,
            sienna_fee: self.sienna_fee,
            sienna_burner: if let Some(info) = &self.sienna_burner {
                Some(info.canonize(api)?)
            } else {
                None
            },
        })
    }
}

impl ExchangeSettings<CanonicalAddr> {
    pub fn humanize(self, api: &impl Api) -> StdResult<ExchangeSettings<HumanAddr>> {
        Ok(ExchangeSettings {
            swap_fee: self.swap_fee,
            sienna_fee: self.sienna_fee,
            sienna_burner: if let Some(info) = self.sienna_burner {
                Some(info.humanize(api)?)
            } else {
                None
            },
        })
    }
}

#[derive(Serialize, Deserialize, JsonSchema, PartialEq, Clone, Copy, Debug)]
pub struct Fee {
    pub nom: u8,
    pub denom: u16,
}

impl Fee {
    pub fn new(nom: u8, denom: u16) -> Self {
        Self { nom, denom }
    }
}
