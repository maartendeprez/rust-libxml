use std::{
  ffi::CString,
  ops::{Deref, DerefMut},
};

use libxml_sys::bindings::{xmlChar, xmlDoc, xmlDocGetRootElement, xmlFreeDoc, xmlNewDoc};

use crate::{
  error::{Error, Result},
  macros::define_wrapper_types,
  tree::NodeRef,
};

define_wrapper_types!(document, Document, DocumentRef, xmlDoc);

impl Document {
  pub fn new(version: Option<&str>) -> Result<Self> {
    let version = version.map(CString::new).transpose().map_err(Error::CStr)?;
    unsafe {
      let doc_ptr = xmlNewDoc(version.map_or(std::ptr::null(), |s| s.as_ptr() as *const xmlChar));
      Self::maybe_from_ptr(doc_ptr).ok_or(Error::Tree)
    }
  }
}

impl DocumentRef {
  /// Get the root element of the document
  pub fn get_root_element(&self) -> Option<&NodeRef> {
    unsafe {
      let node_ptr = xmlDocGetRootElement(self.as_ptr());
      NodeRef::maybe_from_ptr(node_ptr)
    }
  }

  /// Get the root element of the document
  pub fn get_root_element_mut(&mut self) -> Option<&mut NodeRef> {
    unsafe {
      let node_ptr = xmlDocGetRootElement(self.as_ptr());
      NodeRef::maybe_from_ptr_mut(node_ptr)
    }
  }
}

impl Drop for Document {
  fn drop(&mut self) {
    unsafe {
      xmlFreeDoc(self.as_ptr());
    }
  }
}
