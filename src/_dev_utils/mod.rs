mod dev_db;

use tokio::sync::OnceCell;
use tracing::info;

/// initialize environment for local dev
/// (for early dev only, called from main())
pub async fn init_dev() {
    /// OnceLock doesn't work for async closures.
    /// We need OnceCell instead
    static INIT: OnceCell<()> = OnceCell::const_new();

    INIT.get_or_init(|| async {
        info!("{:<12} - init_dev_all()", "FOR-DEV-ONLY");

        dev_db::init_dev_db().await.unwrap();
    })
        .await;
}