//! Node and wallet harness for integration tests: spawn regtest bitcoind, expose
//! blocks_dir and RPC helpers, and run the action + expected-results test harness.

use std::path::PathBuf;
use std::time::Duration;

use anyhow::Result;
use bitcoin::{Address, Amount, Block, BlockHash, Transaction, Txid};
use corepc_node::{Conf, Node};

use std::collections::HashMap;

use crate::dense::{DenseStorageBuilder, TxId};

use crate::test_utils::temp_dir;

/// Holds a running regtest node and its wallet RPC client. Dropping stops the node.
pub struct NodeHarness {
    _node: Node,
    pub data_dir: PathBuf,
}

impl NodeHarness {
    /// Start a regtest node with optional config. If `conf` is `None`, uses default
    /// (regtest, default wallet). Sets `-txindex=1` so we can query by txid.
    pub fn new(conf: Option<Conf>) -> Result<Self> {
        let mut conf = conf.unwrap_or_default();
        conf.args.push("-txindex=1");
        conf.args.push("-fallbackfee=0.00001");
        let node = Node::from_downloaded_with_conf(&conf)?;
        // Regtest stores chain data under <datadir>/regtest/; blocks are in regtest/blocks.
        let data_dir = node.workdir().join("regtest");
        Ok(Self {
            _node: node,
            data_dir,
        })
    }

    /// RPC client for the default wallet.
    pub fn client(&self) -> &corepc_node::Client {
        &self._node.client
    }

    /// Generate `n` blocks to `address`; returns block hashes.
    pub fn generate_blocks(
        &self,
        n: u64,
        address: &Address<bitcoin::address::NetworkChecked>,
    ) -> Result<Vec<BlockHash>> {
        let nblocks: usize = n
            .try_into()
            .map_err(|_| anyhow::anyhow!("block count too large"))?;
        let hashes = self.client().generate_to_address(nblocks, address)?;
        hashes
            .0
            .iter()
            .map(|s| {
                s.parse()
                    .map_err(|e| anyhow::anyhow!("parse block hash: {}", e))
            })
            .collect()
    }

    /// Send `amount` to `address`; returns the txid of the created tx.
    pub fn send_to_address(
        &self,
        address: &Address<bitcoin::address::NetworkChecked>,
        amount: Amount,
    ) -> Result<Txid> {
        let res = self.client().send_to_address(address, amount)?;
        Ok(res.txid()?)
    }

    /// Current block count (height + 1, 0-indexed chain length).
    pub fn get_block_count(&self) -> Result<u64> {
        let count = self.client().get_block_count()?;
        Ok(count.0)
    }

    /// Fetch block by hash.
    pub fn get_block(&self, hash: BlockHash) -> Result<Block> {
        let block = self
            .client()
            .get_block(hash)
            .map_err(|e| anyhow::anyhow!("get_block: {:?}", e))?;
        Ok(block)
    }

    pub fn get_block_by_height(&self, height: u64) -> Result<Block> {
        let hash = self.client().get_block_hash(height)?.block_hash().unwrap();
        let block = self.get_block(hash)?;
        Ok(block)
    }

    /// Fetch raw transaction by txid (returns the transaction).
    pub fn get_raw_transaction(&self, txid: Txid) -> Result<Transaction> {
        let raw = self.client().get_raw_transaction(txid)?;
        Ok(raw.transaction()?)
    }

    /// Best block hash (chain tip).
    pub fn best_block_hash(&self) -> Result<BlockHash> {
        Ok(self.client().best_block_hash()?)
    }
}

/// Run the full harness: create node, run action, sync, build parser, run expected.
pub fn run_harness<A, E>(action: A, expected: E) -> Result<()>
where
    A: FnOnce(&mut NodeHarness) -> Result<()>,
    E: FnOnce(
        &NodeHarness,
        &crate::dense::DenseStorage,
        &HashMap<bitcoin::Txid, TxId>,
    ) -> Result<()>,
{
    let mut harness = NodeHarness::new(None)?;
    let address = harness.client().new_address()?;
    harness.generate_blocks(101, &address)?;
    let block_height_before = harness.get_block_count()?;

    action(&mut harness)?;

    let block_height_after = harness.get_block_count()?;
    let mut expected_txids = HashMap::new();
    // get_block_count() returns tip height (0-based). New blocks are at (block_height_before + 1)..=block_height_after.
    let mut running_index = block_height_before + 1;
    for i in (block_height_before + 1)..=block_height_after {
        let block = harness.get_block_by_height(i)?;
        for tx in block.txdata {
            expected_txids.insert(tx.compute_txid(), TxId::new(running_index as u32));
            running_index += 1;
        }
    }
    // Give bitcoind time to flush block files to disk.
    std::thread::sleep(Duration::from_secs(2));

    let blocks_dir = &harness.data_dir.join("blocks");
    let blk0 = blocks_dir.join("blk00000.dat");
    if !blk0.exists() {
        let entries_len = std::fs::read_dir(blocks_dir)
            .map(|d| d.count())
            .unwrap_or(0);
        return Err(anyhow::anyhow!(
            "blk00000.dat not found at {} (blocks_dir exists: {}, entries: {})",
            blk0.display(),
            blocks_dir.exists(),
            entries_len
        ));
    }

    // Dense parser expects Bitcoin Core's chain datadir (`.../regtest`), not `.../regtest/blocks`.
    let data_dir = harness.data_dir.to_path_buf();
    // Tip height is 0-based; number of blocks = tip + 1.
    let block_count = block_height_after + 1;
    // Integration tests build a full in-memory picture from blk files directly,
    // which avoids depending on bitcoind's LevelDB block index flush timing.
    let builder = DenseStorageBuilder::new(
        data_dir,
        temp_dir("primitives_integration"),
        0..block_count,
        Vec::new(),
    );
    let storage = builder.build()?;

    expected(&harness, &storage, &expected_txids)
}
