use std::ops::{Deref, DerefMut};

use libxml_sys::bindings::{xmlAttr, xmlChar, xmlFreeProp};

use crate::{
  macros::define_wrapper_types,
  string::str_from_xml_str,
  tree::{NamespaceRef, NodeType},
};

define_wrapper_types!(attr, Attr, AttrRef, xmlAttr);

impl Drop for Attr {
  fn drop(&mut self) {
    unsafe {
      xmlFreeProp(self.as_ptr());
    }
  }
}

impl AttrRef {
  /// Get the node type
  pub fn get_type(&self) -> Option<NodeType> {
    NodeType::from_int(self.0.type_)
  }

  pub fn get_next(&self) -> Option<&Self> {
    unsafe { Self::maybe_from_ptr(self.0.next) }
  }

  pub fn get_name(&self) -> &str {
    unsafe { str_from_xml_str(self.0.name) }
  }

  pub(crate) fn get_name_ptr(&self) -> *const xmlChar {
    self.0.name
  }

  pub fn get_namespace(&self) -> Option<&NamespaceRef> {
    unsafe { NamespaceRef::maybe_from_ptr(self.0.ns) }
  }
}
