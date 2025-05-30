use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use anyhow::Result;

use crate::api::APIResult;
use crate::encore::parser::meta::v1 as meta;
use crate::encore::runtime::v1 as pb;
use crate::pubsub;
use crate::pubsub::manager::SubHandler;

#[derive(Debug)]
pub struct NoopCluster;

#[derive(Debug)]
pub struct NoopTopic;
#[derive(Debug)]
pub struct NoopSubscription;

impl pubsub::Cluster for NoopCluster {
    fn topic(&self, _cfg: &pb::PubSubTopic, _publisher_id: xid::Id) -> Arc<dyn pubsub::Topic> {
        Arc::new(NoopTopic)
    }

    fn subscription(
        &self,
        _cfg: &pb::PubSubSubscription,
        _meta: &meta::pub_sub_topic::Subscription,
    ) -> Arc<dyn pubsub::Subscription> {
        Arc::new(NoopSubscription)
    }
}

impl pubsub::Topic for NoopTopic {
    fn publish(
        &self,
        _: pubsub::MessageData,
        _: Option<String>,
    ) -> Pin<Box<dyn Future<Output = Result<pubsub::MessageId>> + Send + '_>> {
        Box::pin(async {
            anyhow::bail!("topic not configured");
        })
    }
}

impl pubsub::Subscription for NoopSubscription {
    fn subscribe(
        &self,
        _: Arc<SubHandler>,
    ) -> Pin<Box<dyn Future<Output = APIResult<()>> + Send + 'static>> {
        Box::pin(futures::future::pending())
    }
}
