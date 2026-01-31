use tonic::{transport::Server, Request, Response, Status};
use tonic_health::server::health_reporter;
use tonic_health::ServingStatus;
use tracing::info;
use std::net::SocketAddr;
use std::sync::Arc;
use base64::{Engine as _, engine::general_purpose};

// Include generated protobuf code
pub mod authentication {
    tonic::include_proto!("heimdall.authentication");
}

pub mod authorization {
    tonic::include_proto!("heimdall.authorization");
}

pub mod token {
    tonic::include_proto!("heimdall.token");
}

pub mod bifrost_validation {
    tonic::include_proto!("heimdall.bifrost_validation");
}

pub mod mesh_membership {
    tonic::include_proto!("heimdall.mesh_membership");
}

use authentication::authentication_service_server::{AuthenticationService, AuthenticationServiceServer};
use authorization::authorization_service_server::{AuthorizationService, AuthorizationServiceServer};
use token::token_service_server::{TokenService, TokenServiceServer};
use bifrost_validation::bifrost_validation_service_server::{BifrostValidationService, BifrostValidationServiceServer};
use mesh_membership::mesh_membership_service_server::{MeshMembershipService, MeshMembershipServiceServer};

// Service implementations
pub struct AuthenticationServiceImpl {
    auth_manager: Arc<crate::auth::AuthenticationManager>,
    signing_keypair: Arc<ring::signature::Ed25519KeyPair>,
}

impl AuthenticationServiceImpl {
    pub fn new(
        auth_manager: Arc<crate::auth::AuthenticationManager>,
        signing_keypair: Arc<ring::signature::Ed25519KeyPair>,
    ) -> Self {
        Self {
            auth_manager,
            signing_keypair,
        }
    }
}

#[tonic::async_trait]
impl AuthenticationService for AuthenticationServiceImpl {
    async fn request_challenge(
        &self,
        request: Request<authentication::ChallengeRequest>,
    ) -> Result<Response<authentication::ChallengeResponse>, Status> {
        let req = request.into_inner();
        
        // Generate challenge
        let (challenge, expires_at) = self.auth_manager
            .request_challenge(
                &req.device_id,
                &req.public_key,
                &req.signature,
            )
            .await
            .map_err(|e| Status::internal(format!("Challenge generation failed: {}", e)))?;

        // Sign challenge response
        let challenge_data = format!("{}.{}", challenge, expires_at);
        let signature = crate::keys::SignatureManager::sign(
            &self.signing_keypair,
            challenge_data.as_bytes(),
        ).map_err(|e| Status::internal(format!("Signature generation failed: {}", e)))?;

        let response = authentication::ChallengeResponse {
            challenge,
            timestamp: chrono::Utc::now().timestamp(),
            expires_in: expires_at - chrono::Utc::now().timestamp(),
            signature,
        };

        Ok(Response::new(response))
    }

    async fn prove_identity(
        &self,
        request: Request<authentication::ProofRequest>,
    ) -> Result<Response<authentication::AuthenticationTokenResponse>, Status> {
        let req = request.into_inner();
        
        // Validate proof and generate tokens
        let (heimdall_token, token_id, expires_at, refresh_token, refresh_expires_at, permissions) = self.auth_manager
            .prove_identity(&req.device_id, &req.challenge, &req.proof)
            .await
            .map_err(|e| Status::unauthenticated(format!("Proof validation failed: {}", e)))?;

        let response = authentication::AuthenticationTokenResponse {
            token: heimdall_token,
            token_id,
            expires_at,
            refresh_token,
            refresh_expires_at,
            permissions,
        };

        Ok(Response::new(response))
    }

    async fn generate_token(
        &self,
        _request: Request<authentication::TokenGenerationRequest>,
    ) -> Result<Response<authentication::AuthenticationTokenResponse>, Status> {
        // This is a convenience method - for now, return error
        // Token generation should go through prove_identity
        Err(Status::unimplemented("Use prove_identity instead"))
    }
}

pub struct AuthorizationServiceImpl {
    permission_manager: Arc<crate::authz::PermissionManager>,
}

impl AuthorizationServiceImpl {
    pub fn new(permission_manager: Arc<crate::authz::PermissionManager>) -> Self {
        Self { permission_manager }
    }
}

