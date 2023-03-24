// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

// This is required because a diesel macro makes clippy sad
#![allow(clippy::extra_unused_lifetimes)]

use anyhow::{Context, Result};
use aptos_api_types::deserialize_from_string;
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ObjectCore {
    pub allow_ungated_transfer: bool,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub guid_creation_num: BigDecimal,
    pub owner: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum V2TokenResource {
    ObjectCore(ObjectCore),
}

impl V2TokenResource {
    pub fn is_resource_supported(data_type: &str) -> bool {
        matches!(data_type, "0x1::object::ObjectCore")
    }

    pub fn from_resource(
        data_type: &str,
        data: &serde_json::Value,
        txn_version: i64,
    ) -> Result<V2TokenResource> {
        match data_type {
            "0x1::object::ObjectCore" => serde_json::from_value(data.clone())
                .map(|inner| Some(V2TokenResource::ObjectCore(inner))),
            _ => Ok(None),
        }
        .context(format!(
            "version {} failed! failed to parse type {}, data {:?}",
            txn_version, data_type, data
        ))?
        .context(format!(
            "Resource unsupported! Call is_resource_supported first. version {} type {}",
            txn_version, data_type
        ))
    }
}
