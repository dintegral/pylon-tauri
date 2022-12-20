use ic_agent::{agent::http_transport::ReqwestHttpReplicaV2Transport, Agent};

pub fn create_agent(url: &str) -> Agent {
    let transport =
        ReqwestHttpReplicaV2Transport::create(url).expect("Failed to create Transport for Agent");

    Agent::builder()
        .with_transport(transport)
        .build()
        .expect("Failed to build the Agent")
}
