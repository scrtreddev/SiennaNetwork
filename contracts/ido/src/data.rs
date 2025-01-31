use amm_shared::fadroma::{
    scrt_addr::{Canonize, Humanize},
    scrt_link::ContractLink,
    scrt::{
        Api, CanonicalAddr, Extern, HumanAddr, Querier, StdError, StdResult, Storage, Uint128,
    },
    scrt_storage::{load, save},
    scrt_storage_traits::Storable,
    scrt_vk::ViewingKey,
};
use amm_shared::{msg::ido::SaleType, TokenType};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

const KEY_CONTRACT_ADDR: &[u8] = b"this_contract";
const KEY_VIEWING_KEY: &[u8] = b"viewing_key";
const TOTAL_PRE_LOCK_AMOUNT: &[u8] = b"total_pre_lock_amount";

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Config<A> {
    /// The token that is used to buy the sold SNIP20.
    pub input_token: TokenType<A>,
    /// The token that is being sold.
    pub sold_token: ContractLink<A>,
    /// Token constants
    pub swap_constants: SwapConstants,
    /// Number of participants currently
    pub taken_seats: u32,
    /// The maximum number of participants allowed.
    pub max_seats: u32,
    /// The total amount that each participant is allowed to buy.
    pub max_allocation: Uint128,
    /// The minimum amount that each participant is allowed to buy.
    pub min_allocation: Uint128,
    /// Configuration of sale type options (PreLockOnly, SwapOnly, PreLockAndSwap)
    pub sale_type: SaleType,
    /// Info of the launchpad contract that will post whitelisted
    /// addresses to IDO contract after it has been initialized
    pub launchpad: Option<ContractLink<A>>,
    /// The Option<> lets us know if this contract is active,
    /// contract only becomes active once the sold_token funds
    /// are sent to it:
    /// Amount has to be exact to max_seats * max_allocation
    ///
    /// This also means that the sold_token cannot be minted directly to
    /// this contract, it will have to be minted to the owner and then
    /// the owner will have to send funds to IDO contract. This limitation
    /// is due the mint message not having the means to sent the receive
    /// callback to IDO contract.
    pub schedule: Option<SaleSchedule>,
}

pub(crate) fn load_contract_address<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
) -> StdResult<HumanAddr> {
    let address: CanonicalAddr = load(&deps.storage, KEY_CONTRACT_ADDR)?.unwrap();

    address.humanize(&deps.api)
}

pub(crate) fn save_contract_address<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    address: &HumanAddr,
) -> StdResult<()> {
    let address = address.canonize(&deps.api)?;

    save(&mut deps.storage, KEY_CONTRACT_ADDR, &address)
}

pub(crate) fn increment_total_pre_lock_amount<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    add_amount: u128,
) -> StdResult<()> {
    let mut amount: Uint128 = load_total_pre_lock_amount(&deps)?;

    amount = amount
        .u128()
        .checked_add(add_amount)
        .ok_or_else(|| StdError::generic_err("Upper bound overflow detected."))?
        .into();

    save(&mut deps.storage, TOTAL_PRE_LOCK_AMOUNT, &amount)
}

pub(crate) fn load_total_pre_lock_amount<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
) -> StdResult<Uint128> {
    let amount: Uint128 = load(&deps.storage, TOTAL_PRE_LOCK_AMOUNT)?.unwrap_or_default();

    Ok(amount)
}

pub(crate) fn load_viewing_key(storage: &impl Storage) -> StdResult<ViewingKey> {
    let vk: ViewingKey = load(storage, KEY_VIEWING_KEY)?.unwrap();

    Ok(vk)
}

pub(crate) fn save_viewing_key(storage: &mut impl Storage, vk: &ViewingKey) -> StdResult<()> {
    save(storage, KEY_VIEWING_KEY, vk)
}

impl<A> Storable for Config<A>
where
    A: Serialize + serde::de::DeserializeOwned,
{
    fn namespace() -> Vec<u8> {
        b"config".to_vec()
    }
    /// Setting the empty key because config is only one
    fn key(&self) -> StdResult<Vec<u8>> {
        Ok(Vec::new())
    }
}

impl<A> Config<A> {
    pub fn load_self<S: Storage, T: Api, Q: Querier>(
        deps: &Extern<S, T, Q>,
    ) -> StdResult<Config<HumanAddr>> {
        let result = Config::<HumanAddr>::load(deps, b"")?;
        let result =
            result.ok_or_else(|| StdError::generic_err("Config doesn't exist in storage."))?;

        Ok(result)
    }

    /// Check if the contract is active
    pub fn is_active(&self) -> bool {
        self.schedule.is_some()
    }

