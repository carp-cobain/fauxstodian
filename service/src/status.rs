use crate::driver::SolanaDriver;
use std::sync::Arc;
use tokio::time::{self, Duration};
use tonic_health::{
    server::HealthReporter,
    ServingStatus::{NotServing, Serving},
};

/// Status health check for the gRPC server
pub async fn health_check(mut reporter: HealthReporter, driver: Arc<Box<dyn SolanaDriver>>) {
    log::info!("Starting health check");
    loop {
        time::sleep(Duration::from_secs(10)).await;
        match driver.health_check().await {
            Ok(_) => reporter.set_service_status("", Serving).await,
            Err(err) => {
                log::error!("Health check failure: {}", err.to_string());
                reporter.set_service_status("", NotServing).await;
            }
        }
    }
}
