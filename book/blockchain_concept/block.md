```rust
#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Block<TX> {
    /// Hash of the block
    pub hash: Option<H256>,
    /// Hash of the parent
    #[serde(default, rename = "parentHash")]
    pub parent_hash: H256,
    /// Hash of the uncles
    #[cfg(not(feature = "celo"))]
    #[serde(default, rename = "sha3Uncles")]
    pub uncles_hash: H256,
    /// Miner/author's address. None if pending.
    #[serde(default, rename = "miner")]
    pub author: Option<Address>,
    /// State root hash
    #[serde(default, rename = "stateRoot")]
    pub state_root: H256,
    /// Transactions root hash
    #[serde(default, rename = "transactionsRoot")]
    pub transactions_root: H256,
    /// Transactions receipts root hash
    #[serde(default, rename = "receiptsRoot")]
    pub receipts_root: H256,
    /// Block number. None if pending.
    pub number: Option<U64>,
    /// Gas Used
    #[serde(default, rename = "gasUsed")]
    pub gas_used: U256,
    /// Gas Limit
    #[cfg(not(feature = "celo"))]
    #[serde(default, rename = "gasLimit")]
    pub gas_limit: U256,
    /// Extra data
    #[serde(default, rename = "extraData")]
    pub extra_data: Bytes,
    /// Logs bloom
    #[serde(rename = "logsBloom")]
    pub logs_bloom: Option<Bloom>,
    /// Timestamp
    #[serde(default)]
    pub timestamp: U256,
    /// Difficulty
    #[cfg(not(feature = "celo"))]
    #[serde(default)]
    pub difficulty: U256,
    /// Total difficulty
    #[serde(rename = "totalDifficulty")]
    pub total_difficulty: Option<U256>,
    /// Seal fields
    #[serde(default, rename = "sealFields", deserialize_with = "deserialize_null_default")]
    pub seal_fields: Vec<Bytes>,
    /// Uncles' hashes
    #[cfg(not(feature = "celo"))]
    #[serde(default)]
    pub uncles: Vec<H256>,
    /// Transactions
    #[serde(bound = "TX: Serialize + serde::de::DeserializeOwned", default)]
    pub transactions: Vec<TX>,
    /// Size in bytes
    pub size: Option<U256>,
    /// Mix Hash
    #[serde(rename = "mixHash")]
    #[cfg(not(feature = "celo"))]
    pub mix_hash: Option<H256>,
    /// Nonce
    #[cfg(not(feature = "celo"))]
    pub nonce: Option<crate::types::H64>,
    /// Base fee per unit of gas (if past London)
    #[serde(rename = "baseFeePerGas")]
    pub base_fee_per_gas: Option<U256>,
    /// Withdrawals root hash (if past Shanghai)
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "withdrawalsRoot")]
    #[cfg(not(feature = "celo"))]
    pub withdrawals_root: Option<H256>,
    /// Withdrawals (if past Shanghai)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg(not(feature = "celo"))]
    pub withdrawals: Option<Vec<Withdrawal>>,

    // #[cfg(feature = "celo")]
    // #[cfg_attr(docsrs, doc(cfg(feature = "celo")))]
    // /// The block's randomness
    // pub randomness: Randomness,

    /// BLS signatures with a SNARK-friendly hash function
    // #[cfg(feature = "celo")]
    // #[cfg_attr(docsrs, doc(cfg(feature = "celo")))]
    // #[serde(rename = "epochSnarkData", default)]
    // pub epoch_snark_data: Option<EpochSnarkData>,

    /// Captures unknown fields such as additional fields used by L2s
    #[cfg(not(feature = "celo"))]
    #[serde(flatten)]
    pub other: crate::types::OtherFields,
}
```