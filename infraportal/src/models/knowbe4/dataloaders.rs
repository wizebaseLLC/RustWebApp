use super::model::KnowBe4Results;
use crate::{
    db::TiberiusConnectionManager,
    helpers::requests::keys_to_sql::{
        build_dataloader_result, complete_dataloader_result, keys_to_sql_in_query,
    },
    models::knowbe4::model::KnowBe4ReportedPhishing,
};
use async_graphql::{dataloader::Loader, FieldError};
use async_trait::async_trait;
use bb8::Pool;
use chrono::NaiveDateTime;
use itertools::Itertools;
use std::collections::HashMap;

pub struct Knowbe4ResultsLoader(Pool<TiberiusConnectionManager>);

impl Knowbe4ResultsLoader {
    pub fn new(pool: Pool<TiberiusConnectionManager>) -> Self {
        Self(pool)
    }
}

#[async_trait]
impl Loader<i32> for Knowbe4ResultsLoader {
    type Value = Vec<KnowBe4Results>;
    type Error = FieldError;

    async fn load(&self, keys: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        let keys = keys.iter().map(|x| x.to_string()).collect::<Vec<String>>();
        let keys = keys.as_slice();
        let keys = keys_to_sql_in_query(keys);

        let query = format!(
            "
            SELECT
            CASE
            When (
                r.clicked_at is not null or
                r.replied_at is not null or
                r.attachment_opened_at is not null or
                r.macro_enabled_at is not null or
                r.data_entered_at is not null or
                r.exploited_at is not null    
                ) 
            THEN 'true' ELSE 'false' END as 'failed'
            ,  (
	case when (r.clicked_at is not null) then 1 else 0 end +
	case when (r.replied_at is not null) then 1 else 0 end +
	case when (r.attachment_opened_at is not null) then 1 else 0 end +
	case when (r.macro_enabled_at is not null) then 1 else 0 end +
	case when (r.data_entered_at is not null) then 1 else 0 end +
	case when (r.exploited_at is not null) then 1 else 0 end 
	) as 'total_failures'
                ,r.id
                ,r.userId as 'user_id'
                ,r.first_name
                ,r.last_name
                ,r.email
                ,u.manager_name
                ,u.manager_email
                ,r.recipient_id
                ,r.pst_id
                ,r.scheduled_at
                ,r.delivered_at
                ,r.opened_at
                ,r.clicked_at
                ,r.replied_at
                ,r.attachment_opened_at
                ,r.macro_enabled_at
                ,r.data_entered_at
                ,r.exploited_at
                ,r.reported_at
                ,r.bounced_at
                ,r.ip
                ,r.ip_location
                ,r.browser
                ,r.browser_version
                ,r.os
                ,r.testsId as 'tests_id'
                ,u.custom_field_2
                ,u.division
            FROM infraportal.knowbe4.results r
            INNER JOIN knowbe4.users u
            ON r.userId = u.id
            WHERE r.testsId IN ({})
            AND r.testsId <> '';
            ",
            keys.join(",")
        );
        let stream = build_dataloader_result::<KnowBe4Results>(self.0.clone(), query).await?;

        let stream = stream
            .clone()
            .into_iter()
            .into_group_map_by(|data| data.tests_id.unwrap().clone());

        Ok(stream)
    }
}

pub struct Knowbe4ReportedEmailLoader(Pool<TiberiusConnectionManager>);

impl Knowbe4ReportedEmailLoader {
    pub fn new(pool: Pool<TiberiusConnectionManager>) -> Self {
        Self(pool)
    }
}

#[async_trait]
impl Loader<String> for Knowbe4ReportedEmailLoader {
    type Value = NaiveDateTime;
    type Error = FieldError;

    async fn load(&self, keys: &[String]) -> Result<HashMap<String, Self::Value>, Self::Error> {
        let keys = keys_to_sql_in_query(keys);

        let query = format!(
            "
        SELECT 
            [from] as submitter
            ,subject
            ,sent
            ,DATEADD(s,CONVERT(int, LEFT(sent,10)),'1970-01-01') as timestamp
        FROM suspicious_emails 
        WHERE message_id LIKE '%.knowbe4.com>' 
            AND sent <100000000000 
            AND category=512
         -- AND DATEADD(s,CONVERT(int, LEFT(sent,10)),'1970-01-01') > CAST(DATEADD(m, -6, GetDate()) as date)
            AND [from] IN ({})
        ORDER BY timestamp DESC
        ;
            ",
            keys.join(",")
        );
        let stream =
            build_dataloader_result::<KnowBe4ReportedPhishing>(self.0.clone(), query).await;

        match stream {
            Ok(stream) => {
                let stream = complete_dataloader_result(stream, |ci| {
                    (ci.submitter.clone().to_lowercase(), ci.timestamp)
                });

                Ok(stream)
            }
            Err(err) => {
                eprintln!("{}", err);
                Ok(HashMap::<String, Self::Value>::new())
            }
        }
    }
}
