// Tests for gRPC server setup and health check service.

#[cfg(test)]
mod tests {
    use tonic::transport::Server;
    use tonic_health::server::health_reporter;
    use tonic_health::ServingStatus;

    #[tokio::test]
    async fn health_service_can_be_created_and_set_serving() {
        let (mut reporter, health_service) = health_reporter();
        reporter
            .set_service_status("", ServingStatus::Serving)
            .await;
        // Server builder accepts health service (validates type and setup)
        let _builder = Server::builder().add_service(health_service);
    }
}
