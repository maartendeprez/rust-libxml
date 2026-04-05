use std::{
  collections::{HashMap, HashSet},
  ffi::CString,
  ops::{Deref, DerefMut},
};

use libxml_sys::bindings::{
  xmlFreeNode, xmlGetLastChild, xmlGetNoNsProp, xmlGetNsList, xmlGetNsProp, xmlGetProp,
  xmlHasNsProp, xmlHasProp, xmlNode, xmlNodeGetContent, xmlNodePtr, xmlSearchNs, xmlSearchNsByHref,
};

use crate::{
  error::Result,
  list::XmlList,
  macros::define_wrapper_types,
  string::{XmlString, maybe_str_from_xml_str},
  tree::{AttrRef, DocumentRef, NamespaceRef, node_type::NodeType},
  xpath::XPathContext,
};

define_wrapper_types!(node, Node, NodeRef, xmlNode);

impl Drop for Node {
  fn drop(&mut self) {
    unsafe {
      xmlFreeNode(self.as_ptr());
    }
  }
}

impl PartialEq for Node {
  fn eq(&self, other: &Self) -> bool {
    self.deref() == other.deref()
  }
}

impl PartialEq for NodeRef {
  /// Two nodes are considered equal, if they point to the same xmlNode.
  fn eq(&self, other: &Self) -> bool {
    std::ptr::eq(self.as_ptr(), other.as_ptr())
  }
}
impl Eq for Node {}
impl Eq for NodeRef {}

impl Node {
  /// Immutably borrows the underlying libxml2 `xmlNodePtr` pointer
  #[deprecated(note = "use RoNode::as_ptr")]
  pub fn node_ptr(&self) -> xmlNodePtr {
    self.as_ptr()
  }
}

impl NodeRef {
  /// Returns the next sibling if it exists
  pub fn get_next_sibling(&self) -> Option<&Self> {
    unsafe { Self::maybe_from_ptr(self.0.next) }
  }

  /// Returns the previous sibling if it exists
  pub fn get_prev_sibling(&self) -> Option<&Self> {
    unsafe { Self::maybe_from_ptr(self.0.prev) }
  }

  /// Returns the first child if it exists
  pub fn get_first_child(&self) -> Option<&Self> {
    unsafe { Self::maybe_from_ptr(self.0.children) }
  }

  /// Returns the last child if it exists
  pub fn get_last_child(&self) -> Option<&Self> {
    unsafe { Self::maybe_from_ptr(xmlGetLastChild(self.as_ptr())) }
  }

  /// Iterate over next siblings
  pub fn iter_next_siblings(&self) -> impl Iterator<Item = &Self> {
    std::iter::successors(self.get_next_sibling(), |node| node.get_next_sibling())
  }

  /// Iterate over next siblings
  pub fn iter_prev_siblings(&self) -> impl Iterator<Item = &Self> {
    std::iter::successors(self.get_prev_sibling(), |node| node.get_prev_sibling())
  }

  /// Returns the next element sibling if it exists
  pub fn get_next_element_sibling(&self) -> Option<&Self> {
    self
      .iter_next_siblings()
      .find(|node| node.is_element_node())
  }

  /// Returns the previous element sibling if it exists
  pub fn get_prev_element_sibling(&self) -> Option<&Self> {
    self
      .iter_prev_siblings()
      .find(|node| node.is_element_node())
  }

  /// Returns the first element child if it exists
  pub fn get_first_element_child(&self) -> Option<&Self> {
    std::iter::successors(self.get_first_child(), |node| node.get_next_sibling())
      .find(|node| node.is_element_node())
  }

  /// Returns the last element child if it exists
  pub fn get_last_element_child(&self) -> Option<&Self> {
    std::iter::successors(self.get_last_child(), |node| node.get_prev_sibling())
      .find(|node| node.is_element_node())
  }

  /// Iterator over child nodes of this node
  pub fn iter_child_nodes(&self) -> impl Iterator<Item = &Self> {
    std::iter::successors(self.get_first_child(), |node| node.get_next_sibling())
  }

  /// Returns all child nodes of the given node as a vector
  pub fn get_child_nodes(&self) -> Vec<&Self> {
    self.iter_child_nodes().collect()
  }

  /// Returns all child elements of the given node as a vector
  pub fn get_child_elements(&self) -> Vec<&Self> {
    self
      .iter_child_nodes()
      .filter(|node| node.is_element_node())
      .collect()
  }

  /// Returns the parent if it exists
  pub fn get_parent(&self) -> Option<&Self> {
    unsafe { Self::maybe_from_ptr(self.0.parent) }
  }

  /// Get the node type
  pub fn get_type(&self) -> Option<NodeType> {
    NodeType::from_int(self.0.type_)
  }

  /// Returns true if it is a text node
  pub fn is_text_node(&self) -> bool {
    self.get_type() == Some(NodeType::TextNode)
  }

  /// Checks if the given node is an Element
  pub fn is_element_node(&self) -> bool {
    self.get_type() == Some(NodeType::ElementNode)
  }

