#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate dotenv_codegen;

use anyhow::Error;
use domain::sonar::QualityStatus;
use infra::sonar_client::SonarClient;
use tokio::runtime::Runtime;

pub mod domain;
pub mod infra;

pub fn yolo(analysis_id: &str) -> Result<QualityStatus, Error> {
    let sonar_client = SonarClient::new(dotenv!("SONAR_URL"), dotenv!("SONAR_TOKEN"));

    let mut rt = Runtime::new().expect("tokio runtime can be initialized");
    rt.block_on(async move { sonar_client.quality_gate_status(analysis_id).await })
}
