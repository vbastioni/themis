use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

use elasticsearch::{
    auth::Credentials,
    cert::{Certificate, CertificateValidation},
    http::{
        transport::{SingleNodeConnectionPool, TransportBuilder},
        Url,
    },
    BulkOperation, BulkParts, Elasticsearch, IndexParts,
};
use futures::executor::block_on;
use serde::Serialize;

#[derive(Debug)]
pub enum CertError {
    Missing,
    Path,
    File,
    Data,
}

#[derive(Debug)]
pub enum Error {
    InvalidCert(CertError),
    InvalidURL,
    TransportError,
    MissingPass,
}

#[derive(Clone, Debug)]
pub struct ElasticBuilder {
    host: Option<String>,
    port: Option<u16>,
    user: Option<String>,
    pass: Option<String>,
    cert: Option<String>,
}

impl Default for ElasticBuilder {
    fn default() -> Self {
        Self {
            host: None,
            port: None,
            user: None,
            pass: None,
            cert: None,
        }
    }
}

impl ElasticBuilder {
    pub fn host(self, s: impl Into<String>) -> Self {
        Self {
            host: Some(s.into()),
            ..self
        }
    }
    pub fn port(self, n: u16) -> Self {
        Self {
            port: Some(n),
            ..self
        }
    }
    pub fn user(self, s: impl Into<String>) -> Self {
        Self {
            user: Some(s.into()),
            ..self
        }
    }
    pub fn pass(self, s: impl Into<String>) -> Self {
        Self {
            pass: Some(s.into()),
            ..self
        }
    }
    pub fn cert(self, s: impl Into<String>) -> Self {
        Self {
            cert: Some(s.into()),
            ..self
        }
    }

    fn get_cert<P>(p: P) -> Result<CertificateValidation, CertError>
    where
        P: AsRef<Path>,
    {
        let f = File::open(p).map_err(|_| CertError::Path)?;
        let mut reader = BufReader::new(f);
        let mut buf: Vec<u8> = Vec::new();
        reader.read_to_end(&mut buf).map_err(|_| CertError::File)?;
        Ok(elasticsearch::cert::CertificateValidation::Certificate(
            Certificate::from_pem(&buf).map_err(|_| CertError::Data)?,
        ))
    }

    pub fn build(&self) -> Result<Elastic, Error> {
        match (&self.pass, &self.cert) {
            (Some(pass), Some(cert)) => {
                let host = match self.host {
                    Some(ref s) => s.clone(),
                    None => "localhost".into(),
                };
                let port = &self.port.unwrap_or(9200);
                let url =
                    Url::parse(&format!("https://{host}:{port}")).map_err(|_| Error::InvalidURL)?;
                let user = match self.user {
                    Some(ref s) => s.clone(),
                    None => "elastic".into(),
                };
                let credentials = Credentials::Basic(user.into(), pass.into());
                let cert = Self::get_cert(&cert).map_err(|e| Error::InvalidCert(e))?;
                let pool = SingleNodeConnectionPool::new(url);
                let transport = TransportBuilder::new(pool)
                    .cert_validation(cert)
                    .auth(credentials)
                    .build()
                    .map_err(|_| Error::TransportError)?;
                let client = Elasticsearch::new(transport);
                Ok(Elastic { client })
            }
            (None, _) => Err(Error::MissingPass), // missing pass
            (_, None) => Err(Error::InvalidCert(CertError::Missing)), // missing cert
        }
    }
}

pub struct Elastic {
    client: Elasticsearch,
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
