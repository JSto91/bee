// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::net::IpAddr;

use bee_gossip::{Command::RemovePeer, NetworkCommandSender, PeerId};
use bee_runtime::resource::ResourceHandle;
use warp::{filters::BoxedFilter, http::StatusCode, reject, Filter, Rejection, Reply};

use crate::endpoints::{
    config::ROUTE_REMOVE_PEER, filters::with_network_command_sender, path_params::peer_id, permission::has_permission,
    rejection::CustomRejection,
};

fn path() -> impl Filter<Extract = (PeerId,), Error = warp::Rejection> + Clone {
    super::path()
        .and(warp::path("peers"))
        .and(peer_id())
        .and(warp::path::end())
}

pub(crate) fn filter(
    public_routes: Box<[String]>,
    allowed_ips: Box<[IpAddr]>,
    network_command_sender: ResourceHandle<NetworkCommandSender>,
) -> BoxedFilter<(impl Reply,)> {
    self::path()
        .and(warp::delete())
        .and(has_permission(ROUTE_REMOVE_PEER, public_routes, allowed_ips))
        .and(with_network_command_sender(network_command_sender))
        .and_then(|peer_id, network_controller| async move { remove_peer(peer_id, network_controller) })
        .boxed()
}

pub(crate) fn remove_peer(
    peer_id: PeerId,
    network_controller: ResourceHandle<NetworkCommandSender>,
) -> Result<impl Reply, Rejection> {
    if let Err(e) = network_controller.send(RemovePeer { peer_id }) {
        return Err(reject::custom(CustomRejection::NotFound(format!(
            "failed to remove peer: {}",
            e
        ))));
    }
    Ok(StatusCode::NO_CONTENT)
}
