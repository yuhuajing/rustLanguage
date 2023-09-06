Logs数据结构：
```rust
Log { address: 0x1f98431c8ad98523631ae4a59f267346ea31f984, 
topics: [0x783cca1c0412dd0d695e784568c96da2e9c22ff989357a2e8b1d9b2b4e6b7118, 0x00000000000000000000000004c54cc7d44c93f5be2b1f549611ba60a8252251, 0x000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2, 0x0000000000000000000000000000000000000000000000000000000000000bb8],
 data: Bytes(0x000000000000000000000000000000000000000000000000000000000000003c000000000000000000000000fb307df07f6a74a151328214857cb6e451fc790f), 
 block_hash: Some(0xa04fc64b27257f38da4f3ae574069595573d3ba9aef37928834316be1248b4d0), 
 block_number: Some(18070666), 
 transaction_hash: Some(0xa33dc60d8f7b990ab2d1dc5790e75fd95f662a5f2f0b57f0e24b7ebfaddb94e4), 
 transaction_index: Some(118), 
 log_index: Some(266), 
 transaction_log_index: None, 
 log_type: None, 
 removed: Some(false)
}
```

监听logs时的带有的Metadata 数据：
```rust
pub struct LogMeta {
    pub address: H160,
    pub block_number: U64,
    pub block_hash: H256,
    pub transaction_hash: H256,
    pub transaction_index: U64,
    pub log_index: U256,
}
```