//use crate::attributes::{Attribute, Attributes, ExpandedName};
use html5ever::tendril::StrTendril;
use html5ever::tree_builder::{ElementFlags, NodeOrText, QuirksMode, TreeSink};
use html5ever::{Attribute, QualName};
use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Clone)]
pub enum NodeData {
    /// Element node
    Element(ElementData),

    /// Text node
    Text(String),

    /// Comment node
    // Comment(RefCell<String>),
    Comment(String),

    /// Processing instruction node
    //ProcessingInstruction(RefCell<(String, String)>),
    ProcessingInstruction((String, String)),

    /// Doctype node
    Doctype(Doctype),

    /// Document node
    Document(DocumentData),

    /// Document fragment node
    DocumentFragment,
}

#[derive(Debug, PartialEq)]
pub struct NodeFactory {
    current_ref: NodeRef,
    nodes: HashMap<NodeRef, Node>,
    orphan_nodes: HashSet<NodeRef>,
}

impl NodeFactory {
    pub fn new() -> Self {
        let current_ref = NodeRef(0);
        let nodes = HashMap::new();
        let orphan_nodes = HashSet::new();
        NodeFactory {
            current_ref,
            nodes,
            orphan_nodes,
        }
    }

    // could return error if "node" or "child" don't exist
    pub fn append(&mut self, parent_ref: NodeRef, new_child_ref: NodeRef) {
        //TODO:
        // add assertion to check if child has indeed been removed before being appended again.
        // a node should never be in 2 places in the tree at once. check if the internal
        // variables of Node have been reset to None

        //TODO: check if new_child_ref exists

        //TODO:  will need to match against error type or option and return some_error
        let parent = self.get_node_mut(parent_ref);

        let first_child_ref = match parent.first_child_ref {
            Some(first_child_ref) => first_child_ref,
            None => {
                parent.first_child_ref = Some(new_child_ref);
                let first_child = self.get_node_mut(new_child_ref);

                // only the first child of each node shall know who their parent is
                first_child.parent_ref = Some(parent_ref);

                return;
            }
        };

        // last_child == Some(_) && first_child == None should never happen.    the opposite is OK

        let last_child_ref = match parent.first_child_ref {
            Some(last_child_ref) => last_child_ref,
            None => {
                //? can assume first_child is Some(_). maybe add an assertion?
                parent.last_child_ref = Some(new_child_ref);

                let first_child = self.get_node_mut(first_child_ref);
                first_child.next_sibling_ref = Some(new_child_ref);

                let new_child = self.get_node_mut(new_child_ref);
                new_child.previous_sibling_ref = Some(first_child_ref);

                return;
            }
        };

        parent.last_child_ref = Some(new_child_ref);

        //TODO:  will need to match against error type or option and return some_error
        let last_child = self.get_node_mut(last_child_ref);
        last_child.next_sibling_ref = Some(new_child_ref);

        //TODO:  will need to match against error type or option and return some_error
        let new_child = self.get_node_mut(new_child_ref);
        new_child.previous_sibling_ref = Some(last_child_ref);
    }

    pub fn new_node(&mut self, data: NodeData) -> NodeRef {
        let node = Node::new(data);
        let node_ref = self.current_ref;

        self.nodes.insert(node_ref, node);
        self.orphan_nodes.insert(node_ref);

        self.current_ref = NodeRef(node_ref.0 + 1);

        node_ref
    }

    pub fn get_node(&self, node_ref: NodeRef) -> &Node {
        // should unwrap? every NodeRef in the wild will have been valid at some point.
        // if this struct exposes a way to remove nodes this has to handle the possible error.
        // if a Node has been removed there can still be NodeRefs to it in the wild
        self.nodes.get(&node_ref).unwrap()
    }

