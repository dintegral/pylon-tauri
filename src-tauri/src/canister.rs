use ic_agent::{export::Principal, Agent, AgentError};
use ic_utils::{
    call::SyncCall,
    interfaces::{
        http_request::{HeaderField, HttpResponse},
        HttpRequestCanister,
    },
};

pub async fn canister_http_request<'a>(
    agent: &Agent,
    canister_id: &Principal,
    path: &str,
    headers: Vec<HeaderField<'a>>,
) -> Result<HttpResponse, AgentError> {
    let canister = HttpRequestCanister::create(agent, canister_id.to_owned());

    let (res,) = canister
        .http_request_custom("get", path, headers.into_iter(), &[])
        .call()
        .await?;

    Ok(res)
}
