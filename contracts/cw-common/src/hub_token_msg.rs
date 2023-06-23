use cosmwasm_schema::{cw_serde, QueryResponses};

use crate::data_types::types;

pub const X_CROSS_TRANSFER: &str = "XCrossTransfer";
pub const X_CROSS_TRANSFER_REVERT: &str = "XCrossTransferRevert";

#[cw_serde]
pub struct InstantiateMsg {
    pub x_call: String,
    pub hub_address: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    Setup {
        x_call: String,
        hub_address: String,
    },
    HandleCallMessage {
        from: String,
        data: Vec<u8>,
    },
    CrossTransfer {
        to: String,
        amount: u128,
        data: Vec<u8>,
    },
    XCrossTransfer {
        from: String,
        cross_transfer_data: types::CrossTransfer,
    },
    XCrossTransferRevert {
        from: String,
        cross_transfer_revert_data: types::CrossTransferRevert,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}