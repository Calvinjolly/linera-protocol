// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

//! Conversions to WIT types from the original types.

use linera_base::{
    crypto::CryptoHash,
    data_types::{Amount, Timestamp},
    identifiers::{ApplicationId, BytecodeId, ChainId, MessageId},
};

use super::wit;

impl From<Timestamp> for wit::Timestamp {
    fn from(timestamp: Timestamp) -> Self {
        wit::Timestamp {
            inner0: timestamp.micros(),
        }
    }
}

impl From<ChainId> for wit::ChainId {
    fn from(chain_id: ChainId) -> Self {
        wit::ChainId {
            inner0: chain_id.0.into(),
        }
    }
}

impl From<CryptoHash> for wit::CryptoHash {
    fn from(crypto_hash: CryptoHash) -> Self {
        let parts = <[u64; 4]>::from(crypto_hash);

        wit::CryptoHash {
            part1: parts[0],
            part2: parts[1],
            part3: parts[2],
            part4: parts[3],
        }
    }
}

impl From<ApplicationId> for wit::ApplicationId {
    fn from(application_id: ApplicationId) -> Self {
        wit::ApplicationId {
            bytecode_id: application_id.bytecode_id.into(),
            creation: application_id.creation.into(),
        }
    }
}

impl From<BytecodeId> for wit::BytecodeId {
    fn from(bytecode_id: BytecodeId) -> Self {
        wit::BytecodeId {
            message_id: bytecode_id.message_id.into(),
        }
    }
}

impl From<MessageId> for wit::MessageId {
    fn from(message_id: MessageId) -> Self {
        wit::MessageId {
            chain_id: message_id.chain_id.into(),
            height: wit::BlockHeight {
                inner0: message_id.height.0,
            },
            index: message_id.index,
        }
    }
}

impl From<Amount> for wit::Amount {
    fn from(balance: Amount) -> Self {
        wit::Amount {
            inner0: (balance.lower_half(), balance.upper_half()),
        }
    }
}

impl From<wit::LogLevel> for log::Level {
    fn from(level: wit::LogLevel) -> Self {
        match level {
            wit::LogLevel::Trace => log::Level::Trace,
            wit::LogLevel::Debug => log::Level::Debug,
            wit::LogLevel::Info => log::Level::Info,
            wit::LogLevel::Warn => log::Level::Warn,
            wit::LogLevel::Error => log::Level::Error,
        }
    }
}
