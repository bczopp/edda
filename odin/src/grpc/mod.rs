pub mod odin {
    tonic::include_proto!("odin");
}
pub mod server;

pub use server::*;
