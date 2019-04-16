use super::origin_authority;
use crate::network::{
    p2p_topology as p2p, propagate, subscription, BlockConfig, Channels, ConnectionState,
    GlobalStateR,
};

use network_core::{
    client::{block::BlockService, gossip::GossipService},
    gossip::Node,
};
use network_grpc::{
    client::{Connect, Connection},
    peer as grpc_peer,
};

use futures::prelude::*;
use http::uri;
use tokio::{executor::DefaultExecutor, net::TcpStream};
use tower_service::Service as _;

use std::net::SocketAddr;

pub fn connect(
    addr: SocketAddr,
    state: ConnectionState,
    channels: Channels,
) -> impl Future<Item = (p2p::NodeId, propagate::PeerHandles), Error = ()> {
    info!("connecting to subscription peer {}", state.connection);
    info!("address: {}", addr);
    let peer = grpc_peer::TcpPeer::new(addr);
    let origin = origin_authority(addr);

    Connect::new(peer, DefaultExecutor::current())
        .origin(uri::Scheme::HTTP, origin)
        .node_id(state.global.node.id().clone())
        .call(())
        .map_err(move |err| {
            error!("Error connecting to peer {}: {:?}", addr, err);
        })
        .and_then(move |client| subscribe(client, state.global, channels))
}

fn subscribe(
    mut client: Connection<BlockConfig, TcpStream, DefaultExecutor>,
    global_state: GlobalStateR,
    channels: Channels,
) -> impl Future<Item = (p2p::NodeId, propagate::PeerHandles), Error = ()> {
    let block_box = channels.block_box;
    let mut prop_handles = propagate::PeerHandles::new();
    let block_sub = client.block_subscription(prop_handles.blocks.subscribe());
    let gossip_sub = client.gossip_subscription(prop_handles.gossip.subscribe());
    block_sub
        .join(gossip_sub)
        .map_err(move |err| {
            error!("Subscription request failed: {:?}", err);
        })
        .and_then(move |((block_sub, node_id), (gossip_sub, node_id_1))| {
            if node_id != node_id_1 {
                warn!(
                    "peer subscription IDs do not match: {} != {}",
                    node_id, node_id_1
                );
                return Err(());
            }
            subscription::process_blocks(block_sub, block_box);
            subscription::process_gossip(gossip_sub, global_state);
            Ok((node_id, prop_handles))
        })
}