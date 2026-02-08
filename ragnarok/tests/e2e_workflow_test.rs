//! E2E tests (Phase 8): CLI → Odin Service → Response.
//! Run in container with mock-odin (docker-compose); skips if Odin unreachable.

use ragnarok::services::OdinServiceIntegration;

fn parse_odin_url(url: &str) -> (String, u16) {
    let rest = url
        .strip_prefix("http://")
        .or_else(|| url.strip_prefix("https://"))
        .unwrap_or(url);
    let (host, port_str) = rest.split_once(':').unwrap_or((rest, "50050"));
    let port = port_str.parse().unwrap_or(50050);
    (host.to_string(), port)
}

#[tokio::test]
async fn e2e_chat_via_odin_returns_response() {
    let url = std::env::var("ODIN_URL").unwrap_or_else(|_| "http://mock-odin:50050".to_string());
    let (host, port) = parse_odin_url(&url);

    let mut integration = match OdinServiceIntegration::new(&host, port).await {
        Ok(i) => i,
        Err(_) => {
            eprintln!("E2E skip: Odin unreachable at {}:{} (run in container with mock-odin)", host, port);
            return;
        }
    };

    let response = integration.send_chat("e2e").await.expect("send_chat");
    assert!(
        !response.is_empty(),
        "Odin response must be non-empty"
    );
    assert!(
        response.contains("e2e") || response.contains("mock"),
        "Response should contain echo or mock; got: {}",
        response
    );
}
