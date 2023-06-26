//Constants for Reply messages

use cosmwasm_std::Uint128;

pub const REPLY_MSG_SUCCESS: u64 = 1;
pub const X_CROSS_TRANSFER: &str = "xCrossTransfer";
pub const X_CROSS_TRANSFER_REVERT: &str = "xCrossTransferRevert";
pub const TOKEN_NAME: &str = "HubToken";
pub const TOKEN_SYMBOL: &str = "HUB";
pub const TOKEN_DECIMALS: u8 = 18;
pub const TOKEN_TOTAL_SUPPLY: Uint128 = Uint128::zero();