use crate::node::kind::NodeKind as NodeKind;

#[derive(Clone)]
pub struct Node {
  pub kind: NodeKind,
  pub lhs: Option<usize>,
  pub rhs: Option<usize>,
}
#[derive(Clone)]
pub struct NodeArray {
  pub nodes: Vec<Node>,
  pub idx: usize,
}
impl NodeArray {
  pub fn new_node(&mut self, kind: NodeKind, lhs: Option<usize>, rhs: Option<usize>) -> usize {
    self.nodes.push(Node{
      kind: kind,
      lhs: lhs,
      rhs: rhs,
    });
    self.nodes.len() - 1
  }
  pub fn new_node_usize(&mut self, kind: NodeKind, lhs: usize, rhs: usize) -> usize {
    self.nodes.push(Node{
      kind: kind,
      lhs: Some(lhs),
      rhs: Some(rhs),
    });
    self.nodes.len() - 1
  }
  pub fn new_node_num(&mut self, num: i64) -> usize {
    self.new_node(NodeKind::NUM(num), None, None)
  }
}