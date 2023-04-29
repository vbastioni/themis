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
