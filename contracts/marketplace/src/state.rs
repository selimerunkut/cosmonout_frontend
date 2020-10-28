use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{CanonicalAddr, Coin, Storage, StdResult};
use cw_storage_plus::{Map, Item, IndexedMap, Index, IndexList, MultiIndex};

pub static CONFIG_KEY: &[u8] = b"config";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub count: i32,
    pub owner: CanonicalAddr,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct Offering {
    pub token_id: String,

    pub contract_addr: CanonicalAddr,

    pub seller: CanonicalAddr,

    pub list_price: Coin,
}

pub const OFFERINGS: Map<(&[u8], &[u8]), Offering> = Map::new(b"offerings");

pub const OFFERINGS_COUNT: Item<u64> = Item::new(b"num_offerings");

pub fn num_offerings<S: Storage>(storage: &S) -> StdResult<u64> {
    Ok(OFFERINGS_COUNT.may_load(storage)?.unwrap_or_default())
}

pub fn increment_offerings<S: Storage>(storage: &mut S) -> StdResult<u64> {
    let val = num_offerings(storage)? + 1;
    OFFERINGS_COUNT.save(storage, &val)?;
    Ok(val)
}

pub struct OfferingIndexes<'a, S: Storage> {
    pub seller: MultiIndex<'a, S, Offering>,
    pub contract: MultiIndex<'a, S, Offering>
}

impl<'a, S: Storage> IndexList<S, Offering> for OfferingIndexes<'a, S> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<S, Offering>> + '_> {
        let v: Vec<&dyn Index<S, Offering>> = vec![&self.seller, &self.contract];
        Box::new(v.into_iter())
    }
}

pub fn offerings<'a, S: Storage>() -> IndexedMap<'a, &'a str, Offering, S, OfferingIndexes<'a, S>> {
    let indexes = OfferingIndexes {
        seller: MultiIndex::new(|o| o.seller.to_vec(), b"offerings", b"offerings__seller"),
        contract: MultiIndex::new(|o| o.contract_addr.to_vec(), b"offerings", b"offerings__contract"),
    };
    IndexedMap::new(b"offerings", indexes)
}
