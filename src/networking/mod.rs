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

#[derive(Debug, PartialEq, Clone)]
pub enum Scheme {
    Gemini,
    File,
    HelpScreen,
    Https,
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
        let edited_name: String;
        let edited_request: String;
        let scheme: Scheme;

        if name.starts_with("file://") {
            scheme = Scheme::File;
            edited_name = String::from(name).trim_start_matches("file://").to_string();
            edited_request = edited_name.clone();
        } else if name == "help://" {
            scheme = Scheme::HelpScreen;
            edited_name = String::from("Help screen");
            edited_request = String::from("Never used!");
        } else if name.starts_with("https://") {
            scheme = Scheme::Https;
            edited_name = name.to_string().trim_start_matches("https://").to_string();
            edited_request = "GET / HTTP/1.1 Host: ".to_string()
                + edited_name.clone().as_str()
                + " Accept: text/html\r\n";
        } else if !name.starts_with("gemini://") {
            scheme = Scheme::Gemini;
            edited_name = String::from(name);
            edited_request = "gemini://".to_string() + name + "/\r\n";
        } else {
            scheme = Scheme::Gemini;
            edited_name = name
                .trim_start_matches("gemini://")
                .splitn(2, "/")
                .nth(0)
                .unwrap()
                .to_string();
            edited_request = name.to_string() + "/\r\n";
        }

        UrlParsed {
            scheme: scheme.clone(),
            dns_name: edited_name,
            request: edited_request,
            port: match scheme {
                Scheme::Gemini => String::from(":1965"),
                _ => String::from(":443"),
            },
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
    //!This function uses information from a UrlParsed object to navigate over gemini, HTTPS, or through
    //!the local file system
    match url.scheme {
    Scheme::Gemini => {
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
    }
    Scheme::HelpScreen => {
        String::from("Gremlin is a TUI client primarily for the Gemini protocol.\n\n
* Hit Alt+n to navigate to a new URL\n\n
* Hit Enter to navigate after entering the URL.\n\n
* Use the 'https://' scheme to browse the web, or the 'file://' to open a file in your local file system\n\n
* Use the arrow keys to scroll and Ctrl+k and Ctrl+l to scroll links\n\n
* Use Alt+k and Alt+l to scroll through your history.\n\n
* This menu may be accesed at any time using 'help://' from the navigation popup.\n\n
=> https://wikipedia.com Here's not a link to the Gemini homepage")
    }
    Scheme::File => {
        let mut file = fs::File::open(url.get_request()).unwrap();
        let mut buffer: String = String::new();
        file.read_to_string(&mut buffer).unwrap();
        buffer
    }
    Scheme::Https => {
        use voca_rs;
        let res = reqwest::blocking::get("https://en.wikipedia.org/w/api.php?action=parse&format=json&titles=Jesus&prop=revisions&rvprop=content", ).unwrap();
        let content = res.text().unwrap();
        let content = voca_rs::strip::strip_tags(&content);
        let mut f = std::fs::File::create("recieved.txt").unwrap();
        f.write_all(content.clone().as_bytes()).unwrap();
        //let content = crate::parser::parse_html(content.as_str()).to_string();
        content
    }
}
}