  /// Checks if the underlying libxml2 pointer is `NULL`
  #[deprecated(note = "RoNodePtr can never be null")]
  pub fn is_null(&self) -> bool {
    false
  }

  /// Returns the name of the node
  pub fn get_name(&self) -> Option<&str> {
    unsafe { maybe_str_from_xml_str(self.0.name) }
  }

  /// Returns the content of the node
  /// (assumes UTF-8 XML document)
  pub fn get_content(&self) -> Option<XmlString> {
    unsafe {
      let content_ptr = xmlNodeGetContent(self.as_ptr());
      XmlString::maybe_new(content_ptr)
    }
  }

  /// Returns the value of property `name`
  pub fn get_property(&self, name: &str) -> Option<XmlString> {
    let c_name = CString::new(name).unwrap();
    unsafe {
      let value_ptr = xmlGetProp(self.as_ptr(), c_name.as_bytes().as_ptr());
      XmlString::maybe_new(value_ptr)
    }
  }

  /// Returns the value of property `name` in namespace `ns`
  pub fn get_property_ns(&self, name: &str, ns: &str) -> Option<XmlString> {
    let c_name = CString::new(name).unwrap();
    let c_ns = CString::new(ns).unwrap();
    unsafe {
      let value_ptr = xmlGetNsProp(
        self.as_ptr(),
        c_name.as_bytes().as_ptr(),
        c_ns.as_bytes().as_ptr(),
      );
      XmlString::maybe_new(value_ptr)
    }
  }

  /// Returns the value of property `name` with no namespace
  pub fn get_property_no_ns(&self, name: &str) -> Option<XmlString> {
    let c_name = CString::new(name).unwrap();
    unsafe {
      let value_ptr = xmlGetNoNsProp(self.as_ptr(), c_name.as_bytes().as_ptr());
      XmlString::maybe_new(value_ptr)
    }
  }

  pub fn get_property_by_node(&self, prop: &AttrRef) -> Option<XmlString> {
    unsafe {
      let value = xmlGetProp(self.as_ptr(), prop.get_name_ptr());
      XmlString::maybe_new(value)
    }
  }

  /// Return an attribute as a `Node` struct of type AttributeNode
  pub fn get_property_node(&self, name: &str) -> Option<&AttrRef> {
    let c_name = CString::new(name).unwrap();
    unsafe {
      let attr_node = xmlHasProp(self.as_ptr(), c_name.as_bytes().as_ptr());
      AttrRef::maybe_from_ptr(attr_node)
    }
  }

  /// Return an attribute in a namespace `ns` as a `Node` of type AttributeNode
  pub fn get_property_node_ns(&self, name: &str, ns: &str) -> Option<&AttrRef> {
    let c_name = CString::new(name).unwrap();
    let c_ns = CString::new(ns).unwrap();
    unsafe {
      let attr_node = xmlHasNsProp(
        self.as_ptr(),
        c_name.as_bytes().as_ptr(),
        c_ns.as_bytes().as_ptr(),
      );
      AttrRef::maybe_from_ptr(attr_node)
    }
  }

  /// Return an attribute with no namespace as a `Node` of type AttributeNode
  pub fn get_property_node_no_ns(&self, name: &str) -> Option<&AttrRef> {
    let c_name = CString::new(name).unwrap();
    unsafe {
      let attr_node = xmlHasNsProp(self.as_ptr(), c_name.as_bytes().as_ptr(), std::ptr::null());
      AttrRef::maybe_from_ptr(attr_node)
    }
  }

  /// Alias for get_property
  pub fn get_attribute(&self, name: &str) -> Option<XmlString> {
    self.get_property(name)
  }

  /// Alias for get_property_ns
  pub fn get_attribute_ns(&self, name: &str, ns: &str) -> Option<XmlString> {
    self.get_property_ns(name, ns)
  }

  /// Alias for get_property_no_ns
  pub fn get_attribute_no_ns(&self, name: &str) -> Option<XmlString> {
    self.get_property_no_ns(name)
  }

  /// Alias for get_property_node
  pub fn get_attribute_node(&self, name: &str) -> Option<&AttrRef> {
    self.get_property_node(name)
  }

  /// Alias for get_property_node_ns
  pub fn get_attribute_node_ns(&self, name: &str, ns: &str) -> Option<&AttrRef> {
    self.get_property_node_ns(name, ns)
  }

  /// Alias for get_property_node_no_ns
  pub fn get_attribute_node_no_ns(&self, name: &str) -> Option<&AttrRef> {
    self.get_property_node_no_ns(name)
  }

  pub fn iter_properties(&self) -> impl Iterator<Item = &AttrRef> {
    unsafe {
      std::iter::successors(AttrRef::maybe_from_ptr(self.0.properties), |attr| {
        attr.get_next()
      })
    }
  }

  /// Get a copy of the attributes of this node
  pub fn get_properties(&self) -> HashMap<&str, Option<XmlString>> {
    self
      .iter_properties()
      .map(|prop| (prop.get_name(), self.get_property_by_node(prop)))
      .collect()
  }