#[tonic::async_trait]
impl AuthorizationService for AuthorizationServiceImpl {
    async fn check_permission(
        &self,
        request: Request<authorization::PermissionCheckRequest>,
    ) -> Result<Response<authorization::PermissionCheckResponse>, Status> {
        let req = request.into_inner();
        
        let user_id = uuid::Uuid::parse_str(&req.user_id)
            .map_err(|e| Status::invalid_argument(format!("Invalid user_id: {}", e)))?;

        let allowed = self.permission_manager
            .check_permission(
                &req.device_id,
                &user_id,
                &req.resource_type,
                &req.action,
            )
            .await
            .map_err(|e| Status::internal(format!("Permission check failed: {}", e)))?;

        let response = authorization::PermissionCheckResponse {
            allowed,
            reason: if allowed {
                "Permission granted".to_string()
            } else {
                "Permission denied".to_string()
            },
            granted_permissions: vec![], // Permissions are already in the response
        };

        Ok(Response::new(response))
    }

    async fn check_role(
        &self,
        request: Request<authorization::RoleCheckRequest>,
    ) -> Result<Response<authorization::RoleCheckResponse>, Status> {
        let req = request.into_inner();
        
        let has_role = self.permission_manager
            .check_role(&req.device_id, &req.role_name)
            .await
            .map_err(|e| Status::internal(format!("Role check failed: {}", e)))?;

        let all_roles = self.permission_manager
            .get_roles(&req.device_id)
            .await
            .map_err(|e| Status::internal(format!("Failed to get roles: {}", e)))?;

        let response = authorization::RoleCheckResponse {
            has_role,
            roles: all_roles,
        };

        Ok(Response::new(response))
    }
}

pub struct TokenServiceImpl {
    token_validator: Arc<crate::token::TokenValidator>,
    token_repo: Arc<crate::utils::TokenRepository>,
    token_generator: Arc<crate::token::TokenGenerator>,
}

impl TokenServiceImpl {
    pub fn new(
        token_validator: Arc<crate::token::TokenValidator>,
        token_repo: Arc<crate::utils::TokenRepository>,
        token_generator: Arc<crate::token::TokenGenerator>,
    ) -> Self {
        Self {
            token_validator,
            token_repo,
            token_generator,
        }
    }
}

#[tonic::async_trait]
impl TokenService for TokenServiceImpl {
    async fn validate_token(
        &self,
        request: Request<token::ValidateTokenRequest>,
    ) -> Result<Response<token::ValidateTokenResponse>, Status> {
        let req = request.into_inner();
        
        // Validate token signature and expiration
        let payload = self.token_validator
            .validate_token(&req.token)
            .await
            .map_err(|e| Status::unauthenticated(format!("Token validation failed: {}", e)))?;

        // Check if token is revoked in database
        let db_token = self.token_repo
            .get_by_token_id(&payload.token_id)
            .await
            .map_err(|_| Status::unauthenticated("Token not found"))?;

        if db_token.is_revoked {
            return Ok(Response::new(token::ValidateTokenResponse {
                valid: false,
                token_id: payload.token_id,
                device_id: payload.device_id,
                user_id: payload.user_id,
                expires_at: payload.expires_at,
                is_revoked: true,
                permissions: payload.permissions,
                reason: "Token has been revoked".to_string(),
            }));
        }

        // Optional: Check device binding
        if !req.device_id.is_empty() && req.device_id != payload.device_id {
            return Ok(Response::new(token::ValidateTokenResponse {
                valid: false,
                token_id: payload.token_id.clone(),
                device_id: payload.device_id.clone(),
                user_id: payload.user_id.clone(),
                expires_at: payload.expires_at,
                is_revoked: false,
                permissions: payload.permissions.clone(),
                reason: "Token device binding mismatch".to_string(),
            }));
        }

        Ok(Response::new(token::ValidateTokenResponse {
            valid: true,
            token_id: payload.token_id,
            device_id: payload.device_id,
            user_id: payload.user_id,
            expires_at: payload.expires_at,
            is_revoked: false,
            permissions: payload.permissions,
            reason: "Token is valid".to_string(),
        }))
    }

