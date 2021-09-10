use std::time::{Duration as TimeDuration};
use scylla::speculative_execution::PercentileSpeculativeExecutionPolicy;
use scylla::load_balancing::RoundRobinPolicy;
use scylla::load_balancing::TokenAwarePolicy;
use scylla::transport::errors::NewSessionError;
use std::sync::Arc;

use anyhow::Result;
use scylla::transport::session::Session;
use scylla::SessionBuilder;

const KS_NAME: &str = "user_data";
const USER_TAB_NAME: &str = "user_info";

// const USER:&str = "user";
// const PASSWORD: &str = "password";
const ADDRESS: &str = "cassandra:9042";

pub async fn create_session() -> Result<Arc<Session>,NewSessionError> {
    //let uri = env::var("cassandra:9042").unwrap_or_else(|_| "127.0.0.1:9042".to_string());

    println!("Connecting to {} ...", ADDRESS);

    let robin = Box::new(RoundRobinPolicy::new());
    //let dc_robin = Box::new(DcAwareRoundRobinPolicy::new(local_dc));
    let load_balance_policy = Arc::new(TokenAwarePolicy::new(robin));

    let execution_policy = PercentileSpeculativeExecutionPolicy  {
        max_retry_count: 3,
        percentile: 99.0,
    };

    let session: Session = SessionBuilder::new()
        .known_node(ADDRESS)
        // .known_node("cassandra-0.cassandra.cass.svc.cluster.local")
        // .known_node("cassandra-1.cassandra.cass.svc.cluster.local")
        // .known_node("cassandra-2.cassandra.cass.svc.cluster.local")
        //.known_node("localhost")
        .connection_timeout(TimeDuration::from_secs(3))
        .load_balancing(load_balance_policy)
        .speculative_execution(Arc::new(execution_policy))
        .build().await?;
    Ok(Arc::new(session))
}