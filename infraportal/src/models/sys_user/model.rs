use crate::graphql::context::{GraphQLContext, MyToken};
use async_graphql::{ComplexObject, Context, FieldResult, SimpleObject};
use query_tiberius_derive::Queryable;
#[derive(Debug, SimpleObject, Clone, Queryable)]
#[graphql(complex)]
pub struct SysUser {
    pub sys_id: String,
    pub name: Option<String>,
    pub employee_number: Option<String>,
    pub user_name: Option<String>,
    pub email: Option<String>,
    pub dv_department: Option<String>,
    pub u_cbt_: Option<String>,
    pub dv_u_business_head_name: Option<String>,
    pub u_business_head: Option<String>,
    pub dv_location: Option<String>,
    pub title: Option<String>,
    pub u_display_name: Option<String>,
    pub phone: Option<String>,
    pub mobile_phone: Option<String>,
}

#[ComplexObject]
impl SysUser {
    async fn is_cbt(&self) -> bool {
        if let Some(u_cbt) = &self.u_cbt_ {
            if u_cbt == &String::from("true") {
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    async fn is_admin(&self, ctx: &Context<'_>) -> FieldResult<bool> {
        let conn = ctx.data::<GraphQLContext>()?.sn_mirror_connection.clone();
        let mut conn = conn.get().await?;
        let user_name = ctx
            .data_opt::<MyToken>()
            .map(|token| token.0.as_str())
            .expect("Not logged in");
        let result = Self::check_is_admin(&mut conn, user_name).await?;

        if result.len() > 0 {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    async fn is_impersonated_user_admin(&self, ctx: &Context<'_>) -> FieldResult<bool> {
        let conn = ctx.data::<GraphQLContext>()?.sn_mirror_connection.clone();
        let mut conn = conn.get().await?;
        let user_name = ctx.data_opt::<MyToken>().map(|token| token.1.as_str());

        if let Some(user_name) = user_name {
            let result = Self::check_is_admin(&mut conn, user_name).await?;

            if result.len() > 0 {
                Ok(true)
            } else {
                Ok(false)
            }
        } else {
            Ok(false)
        }
    }

    async fn is_support(&self, ctx: &Context<'_>) -> FieldResult<bool> {
        let conn = ctx.data::<GraphQLContext>()?.sn_mirror_connection.clone();
        let mut conn = conn.get().await?;
        let user_name = ctx
            .data_opt::<MyToken>()
            .map(|token| token.0.as_str())
            .expect("Not logged in");
        let result = Self::check_is_support(&mut conn, user_name).await?;

        if result.len() > 0 {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    async fn is_impersonated_user_support(&self, ctx: &Context<'_>) -> FieldResult<bool> {
        let conn = ctx.data::<GraphQLContext>()?.sn_mirror_connection.clone();
        let mut conn = conn.get().await?;
        let user_name = ctx.data_opt::<MyToken>().map(|token| token.1.as_str());

        if let Some(user_name) = user_name {
            let result = Self::check_is_support(&mut conn, user_name).await?;

            if result.len() > 0 {
                Ok(true)
            } else {
                Ok(false)
            }
        } else {
            Ok(false)
        }
    }

    async fn is_manager(&self, ctx: &Context<'_>) -> FieldResult<bool> {
        let conn = ctx.data::<GraphQLContext>()?.sn_mirror_connection.clone();
        let mut conn = conn.get().await?;
        let user_name = self.sys_id.as_str();
        let result = Self::check_is_manager(&mut conn, user_name).await?;

        if result.len() > 0 {
            Ok(true)
        } else {
            Ok(false)
        }
    }
    async fn is_impersonated_user(&self, ctx: &Context<'_>) -> FieldResult<bool> {
        let user_name = ctx.data_opt::<MyToken>().map(|token| token.1.as_str());

        if let Some(user_name) = user_name {
            if user_name.len() > 0 {
                Ok(true)
            } else {
                Ok(false)
            }
        } else {
            Ok(false)
        }
    }

    async fn original_user(&self, ctx: &Context<'_>) -> FieldResult<Option<String>> {
        Ok(ctx.data_opt::<MyToken>().map(|token| token.0.clone()))
    }
}

#[derive(Debug, SimpleObject, Queryable)]
pub struct SysUserIsAdmin {
    pub u_members: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, SimpleObject, Queryable)]
pub struct SysUserIsManager {
    pub manager: Option<String>,
    pub dv_manager: Option<String>,
}
