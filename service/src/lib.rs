pub mod proto {
    tonic::include_proto!("fauxstodian");
}

pub mod api;
pub mod config;
pub mod driver;
pub mod entity;
pub mod service;
