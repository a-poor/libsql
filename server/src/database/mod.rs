use crate::query::{QueryResult, Value};
use crate::query_analysis::Statement;

pub mod libsql;
pub mod service;
pub mod write_proxy;

const TXN_TIMEOUT_SECS: u64 = 5;

#[async_trait::async_trait]
pub trait Database {
    async fn execute(&self, query: Statement, params: Vec<Value>) -> QueryResult;
}