    /// Check if tokens can be swaped
    pub fn is_swapable(&self, time: u64) -> StdResult<()> {
        if let Some(schedule) = self.schedule {
            if !schedule.has_started(time) {
                return Err(StdError::generic_err(format!(
                    "Sale hasn't started yet, come back in {} seconds",
                    schedule.start - time
                )));
            }
            if schedule.has_ended(time) {
                return Err(StdError::generic_err("Sale has ended"));
            }

            return Ok(());
        }

        Err(StdError::generic_err("Contract is not yet active"))
    }

    /// Check if the contract can be refunded
    pub fn is_refundable(&self, time: u64) -> StdResult<()> {
        if let Some(schedule) = self.schedule {
            if !schedule.has_ended(time) {
                return Err(StdError::generic_err(format!(
                    "Sale hasn't finished yet, come back in {} seconds",
                    schedule.end - time
                )));
            }

            return Ok(());
        }

        Err(StdError::generic_err("Contract is not yet active"))
    }

    /// Returns the total amount required for the sale to take place.
    /// Overflow checking needs to be performed at init time.
    pub fn total_allocation(&self) -> Uint128 {
        Uint128(self.max_allocation.u128() * self.max_seats as u128)
    }
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, Copy, Debug)]
pub(crate) struct SaleSchedule {
    /// Time when the sale will start
    pub start: u64,
    /// Time when the sale will end
    pub end: u64,
}

impl SaleSchedule {
    pub fn new(now: u64, start: Option<u64>, end: u64) -> StdResult<Self> {
        let start = start.unwrap_or(now);
        if start >= end {
            return Err(StdError::generic_err(format!(
                "End time of the sale has to be after {}.",
                start
            )));
        }
        if end <= now {
            return Err(StdError::generic_err(
                "End time of the sale must be any time after now.",
            ));
        }

        Ok(Self { start, end })
    }

    /// Check if the contract has started
    pub fn has_started(&self, time: u64) -> bool {
        self.start <= time
    }

    /// Check if the contract has ended
    pub fn has_ended(&self, time: u64) -> bool {
        time >= self.end
    }
}

/// Used when calculating the swap. These do not change
/// throughout the lifetime of the contract.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub(crate) struct SwapConstants {
    pub rate: Uint128,
    pub input_token_decimals: u8,
    pub sold_token_decimals: u8,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct Account {
    pub owner: HumanAddr,
    pub total_bought: Uint128,
    pub pre_lock_amount: Uint128,
}

impl Storable for Account {
    /// Global accounts namespace
    fn namespace() -> Vec<u8> {
        b"accounts".to_vec()
    }

    /// Individual account key
    fn key(&self) -> StdResult<Vec<u8>> {
        Ok(self.owner.to_string().as_bytes().to_vec())
    }
}

impl Account {
    pub fn new(owner: HumanAddr) -> Account {
        Account {
            owner,
            total_bought: Uint128::zero(),
            pre_lock_amount: Uint128::zero(),
        }
    }

    /// Load the account if its whitelisted
    pub fn load_self<S: Storage, T: Api, Q: Querier>(
        deps: &Extern<S, T, Q>,
        address: &HumanAddr,
    ) -> StdResult<Account> {
        Self::load(&deps, address.to_string().as_bytes())?
            .ok_or_else(|| StdError::generic_err("This address is not whitelisted."))
    }
}

impl Canonize<Config<CanonicalAddr>> for Config<HumanAddr> {
    fn canonize(&self, api: &impl Api) -> StdResult<Config<CanonicalAddr>> {
        let launchpad = match &self.launchpad {
            Some(l) => Some(l.canonize(api)?),
            None => None,
        };

        Ok(Config {
            input_token: self.input_token.canonize(api)?,
            sold_token: self.sold_token.canonize(api)?,
            swap_constants: self.swap_constants.clone(),
            taken_seats: self.taken_seats,
            max_seats: self.max_seats,
            max_allocation: self.max_allocation,
            min_allocation: self.min_allocation,
            sale_type: self.sale_type.clone(),
            launchpad,
            schedule: self.schedule,
        })
    }
}

impl Humanize<Config<HumanAddr>> for Config<CanonicalAddr> {
    fn humanize(&self, api: &impl Api) -> StdResult<Config<HumanAddr>> {
        let launchpad = match &self.launchpad {
            Some(l) => Some(l.humanize(api)?),
            None => None,
        };

        Ok(Config {
            input_token: self.input_token.humanize(api)?,
            sold_token: self.sold_token.humanize(api)?,
            swap_constants: self.swap_constants.clone(),
            taken_seats: self.taken_seats,
            max_seats: self.max_seats,
            max_allocation: self.max_allocation,
            min_allocation: self.min_allocation,
            sale_type: self.sale_type.clone(),
            launchpad,
            schedule: self.schedule,
        })
    }
}
