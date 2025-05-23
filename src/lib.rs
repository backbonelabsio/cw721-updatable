mod contract_tests;
mod error;
mod execute;
pub mod helpers;
pub mod msg;
mod query;
pub mod state;

pub use crate::error::ContractError;
pub use crate::msg::{ExecuteMsg, InstantiateMsg, MintMsg, MinterResponse, QueryMsg};
pub use crate::state::Cw721Contract;
use cosmwasm_std::Empty;

// This is a simple type to let us handle empty extensions
pub type Extension = Option<Empty>;

pub mod entry {
    use super::*;

    #[cfg(not(feature = "library"))]
    use cosmwasm_std::entry_point;
    use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
    use cw2::set_contract_version;
    use crate::msg::MigrateMsg;

    // This makes a conscious choice on the various generics used by the contract
    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn instantiate(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> StdResult<Response> {
        let tract = Cw721Contract::<Extension, Empty, Empty, Empty>::default();
        tract.instantiate(deps, env, info, msg)
    }

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg<Extension, Empty>,
    ) -> Result<Response, ContractError> {
        let tract = Cw721Contract::<Extension, Empty, Empty, Empty>::default();
        tract.execute(deps, env, info, msg)
    }

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn query(deps: Deps, env: Env, msg: QueryMsg<Empty>) -> StdResult<Binary> {
        let tract = Cw721Contract::<Extension, Empty, Empty, Empty>::default();
        tract.query(deps, env, msg)
    }

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn migrate(
        deps: DepsMut,
        _env: Env,
        _msg: MigrateMsg,
    ) -> Result<Response, ContractError> {
        // Constants for contract info
        const CONTRACT_NAME: &str = "crates.io:cw721-bbl";
        const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

        // Get the version of the contract we're migrating from
        let ver = cw2::get_contract_version(deps.storage)?;

        // We allow migration from standard cw721 contracts or previous versions of our own
        let allowed_contracts = ["crates.io:cw721-metadata-onchain", CONTRACT_NAME];

        if !allowed_contracts.contains(&ver.contract.as_str()) {
            return Err(ContractError::CannotMigrate {
                previous_contract: ver.contract,
            });
        }

        // Update the contract version
        set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

        // Return success
        Ok(Response::default()
            .add_attribute("method", "migrate")
            .add_attribute("from_version", ver.version)
            .add_attribute("to_version", CONTRACT_VERSION)
            .add_attribute("from_contract", ver.contract))
    }
}
