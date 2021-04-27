use rustls::{Certificate, RootCertStore, ServerCertVerified, ServerCertVerifier, TLSError};
use std::fs;
use std::io::{Read, Write};
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

#[derive(Debug, PartialEq)]
pub enum Scheme {
    Gemini,
    File,
    HelpScreen,
}

#[derive(Debug)]
pub struct UrlParsed {
    scheme: Scheme,
    dns_name: String,
    request: String,
    port: String,
}

impl UrlParsed {
    //!URLParsed::new expects a string as parameter that contains the final request, possibly including "gemini://".
    pub fn new(name: &str) -> Self {
        let mut edited_name: String = String::from(name);
        let mut scheme: Scheme = Scheme::Gemini;

        if name.starts_with("file://") {
            scheme = Scheme::File;
            edited_name = String::from(edited_name)
                .trim_start_matches("file://")
                .to_string();
        } else if name == "help://" {
            scheme = Scheme::HelpScreen;
            edited_name = String::from("Help screen");
        } else if !name.starts_with("gemini://") {
            edited_name = "gemini://".to_string() + name;
            edited_name = edited_name.to_string() + "/\r\n"
        }

        UrlParsed {
            scheme: scheme,
            dns_name: edited_name
                .trim_start_matches("gemini://")
                .splitn(2, "/")
                .nth(0)
                .unwrap()
                .to_string(),
            request: edited_name,
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

pub fn navigate(url: UrlParsed) -> String {
    //!This function uses information from a UrlParsed object to navigate over gmini, HTTPS, or through
    //!the local file system
    if url.scheme == Scheme::Gemini {
        let mut config = rustls::ClientConfig::new();
        let mut config2 = rustls::DangerousClientConfig { cfg: &mut config };
        let certificate_verifier = std::sync::Arc::new(CertVerifier::new());
        config2.set_certificate_verifier(certificate_verifier);
        let shared_cfg = std::sync::Arc::new(config);
        let dns_name = webpki::DNSNameRef::try_from_ascii_str(url.get_name()).unwrap();
        let mut client = rustls::ClientSession::new(&shared_cfg, dns_name);
        let mut socket = std::net::TcpStream::connect(url.get_name().to_string() + url.get_port())
            .expect("Error encountered. Check your internet connection!");
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
    } else if url.scheme == Scheme::HelpScreen {
        String::from("Gremlin is a TUI client primarily for the Gemini protocol.\n\n
* Hit Alt+n to navigate to a new URL\n\n
* Hit Enter to navigate after entering the URL.\n\n
* Use the 'https://' scheme to browse the web, or the 'file://' to open a file in your local file system\n\n
* Use the arrow keys to scroll and Ctrl+k and Ctrl+l to scroll links\n\n
* Use Alt+k and Alt+l to scroll through your history.\n\n
* This menu may be accesed at any time using 'help://' from the navigation popup.\n\n
=> gemini://gemini.circumlunar.space Here's a link to the Gemini homepage")
    } else {
        let mut file = fs::File::open(url.get_request()).unwrap();
        let mut buffer: String = String::new();
        file.read_to_string(&mut buffer).unwrap();
        buffer
    }
}
