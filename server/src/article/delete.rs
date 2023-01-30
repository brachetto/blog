use std::sync::Arc;

use ntex::web::types::{Path, State};

use crate::{errors::CustomError, models::user::Admin, AppState};

/// 删除文章
/// 需要管理员权限
pub async fn delete_article(
    _: Admin,
    id: Path<(u32,)>,
    state: State<Arc<AppState>>,
) -> Result<String, CustomError> {
    let db_pool = &state.db_pool;

    sqlx::query!("DELETE FROM articles WHERE id = $1", id.0 as i32)
        .execute(db_pool)
        .await?;

    Ok("删除文章成功！".into())
}
