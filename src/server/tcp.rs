use std::sync::Arc;

use crate::resolver;

pub async fn handle(buf: &[u8], res: Arc<impl resolver::ToResolver>) {}
