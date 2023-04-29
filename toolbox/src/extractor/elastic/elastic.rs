use elasticsearch::{BulkOperation, BulkParts, Elasticsearch, IndexParts};
use futures::executor::block_on;
use serde::Serialize;

use super::builder::ElasticBuilder;

pub struct Elastic {
    pub(super) client: Elasticsearch,
}

impl Elastic {
    pub fn builder() -> ElasticBuilder {
        ElasticBuilder::default()
    }

    pub fn send<S, I>(
        &self,
        s: S,
        id: I,
    ) -> Result<elasticsearch::http::response::Response, elasticsearch::Error>
    where
        S: Serialize,
        I: AsRef<str>,
    {
        block_on(async move {
            self.client
                .index(IndexParts::IndexId("acco", id.as_ref()))
                .body(s)
                .pretty(true)
                .send()
                .await
        })
    }

    pub fn bulk<S>(&self, data: &[S]) -> Result<(), elasticsearch::Error>
    where
        S: Serialize + crate::domain::traits::Id,
    {
        let body: Vec<BulkOperation<_>> = data
            .iter()
            .map(|d| {
                let id = d.id();
                BulkOperation::index(d).id(id).into()
            })
            .collect();
        block_on(async move {
            self.client
                .bulk(BulkParts::Index("acco"))
                .body(body)
                .send()
                .await
                .map(|_| ())
        })
    }
}
