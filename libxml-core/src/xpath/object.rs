use std::{
  fmt::Display,
  marker::PhantomData,
  ops::{Deref, DerefMut},
};

use libxml_sys::bindings::{
  xmlXPathCastNodeToString, xmlXPathCastToString, xmlXPathFreeObject, xmlXPathObject,
};

use crate::{
  macros::define_wrapper_types_with_lifetime,
  string::XmlString,
  tree::{DocumentRef, NodeRef},
  xpath::node_set::NodeSetRef,
};

define_wrapper_types_with_lifetime!(
  xpathobject,
  XPathObject,
  XPathObjectRef,
  DocumentRef,
  xmlXPathObject
);

impl Drop for XPathObject<'_> {
  fn drop(&mut self) {
    unsafe {
      xmlXPathFreeObject(self.as_ptr());
    }
  }
}

impl<'a> XPathObjectRef<'a> {
  pub fn get_node_set(&self) -> Option<&NodeSetRef<'a>> {
    unsafe { NodeSetRef::maybe_from_ptr(self.0.nodesetval) }
  }

  ///get the number of nodes in the result set
  pub fn get_number_of_nodes(&self) -> usize {
    self.get_node_set().map_or(0, |s| s.len())
  }

  pub fn get_nodes(&self) -> &[&'a NodeRef] {
    static EMPTY_NODE_SET: &[&NodeRef] = &[];
    self.get_node_set().map_or(EMPTY_NODE_SET, |s| s.deref())
  }

  /// returns the result set as a vector of `Node` objects
  pub fn get_nodes_as_vec(&self) -> Vec<&'a NodeRef> {
    self.get_nodes().to_vec()
  }

  /// returns the result set as a vector of Strings
  pub fn get_nodes_as_str(&self) -> Vec<XmlString> {
    self
      .get_nodes()
      .iter()
      .copied()
      .map(|node| unsafe {
        let value_ptr = xmlXPathCastNodeToString(node.as_ptr());
        XmlString::new(value_ptr)
      })
      .collect()
  }
}

impl Display for XPathObject<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let s = unsafe {
      let ptr = xmlXPathCastToString(self.as_ptr());
      XmlString::new(ptr)
    };
    write!(f, "{s}")
  }
}
