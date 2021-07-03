    info!("Attempting to start up Sled database");
    let db = sled::open(&cfg.data.path)?;
    info!("Database opened. Was recovered? {}", db.was_recovered());
    let peer_tree = db.open_tree("peer_configs")?;
    info!("Peer configuration tree opened");
    let mem_tree = db.open_tree("members")?;
    info!("membership data tree opened");
    let mesh = Mesh::low_latency();
