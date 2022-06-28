use super::{constants::*, model::*};
use crate::db::TiberiusConnectionManager;
use bb8::PooledConnection;
use query_tiberius::query_tiberius::*;

impl SysUser {
    pub async fn find<'a>(
        conn: &mut PooledConnection<'a, TiberiusConnectionManager>,
        user_name: &str,
    ) -> anyhow::Result<Self> {
        let stream = conn.query(QUERY_FIND_USER, &[&user_name]).await?;
        Self::build_result(stream).await
    }

    pub async fn find_many<'a>(
        conn: &mut PooledConnection<'a, TiberiusConnectionManager>,
    ) -> anyhow::Result<Vec<Self>> {
        let stream = conn.query(QUERY_FIND_USER_MANY, &[]).await?;
        Self::build_result_many(stream).await
    }

    pub async fn find_me<'a>(
        conn: &mut PooledConnection<'a, TiberiusConnectionManager>,
        user_name: (&str, &str),
    ) -> anyhow::Result<Self> {
        let stream = conn
            .query(
                QUERY_FIND_USER,
                &[{
                    if user_name.1.len() > 1 {
                        &user_name.1
                    } else {
                        &user_name.0
                    }
                }],
            )
            .await?;
        Self::build_result(stream).await
    }

    pub async fn check_is_admin<'a>(
        conn: &mut PooledConnection<'a, TiberiusConnectionManager>,
        user_name: &str,
    ) -> anyhow::Result<Vec<SysUserIsAdmin>> {
        let stream = conn.query(QUERY_IS_ADMIN, &[&user_name]).await?;
        SysUserIsAdmin::build_result_many(stream).await
    }

    pub async fn check_is_manager<'a>(
        conn: &mut PooledConnection<'a, TiberiusConnectionManager>,
        user_name: &str,
    ) -> anyhow::Result<Vec<SysUserIsManager>> {
        let stream = conn.query(QUERY_IS_MANAGER, &[&user_name]).await?;
        SysUserIsManager::build_result_many(stream).await
    }

    pub async fn check_is_support<'a>(
        conn: &mut PooledConnection<'a, TiberiusConnectionManager>,
        user_name: &str,
    ) -> anyhow::Result<Vec<SysUserIsAdmin>> {
        let stream = conn.query(QUERY_IS_SUPPORT, &[&user_name]).await?;
        SysUserIsAdmin::build_result_many(stream).await
    }

    pub async fn find_manager_from_business_head<'a>(
        conn: &mut PooledConnection<'a, TiberiusConnectionManager>,
        display_name: &str,
    ) -> anyhow::Result<Vec<Self>> {
        let stream = conn
            .query(
                {
                    if display_name.to_lowercase() == "walker, george" {
                        QUERY_FIND_MANAGERS_FROM_BUSINESS_HEAD_IF_GEORGE_WALKER
                    } else {
                        QUERY_FIND_MANAGERS_FROM_BUSINESS_HEAD
                    }
                },
                &[&display_name],
            )
            .await?;
        Self::build_result_many(stream).await
    }
}
