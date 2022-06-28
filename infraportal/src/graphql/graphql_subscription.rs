use async_graphql::{Enum, Subscription};
use futures::{Stream, StreamExt};
use std::time::Duration;

use crate::models::cmdb_ci_service::model::AppSubmitted;

use super::simple_broker::SimpleBroker;

pub struct SubscriptionRoot;

#[derive(Enum, Eq, PartialEq, Copy, Clone)]
pub enum MutationType {
    Created,
    Deleted,
    Updated,
}

#[Subscription]
impl SubscriptionRoot {
    async fn interval(&self, #[graphql(default = 1)] n: i32) -> impl Stream<Item = i32> {
        let mut value = 0;
        async_stream::stream! {
            loop {
                futures_timer::Delay::new(Duration::from_secs(1)).await;
                value += n;
                yield value;
            }
        }
    }

    async fn app_form_submitted(
        &self,
        mutation_type: Option<MutationType>,
    ) -> impl Stream<Item = AppSubmitted> {
        SimpleBroker::<AppSubmitted>::subscribe().filter(move |event| {
            let res = if let Some(mutation_type) = mutation_type {
                event.mutation_type == mutation_type
            } else {
                true
            };
            async move { res }
        })
    }
}
