// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::types::ReservationID;
use fastcrypto::encoding::Base64;
use iota_json_rpc_types::iota_primitives::Address as AddressSchema;
use iota_json_rpc_types::{IotaTransactionBlockEffects, ObjectRefSchema};
use iota_sdk_types::Address as IotaAddress;
use iota_types::{
    base_types::ObjectRef,
    quorum_driver_types::ExecuteTransactionRequestType as IotaExecuteTransactionRequestType,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize)]
pub struct ReserveGasRequest {
    pub gas_budget: u64,
    pub reserve_duration_secs: u64,
}

#[derive(Debug, JsonSchema, Serialize, Deserialize)]
pub struct ReserveGasResponse {
    pub result: Option<ReserveGasResult>,
    pub error: Option<String>,
}

#[serde_as]
#[derive(Debug, JsonSchema, Serialize, Deserialize)]
pub struct ReserveGasResult {
    #[serde_as(as = "AddressSchema")]
    #[schemars(with = "AddressSchema")]
    pub sponsor_address: IotaAddress,
    pub reservation_id: ReservationID,
    #[serde_as(as = "Vec<ObjectRefSchema>")]
    #[schemars(with = "Vec<ObjectRefSchema>")]
    pub gas_coins: Vec<ObjectRef>,
}

impl ReserveGasResponse {
    pub fn new_ok(
        sponsor_address: IotaAddress,
        reservation_id: ReservationID,
        gas_coins: Vec<ObjectRef>,
    ) -> Self {
        Self {
            result: Some(ReserveGasResult {
                sponsor_address,
                reservation_id,
                gas_coins,
            }),
            error: None,
        }
    }

    pub fn new_err(error: anyhow::Error) -> Self {
        Self {
            result: None,
            error: Some(error.to_string()),
        }
    }
}

#[derive(Debug, JsonSchema, Serialize, Deserialize)]
pub struct ExecuteTxRequest {
    pub reservation_id: ReservationID,
    pub tx_bytes: Base64,
    pub user_sig: Base64,
    pub request_type: Option<ExecuteTransactionRequestType>,
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ExecuteTransactionRequestType {
    WaitForEffectsCert,
    WaitForLocalExecution,
}

impl Into<Option<IotaExecuteTransactionRequestType>> for ExecuteTransactionRequestType {
    fn into(self) -> Option<IotaExecuteTransactionRequestType> {
        match self {
            ExecuteTransactionRequestType::WaitForEffectsCert => {
                Some(IotaExecuteTransactionRequestType::WaitForEffectsCert)
            }
            ExecuteTransactionRequestType::WaitForLocalExecution => {
                Some(IotaExecuteTransactionRequestType::WaitForLocalExecution)
            }
        }
    }
}

#[derive(Debug, JsonSchema, Serialize, Deserialize)]
pub struct ExecuteTxResponse {
    pub effects: Option<IotaTransactionBlockEffects>,
    pub error: Option<String>,
}

impl ExecuteTxResponse {
    pub fn new_ok(effects: IotaTransactionBlockEffects) -> Self {
        Self {
            effects: Some(effects),
            error: None,
        }
    }

    pub fn new_err(error: anyhow::Error) -> Self {
        Self {
            effects: None,
            error: Some(error.to_string()),
        }
    }
}

#[derive(Debug, JsonSchema, Serialize, Deserialize)]
pub struct GasStationResponse<D = ()> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<D>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl<D> GasStationResponse<D> {
    pub fn new_ok(d: D) -> GasStationResponse<D> {
        Self {
            result: Some(d),
            error: None,
        }
    }

    pub fn new_err(error: anyhow::Error) -> Self {
        Self {
            result: None,
            error: Some(error.to_string()),
        }
    }

    pub fn new_err_from_str(error: impl AsRef<str>) -> Self {
        Self {
            result: None,
            error: Some(error.as_ref().to_string()),
        }
    }
}