    async fn renew_token(
        &self,
        request: Request<token::RenewTokenRequest>,
    ) -> Result<Response<token::RenewTokenResponse>, Status> {
        let req = request.into_inner();
        
        // Validate refresh token
        let refresh_payload = self.token_validator
            .validate_token(&req.refresh_token)
            .await
            .map_err(|e| Status::unauthenticated(format!("Refresh token validation failed: {}", e)))?;

        if refresh_payload.token_type != "refresh" {
            return Err(Status::invalid_argument("Not a refresh token"));
        }

        // Check device binding
        if refresh_payload.device_id != req.device_id {
            return Err(Status::permission_denied("Device binding mismatch"));
        }

        // Generate new tokens
        let (new_token, new_token_id, new_expires_at) = self.token_generator
            .generate_heimdall_token(
                &refresh_payload.device_id,
                &refresh_payload.user_id,
                refresh_payload.permissions.clone(),
            )
            .map_err(|e| Status::internal(format!("Token generation failed: {}", e)))?;

        let (new_refresh_token, new_refresh_token_id, new_refresh_expires_at) = self.token_generator
            .generate_refresh_token(
                &refresh_payload.device_id,
                &refresh_payload.user_id,
            )
            .map_err(|e| Status::internal(format!("Refresh token generation failed: {}", e)))?;

        // Store new tokens in database
        use chrono::TimeZone;
        let expires_at_dt = chrono::Utc.timestamp_opt(new_expires_at, 0).unwrap();
        let refresh_expires_at_dt = chrono::Utc.timestamp_opt(new_refresh_expires_at, 0).unwrap();

        let device_id_uuid = uuid::Uuid::parse_str(&refresh_payload.device_id)
            .map_err(|e| Status::internal(format!("Invalid device_id: {}", e)))?;
        let user_id_uuid = uuid::Uuid::parse_str(&refresh_payload.user_id)
            .map_err(|e| Status::internal(format!("Invalid user_id: {}", e)))?;

        self.token_repo.create(
            &new_token_id,
            device_id_uuid,
            user_id_uuid,
            "heimdall",
            &new_token,
            expires_at_dt,
        ).await.map_err(|e| Status::internal(format!("Failed to store token: {}", e)))?;

        self.token_repo.create(
            &new_refresh_token_id,
            device_id_uuid,
            user_id_uuid,
            "refresh",
            &new_refresh_token,
            refresh_expires_at_dt,
        ).await.map_err(|e| Status::internal(format!("Failed to store refresh token: {}", e)))?;

        Ok(Response::new(token::RenewTokenResponse {
            token: new_token,
            token_id: new_token_id,
            expires_at: new_expires_at,
            refresh_token: new_refresh_token,
            refresh_expires_at: new_refresh_expires_at,
        }))
    }

    async fn revoke_token(
        &self,
        request: Request<token::RevokeTokenRequest>,
    ) -> Result<Response<token::RevokeTokenResponse>, Status> {
        let req = request.into_inner();
        
        self.token_repo
            .revoke(&req.token_id)
            .await
            .map_err(|e| Status::internal(format!("Token revocation failed: {}", e)))?;

        Ok(Response::new(token::RevokeTokenResponse {
            revoked: true,
            message: if req.reason.is_empty() { "Token revoked".to_string() } else { req.reason },
        }))
    }
}

pub struct BifrostValidationServiceImpl {
    connection_validator: Arc<crate::bifrost::ConnectionValidator>,
    signing_keypair: Arc<ring::signature::Ed25519KeyPair>,
    token_generator: Arc<crate::token::TokenGenerator>,
}

impl BifrostValidationServiceImpl {
    pub fn new(
        connection_validator: Arc<crate::bifrost::ConnectionValidator>,
        signing_keypair: Arc<ring::signature::Ed25519KeyPair>,
        token_generator: Arc<crate::token::TokenGenerator>,
    ) -> Self {
        Self {
            connection_validator,
            signing_keypair,
            token_generator,
        }
    }
}

