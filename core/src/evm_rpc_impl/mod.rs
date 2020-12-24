use crate::rpc::JsonRpcRequestProcessor;
use evm_rpc::basic::BasicERPC;
use evm_rpc::chain_mock::ChainMockERPC;
use evm_rpc::*;
use evm_state::*;
use sha3::{Digest, Keccak256};

use solana_sdk::commitment_config::{CommitmentConfig, CommitmentLevel};
use std::convert::TryInto;

const CHAIN_ID: u64 = 0x77;

const DEFAULT_COMITTMENT: Option<CommitmentConfig> = Some(CommitmentConfig {
    commitment: CommitmentLevel::Recent,
});

fn block_to_commitment(block: Option<String>) -> Option<CommitmentConfig> {
    let commitment = match block?.as_str() {
        "earliest" => CommitmentLevel::Root,
        "latest" => CommitmentLevel::Single,
        "pending" => CommitmentLevel::Single,
        v => {
            // Try to parse newest version of block commitment.
            if let Ok(c) = serde_json::from_str::<CommitmentLevel>(v) {
                c
            } else {
                // Probably user provide specific slot number, we didn't support bank from future, so just return default.
                return None;
            }
        }
    };
    Some(CommitmentConfig { commitment })
}

pub struct ChainMockERPCImpl;
impl ChainMockERPC for ChainMockERPCImpl {
    type Metadata = JsonRpcRequestProcessor;

    fn network_id(&self, _meta: Self::Metadata) -> Result<String, Error> {
        Ok(format!("0x{:x}", CHAIN_ID))
    }

    fn chain_id(&self, _meta: Self::Metadata) -> Result<Hex<u64>, Error> {
        Ok(Hex(CHAIN_ID))
    }

    // TODO: Add network info
    fn is_listening(&self, _meta: Self::Metadata) -> Result<bool, Error> {
        Ok(true)
    }

    fn peer_count(&self, _meta: Self::Metadata) -> Result<Hex<usize>, Error> {
        Ok(Hex(0))
    }

    fn sha3(&self, _meta: Self::Metadata, bytes: Bytes) -> Result<Hex<H256>, Error> {
        Ok(Hex(H256::from_slice(
            Keccak256::digest(bytes.0.as_slice()).as_slice(),
        )))
    }

    fn client_version(&self, _meta: Self::Metadata) -> Result<String, Error> {
        Ok(String::from("SolanaEvm/v0.1.0"))
    }

    fn protocol_version(&self, _meta: Self::Metadata) -> Result<String, Error> {
        Ok(String::from("0"))
    }

    fn is_syncing(&self, _meta: Self::Metadata) -> Result<bool, Error> {
        Ok(false)
    }

    fn coinbase(&self, _meta: Self::Metadata) -> Result<Hex<Address>, Error> {
        Ok(Hex(Address::from_low_u64_be(0)))
    }

    fn is_mining(&self, _meta: Self::Metadata) -> Result<bool, Error> {
        Ok(false)
    }

    fn hashrate(&self, _meta: Self::Metadata) -> Result<String, Error> {
        Ok(String::from("0x00"))
    }

    fn block_by_hash(
        &self,
        _meta: Self::Metadata,
        _block_hash: Hex<H256>,
        _full: bool,
    ) -> Result<Option<RPCBlock>, Error> {
        Ok(Some(RPCBlock {
            number: U256::zero().into(),
            hash: H256::zero().into(),
            parent_hash: H256::zero().into(),
            nonce: 0.into(),
            sha3_uncles: H256::zero().into(),
            logs_bloom: H256::zero().into(), // H2048
            transactions_root: H256::zero().into(),
            state_root: H256::zero().into(),
            receipts_root: H256::zero().into(),
            miner: Address::zero().into(),
            difficulty: U256::zero().into(),
            total_difficulty: U256::zero().into(),
            extra_data: vec![].into(),
            size: 0.into(),
            gas_limit: Gas::zero().into(),
            gas_used: Gas::zero().into(),
            timestamp: 0.into(),
            transactions: Either::Left(vec![]),
            uncles: vec![],
        }))
    }

