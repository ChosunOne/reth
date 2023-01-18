use jsonrpsee::core::RpcResult;
use reth_network::{peers::PeerKind, NetworkHandle};
use reth_network_api::PeersInfo;
use reth_primitives::NodeRecord;
use reth_rpc_api::AdminApiServer;
use reth_rpc_types::NodeInfo;

/// `admin` API implementation.
///
/// This type provides the functionality for handling `admin` related requests.
pub struct AdminApi {
    /// An interface to interact with the network
    network: NetworkHandle,
}

impl AdminApi {
    /// Creates a new instance of `AdminApi`.
    pub fn new(network: NetworkHandle) -> AdminApi {
        AdminApi { network }
    }
}

impl AdminApiServer for AdminApi {
    fn add_peer(&self, record: NodeRecord) -> RpcResult<bool> {
        self.network.add_peer(record.id, record.tcp_addr());
        Ok(true)
    }

    fn remove_peer(&self, record: NodeRecord) -> RpcResult<bool> {
        self.network.remove_peer(record.id, PeerKind::Basic);
        Ok(true)
    }

    fn add_trusted_peer(&self, record: NodeRecord) -> RpcResult<bool> {
        self.network.add_trusted_peer(record.id, record.tcp_addr());
        Ok(true)
    }

    fn remove_trusted_peer(&self, record: NodeRecord) -> RpcResult<bool> {
        self.network.remove_peer(record.id, PeerKind::Trusted);
        Ok(true)
    }

    fn subscribe(
        &self,
        _subscription_sink: jsonrpsee::SubscriptionSink,
    ) -> jsonrpsee::types::SubscriptionResult {
        todo!()
    }

    fn node_info(&self) -> RpcResult<NodeInfo> {
        let enr = self.network.local_node_record();

        Ok(NodeInfo::new(enr))
    }
}

impl std::fmt::Debug for AdminApi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AdminApi").finish_non_exhaustive()
    }
}