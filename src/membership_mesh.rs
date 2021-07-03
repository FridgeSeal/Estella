//! Rapid mesh-membership based functionality
use crate::settings::Settings;
use blip::{Member, Mesh, MeshService, Subscription};
use chrono::Utc;
use evmap::{ReadHandle, WriteHandle};
use log::info;
use rand::Rng;
use sled;
use std::{
    net::{IpAddr, Ipv6Addr, SocketAddr},
    thread,
};
use tonic::{
    self,
    transport::{self, Channel},
};

/// Attempt to join a mesh if one is available, and if not, starts one up
/// Returns an EVMap that readers can clone and used to read active members
pub async fn boostrap(
    cfg: &Settings,
) -> anyhow::Result<(String, ReadHandle<MeshKey, Vec<PeerNode>>)> {
    info!("Attempting to start up Sled database");
    let db = sled::open(&cfg.data.path)?;
    info!("Database opened. Was recovered? {}", db.was_recovered());
    let peer_tree = db.open_tree("peer_configs")?;
    info!("Peer configuration tree opened");
    let mem_tree = db.open_tree("members")?;
    info!("membership data tree opened");
    let mesh = Mesh::low_latency();