    fn block_by_number(
        &self,
        meta: Self::Metadata,
        block: String,
        _full: bool,
    ) -> Result<Option<RPCBlock>, Error> {
        error!("Remove unwraps");
        let num = match &*block {
            "pending" => None,
            "earliest" => Some(meta.get_first_available_block()),
            "latest" => Some(meta.get_slot(None)),
            v => v.parse::<u64>().ok(),
        };
        let block_num = num.unwrap_or(0);
        Ok(meta
            .get_confirmed_block(block_num, None)
            .unwrap()
            .map(|block| {
                use std::str::FromStr;
                let hash = solana_sdk::hash::Hash::from_str(&block.blockhash).unwrap();
                let parent_hash =
                    solana_sdk::hash::Hash::from_str(&block.previous_blockhash).unwrap();
                // let transactions = block.transactions
                // .into_iter()
                // .filter_map(|tx|
                //     tx.transaction.decode())
                // .filter(|tx|
                //     tx.

                // )

                RPCBlock {
                    number: U256::from(block_num).into(),
                    hash: H256::from_slice(&hash.0).into(),
                    parent_hash: H256::from_slice(&parent_hash.0).into(),
                    size: 0.into(),
                    gas_limit: Gas::max_value().into(),
                    gas_used: Gas::zero().into(),
                    timestamp: 0.into(),
                    transactions: Either::Left(vec![]),

                    nonce: 0x7bb9369dcbaec019.into(),
                    sha3_uncles: H256::zero().into(),
                    logs_bloom: H256::zero().into(), // H2048
                    transactions_root: H256::zero().into(),
                    state_root: H256::zero().into(),
                    receipts_root: H256::zero().into(),
                    miner: Address::zero().into(),
                    difficulty: U256::zero().into(),
                    total_difficulty: U256::zero().into(),
                    extra_data: vec![].into(),
                    uncles: vec![],
                }
            }))
    }

    fn block_transaction_count_by_number(
        &self,
        _meta: Self::Metadata,
        _block: String,
    ) -> Result<Option<Hex<usize>>, Error> {
        Ok(None)
    }

    fn block_transaction_count_by_hash(
        &self,
        _meta: Self::Metadata,
        _block_hash: Hex<H256>,
    ) -> Result<Option<Hex<usize>>, Error> {
        Err(Error::NotFound)
    }

    fn uncle_by_block_hash_and_index(
        &self,
        _meta: Self::Metadata,
        _block_hash: Hex<H256>,
        _uncle_id: Hex<U256>,
    ) -> Result<Option<RPCBlock>, Error> {
        Err(Error::NotFound)
    }

    fn uncle_by_block_number_and_index(
        &self,
        _meta: Self::Metadata,
        _block: String,
        _uncle_id: Hex<U256>,
    ) -> Result<Option<RPCBlock>, Error> {
        Err(Error::NotFound)
    }

    fn block_uncles_count_by_hash(
        &self,
        _meta: Self::Metadata,
        _block_hash: Hex<H256>,
    ) -> Result<Option<Hex<usize>>, Error> {
        Err(Error::NotFound)
    }

    fn block_uncles_count_by_number(
        &self,
        _meta: Self::Metadata,
        _block: String,
    ) -> Result<Option<Hex<usize>>, Error> {
        Err(Error::NotFound)
    }

    fn transaction_by_block_hash_and_index(
        &self,
        _meta: Self::Metadata,
        _block_hash: Hex<H256>,
        _tx_id: Hex<U256>,
    ) -> Result<Option<RPCTransaction>, Error> {
        Err(Error::NotFound)
    }

    fn transaction_by_block_number_and_index(
        &self,
        _meta: Self::Metadata,
        _block: String,
        _tx_id: Hex<U256>,
    ) -> Result<Option<RPCTransaction>, Error> {
        Err(Error::NotFound)
    }
}

pub struct BasicERPCImpl;
impl BasicERPC for BasicERPCImpl {
    type Metadata = JsonRpcRequestProcessor;

    // The same as get_slot
    fn block_number(&self, meta: Self::Metadata) -> Result<Hex<usize>, Error> {
        let bank = meta.bank(CommitmentConfig::recent().into());
        Ok(Hex(bank.slot() as usize))
    }

    fn balance(
        &self,
        meta: Self::Metadata,
        address: Hex<Address>,
        block: Option<String>,
    ) -> Result<Hex<U256>, Error> {
        let bank = meta.bank(block_to_commitment(block));
        let evm_state = bank.evm_state.read().unwrap();
        Ok(Hex(evm_state.basic(address.0).balance))
    }

    fn storage_at(
        &self,
        meta: Self::Metadata,
        address: Hex<Address>,
        data: Hex<H256>,
        block: Option<String>,
    ) -> Result<Hex<H256>, Error> {
        let bank = meta.bank(block_to_commitment(block));
        let evm_state = bank.evm_state.read().unwrap();
        Ok(Hex(evm_state
            .get_storage(address.0, data.0)
            .unwrap_or_default()))
    }

    fn transaction_count(
        &self,
        meta: Self::Metadata,
        address: Hex<Address>,
        block: Option<String>,
    ) -> Result<Hex<U256>, Error> {
        let bank = meta.bank(block_to_commitment(block));
        let evm_state = bank.evm_state.read().unwrap();
        Ok(Hex(evm_state.basic(address.0).nonce))
    }

