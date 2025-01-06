use loco_rs::app::AppContext;
use loco_rs::prelude::*;
use loco_rs::storage::{self, Storage};

async fn after_context(ctx: AppContext) -> Result<AppContext> {
    Ok(AppContext {
        storage: Storage::single(storage::drivers::mem::new()).into(),
        ..ctx
    })
}
