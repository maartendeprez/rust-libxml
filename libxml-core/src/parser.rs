use std::{
  ffi::{CString, c_char, c_int},
  ops::{Deref, DerefMut},
  path::Path,
};

use libxml_sys::bindings::{
  htmlCtxtReadMemory, htmlFreeParserCtxt, htmlNewParserCtxt, htmlParserCtxt, xmlCtxtReadMemory,
  xmlFreeParserCtxt, xmlNewParserCtxt, xmlParserCtxt,
};

use crate::{
  error::{Error, Result},
  macros::define_wrapper_types,
  tree::Document,
};

define_wrapper_types!(parsercontext, XmlParser, XmlParserRef, xmlParserCtxt);
define_wrapper_types!(parsercontext, HtmlParser, HtmlParserRef, htmlParserCtxt);

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum ParseFormat {
  Xml,
  Html,
}

#[derive(Clone, Copy, Default)]
pub struct XmlParserOptions(c_int);

#[derive(Clone, Copy, Default)]
pub struct HtmlParserOptions(c_int);

impl Drop for XmlParser {
  fn drop(&mut self) {
    unsafe {
      xmlFreeParserCtxt(self.as_ptr());
    }
  }
}

impl Drop for HtmlParser {
  fn drop(&mut self) {
    unsafe {
      htmlFreeParserCtxt(self.as_ptr());
    }
  }
}

impl XmlParser {
  pub fn new() -> Self {
    unsafe {
      let ctx_ptr = xmlNewParserCtxt();
      // returns NULL on allocation error.
      Self::maybe_from_ptr(ctx_ptr).unwrap()
    }
  }
}

impl Default for XmlParser {
  fn default() -> Self {
    Self::new()
  }
}

impl XmlParserRef {
  pub fn read_memory<T: AsRef<[u8]>>(
    &mut self,
    bytes: T,
    url: Option<&str>,
    encoding: Option<&str>,
    options: XmlParserOptions,
  ) -> Result<Document> {
    let bytes = bytes.as_ref();
    let url = url.map(|s| CString::new(s).unwrap());
    let encoding = encoding.map(|s| CString::new(s).unwrap());

    unsafe {
      let doc_ptr = xmlCtxtReadMemory(
        self.as_ptr(),
        bytes.as_ptr() as *const c_char,
        bytes.len() as c_int,
        url.map_or(std::ptr::null(), |url| url.as_ptr()),
        encoding.map_or(std::ptr::null(), |url| url.as_ptr()),
        options.0,
      );
      Document::maybe_from_ptr(doc_ptr).ok_or(Error::Parser)
    }
  }

  pub fn parse_file<T: AsRef<Path>>(
    &mut self,
    path: T,
    url: Option<&str>,
    encoding: Option<&str>,
    options: XmlParserOptions,
  ) -> Result<Document> {
    let bytes = std::fs::read(path).map_err(|_| Error::Parser)?;
    self.read_memory(bytes, url, encoding, options)
  }
}

impl HtmlParser {
  pub fn new() -> Self {
    unsafe {
      let ctx_ptr = htmlNewParserCtxt();
      // returns NULL on allocation error.
      Self::maybe_from_ptr(ctx_ptr).unwrap()
    }
  }
}

impl Default for HtmlParser {
  fn default() -> Self {
    Self::new()
  }
}

impl HtmlParserRef {
  pub fn read_memory<T: AsRef<[u8]>>(
    &mut self,
    bytes: T,
    url: Option<&str>,
    encoding: Option<&str>,
    options: HtmlParserOptions,
  ) -> Result<Document> {
    let bytes = bytes.as_ref();
    let url = url.map(|s| CString::new(s).unwrap());
    let encoding = encoding.map(|s| CString::new(s).unwrap());

    unsafe {
      let doc_ptr = htmlCtxtReadMemory(
        self.as_ptr(),
        bytes.as_ptr() as *const c_char,
        bytes.len() as c_int,
        url.map_or(std::ptr::null(), |url| url.as_ptr()),
        encoding.map_or(std::ptr::null(), |url| url.as_ptr()),
        options.0,
      );
      Document::maybe_from_ptr(doc_ptr).ok_or(Error::Parser)
    }
  }

  pub fn parse_file<T: AsRef<Path>>(
    &mut self,
    path: T,
    url: Option<&str>,
    encoding: Option<&str>,
    options: HtmlParserOptions,
  ) -> Result<Document> {
    let bytes = std::fs::read(path).map_err(|_| Error::Parser)?;
    self.read_memory(bytes, url, encoding, options)
  }
}
