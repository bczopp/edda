use tracing::info;
use std::path::PathBuf;
use ragnarok::utils::config::SettingsManager;
use ragnarok::services::OdinServiceIntegration;
use ragnarok::cli::parse_args;
use ragnarok::grpc_client::{ThorClient, thor, GeriClient, FrekiClient, HuginnClient, MuninnClient, SkuldClient};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    info!("Ragnarok Terminal Platform starting...");

    // Load configuration
    let config_path = PathBuf::from("config/ragnarok.json");
    let settings_manager = SettingsManager::new(config_path.clone());
    settings_manager.load().await?;
    settings_manager.start_hot_reload()?;
    
    let settings = settings_manager.get().await;
    info!("Configuration loaded: Odin port={}", settings.odin.port);

    // Setup Gladsheim (Official Service Manager)
    let gladsheim = gladsheim::Gladsheim::new()?;
    let gladsheim_port = settings.gladsheim.port;
    
    // Start Gladsheim gRPC server in background
    tokio::spawn(async move {
        let addr = format!("127.0.0.1:{}", gladsheim_port).parse().unwrap();
        if let Err(e) = gladsheim::grpc::server::run_server(addr).await {
            tracing::error!("Gladsheim server error: {}", e);
        }
    });
    info!("Gladsheim service manager started on port {}", gladsheim_port);

    // Register services with Gladsheim (replacing custom lifecycle manager)
    // In a real scenario, Gladsheim would manage the actual processes.
    // Here we register them for health tracking and status reporting.
    let mut skirnir = gladsheim.skirnir().clone();
    skirnir.register_service("Odin".to_string()).await?;
    if settings.thor.is_some() { skirnir.register_service("Thor".to_string()).await?; }
    if settings.geri.is_some() { skirnir.register_service("Geri".to_string()).await?; }
    if settings.freki.is_some() { skirnir.register_service("Freki".to_string()).await?; }
    if settings.huginn.is_some() { skirnir.register_service("Huginn".to_string()).await?; }
    if settings.muninn.is_some() { skirnir.register_service("Muninn".to_string()).await?; }
    if settings.skuld.is_some() { skirnir.register_service("Skuld".to_string()).await?; }

    // Parse CLI arguments
    let cli = parse_args();

    // Odin service integration (Phase 4)
    let mut odin_integration = OdinServiceIntegration::new(settings.odin.port).await?;
    info!("Odin service integration initialized");

    // Execute command
    if let Some(command) = cli.command {
        match command {
            ragnarok::cli::Commands::Chat { message } => {
                if let Some(msg) = message {
                    let response = odin_integration.send_chat(&msg).await?;
                    println!("{}", response.response);
                    if !response.actions_taken.is_empty() {
                        println!("\nActions taken:");
                        for action in response.actions_taken {
                            println!("- {}", action);
                        }
                    }
                } else {
                    println!("Entering interactive chat (Odin). Type 'exit' or 'quit' to leave.");
                    use std::io::{self, Write};
                    loop {
                        print!("> ");
                        io::stdout().flush()?;
                        
                        let mut input = String::new();
                        io::stdin().read_line(&mut input)?;
                        let input = input.trim();
                        
                        if input.eq_ignore_ascii_case("exit") || input.eq_ignore_ascii_case("quit") {
                            break;
                        }
                        
                        if input.is_empty() {
                            continue;
                        }
                        
                        match odin_integration.send_chat(input).await {
                            Ok(response) => {
                                println!("\n{}\n", response.response);
                                if !response.actions_taken.is_empty() {
                                    println!("Actions taken:");
                                    for action in response.actions_taken {
                                        println!("  - {}", action);
                                    }
                                    println!();
                                }
                            }
                            Err(e) => eprintln!("Odin error: {}", e),
                        }
                    }
                }
            }
            ragnarok::cli::Commands::Action { action } => {
                if let Some(ref thor_cfg) = settings.thor {
                    match ThorClient::new(thor_cfg.port).await {
                        Ok(mut thor_client) => {
                            let action_id = Uuid::new_v4().to_string();
                            let action_data = serde_json::to_vec(&serde_json::json!({ "command": action }))
                                .unwrap_or_default();
                            let thor_action = thor::ThorAction {
                                action_id: action_id.clone(),
                                action_type: "SYSTEM_COMMAND".to_string(),
                                device_id: String::new(),
                                user_id: String::new(),
                                action_data,
                                metadata: std::collections::HashMap::new(),
                            };
                            match thor_client.execute_action(thor_action).await {
                                Ok(result) => {
                                    if result.success {
                                        let out = String::from_utf8_lossy(&result.result_data);
                                        if !out.is_empty() {
                                            println!("{}", out);
                                        } else {
                                            println!("Action completed.");
                                        }
                                    } else {
                                        eprintln!("Action failed: {}", result.error_message);
                                    }
                                }
                                Err(e) => eprintln!("Thor error: {}", e),
                            }
                        }
                        Err(e) => eprintln!("Thor not reachable (port {}): {}", thor_cfg.port, e),
                    }
                } else {
                    println!("Executing action: {} (Thor not configured; add 'thor' to config to run via Thor)", action);
                }
            }
            ragnarok::cli::Commands::Prompt { prompt } => {
                if let Some(ref geri_cfg) = settings.geri {
                    match GeriClient::new(geri_cfg.port).await {
                        Ok(mut geri_client) => {
                            let request = ragnarok::grpc_client::geri::ProcessPromptRequest {
                                prompt: prompt.clone(),
                                context: String::new(),
                                model_name: String::new(),
                                max_tokens: 1024,
                            };
                            match geri_client.process_prompt(request).await {
                                Ok(resp) => println!("{}", resp.text),
                                Err(e) => eprintln!("Geri error: {}", e),
                            }
                        }
                        Err(e) => eprintln!("Geri not reachable (port {}): {}", geri_cfg.port, e),
                    }
                } else {
                    println!("Geri not configured; add 'geri' (port) to config to use direct prompt.");
                }
            }
            ragnarok::cli::Commands::Models => {
                if let Some(ref geri_cfg) = settings.geri {
                    match GeriClient::new(geri_cfg.port).await {
                        Ok(mut geri_client) => {
                            let request = ragnarok::grpc_client::geri::ListModelsRequest {
                                model_type: String::new(),
                                provider: String::new(),
                            };
                            match geri_client.list_models(request).await {
                                Ok(resp) => {
                                    for m in &resp.models {
                                        println!("{} ({}): {}", m.name, m.provider, m.model_type);
                                    }
                                    if resp.models.is_empty() {
                                        println!("No models returned.");
                                    }
                                }
                                Err(e) => eprintln!("Geri error: {}", e),
                            }
                        }
                        Err(e) => eprintln!("Geri not reachable (port {}): {}", geri_cfg.port, e),
                    }
                } else {
                    println!("Geri not configured; add 'geri' to config to list models.");
                }
            }
            ragnarok::cli::Commands::Transcribe { file } => {
                if let Some(ref huginn_cfg) = settings.huginn {
                    match HuginnClient::new(huginn_cfg.port).await {
                        Ok(mut huginn_client) => {
                            let audio_data = std::fs::read(&file).unwrap_or_default();
                            let format = file
                                .extension()
                                .and_then(|e| e.to_str())
                                .unwrap_or("wav")
                                .to_string();
                            let request = ragnarok::grpc_client::huginn::TranscribeAudioRequest {
                                audio_data,
                                audio_format: format,
                                sample_rate: 0,
                                channels: 0,
                                language: String::new(),
                                user_id: String::new(),
                                device_id: String::new(),
                            };
                            match huginn_client.transcribe_audio(request).await {
                                Ok(resp) => {
                                    if resp.success {
                                        if let Some(ref msg) = resp.message {
                                            println!("{}", msg.content);
                                        } else {
                                            println!("(no text returned)");
                                        }
                                    } else {
                                        eprintln!("Transcription failed: {}", resp.error_message);
                                    }
                                }
                                Err(e) => eprintln!("Huginn error: {}", e),
                            }
                        }
                        Err(e) => eprintln!("Huginn not reachable (port {}): {}", huginn_cfg.port, e),
                    }
                } else {
                    println!("Huginn not configured; add 'huginn' (port) to config to transcribe audio.");
                }
            }
            ragnarok::cli::Commands::Speak { text } => {
                if let Some(ref muninn_cfg) = settings.muninn {
                    match MuninnClient::new(muninn_cfg.port).await {
                        Ok(mut muninn_client) => {
                            let request = ragnarok::grpc_client::muninn::TtsRequest {
                                text: text.clone(),
                                language: String::new(),
                                voice: 2, // NEUTRAL
                                settings: None,
                                user_id: String::new(),
                                device_id: String::new(),
                            };
                            match muninn_client.generate_speech(request).await {
                                Ok(resp) => {
                                    if resp.success {
                                        println!("Speech generated ({} bytes)", resp.audio_data.len());
                                    } else {
                                        eprintln!("TTS failed: {}", resp.error_message);
                                    }
                                }
                                Err(e) => eprintln!("Muninn error: {}", e),
                            }
                        }
                        Err(e) => eprintln!("Muninn not reachable (port {}): {}", muninn_cfg.port, e),
                    }
                } else {
                    println!("Muninn not configured; add 'muninn' (port) to config to generate speech.");
                }
            }
            ragnarok::cli::Commands::Retrieve { query: _query } => {
                if let Some(ref freki_cfg) = settings.freki {
                    match FrekiClient::new(freki_cfg.port).await {
                        Ok(mut freki_client) => {
                            let request = ragnarok::grpc_client::freki::RetrieveContextRequest {
                                query_embedding: vec![],
                                limit: 5,
                                collection_name: String::new(),
                            };
                            match freki_client.retrieve_context(request).await {
                                Ok(resp) => {
                                    for doc in &resp.documents {
                                        println!("--- {} (score: {}) ---", doc.id, doc.score);
                                        println!("{}", doc.content);
                                    }
                                    if resp.documents.is_empty() {
                                        println!("No documents returned (Freki may require query embeddings).");
                                    }
                                }
                                Err(e) => eprintln!("Freki error: {}", e),
                            }
                        }
                        Err(e) => eprintln!("Freki not reachable (port {}): {}", freki_cfg.port, e),
                    }
                } else {
                    println!("Freki not configured; add 'freki' to config to retrieve RAG context.");
                }
            }
            ragnarok::cli::Commands::Status => {
                println!("Ragnarok Terminal Platform - Status");
                println!("------------------------------------");
                println!("{:<10} {:<10} {:<10} {:<10} {:<15}", "Service", "Status", "Memory", "CPU", "Health");
                let services = gladsheim.skirnir().list_services().await;
                if services.is_empty() {
                    println!("No services registered.");
                } else {
                    for s in services {
                        let mem_mb = s.resource_usage.memory_bytes as f32 / (1024.0 * 1024.0);
                        let health_desc = if s.health.is_healthy { "HEALTHY" } else { "UNHEALTHY" };
                        println!("{:<10} [{:?}] {:>7.1}MB {:>5.1}% {:<15}", 
                            s.name, s.status, mem_mb, s.resource_usage.cpu_percent, health_desc);
                    }
                }
            }
            ragnarok::cli::Commands::Settings => {
                println!("Config path: {:?}", config_path);
                println!("Gladsheim Port: {}", settings.gladsheim.port);
                println!("Odin Port: {}", settings.odin.port);
                if let Some(ref t) = settings.thor { println!("Thor Port: {}", t.port); }
                if let Some(ref g) = settings.geri { println!("Geri Port: {}", g.port); }
                if let Some(ref f) = settings.freki { println!("Freki Port: {}", f.port); }
                if let Some(ref h) = settings.huginn { println!("Huginn Port: {}", h.port); }
                if let Some(ref m) = settings.muninn { println!("Muninn Port: {}", m.port); }
                if let Some(ref s) = settings.skuld { println!("Skuld Port: {}", s.port); }
            }
            ragnarok::cli::Commands::Tui => {
                let state = ragnarok::tui::TuiState::default();
                state.add_status_line(format!("Odin Port: {}", settings.odin.port));
                state.set_odin_connected(true);
                state.add_chat_message("system", "TUI started. Press 'q' (when input empty) or Ctrl+Q/Esc to quit.");
                let mut skirnir_clone = gladsheim.skirnir().clone();
                let manager = ragnarok::tui::TuiManager::new(state, odin_integration, skirnir_clone);
                manager.run().await?;
            }
        }
    } else {
        println!("Ragnarok Terminal Platform - Use --help for usage");
    }

    Ok(())
}