  /// Get a copy of the attributes of this node
  pub fn get_properties_owned(&self) -> HashMap<String, Option<XmlString>> {
    self
      .iter_properties()
      .map(|prop| (prop.get_name().to_string(), self.get_property_by_node(prop)))
      .collect()
  }

  /// Get a copy of this node's attributes and their namespaces
  pub fn get_properties_ns(&self) -> HashMap<(&str, Option<&NamespaceRef>), Option<XmlString>> {
    self
      .iter_properties()
      .map(|prop| {
        let name = prop.get_name();
        let ns = prop.get_namespace();
        let value = match ns {
          Some(ns) => self.get_property_ns(name, ns.get_href().unwrap_or("")),
          None => self.get_property_no_ns(name),
        };
        ((name, ns), value)
      })
      .collect()
  }

  /// Alias for `get_properties`
  pub fn get_attributes(&self) -> HashMap<&str, Option<XmlString>> {
    self.get_properties()
  }

  /// Alias for `get_properties_ns`
  pub fn get_attributes_ns(&self) -> HashMap<(&str, Option<&NamespaceRef>), Option<XmlString>> {
    self.get_properties_ns()
  }

  /// Check if a property has been defined, without allocating its value
  pub fn has_property(&self, name: &str) -> bool {
    self.get_property_node(name).is_some()
  }

  /// Check if property `name` in namespace `ns` exists
  pub fn has_property_ns(&self, name: &str, ns: &str) -> bool {
    self.get_property_node_ns(name, ns).is_some()
  }

  /// Check if property `name` with no namespace exists
  pub fn has_property_no_ns(&self, name: &str) -> bool {
    self.get_property_node_no_ns(name).is_some()
  }

  /// Alias for has_property
  pub fn has_attribute(&self, name: &str) -> bool {
    self.has_property(name)
  }

  /// Alias for has_property_ns
  pub fn has_attribute_ns(&self, name: &str, ns: &str) -> bool {
    self.has_property_ns(name, ns)
  }

  /// Alias for has_property_no_ns
  pub fn has_attribute_no_ns(&self, name: &str) -> bool {
    self.has_property_no_ns(name)
  }

  /// Gets the active namespace associated to this node
  pub fn get_namespace(&self) -> Option<&NamespaceRef> {
    unsafe { NamespaceRef::maybe_from_ptr(self.0.ns) }
  }

  /// Gets a list of namespaces associated with this node
  pub fn get_namespaces(&self, doc: &DocumentRef) -> Option<XmlList<&NamespaceRef>> {
    unsafe {
      let ns_list = xmlGetNsList(doc.as_ptr(), self.as_ptr());
      // TODO: see comment in RoNode::get_namespaces on deallocation.
      XmlList::maybe_from_ptr(ns_list)
    }
  }

  /// Get a list of namespaces declared with this node
  pub fn get_namespace_declarations(&self) -> Vec<&NamespaceRef> {
    if !self.is_element_node() {
      // only element nodes can have declarations
      return Vec::new();
    }

    std::iter::successors(
      unsafe { NamespaceRef::maybe_from_ptr(self.0.nsDef) },
      |ns| ns.get_next(),
    )
    .filter(|ns| ns.get_prefix().is_some() || ns.get_href().is_some())
    .collect()
  }

  /// Looks up the prefix of a namespace from its URI, based around a given `Node`
  pub fn lookup_namespace_prefix(&self, href: &str) -> Option<&str> {
    if href.is_empty() {
      return None;
    }
    let c_href = CString::new(href).unwrap();
    let ns = unsafe {
      let ns_ptr = xmlSearchNsByHref(self.0.doc, self.as_ptr(), c_href.as_bytes().as_ptr());
      NamespaceRef::maybe_from_ptr(ns_ptr)?
    };
    ns.get_prefix()
  }

  /// Looks up the uri of a namespace from its prefix, based around a given `Node`
  pub fn lookup_namespace_uri(&self, prefix: &str) -> Option<&str> {
    if prefix.is_empty() {
      return None;
    }
    let c_prefix = CString::new(prefix).unwrap();
    let ns = unsafe {
      let ns_ptr = xmlSearchNs(self.0.doc, self.as_ptr(), c_prefix.as_bytes().as_ptr());
      NamespaceRef::maybe_from_ptr(ns_ptr)?
    };
    ns.get_href()
  }

  /// Get a set of class names from this node's attributes
  pub fn get_class_names(&self) -> HashSet<String> {
    let mut set = HashSet::new();
    if let Some(value) = self.get_property("class") {
      for n in value.split(' ') {
        set.insert(n.to_owned());
      }
    }
    set
  }

  /// find read-only nodes via xpath, at the specified node and a given document
  pub fn findnodes<'a, 'b: 'a>(
    &'b self,
    xpath: &str,
    owner: &'a DocumentRef,
  ) -> Result<Vec<&'a Self>> {
    let mut context = XPathContext::new(owner)?;
    let evaluated = context.node_eval(xpath, self)?;
    Ok(evaluated.get_nodes_as_vec())
  }
}