    fn code(
        &self,
        meta: Self::Metadata,
        address: Hex<Address>,
        block: Option<String>,
    ) -> Result<Bytes, Error> {
        let bank = meta.bank(block_to_commitment(block));
        let evm_state = bank.evm_state.read().unwrap();
        Ok(Bytes(evm_state.basic(address.0).code))
    }

    fn transaction_by_hash(
        &self,
        meta: Self::Metadata,
        tx_hash: Hex<H256>,
    ) -> Result<Option<RPCTransaction>, Error> {
        let bank = meta.bank(CommitmentConfig::recent().into());
        let evm_state = bank.evm_state.read().unwrap();
        let receipt = evm_state.get_tx_receipt_by_hash(tx_hash.0);
        Ok(match receipt {
            Some(tx) => Some(RPCTransaction {
                transaction_index: Some(0.into()),
                block_hash: Some(H256::zero().into()),
                block_number: Some(U256::zero().into()),
                from: Some(
                    tx.transaction
                        .caller()
                        .map_err(|_| Error::InvalidParams)?
                        .into(),
                ),
                to: Some(
                    tx.transaction
                        .address()
                        .map_err(|_| Error::InvalidParams)?
                        .into(),
                ),
                gas: Some(tx.transaction.gas_limit.into()),
                gas_price: Some(tx.transaction.gas_price.into()),
                value: Some(tx.transaction.value.into()),
                data: Some(tx.transaction.input.clone().into()),
                nonce: Some(tx.transaction.nonce.into()),
                hash: Some(tx_hash),
            }),
            None => None,
        })
    }

    fn transaction_receipt(
        &self,
        meta: Self::Metadata,
        tx_hash: Hex<H256>,
    ) -> Result<Option<RPCReceipt>, Error> {
        let bank = meta.bank(CommitmentConfig::recent().into());
        let evm_state = bank.evm_state.read().unwrap();
        let receipt = evm_state.get_tx_receipt_by_hash(tx_hash.0);
        Ok(match receipt {
            Some(tx) => Some(RPCReceipt {
                transaction_index: 0.into(),
                block_hash: H256::zero().into(),
                block_number: U256::zero().into(),
                cumulative_gas_used: tx.used_gas.into(),
                gas_used: tx.used_gas.into(),
                transaction_hash: tx_hash,
                logs: vec![],
                contract_address: Some(Hex(tx
                    .transaction
                    .address()
                    .map_err(|_| Error::InvalidParams)?)),
                root: H256::zero().into(),
                status: if let evm_state::ExitReason::Succeed(_) = tx.status {
                    1
                } else {
                    0
                },
            }),
            None => None,
        })
    }

    fn call(
        &self,
        meta: Self::Metadata,
        tx: RPCTransaction,
        block: Option<String>,
    ) -> Result<Bytes, Error> {
        let result = call(meta, tx, block)?;
        Ok(Bytes(result.1))
    }

    fn estimate_gas(
        &self,
        meta: Self::Metadata,
        tx: RPCTransaction,
        block: Option<String>,
    ) -> Result<Hex<Gas>, Error> {
        let result = call(meta, tx, block)?;
        Ok(Hex(result.2.into()))
    }
}

fn call(
    meta: JsonRpcRequestProcessor,
    tx: RPCTransaction,
    _block: Option<String>,
) -> Result<(evm_state::ExitReason, Vec<u8>, usize), Error> {
    let caller = tx.from.map(|a| a.0).unwrap_or_default();

    let value = tx.value.map(|a| a.0).unwrap_or_else(|| 0.into());
    let input = tx.data.map(|a| a.0).unwrap_or_else(Vec::new);
    let gas_limit: u64 = tx
        .gas
        .map(|a| a.0)
        .unwrap_or_else(|| 300000.into())
        .try_into()
        .map_err(|_| Error::InvalidParams)?;
    let gas_limit = gas_limit as usize;

    let bank = meta.bank(DEFAULT_COMITTMENT);
    let evm_state = bank
        .evm_state
        .read()
        .expect("meta bank EVM state was poisoned");

    let evm_state = evm_state.clone(); // TODO: revise

    let mut executor = evm_state::Executor::with_config(evm_state, Config::istanbul(), gas_limit);
    let address = tx.to.map(|h| h.0).unwrap_or_default();
    let result =
        executor.with_executor(|e| e.transact_call(caller, address, value, input, gas_limit));
    let gas_used = executor.with_executor(|e| e.used_gas());
    Ok((result.0, result.1, gas_used))
}
