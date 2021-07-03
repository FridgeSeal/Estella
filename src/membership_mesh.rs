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
    info!("Attempting to start low-latency mesh");
    let listen_addr = acquire_local_address();
    log::info!("Listening on port: {}", listen_addr.port());
    let (r_map, mut w_map) = evmap::new();
    let live_peer_map = PeerMesh { member_map: w_map };
    let mesh_future = match reconcile_addrs(listen_addr, &cfg.peers.address, None) {
        StartupMode::Solo => mesh.serve(listen_addr),
        StartupMode::JoinCluster(peer_addr) => {
            mesh.join_seed(peer_addr, cfg.use_tls).serve(listen_addr)
        }
    };
    mesh_future.await; // Send this to a specific thread
    Ok(("placehold_thread_handle_name".to_string(), r_map))
}

fn acquire_local_address() -> SocketAddr {
    let mut rng = rand::thread_rng();
    let base_addr = Ipv6Addr::LOCALHOST;
    let rand_port = rng.gen_range(1024..65000);
    let socket_addr = SocketAddr::new(IpAddr::V6(base_addr), rand_port);
    socket_addr
}

enum StartupMode {
    Solo,
    JoinCluster(SocketAddr),
}

fn reconcile_addrs(
    local_addr: SocketAddr,
    cfg_addr: &Option<SocketAddr>,
    pre_existing: Option<Vec<SocketAddr>>,
) -> StartupMode {
    // Used to simplify startup config by reconciling all local and provided and cached/stored addresses
    // removes it own, and attempts to connect to the remaining. If there are no remaining addresses, it will signal to
    // thus start up in solo/single-node mode, and wait for incorming connections.
    unimplemented!() // TODO
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum MeshKey {
    LiveMembers,
    CutMembers,
    KickedMembers,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct PeerNode {
    pub addr: SocketAddr,
    pub name: String,
    pub last_seen: chrono::DateTime<Utc>,
    // pub channel: transport::Channel,
}

impl PeerNode {
    fn open_channel(&self) -> Channel {
        unimplemented!() // TODO
    }
}

struct PeerMesh {
    member_map: WriteHandle<MeshKey, Vec<PeerNode>>,
}

#[blip::async_trait]
impl MeshService for PeerMesh {
    async fn accept(self: Box<Self>, mut cuts: Subscription) {
        while let Ok(cut) = cuts.recv().await {
            // handle membership change
            let m = cut.members();
            let j = cut.joined();
            let k = cut.kicked();
            dbg!(&m, &j, &k);
        }
    }
}

impl PeerMesh {
    fn cas(&mut self, member_type: MeshKey, member_arr: &[Member]) {
        self.member_map
            .update(member_type, member_arr.iter().map(|x| x.into()).collect())
            .refresh();
    }
}

impl From<Member> for PeerNode {
    fn from(m: Member) -> Self {
        Self {
            addr: m.addr(),
            name: String::from_utf8(m.metadata()["name"]).unwrap(), // unwrap because we _promise_ the value is legit. lol.
            last_seen: chrono::Utc::now(),
            // channel: m.channel(),
        }
    }
}
