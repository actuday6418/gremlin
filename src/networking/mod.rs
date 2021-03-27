use rustls::{Certificate, RootCertStore, ServerCertVerified, ServerCertVerifier, TLSError};
use webpki::DNSNameRef;
use std::io::{Read, Write};

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
    route: String,
}

impl UrlParser {
    pub fn new(name: &str) -> Self {
        let mut d_vec = name.splitn(2,"/").collect::<Vec<&str>>();
        if d_vec.len() == 1 {
            d_vec.push("");
        }
        UrlParser {
            dns_name: d_vec[0].to_string(),
            request: "gemini://".to_string() + name + "/\r\n",
            port: String::from(":1965"),
            route: d_vec[1].to_string(),
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

pub fn navigate(url: UrlParser) -> String {
    //!Tries to access the gemini space and returns whatever content is acquired.
    let mut config = rustls::ClientConfig::new();
    let mut config2 = rustls::DangerousClientConfig { cfg: &mut config };
    let certificate_verifier = std::sync::Arc::new(CertVerifier::new());
    config2.set_certificate_verifier(certificate_verifier);
    let shared_cfg = std::sync::Arc::new(config);
    let dns_name = webpki::DNSNameRef::try_from_ascii_str(url.get_name()).unwrap();
    let mut client = rustls::ClientSession::new(&shared_cfg, dns_name);
    let mut socket =
    std::net::TcpStream::connect(url.get_name().to_string() + url.get_port()).expect("Error encountered. Check your internet connection!");
    let mut stream = rustls::Stream::new(&mut client, &mut socket);
    stream.write_all(url.get_request().as_bytes()).unwrap();

    let mut data = Vec::new();
    let _ = stream.read_to_end(&mut data);
    let data = String::from(String::from_utf8_lossy(&data));
    let mut status_string = String::new();
    let mut content_string: String;
    let mut chars = data.chars();
    let mut no_chars: i32 = 0;
    loop {
        no_chars += 1;
        let c = chars.next().unwrap();
        if c == '\n' {
            break;
        } else {
            status_string.push(c);
        }
    }
    content_string = data;
    content_string.drain(..no_chars as usize);
    let status = status::Status::new(status_string);
    if status.is_ok() {
        content_string
    } else {
        panic!("Server returned error status!");
    }
}