#[tonic::async_trait]
impl BifrostValidationService for BifrostValidationServiceImpl {
    async fn validate_connection(
        &self,
        request: Request<bifrost_validation::ConnectionValidationRequest>,
    ) -> Result<Response<bifrost_validation::ConnectionValidationResponse>, Status> {
        let req = request.into_inner();
        
        // Validate connection
        let allowed = self.connection_validator
            .validate_connection(
                &req.source_device_id,
                &req.target_device_id,
                &req.connection_type,
            )
            .await
            .map_err(|e| Status::permission_denied(format!("Connection validation failed: {}", e)))?;

        if !allowed {
            return Ok(Response::new(bifrost_validation::ConnectionValidationResponse {
                allowed: false,
                reason: "Connection not allowed".to_string(),
                validation_token: String::new(),
                expires_at: 0,
                status: "BLOCKED".to_string(),
                signature: vec![],
            }));
        }

        // Generate validation token
        let (validation_token, token_id, expires_at) = self.token_generator
            .generate_session_token(
                &req.source_device_id,
                &req.source_user_id,
            )
            .map_err(|e| Status::internal(format!("Token generation failed: {}", e)))?;

        // Sign response
        let response_data = format!("{}.{}.{}", allowed, token_id, expires_at);
        let signature = crate::keys::SignatureManager::sign(
            &self.signing_keypair,
            response_data.as_bytes(),
        ).map_err(|e| Status::internal(format!("Signature generation failed: {}", e)))?;

        Ok(Response::new(bifrost_validation::ConnectionValidationResponse {
            allowed: true,
            reason: "Connection validated".to_string(),
            validation_token,
            expires_at,
            status: "ACTIVE".to_string(),
            signature,
        }))
    }

    async fn validate_message(
        &self,
        request: Request<bifrost_validation::MessageValidationRequest>,
    ) -> Result<Response<bifrost_validation::MessageValidationResponse>, Status> {
        let req = request.into_inner();
        
        // Validate connection token
        let token_validator = crate::token::TokenValidator::new(
            std::path::PathBuf::from("keys"),
        );
        
        let payload = token_validator
            .validate_token(&req.connection_token)
            .await
            .map_err(|e| Status::unauthenticated(format!("Invalid connection token: {}", e)))?;

        // Check if token is expired
        if payload.expires_at < chrono::Utc::now().timestamp() {
            return Ok(Response::new(bifrost_validation::MessageValidationResponse {
                valid: false,
                reason: "Connection token expired".to_string(),
            }));
        }

        // Validate message signature (basic check - signature should be valid)
        if req.signature.is_empty() {
            return Ok(Response::new(bifrost_validation::MessageValidationResponse {
                valid: false,
                reason: "Message signature missing".to_string(),
            }));
        }

        // Check timestamp (prevent replay attacks)
        let now = chrono::Utc::now().timestamp();
        let time_diff = (now - req.timestamp).abs();
        if time_diff > 300 { // 5 minutes tolerance
            return Ok(Response::new(bifrost_validation::MessageValidationResponse {
                valid: false,
                reason: "Message timestamp too old or too far in future".to_string(),
            }));
        }

        Ok(Response::new(bifrost_validation::MessageValidationResponse {
            valid: true,
            reason: "Message validated".to_string(),
        }))
    }
}

pub struct MeshMembershipServiceImpl {
    mesh_registry: Arc<crate::mesh::MeshDeviceRegistry>,
    token_generator: Arc<crate::token::TokenGenerator>,
    signing_keypair: Arc<ring::signature::Ed25519KeyPair>,
}

impl MeshMembershipServiceImpl {
    pub fn new(
        mesh_registry: Arc<crate::mesh::MeshDeviceRegistry>,
        token_generator: Arc<crate::token::TokenGenerator>,
        signing_keypair: Arc<ring::signature::Ed25519KeyPair>,
    ) -> Self {
        Self {
            mesh_registry,
            token_generator,
            signing_keypair,
        }
    }
}

#[tonic::async_trait]
impl MeshMembershipService for MeshMembershipServiceImpl {
    async fn register_device(
        &self,
        request: Request<mesh_membership::MeshMembershipRequest>,
    ) -> Result<Response<mesh_membership::MeshMembershipResponse>, Status> {
        let req = request.into_inner();
        
        let owner_user_id = uuid::Uuid::parse_str(&req.owner_user_id)
            .map_err(|e| Status::invalid_argument(format!("Invalid owner_user_id: {}", e)))?;

        let mesh_public_key_str = general_purpose::STANDARD.encode(&req.mesh_public_key);
        
        let (mesh_device, is_new) = self.mesh_registry
            .register_device(
                &req.device_id,
                &req.device_name,
                &req.device_type,
                &mesh_public_key_str,
                owner_user_id,
            )
            .await
            .map_err(|e| Status::internal(format!("Mesh membership registration failed: {}", e)))?;

        if is_new {
            crate::mesh::notify_owner_new_device(owner_user_id, &req.device_id);
        }

        let response = mesh_membership::MeshMembershipResponse {
            registered: true,
            requires_approval: is_new, // New devices require approval
            message: if is_new {
                "Device registered for mesh membership, approval required".to_string()
            } else {
                "Device already registered for mesh membership".to_string()
            },
            mesh_device_id: mesh_device.id.to_string(),
        };

        Ok(Response::new(response))
    }

