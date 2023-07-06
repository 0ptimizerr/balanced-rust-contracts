use crate::error::ContractError;
use cw_common::xcall_data_types::{DepositRevert, WithdrawTo};
use rlp::{DecoderError, Rlp};

#[derive(Debug)]
pub enum DecodedStruct {
    WithdrawTo(WithdrawTo),
    DepositRevert(DepositRevert),
    // Add other struct types as needed
}

pub fn decode_encoded_bytes(data: &[u8]) -> Result<(&str, DecodedStruct), ContractError> {
    // Decode RLP bytes into an Rlp object
    let rlp = Rlp::new(data);

    if !rlp.is_list() {
        return Err(DecoderError::RlpExpectedToBeList.into());
    }

    // Extract method name
    let method: String = rlp.val_at(0)?;

    // Convert method: String -> &str
    match method.as_str() {
        "WithdrawTo" => {
            if rlp.item_count()? != 4 {
                return Err(DecoderError::RlpInvalidLength.into());
            }

            // Extract the fields
            let token: String = rlp.val_at(1)?;
            let user_address: String = rlp.val_at(2)?;
            let amount: u128 = rlp.val_at(3)?;

            // Create a new Withdraw instance
            let withdraw_to = WithdrawTo {
                token_address: token,
                user_address,
                amount,
            };

            // Return the decoded struct as a OK variant
            Ok(("WithdrawTo", DecodedStruct::WithdrawTo(withdraw_to)))
        }

        "DepositRevert" => {
            if rlp.item_count()? != 4 {
                return Err(DecoderError::RlpInvalidLength.into());
            }

            // Extract the fields
            let token_address = rlp.val_at(1)?;
            let account: String = rlp.val_at(2)?;
            let amount: u128 = rlp.val_at(3)?;

            // Create a new Deposit instance
            let deposit_revert = DepositRevert {
                token_address,
                account,
                amount,
            };

            // Return the decoded struct as an OK variant
            Ok((
                "DepositRevert",
                DecodedStruct::DepositRevert(deposit_revert),
            ))
        }

        // Handle other struct types here
        _ => Err(ContractError::UnknownMethod),
    }
}

#[cfg(test)]

mod tests {
    use cw_common::xcall_data_types::WithdrawRequest;
    use rlp::Encodable;

    use super::*;

    #[test]
    fn test_encode_decode_for_expected_msg_data() {
        let withdraw_to = WithdrawTo {
            token_address: String::from("token"),
            user_address: String::from("user"),
            amount: 1000,
        };

        let encoded_withdraw_to = withdraw_to.rlp_bytes();
        let (method, decoded_struct) = decode_encoded_bytes(&encoded_withdraw_to).unwrap();
        println!("decode:{:?}", decoded_struct);
        assert_eq!(method, "WithdrawTo");

        if let DecodedStruct::WithdrawTo(decoded_withdraw_to) = decoded_struct {
            assert_eq!(decoded_withdraw_to, withdraw_to);
        } else {
            panic!("Expected DecodedStruct::WithdrawTo variant");
        }
    }

    #[test]
    fn check_for_unwantend_msg_data() {
        let unknown_method = WithdrawRequest {
            token_address: String::from("token"),
            from: String::from("user"),
            amount: 1000,
        };

        let encoded_bytes = unknown_method.rlp_bytes();

        let result = decode_encoded_bytes(&encoded_bytes);
        //check if result contains err variant
        assert!(result.is_err());

        //compare error with expected error variant
        assert_eq!(result.unwrap_err(), ContractError::UnknownMethod);
    }
}