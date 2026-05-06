use crate::object_ops::{ObjectNumber, PdfObject};
use crate::version::Version;
use crate::xref_ops::XRefOps;
use crate::{PdfDictionaryObject, PdfError, PdfStreamObject};
use std::fs::File;

const XMP_HEADER: &str = r#"<?xpacket begin="" id="W5M0MpCehiHzreSzNTczkc9d"?>
<x:xmpmeta xmlns:x="adobe:ns:meta/">
<rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#">
<rdf:Description rdf:about="" xmlns:dc="http://purl.org/dc/elements/1.1/">"#;

const XMP_FOOTER: &str = r#"</rdf:Description>
</rdf:RDF>
</x:xmpmeta>
<?xpacket end="w"?>"#;

// todo: check this bs ai data
/*
Info dict key XMP equivalent   XMP wrapper
============= ================ ===========
Title         dc:title         rdf:Alt
Author        dc:creator       rdf:Seq
Subject       dc:description   rdf:Alt
Keywords      pdf:Keywords     plain text
Creator       xmp:CreatorTool  plain text
Producer      pdf:Producer     plain text
CreationDate  xmp:CreateDate   plain text
ModDate       xmp:ModifyDate   plain text
Trapped       pdf:Trapped      plain text
              dc:contributor   rdf:Seq
              dc:publisher     rdf:Seq
              dc:rights        rdf:Alt
              dc:language      rdf:Bag
              dc:subject       rdf:Bag
              dc:date          rdf:Seq
              dc:type          rdf:Bag
              dc:format        plain text
              dc:identifier    plain text
              dc:source        plain text
              dc:relation      rdf:Bag
              dc:coverage      plain text
*/

pub enum XmpType {
    Alt,
    Seq,
    Bag,
    None
}


pub struct MetadataOps {
    dict: PdfDictionaryObject,
    xmp: PdfStreamObject,
    xmp_body: String,
}

impl MetadataOps {
    pub fn new(info_object_number: ObjectNumber, xmp_object_number: ObjectNumber) -> Self {
        Self {
            dict: PdfDictionaryObject::new().with_object_number(info_object_number),
            xmp: PdfStreamObject::new().with_object_number(xmp_object_number),
            xmp_body: String::new(),
        }
    }

    fn xmp_add(&mut self, str:&str){
        self.xmp_body.push_str(str);
    }
 fn xalt(&mut self, tag: &str, value: &str)->String {
    format!("<{tag}><rdf:Alt><rdf:li xml:lang=\"x-default\">{value}</rdf:li></rdf:Alt></{tag}>")
}

fn xseq(&mut self, tag: &str, value: &str) ->String{
    format!("<{tag}><rdf:Seq><rdf:li>{value}</rdf:li></rdf:Seq></{tag}>")
}

fn xbag(&mut self, tag: &str, value: &str) ->String {
    format!("<{tag}><rdf:Bag><rdf:li>{value}</rdf:li></rdf:Bag></{tag}>")
}

fn xtext(&mut self, tag: &str, value: &str)->String {
    format!("<{tag}>{value}</{tag}>")
}

fn xdate(&mut self, tag: &str, value: &str) ->String{
    format!("<{tag}>{value}</{tag}>")
}

   pub fn add_title(&mut self, title: &str) -> Result<(), PdfError> {
        self.dict.add("Title", PdfObject::string(title))?;
       //self.xmp_add(&*self.xalt(title));
        Ok(())
    }
    pub fn add_author(&mut self, author: &str) -> Result<(), PdfError> {
        self.dict.add("Author", PdfObject::string(author))?;
        Ok(())
    }
    pub fn add_subject(&mut self, subject: &str) -> Result<(), PdfError> {
        self.dict.add("Subject", PdfObject::string(subject))?;
        Ok(())
    }
    pub fn add_keywords(&mut self, keywords: &str) -> Result<(), PdfError> {
        self.dict.add("Keywords", PdfObject::string(keywords))?;
        Ok(())
    }
    pub fn add_creator(&mut self, creator: &str) -> Result<(), PdfError> {
        self.dict.add("Creator", PdfObject::string(creator))?;
        Ok(())
    }
    pub fn add_producer(&mut self, producer: &str) -> Result<(), PdfError> {
        self.dict.add("Producer", PdfObject::string(producer))?;
        Ok(())
    }
    pub fn add_creation_date(&mut self, date: &str) -> Result<(), PdfError> {
        self.dict.add("CreationDate", PdfObject::string(date))?;
        Ok(())
    }
    pub fn add_mod_date(&mut self, date: &str) -> Result<(), PdfError> {
        self.dict.add("ModDate", PdfObject::string(date))?;
        Ok(())
    }

    pub fn serialize(
        &mut self,
        version: Version,
        xref: &mut XRefOps,
        file: &mut File,
    ) -> Result<(), PdfError> {
        self.dict.serialize(version, xref, file)?;

        // build and serialise xmp stream
        self.xmp
            .add(format!("{}{}{}", XMP_HEADER, self.xmp_body, XMP_FOOTER).into_bytes());
        self.xmp.serialize(version, xref, file)?;

        Ok(())
    }
}

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}
