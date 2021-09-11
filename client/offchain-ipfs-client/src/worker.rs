use sp_runtime::traits::Block;
use sp_application_crypto::{Pair, AppPublic};
use sp_core::sp_std::fmt::Debug;
use std::convert::TryFrom;
use crate::{Client, IPFSParams};
use std::marker::PhantomData;
use sp_keystore::SyncCryptoStorePtr;
use polkadex_offchain_ipfs::primitives::OffchainIPFSApi;
use sc_client_api::{FinalityNotifications, FinalityNotification, Backend};
use std::sync::Arc;
use codec::Codec;
use futures::{StreamExt,FutureExt};
use sp_runtime::traits::Header;
use log::*;

pub struct IPFSWorker<B, C, BE, P>
    where
        B: Block,
        BE: Backend<B>,
        P: Pair,
        P::Public: AppPublic,
        P::Signature: Clone + Codec + Debug + PartialEq + TryFrom<Vec<u8>>,
        C: Client<B, BE, P>,
        C::Api: OffchainIPFSApi<B, P::Public>,
{
    client: Arc<C>,
    key_store: Option<SyncCryptoStorePtr>,
    finality_notifications: FinalityNotifications<B>,

    // keep rustc happy
    _backend: PhantomData<BE>,
    _pair: PhantomData<P>,

}


impl<B, C, BE, P> IPFSWorker<B, C, BE, P>
    where
        B: Block,
        BE: Backend<B>,
        P: Pair,
        P::Public: AppPublic,
        P::Signature: Clone + Codec + Debug + PartialEq + TryFrom<Vec<u8>>,
        C: Client<B, BE, P>,
        C::Api: OffchainIPFSApi<B, P::Public>,
{
    pub(crate) fn new(params: IPFSParams<BE,C>) -> IPFSWorker<B, C, BE, P>{
        let IPFSParams{
            client, 
            backend: _,
            key_store,
            
        } = params;
        
        IPFSWorker{
            client: client.clone(),
            key_store,
            finality_notifications: client.finality_notification_stream(),
            _backend: Default::default(),
            _pair: Default::default()
        }
    }

    fn handle_finality_notification(&mut self, notification: FinalityNotification<B>){
        trace!(target: "offchain-ipfs", "🥩 Got New Finality notification: {:?}", notification.header.number());
    }

    pub(crate) async fn run(mut self) {
        loop {
            futures::select! {
                notification = self.finality_notifications.next().fuse() => {
                    if let Some(notification) = notification {
                        self.handle_finality_notification(notification);
                    } else {
                        return;
                    }
                },
            }
        }
    }
}