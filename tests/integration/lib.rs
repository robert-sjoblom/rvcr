use std::sync::{Arc, LazyLock};

use tokio::sync::Mutex;
use tracing_subscriber::{
    Layer, filter, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt,
};

mod e2e;
mod modifiers;

#[derive(Clone)]
pub struct TestScope {
    pub initialized: Arc<Mutex<bool>>,
}

impl Default for TestScope {
    fn default() -> Self {
        Self {
            initialized: Arc::new(Mutex::new(false)),
        }
    }
}

impl TestScope {
    pub async fn init(self) {
        let mut inited = self.initialized.lock().await;
        if !(*inited) {
            if std::env::var("TEST_LOG").is_ok() {
                let stdout_log = tracing_subscriber::fmt::layer().pretty();
                tracing_subscriber::registry()
                    .with(
                        stdout_log
                            // Add an `INFO` filter to the stdout logging layer
                            .with_filter(filter::LevelFilter::DEBUG),
                    )
                    .init();
            }
            *inited = true;
        }
    }
}

static SCOPE: LazyLock<TestScope> = LazyLock::new(TestScope::default);
static ADDRESS: LazyLock<String> = LazyLock::new(|| String::from("http://127.0.0.1:38282"));
