use async_graphql::{dataloader::DataLoader, ComplexObject, Context, FieldResult, SimpleObject};
use chrono::{Datelike, NaiveDateTime};
use query_tiberius_derive::Queryable;

use super::dataloaders::{Knowbe4ReportedEmailLoader, Knowbe4ResultsLoader};

#[derive(Debug, SimpleObject, Queryable, Clone)]
#[graphql(rename_fields = "snake_case")]
#[graphql(complex)]
pub struct KnowBe4Test {
    pub id: i32,
    pub campaign_id: Option<String>,
    pub pst_id: Option<String>,
    pub status: Option<String>,
    pub name: Option<String>,
    pub phish_prone_percentage: Option<f64>,
    pub started_at: Option<NaiveDateTime>,
    pub duration: Option<String>,
    pub scheduled_count: Option<f64>,
    pub delivered_count: Option<f64>,
    pub opened_count: Option<f64>,
    pub clicked_count: Option<f64>,
    pub replied_count: Option<f64>,
    pub attachment_open_count: Option<f64>,
    pub macro_enabled_count: Option<f64>,
    pub data_entered_count: Option<f64>,
    pub vulnerable_plugin_count: Option<f64>,
    pub exploited_count: Option<f64>,
    pub reported_count: Option<f64>,
    pub bounced_count: Option<f64>,
}

#[ComplexObject]
impl KnowBe4Test {
    async fn recipients(&self, ctx: &Context<'_>) -> FieldResult<Option<Vec<KnowBe4Results>>> {
        let data: Option<Vec<KnowBe4Results>> = ctx
            .data_unchecked::<DataLoader<Knowbe4ResultsLoader>>()
            .load_one(self.id)
            .await?;

        Ok(data)
    }
}

#[derive(Debug, SimpleObject, Queryable, Clone)]
#[graphql(complex)]
#[graphql(rename_fields = "snake_case")]
pub struct KnowBe4Results {
    pub id: i32,
    pub total_failures: i32,
    pub custom_field_2: Option<String>,
    pub division: Option<String>,
    pub manager_name: Option<String>,
    pub manager_email: Option<String>,
    pub user_id: Option<i32>,  //renamed
    pub tests_id: Option<i32>, //renamed
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub recipient_id: Option<i32>,
    pub pst_id: Option<i32>,
    pub scheduled_at: Option<NaiveDateTime>,
    pub delivered_at: Option<NaiveDateTime>,
    pub opened_at: Option<String>,
    pub clicked_at: Option<NaiveDateTime>,
    pub replied_at: Option<NaiveDateTime>,
    pub attachment_opened_at: Option<NaiveDateTime>,
    pub macro_enabled_at: Option<NaiveDateTime>,
    pub data_entered_at: Option<NaiveDateTime>,
    pub exploited_at: Option<NaiveDateTime>,
    pub reported_at: Option<String>,
    pub bounced_at: Option<NaiveDateTime>,
    pub ip: Option<String>,
    pub ip_location: Option<String>,
    pub browser: Option<String>,
    pub browser_version: Option<String>,
    pub os: Option<String>,
    pub failed: String,
}

#[ComplexObject]
impl KnowBe4Results {
    async fn has_failed(&self) -> bool {
        if self.failed.as_str() == "true" {
            true
        } else {
            false
        }
    }
    async fn has_reported(&self, ctx: &Context<'_>) -> FieldResult<Option<bool>> {
        if let Some(scheduled_at) = self.scheduled_at {
            if let Some(email) = self.email.clone() {
                let data: Option<NaiveDateTime> = ctx
                    .data_unchecked::<DataLoader<Knowbe4ReportedEmailLoader>>()
                    .load_one(email.to_lowercase())
                    .await?;

                if let Some(data) = data {
                    let month = scheduled_at.month();
                    let year = scheduled_at.year();

                    let reported_month = data.month();
                    let reported_year = data.year();

                    // println!("scheluded {:#?} data {:#?} month {:#?} year {:#?} s_month {:#?} s_year {:#?}", scheduled_at, data, month, year, reported_month, reported_year);

                    if month == reported_month && year == reported_year {
                        Ok(Some(true))
                    } else {
                        Ok(Some(false))
                    }
                } else {
                    Ok(Some(false))
                }
            } else {
                Ok(Some(false))
            }
        } else {
            Ok(Some(false))
        }
    }
}

//from seceng table
#[derive(Debug, Queryable, SimpleObject)]
pub struct KnowBe4ReportedPhishing {
    pub submitter: String,
    pub subject: String,
    pub timestamp: NaiveDateTime,
}

#[derive(Debug, SimpleObject, Queryable)]
#[graphql(rename_fields = "snake_case")]
pub struct KnowBe4Training {
    pub enrollment_id: i32,
    pub id: Option<i32>,
    pub content_type: Option<String>,
    pub module_name: Option<String>,
    pub campaign_name: Option<String>,
    pub enrollment_date: Option<NaiveDateTime>,
    pub start_date: Option<NaiveDateTime>,
    pub completion_date: Option<NaiveDateTime>,
    pub status: Option<String>,
    pub time_spent: Option<i32>,
    pub policy_acknowledged: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, SimpleObject, Queryable)]
#[graphql(rename_fields = "snake_case")]
pub struct KnowBe4Users {
    pub id: i32,
    pub employee_number: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub job_title: Option<String>,
    pub email: Option<String>,
    pub phish_prone_percentage: Option<f64>,
    pub phone_number: Option<String>,
    pub mobile_phone_number: Option<String>,
    pub location: Option<String>,
    pub division: Option<String>,
    pub manager_name: Option<String>,
    pub manager_email: Option<String>,
    pub adi_manageable: Option<String>,
    pub current_risk_score: Option<f64>,
    pub joined_on: Option<NaiveDateTime>,
    pub last_sign_in: Option<NaiveDateTime>,
    pub status: Option<String>,
    pub custom_field_1: Option<String>,
    pub custom_field_2: Option<String>,
    pub more_info_id: Option<String>, //rename
    pub roll_ups_to: String,
}
