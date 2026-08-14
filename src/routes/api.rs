use axum::{Json, Router, routing::{get}};
use serde::Deserialize;

use crate::{
    app::AppState, auth::admin::Admin, error::AppError, models::Asset, repository::Repository,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/assets", get(list_assets).post(create_asset).patch(update_asset))
        .route("/assets/{id}", get(get_asset_by_id).delete(delete_asset))
}

#[tracing::instrument(skip_all)]
async fn list_assets(repostiory: Repository) -> Result<Json<Vec<Asset>>, AppError> {
    let assets = repostiory.list_assets().await?;
    Ok(Json(assets))
}

#[tracing::instrument(skip_all)]
async fn get_asset_by_id(
    repostiory: Repository,
    axum::extract::Path(asset_id): axum::extract::Path<i64>,
) -> Result<Json<Asset>, AppError> {
    match repostiory.get_asset_by_id(asset_id).await? {
        Some(asset) => Ok(Json(asset)),
        None => Err(AppError::AssetDoesNotExist(asset_id)),
    }
}

#[derive(Deserialize)]
struct CreateAssetRequest {
    name: String,
    unit_value: f64,
}

#[tracing::instrument(skip_all)]
async fn create_asset(
    _: Admin,
    repostiory: Repository,
    Json(request): Json<CreateAssetRequest>,
) -> Result<Json<Asset>, AppError> {
    let new_asset = repostiory
        .create_asset(request.name, request.unit_value)
        .await?;

    Ok(Json(new_asset))
}

#[derive(Deserialize)]
struct UpdateAssetRequest {
    id: i64,
    name: Option<String>,
    unit_value: Option<f64>,
}

#[tracing::instrument(skip_all)]
async fn update_asset(
    _: Admin,
    repostiory: Repository,
    Json(request): Json<UpdateAssetRequest>,
) -> Result<Json<Asset>, AppError> {
    match repostiory
        .update_asset(request.id, request.name, request.unit_value)
        .await?
    {
        Some(updated_asset) => Ok(Json(updated_asset)),
        None => Err(AppError::AssetDoesNotExist(request.id)),
    }
}

#[tracing::instrument(skip_all)]
async fn delete_asset(
    _: Admin,
    repostiory: Repository,
    axum::extract::Path(asset_id): axum::extract::Path<i64>,
) -> Result<Json<Asset>, AppError> {
    match repostiory.delete_asset(asset_id).await? {
        Some(asset) => Ok(Json(asset)),
        None => Err(AppError::AssetDoesNotExist(asset_id)),
    }
}

#[cfg(test)]
mod tests {
    use sqlx::PgPool;

    use super::*;

    #[sqlx::test]
    async fn test_create_asset(db: PgPool) {
        let request = CreateAssetRequest {
            name: "Bitcoin".to_string(),
            unit_value: 10.0,
        };
        let Json(new_asset) = create_asset(Admin, db.into(), Json(request))
            .await
            .expect("success");

        assert_eq!(new_asset.id, 1);
        assert_eq!(new_asset.name, "Bitcoin");
        assert_eq!(new_asset.unit_value, 10.0);

        insta::assert_json_snapshot!(new_asset);
    }

    #[sqlx::test(fixtures("bitcoin_asset"))]
    async fn test_list_assets(db: PgPool) {
        let Json(assets) = list_assets(db.into()).await.expect("success");

        assert_eq!(assets.len(), 1);
        assert_eq!(assets[0].name, "Bitcoin");

        insta::assert_json_snapshot!(assets);
    }

    #[sqlx::test(fixtures("bitcoin_asset"))]
    async fn test_update_asset(db: PgPool) {
        let request = UpdateAssetRequest {
            id: 1,
            name: Some("Ethereum".to_string()),
            unit_value: Some(20.0),
        };

        let Json(updated_asset) = update_asset(Admin, db.into(), Json(request))
            .await
            .expect("success");

        assert_eq!(updated_asset.id, 1);
        assert_eq!(updated_asset.name, "Ethereum");
        assert_eq!(updated_asset.unit_value, 20.0);

        insta::assert_json_snapshot!(updated_asset);
    }

    #[sqlx::test(fixtures("bitcoin_asset"))]
    async fn test_delete_asset(db: PgPool) {
        let deleted_asset = delete_asset(Admin, db.into(), axum::extract::Path(1))
            .await
            .expect("success");

        assert_eq!(deleted_asset.id, 1);
        assert_eq!(deleted_asset.name, "Bitcoin");
        assert_eq!(deleted_asset.unit_value, 10.0);
    }
}
