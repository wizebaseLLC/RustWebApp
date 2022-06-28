use async_graphql::{InputObject, Object};
use chrono::NaiveDateTime;
use query_tiberius_derive::Queryable;
use serde::{Deserialize, Serialize};
use tiberius::numeric::Decimal;
#[derive(Debug, Clone, Queryable)]
pub struct NbDisable {
    pub id: i32,
    pub item: String,
    pub want_on_gmt: Decimal,
    pub want_on_loc: NaiveDateTime,
    pub have_on_gmt: Option<Decimal>,
    pub want_off_gmt: Decimal,
    pub want_off_loc: NaiveDateTime,
    pub have_off_gmt: Option<Decimal>,
    pub note_bak: Option<String>,
    pub disabled_by: String,
    pub enabled_by: Option<String>,
    pub disable_stamp: String,
    pub enable_stamp: Option<String>,
    pub note: Option<String>,
}

#[Object]
impl NbDisable {
    async fn id(&self) -> i32 {
        self.id
    }

    async fn item(&self) -> String {
        self.item.to_string()
    }

    async fn want_on_gmt(&self) -> String {
        self.want_on_gmt.to_string()
    }

    async fn want_on_loc(&self) -> NaiveDateTime {
        self.want_on_loc
    }

    async fn have_on_gmt(&self) -> Option<String> {
        if let Some(have_on_gmt) = self.have_on_gmt {
            Some(have_on_gmt.to_string())
        } else {
            None
        }
    }

    async fn want_off_gmt(&self) -> String {
        self.want_off_gmt.to_string()
    }

    async fn want_off_loc(&self) -> NaiveDateTime {
        self.want_off_loc
    }

    async fn have_off_gmt(&self) -> Option<String> {
        if let Some(have_off_gmt) = self.have_off_gmt {
            Some(have_off_gmt.to_string())
        } else {
            None
        }
    }

    async fn note_bak(&self) -> Option<&str> {
        self.note_bak.as_deref()
    }

    async fn disabled_by(&self) -> &str {
        self.disabled_by.as_str()
    }

    async fn enabled_by(&self) -> Option<&str> {
        self.enabled_by.as_deref()
    }

    async fn disable_stamp(&self) -> &str {
        self.disable_stamp.as_str()
    }

    async fn enable_stamp(&self) -> Option<&str> {
        self.enable_stamp.as_deref()
    }

    async fn note(&self) -> Option<&str> {
        self.note.as_deref()
    }
}

#[derive(InputObject, Debug)]
pub struct NbDisableInput {
    pub note: String,
    pub by: String,
    pub from: i32,
    pub to: i32,
    pub host: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NbDisableInputBody {
    pub offset: i32,
    pub note: String,
    pub by: String,
    pub from: i32,
    pub to: i32,
    pub host: String,
}

impl NbDisableInputBody {
    pub fn new(values: NbDisableInput) -> Self {
        let NbDisableInput {
            by,
            from,
            host,
            note,
            to,
        } = values;

        Self {
            offset: 14400,
            by,
            from,
            to,
            note,
            host,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, InputObject)]
pub struct NbDisableEnableInput {
    pub enable: i32,
    pub by: String,
}
