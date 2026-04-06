use std::{
  ffi::{CString, c_int},
  ops::{Deref, DerefMut},
};

use libxml_sys::bindings::{
  xmlChar, xmlDoc, xmlDocGetRootElement, xmlDocSetRootElement, xmlFreeDoc, xmlNewDoc, xmlNode,
  xmlReadMemory, xmlUnlinkNode,
};

use crate::{
  error::{Error, Result},
  macros::define_wrapper_types,
  tree::{Attr, AttrRef, Node, NodeRef},
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

  pub fn from_xml_slice<T: AsRef<[u8]>>(input: T) -> Result<Self> {
    let bytes = input.as_ref();
    unsafe {
      let doc_ptr = xmlReadMemory(
        bytes.as_ptr() as *const i8,
        bytes.len() as c_int,
        std::ptr::null(),
        std::ptr::null(),
        0,
      );
      Self::maybe_from_ptr(doc_ptr).ok_or(Error::Parser)
    }
  }
}

impl DocumentRef {
  /// Set the root element of the document. Returns the previous root element.
  pub fn set_root_element(&mut self, root: Node) -> Result<Option<Node>> {
    unsafe {
      // TODO: error handling. How to distinguish between failue and no prev root?
      let prev_ptr = xmlDocSetRootElement(self.as_ptr(), root.as_ptr());
      std::mem::forget(root);
      Ok(Node::maybe_from_ptr(prev_ptr))
    }
  }

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

  pub fn unlink_attr(&mut self, attr: &mut AttrRef) -> Attr {
    unsafe {
      xmlUnlinkNode(attr.as_ptr() as *mut xmlNode);
      Attr::from_ptr(attr.as_ptr())
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
