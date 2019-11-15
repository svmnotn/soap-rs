use crate::{DisplayAction, Error};
use minidom::Element;
use std::fmt::{Error as FmtError, Write};

pub struct Envelope<H: DisplayAction> {
    action_url: String,
    dest_url: String,
    head: Option<H>,
}

impl<H: DisplayAction> Envelope<H> {
    pub fn new(action_url: String, dest_url: String, head: impl Into<Option<H>>) -> Self {
        let head = head.into();
        Envelope {
            action_url,
            dest_url,
            head,
        }
    }

    fn fmt<D: DisplayAction>(
        &self,
        fmt: &mut String,
        action: &str,
        body: D,
    ) -> Result<(), FmtError> {
        write!(
            fmt,
            r#"<?xml version="1.0" encoding="utf-8"?><soap:Envelope xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema" xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/">"#
        )?;
        if let Some(ref h) = self.head {
            write!(fmt, r#"<soap:Header>"#)?;
            h.fmt(fmt, &self.action_url)?;
            write!(fmt, r#"</soap:Header>"#)?;
        }
        write!(
            fmt,
            r#"<soap:Body><{act} xmlns="{acturl}">"#,
            act = action,
            acturl = self.action_url
        )?;
        body.fmt(fmt, "")?;
        write!(fmt, "</{act}></soap:Body></soap:Envelope>", act = action)
    }

    fn send<D: DisplayAction>(&self, action: &str, body: D) -> Result<String, Error> {
        use reqwest::{header::CONTENT_TYPE, Client};
        // create our body
        let mut fmt = String::new();
        self.fmt(&mut fmt, action, body)?;
        #[cfg(debug_assertions)]
        println!("BODY:\n{}\n", fmt);
        // create and send our request
        let req = Client::new()
            .post(&self.dest_url)
            .header(CONTENT_TYPE, "text/xml; charset=utf-8")
            .header("SOAPAction", format!("{}{}", self.action_url, action))
            .body(fmt);
        #[cfg(debug_assertions)]
        println!("REQUEST:\n{:?}\n", req);
        let mut res = req.send()?;
        // retrieve the output
        Ok(res.text()?)
    }

    pub fn response<D: DisplayAction>(
        &self,
        action: &str,
        body: D,
    ) -> Result<(Element, String), Error> {
        // parse the recived response to XML
        let text = self.send(action, body)?;
        #[cfg(debug_assertions)]
        println!("RESPONSE: {}", text);
        Ok((text.parse()?, self.action_url.clone()))
    }
}