    async fn generate_mesh_auth_token(
        &self,
        request: Request<mesh_membership::MeshAuthTokenRequest>,
    ) -> Result<Response<mesh_membership::MeshAuthTokenResponse>, Status> {
        let req = request.into_inner();
        
        // Get mesh device
        let mesh_device = self.mesh_registry
            .get_by_device_id(&req.device_id)
            .await
            .map_err(|e| Status::not_found(format!("Mesh device not found: {}", e)))?;

        if !mesh_device.is_active {
            return Err(Status::permission_denied("Mesh device is not active"));
        }

        // Update last seen
        self.mesh_registry.update_last_seen(&req.device_id).await
            .map_err(|e| Status::internal(format!("Failed to update last seen: {}", e)))?;

        // Generate mesh token (similar to session token)
        let device = self.mesh_registry.device_repo
            .get_by_device_id(&req.device_id)
            .await
            .map_err(|_| Status::not_found("Device not found"))?;

        let (mesh_token, _, expires_at) = self.token_generator
            .generate_session_token(
                &device.device_id,
                &device.user_id.to_string(),
            )
            .map_err(|e| Status::internal(format!("Mesh token generation failed: {}", e)))?;

        // Sign mesh token response
        let token_data = format!("{}.{}.{}", mesh_token, mesh_device.role, expires_at);
        let signature = crate::keys::SignatureManager::sign(
            &self.signing_keypair,
            token_data.as_bytes(),
        ).map_err(|e| Status::internal(format!("Signature generation failed: {}", e)))?;

        Ok(Response::new(mesh_membership::MeshAuthTokenResponse {
            mesh_token,
            role: mesh_device.role,
            expires_at,
            signature,
        }))
    }
}

pub struct GrpcServerDependencies {
    pub auth_manager: Arc<crate::auth::AuthenticationManager>,
    pub permission_manager: Arc<crate::authz::PermissionManager>,
    pub token_validator: Arc<crate::token::TokenValidator>,
    pub token_repo: Arc<crate::utils::TokenRepository>,
    pub token_generator: Arc<crate::token::TokenGenerator>,
    pub connection_validator: Arc<crate::bifrost::ConnectionValidator>,
    pub mesh_registry: Arc<crate::mesh::MeshDeviceRegistry>,
    pub signing_keypair: Arc<ring::signature::Ed25519KeyPair>,
}

pub async fn start_grpc_server(
    addr: SocketAddr,
    deps: GrpcServerDependencies,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting gRPC server on {}", addr);

    let auth_service = AuthenticationServiceImpl::new(
        deps.auth_manager.clone(),
        deps.signing_keypair.clone(),
    );
    let authz_service = AuthorizationServiceImpl::new(deps.permission_manager.clone());
    let token_service = TokenServiceImpl::new(
        deps.token_validator.clone(),
        deps.token_repo.clone(),
        deps.token_generator.clone(),
    );
    let bifrost_service = BifrostValidationServiceImpl::new(
        deps.connection_validator.clone(),
        deps.signing_keypair.clone(),
        deps.token_generator.clone(),
    );
    let mesh_service = MeshMembershipServiceImpl::new(
        deps.mesh_registry.clone(),
        deps.token_generator.clone(),
        deps.signing_keypair.clone(),
    );

    let (mut health_reporter, health_service) = health_reporter();
    health_reporter
        .set_service_status("", ServingStatus::Serving)
        .await;

    Server::builder()
        .add_service(health_service)
        .add_service(AuthenticationServiceServer::new(auth_service))
        .add_service(AuthorizationServiceServer::new(authz_service))
        .add_service(TokenServiceServer::new(token_service))
        .add_service(BifrostValidationServiceServer::new(bifrost_service))
        .add_service(MeshMembershipServiceServer::new(mesh_service))
        .serve(addr)
        .await?;

    Ok(())
}
