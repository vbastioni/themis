use futures::executor::block_on;
use meilisearch_sdk as meili;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub struct Meili {
    index: meili::indexes::Index,
}

impl Meili {
    fn new(host: impl Into<String>, key: impl Into<String>, index: impl Into<String>) -> Self {
        Self {
            index: meilisearch_sdk::client::Client::new(host, key).index(index),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Movie {
    id: usize,
    title: String,
    genres: Vec<String>,
}

impl Meili {
    pub fn send<S>(
        &self,
        s: S,
        primary_key: Option<&str>,
    ) -> Result<meili::task_info::TaskInfo, meili::errors::Error>
    where
        S: Serialize,
    {
        block_on(async move { self.index.add_documents(&[s], primary_key).await })
    }

    pub fn get<T: DeserializeOwned + 'static>(
        &self,
    ) -> Result<meili::documents::DocumentsResults<T>, meili::errors::Error> {
        block_on(async move { self.index.get_documents().await })
    }
}

pub fn new(host: impl Into<String>, key: impl Into<String>, index: impl Into<String>) -> Meili {
    Meili::new(host, key, index)
}
