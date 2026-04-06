//! Tree module tests
//!

use libxml_core::{
  parser::{HtmlParser, HtmlParserOptions},
  tree::{NodeRef, NodeType},
};

fn dfs_node(node: &NodeRef) -> i32 {
  1 + node
    .get_child_nodes()
    .into_iter()
    .map(dfs_node)
    .sum::<i32>()
}

fn dfs_element(node: &NodeRef) -> i32 {
  1 + node
    .get_child_elements()
    .into_iter()
    .map(dfs_element)
    .sum::<i32>()
}

#[test]
fn readonly_scan_test() {
  let mut parser = HtmlParser::new();
  let doc = parser
    .parse_file(
      "../tests/resources/example.html",
      None,
      None,
      HtmlParserOptions::default(),
    )
    .unwrap();

  let root = doc.get_root_element().unwrap();
  assert_eq!(root.get_name().unwrap(), "html");
  // "get_child_nodes" exhaustivity test,
  // 33 nodes, including text, comments, etc
  assert_eq!(dfs_node(root), 33);
  // "get_element_nodes" exhaustivity test,
  // 13 named element nodes in example.html
  assert_eq!(dfs_element(root), 13);

  let text = root.get_first_child().expect("first child is a text node");
  assert_eq!(text.get_name().unwrap(), "text");

  let head = root
    .get_first_element_child()
    .expect("head is first child of html");
  assert_eq!(head.get_name().unwrap(), "head");

  let mut sibling = head
    .get_next_sibling()
    .expect("head should be followed by text");
  assert_eq!(sibling.get_name().unwrap(), "text");
  while let Some(next) = sibling.get_next_sibling() {
    sibling = next;
    if next.get_type() == Some(NodeType::ElementNode) {
      break;
    }
  }
  assert_eq!(sibling.get_type(), Some(NodeType::ElementNode));
  assert_eq!(sibling.get_name().unwrap(), "body");
}
