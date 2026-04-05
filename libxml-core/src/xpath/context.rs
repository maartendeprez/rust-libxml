use std::ops::{Deref, DerefMut};
use std::{ffi::CString, marker::PhantomData};

use libxml_sys::bindings::{
  xmlXPathContext, xmlXPathEvalExpression, xmlXPathFreeContext, xmlXPathNewContext,
  xmlXPathNodeEval, xmlXPathSetContextNode,
};

use crate::{
  error::{Error, Result},
  macros::define_wrapper_types_with_lifetime,
  string::XmlString,
  tree::{DocumentRef, NodeRef},
  xpath::object::XPathObject,
};

define_wrapper_types_with_lifetime!(
  xpathcontext,
  XPathContext,
  XPathContextRef,
  DocumentRef,
  xmlXPathContext
);

impl Drop for XPathContext<'_> {
  fn drop(&mut self) {
    unsafe {
      xmlXPathFreeContext(self.as_ptr());
    }
  }
}

impl<'a> XPathContext<'a> {
  /// create a read-only xpath context for a document
  pub fn new(doc: &'a DocumentRef) -> Result<Self> {
    let ctx_ptr = unsafe { xmlXPathNewContext(doc.as_ptr()) };
    unsafe { Self::maybe_from_ptr(ctx_ptr).ok_or(Error::XPath) }
  }
}

impl<'a> XPathContextRef<'a> {
  /// evaluate an xpath
  pub fn eval(&self, xpath: &str) -> Result<XPathObject<'a>> {
    let c_xpath = CString::new(xpath).unwrap();
    unsafe {
      let ptr = xmlXPathEvalExpression(c_xpath.as_bytes().as_ptr(), self.as_ptr());
      XPathObject::maybe_from_ptr(ptr).ok_or(Error::XPath)
    }
  }

  /// Evaluate an xpath on a context Node. The context node must be part of the
  /// document the context was created with.
  pub fn node_eval<'b: 'a>(&mut self, xpath: &str, node: &'b NodeRef) -> Result<XPathObject<'a>> {
    // assert_eq!();
    let c_xpath = CString::new(xpath).unwrap();
    unsafe {
      let ptr = xmlXPathNodeEval(node.as_ptr(), c_xpath.as_bytes().as_ptr(), self.as_ptr());
      XPathObject::maybe_from_ptr(ptr).ok_or(Error::XPath)
    }
  }

  /// Set the context node.
  pub fn set_context_node<'b: 'a>(&mut self, node: &'b NodeRef) -> Result<()> {
    unsafe {
      (xmlXPathSetContextNode(node.as_ptr(), self.as_ptr()) == 0)
        .then_some(())
        .ok_or(Error::XPath)
    }
  }

  /// find nodes via xpath, at a specified node or the document root
  pub fn findnodes<'b: 'a>(
    &mut self,
    xpath: &str,
    node_opt: Option<&'b NodeRef>,
  ) -> Result<Vec<&'a NodeRef>> {
    let evaluated = if let Some(node) = node_opt {
      self.node_eval(xpath, node)?
    } else {
      self.eval(xpath)?
    };
    Ok(evaluated.get_nodes_as_vec())
  }

  /// find literal values via xpath, at a specified node or the document root
  pub fn findvalues<'b: 'a>(
    &mut self,
    xpath: &str,
    node_opt: Option<&'b NodeRef>,
  ) -> Result<Vec<XmlString>> {
    let evaluated = if let Some(node) = node_opt {
      self.node_eval(xpath, node)?
    } else {
      self.eval(xpath)?
    };
    Ok(evaluated.get_nodes_as_str())
  }

  /// find a literal value via xpath, at a specified node or the document root
  pub fn findvalue<'b: 'a>(
    &mut self,
    xpath: &str,
    node_opt: Option<&'b NodeRef>,
  ) -> Result<String> {
    let evaluated = if let Some(node) = node_opt {
      self.node_eval(xpath, node)?
    } else {
      self.eval(xpath)?
    };
    Ok(evaluated.to_string())
  }
}
