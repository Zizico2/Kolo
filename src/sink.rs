use html5ever::tendril::StrTendril;
use html5ever::tree_builder::{ElementFlags, NodeOrText, QuirksMode, TreeSink};
use html5ever::{self, Attribute, ExpandedName, QualName};
use std::borrow::Cow;
use std::collections::HashSet;
use std::marker::PhantomData;

//use crate::attributes;
use crate::tree::*;

/// Receives new tree nodes during parsing.
pub struct Sink {
    orphan_nodes: HashSet<NodeRef>,
    node_tree: NodeTree,
    on_parse_error: Option<Box<dyn FnMut(Cow<'static, str>)>>,
}

impl TreeSink for Sink {
    type Handle = NodeRef;
    type Output = NodeTree;

    fn finish(self) -> Self::Output {
        todo!()
    }

    fn parse_error(&mut self, msg: Cow<'static, str>) {
        todo!()
    }

    fn get_document(&mut self) -> Self::Handle {
        todo!()
    }

    fn elem_name<'b>(&'b self, target: &'b Self::Handle) -> ExpandedName<'b> {
        todo!()
    }

    fn create_element(
        &mut self,
        name: QualName,
        attrs: Vec<Attribute>,
        _flags: ElementFlags,
    ) -> Self::Handle {
        let node = self.node_tree.new_node(NodeData::Element(ElementData {
            name,
            attrs,
            //template_contents: None,
        }));
        self.orphan_nodes.insert(node);
        node
    }

    fn create_comment(&mut self, text: StrTendril) -> Self::Handle {
        todo!()
    }

    fn create_pi(&mut self, target: StrTendril, data: StrTendril) -> Self::Handle {
        todo!()
    }

    fn append(&mut self, parent: &Self::Handle, child: NodeOrText<Self::Handle>) {
        match child {
            NodeOrText::AppendNode(handle) => self.node_tree.append(*parent, handle),
            NodeOrText::AppendText(_) => todo!(),
        }
    }

    fn append_based_on_parent_node(
        &mut self,
        element: &Self::Handle,
        prev_element: &Self::Handle,
        child: NodeOrText<Self::Handle>,
    ) {
        // orphan_nodes.contains could be abstracted as a method
        if self.orphan_nodes.contains(element) {
            self.append(prev_element, child)
        } else {
            self.append_before_sibling(element, child);
        }
    }

    fn append_doctype_to_document(
        &mut self,
        name: StrTendril,
        public_id: StrTendril,
        system_id: StrTendril,
    ) {
        todo!()
    }

    fn get_template_contents(&mut self, target: &Self::Handle) -> Self::Handle {
        todo!()
    }

    fn same_node(&self, x: &Self::Handle, y: &Self::Handle) -> bool {
        x == y
    }

    fn set_quirks_mode(&mut self, mode: QuirksMode) {
        todo!()
    }

    fn append_before_sibling(
        &mut self,
        sibling: &Self::Handle,
        new_node: NodeOrText<Self::Handle>,
    ) {
        todo!()
    }

    fn add_attrs_if_missing(&mut self, target: &Self::Handle, attrs: Vec<Attribute>) {
        todo!()
    }

    fn remove_from_parent(&mut self, target: &Self::Handle) {}

    fn reparent_children(&mut self, node: &Self::Handle, new_parent: &Self::Handle) {
        todo!()
    }
}
