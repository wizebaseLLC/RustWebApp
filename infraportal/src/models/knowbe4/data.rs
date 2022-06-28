use super::{constants::*, model::*};
use crate::{db::TiberiusConnectionManager, graphql::context::GraphQLContext};
use async_graphql::Context;
use bb8::PooledConnection;
use query_tiberius::query_tiberius::*;

impl KnowBe4Test {
    pub async fn find<'a>(
        conn: &mut PooledConnection<'a, TiberiusConnectionManager>,
        id: Vec<&str>,
    ) -> anyhow::Result<Vec<Self>> {
        let stream = conn
            .query(
                QUERY_KNOWBE4_TESTS,
                &[&id[0], &id[1], &id[2], &id[3], &id[4], &id[5]],
            )
            .await?;
        Self::build_result_many(stream).await
    }

    pub async fn list<'a>(
        conn: &mut PooledConnection<'a, TiberiusConnectionManager>,
    ) -> anyhow::Result<Vec<Self>> {
        let stream = conn.query(LIST_KNOWBE4_TESTS, &[]).await?;
        Self::build_result_many(stream).await
    }
}

impl KnowBe4Results {
    pub async fn find<'a>(
        conn: &mut PooledConnection<'a, TiberiusConnectionManager>,
        id: &str,
    ) -> anyhow::Result<Vec<Self>> {
        let stream = conn.query(QUERY_KNOWBE4_RESULTS, &[&id]).await?;
        Self::build_result_many(stream).await
    }

    pub async fn list<'a>(
        conn: &mut PooledConnection<'a, TiberiusConnectionManager>,
    ) -> anyhow::Result<Vec<Self>> {
        let stream = conn.query(LIST_KNOWBE4_RESULTS, &[]).await?;
        Self::build_result_many(stream).await
    }
}

impl KnowBe4Training {
    pub async fn list<'a>(
        conn: &mut PooledConnection<'a, TiberiusConnectionManager>,
    ) -> anyhow::Result<Vec<Self>> {
        let stream = conn.query(LIST_KNOWBE4_TRAINING, &[]).await?;
        Self::build_result_many(stream).await
    }
}

impl KnowBe4ReportedPhishing {
    pub async fn list<'a>(
        conn: &mut PooledConnection<'a, TiberiusConnectionManager>,
    ) -> anyhow::Result<Vec<Self>> {
        let stream = conn
            .query(QUERY_KNOWBE4_SUBMITTED_PHISHING_EMAIL, &[])
            .await?;
        Self::build_result_many(stream).await
    }
}

impl KnowBe4Users {
    pub async fn find_by_manager_email<'a>(
        ctx: &Context<'_>,
        manager_email: &str,
    ) -> anyhow::Result<Vec<Self>> {
        let conn = ctx
            .data::<GraphQLContext>()
            .map_err(|err| anyhow::Error::msg(err.message))?
            .infraportal_connection
            .clone();
        let mut conn = conn.get().await?;

        let stream = conn
            .query(QUERY_KNOWBE4_USERS_THAT_WORK_FOR_ME, &[&manager_email])
            .await?;

        Self::build_result_many(stream).await
    }

    pub async fn list<'a>(
        conn: &mut PooledConnection<'a, TiberiusConnectionManager>,
    ) -> anyhow::Result<Vec<Self>> {
        let stream = conn.query(LIST_KNOWBE4_USERS, &[]).await?;
        Self::build_result_many(stream).await
    }

    // Gets a list of users that reports to each manager in the list.
    pub async fn find_users_that_works_for_us<'a>(
        ctx: &Context<'_>,
        manager_emails: Vec<&str>,
    ) -> anyhow::Result<Vec<Self>> {
        // Store futures in a clean vec
        let mut futures = vec![];

        // Push each future into the vec
        for i in manager_emails {
            futures.push(Self::find_by_manager_email(ctx, i));
        }

        // Concurrently run all futures, then collect the results
        let results = futures::future::try_join_all(futures).await?;
        let results = results.into_iter().flatten().collect();
        Ok(results)
    }
}
