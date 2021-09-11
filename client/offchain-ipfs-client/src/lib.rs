use std::convert::TryFrom;
use std::fmt::Debug;
use std::sync::Arc;

use codec::Codec;
use sc_client_api::{Backend, BlockchainEvents, Finalizer};
use sp_api::ProvideRuntimeApi;
use sp_application_crypto::AppPublic;
use sp_blockchain::HeaderBackend;
use sp_keystore::SyncCryptoStorePtr;
use sp_runtime::traits::Block;

use polkadex_offchain_ipfs::primitives::OffchainIPFSApi;

use crate::worker::IPFSWorker;

mod worker;

#[cfg(test)]
mod tests;


// Params passed by client for starting the IPFS
pub struct IPFSParams<BE, C> {
    /// Client
    pub client: Arc<C>,
    /// Client Backend
    pub backend: Arc<BE>,
    /// Local key store
    pub key_store: Option<SyncCryptoStorePtr>,
}

pub trait Client<B, BE, P>:
BlockchainEvents<B> + HeaderBackend<B> + Finalizer<B, BE> + ProvideRuntimeApi<B> + Send + Sync
    where
        B: Block,
        BE: Backend<B>,
        P: sp_core::Pair,
        P::Public: AppPublic + Codec,
        P::Signature: Clone + Codec + Debug + PartialEq + TryFrom<Vec<u8>>,
{
    // empty
}

impl<B, BE, P, T> Client<B, BE, P> for T
    where
        B: Block,
        BE: Backend<B>,
        P: sp_core::Pair,
        P::Public: AppPublic + Codec,
        P::Signature: Clone + Codec + Debug + PartialEq + TryFrom<Vec<u8>>,
        T: BlockchainEvents<B>
        + HeaderBackend<B>
        + Finalizer<B, BE>
        + ProvideRuntimeApi<B>
        + Send
        + Sync,
{
    // empty
}


pub async fn start_ipfs_worker<B, P, BE, C>(params: IPFSParams<BE, C>)
    where
        B: Block,
        P: sp_core::Pair,
        P::Public: AppPublic + Codec,
        P::Signature: Clone + Codec + Debug + PartialEq + TryFrom<Vec<u8>>,
        BE: Backend<B>,
        C: Client<B, BE, P>,
        C::Api: OffchainIPFSApi<B, P::Public>,
{
    let worker = IPFSWorker::<_, _, _, _>::new(params);
    worker.run().await;
}