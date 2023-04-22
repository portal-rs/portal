use std::sync::Arc;

use portal_resolver::ToResolver;

pub async fn handle(buf: &[u8], res: Arc<impl ToResolver>) {}