    pub fn get_node_mut(&mut self, node_ref: NodeRef) -> &mut Node {
        // should unwrap? every NodeRef in the wild will have been valid at some point.
        // if this struct exposes a way to remove nodes this has to handle the possible error.
        // if a Node has been removed there can still be NodeRefs to it in the wild
        self.nodes.get_mut(&node_ref).unwrap()
    }
    pub fn append_before_sibling(&mut self, sibling_ref: NodeRef, new_node_ref: NodeRef) {
        todo!()
    }
    pub fn remove_from_parent(&mut self, node_ref: NodeRef) {
        todo!()
    }
    fn reparent_children(&mut self, old_parent_ref: NodeRef, new_parent_ref: NodeRef) {
        //TODO: needs revision
        let old_parent = self.get_node(old_parent_ref);
        let first_child_ref = old_parent.first_child_ref;
        let last_child_ref = old_parent.last_child_ref;

        let new_parent = self.get_node_mut(new_parent_ref);
        new_parent.first_child_ref = first_child_ref;
        new_parent.last_child_ref = last_child_ref;

        let old_parent = self.get_node_mut(old_parent_ref);
        old_parent.first_child_ref = None;
        old_parent.last_child_ref = None;

        match (first_child_ref, last_child_ref) {
            (None, None) => {}
            (Some(first_child_ref), _) => {
                let first_child = self.get_node_mut(first_child_ref);
                first_child.parent_ref = Some(new_parent_ref);
            }
            _ => unreachable!(), // should never happen,
        }
    }
    pub fn append_after_sibling(&mut self, sibling_ref: NodeRef, new_node_ref: NodeRef) {
        unimplemented!()
        // this isn't necessary for my use case but it would make this struct more general purpose.
        // If I ever want to publish it is a crate this would be a worthy addition. This method's implementation
        // shouldn't change the time complexity of any other method and it should be constant itself.
        // it will increase memory usage though, since the last child of any node will have to know its parent
        // (currently, only the first child needs to)
        // it could, maybe, be enabled as a cargo feature. maybe more methods could use this treatment
    }

    /*
    pub fn get_children(&self, node: &NodeRef) -> &Option<Vec<NodeRef>> {
        let node = self.get_node(node);
        &node.children
    }

    pub fn set_children(&mut self, parent: &NodeRef, children: Vec<NodeRef>) {
        let parent = self.get_node_mut(parent);
        parent.children = Some(children);
    }

    pub fn clear_children(&mut self, node: &NodeRef) {
        let node = self.get_node_mut(node);
        node.children = None;
    }
    */
}

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash, PartialOrd, Ord)]
pub struct NodeRef(u64);

#[derive(Debug, PartialEq)]
pub struct Node {
    //TODO: figure out parent.
    //TODO: maybe Node should have no exposed methods and everything should be routed through NodeFactory (NodeTree renaming?)
    // only the "first_child" of a given node will know its parent
    // if parent = None this node could still have a parent, it just means it is not the "first_child"
    //
    // some operations need to know which nodes have parents (or which ones don't) but they don't necessarely care about who their parent is
    // these methods will need to receive a Set with all the "parented" (or orphan) nodes. This set will need to be kept by the user of a NodeFactory
    parent_ref: Option<NodeRef>,
    previous_sibling_ref: Option<NodeRef>,
    next_sibling_ref: Option<NodeRef>,
    data: NodeData,
    first_child_ref: Option<NodeRef>,
    last_child_ref: Option<NodeRef>,
}

impl Node {
    fn new(data: NodeData) -> Self {
        let previous_sibling_ref = None;
        let next_sibling_ref = None;
        let parent_ref = None;
        let first_child_ref = None;
        let last_child_ref = None;
        Node {
            parent_ref,
            previous_sibling_ref,
            next_sibling_ref,
            data,
            first_child_ref,
            last_child_ref,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ElementData {
    /// The namespace and local name of the element, such as `ns!(html)` and `body`.
    pub name: QualName,

    /// The attributes of the elements.
    pub attrs: Vec<Attribute>,
    // If the element is an HTML `<template>` element,
    // the document fragment node that is the root of template contents.
    //pub template_contents: Option<Box<Node>>,
}

/// Data specific to document nodes.
#[derive(Debug, PartialEq, Clone)]
pub struct DocumentData {
    #[doc(hidden)]
    pub _quirks_mode: QuirksMode,
}
/*
impl DocumentData {
    /// The quirks mode of the document, as determined by the HTML parser.
    #[inline]
    pub fn quirks_mode(&self) -> QuirksMode {
        self._quirks_mode
    }
}
*/

/// Data specific to doctype nodes.
#[derive(Debug, PartialEq, Clone)]
pub struct Doctype {
    /// The name of the doctype
    pub name: String,

    /// The public ID of the doctype
    pub public_id: String,

    /// The system ID of the doctype
    pub system_id: String,
}
