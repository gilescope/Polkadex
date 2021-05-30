use std::convert::TryFrom;
use std::fmt::Debug;
use std::sync::Arc;

use codec::Codec;
use log::*;
use prometheus::Registry;
use sc_client_api::{Backend, BlockchainEvents, Finalizer};
use sc_network_gossip::{GossipEngine, Network as GossipNetwork};
use sp_api::ProvideRuntimeApi;
use sp_application_crypto::AppPublic;
use sp_blockchain::HeaderBackend;
use sp_keystore::SyncCryptoStorePtr;
use sp_runtime::traits::Block;
use thea_primitives::TheaApi;

mod gossip;
mod metrics;
mod mpc;
mod mpc_round;
mod worker;

#[cfg(test)]
mod test;

pub const THEA_PROTOCOL_NAME: &str = "/polkadex/thea/1";

/// A convenience THEA client trait that defines all the type bounds a THEA client
/// has to satisfy. Ideally that should actually be a trait alias. Unfortunately as
/// of today, Rust does not allow a type alias to be used as a trait bound. Tracking
/// issue is <https://github.com/rust-lang/rust/issues/41517>.
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

/// t-ECDSA Initialization Params
pub struct TheaParams<BE, C, N> {
    /// THEA client
    pub client: Arc<C>,
    /// Client Backend
    pub backend: Arc<BE>,
    /// Local key store
    pub key_store: Option<SyncCryptoStorePtr>,
    /// Gossip network
    pub network: N,
    /// Index of Thea party
    pub party_idx: u16,
    /// Threshold t
    pub threshold: u16,
    /// Total number of parties
    pub party_count: u16,
    /// Prometheus metric registry
    pub prometheus_registry: Option<Registry>,
}

/// Start the THEA gadget.
///
/// This is a thin shim around running and awaiting a THEA worker.
pub async fn start_thea_gadget<B, P, BE, C, N>(thea_params: TheaParams<BE, C, N>)
where
    B: Block,
    P: sp_core::Pair,
    P::Public: AppPublic + Codec,
    P::Signature: Clone + Codec + Debug + PartialEq + TryFrom<Vec<u8>>,
    BE: Backend<B>,
    C: Client<B, BE, P>,
    C::Api: TheaApi<B, P::Public>,
    N: GossipNetwork<B> + Clone + Send + 'static,
{
    let TheaParams {
        client,
        backend,
        key_store,
        network,
        party_idx,
        threshold,
        party_count,
        prometheus_registry,
    } = thea_params;

    let gossip_validator = Arc::new(gossip::TheaGossipValidator::new());
    let gossip_engine =
        GossipEngine::new(network, THEA_PROTOCOL_NAME, gossip_validator.clone(), None);

    let metrics = prometheus_registry
        .as_ref()
        .map(metrics::Metrics::register)
        .and_then(|result| match result {
            Ok(metrics) => {
                debug!(target: "thea", "🥩 Registered metrics");
                Some(metrics)
            }
            Err(err) => {
                debug!(target: "thea", "🥩 Failed to register metrics: {:?}", err);
                None
            }
        });

    let worker_params = worker::WorkerParams {
        client,
        backend,
        key_store,
        gossip_engine,
        gossip_validator,
        party_idx,
        threshold,
        metrics,
        party_count,
    };

    let worker = worker::TheaWorker::<_, _, _, _>::new(worker_params);

    // worker.run().await
}
