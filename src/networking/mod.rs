use rustls::{Certificate, RootCertStore, ServerCertVerified, ServerCertVerifier, TLSError};
use webpki::DNSNameRef;

pub mod status;

pub struct CertVerifier {}

impl CertVerifier {
    pub fn new() -> Self {
        CertVerifier {}
    }
}

impl ServerCertVerifier for CertVerifier {
    fn verify_server_cert(
        &self,
        _: &RootCertStore,
        _: &[Certificate],
        _: DNSNameRef,
        _: &[u8],
    ) -> Result<ServerCertVerified, TLSError> {
        return Ok(ServerCertVerified::assertion());
    }
}

pub struct UrlParser {
    dns_name: String,
    request: String,
    port: String,
}

impl UrlParser {
    pub fn new(name: &str) -> Self {
        UrlParser {
            dns_name: name.to_string(),
            request: "gemini://".to_string() + name + "/\r\n",
            port: String::from(":1965"),
        }
    }
    pub fn get_request(&self) -> &str {
        self.request.as_str()
    }
    pub fn get_name(&self) -> &str {
        self.dns_name.as_str()
    }
    pub fn get_port(&self) -> &str {
        self.port.as_str()
    }
}
