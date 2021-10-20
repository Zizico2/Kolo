use html5ever::tendril::StrTendril;
use html5ever::tree_builder::{ElementFlags, NodeOrText, QuirksMode, TreeSink};
use html5ever::{self, /*Attribute,*/ /*ExpandedName,*/ QualName};
use std::borrow::Cow;
use std::cell::RefCell;

mod attributes;

use crate::attributes::{Attribute, Attributes, ExpandedName};

/// Node data specific to the node type.
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
#[derive(Debug, PartialEq, Clone)]
pub struct Node {
    children: Option<Vec<Node>>,
    data: NodeData,
}

impl Node {
    pub fn new(data: NodeData) -> Self {
        let children = None;
        Node { children, data }
    }
    /// If this node is a document, return a reference to document-specific data.
    #[inline]
    pub fn as_document(&self) -> Option<&DocumentData> {
        match self.data {
            NodeData::Document(ref value) => Some(value),
            _ => None,
        }
    }
    #[inline]
    pub fn data(&self) -> &NodeData {
        &self.data
    }

    /// If this node is an element, return a reference to element-specific data.
    #[inline]
    pub fn as_element(&self) -> Option<&ElementData> {
        match self.data {
            NodeData::Element(ref value) => Some(value),
            _ => None,
        }
    }

    /// If this node is a text node, return a reference to its contents.
    #[inline]
    pub fn as_text(&self) -> Option<&RefCell<String>> {
        match self.data {
            NodeData::Text(ref value) => Some(value),
            _ => None,
        }
    }

    /// If this node is a comment, return a reference to its contents.
    #[inline]
    pub fn as_comment(&self) -> Option<&RefCell<String>> {
        match self.data {
            NodeData::Comment(ref value) => Some(value),
            _ => None,
        }
    }

    /// If this node is a document, return a reference to doctype-specific data.
    #[inline]
    pub fn as_doctype(&self) -> Option<&Doctype> {
        match self.data {
            NodeData::Doctype(ref value) => Some(value),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ElementData {
    /// The namespace and local name of the element, such as `ns!(html)` and `body`.
    pub name: QualName,

    /// The attributes of the elements.
    pub attributes: RefCell<Attributes>,

    /// If the element is an HTML `<template>` element,
    /// the document fragment node that is the root of template contents.
    pub template_contents: Option<Box<Node>>,
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

/// Options for the HTML parser.
#[derive(Default)]
pub struct ParseOpts {
    /// Options for the HTML tokenizer.
    pub tokenizer: html5ever::tokenizer::TokenizerOpts,

    /// Options for the HTML tree builder.
    pub tree_builder: html5ever::tree_builder::TreeBuilderOpts,

    /// A callback for HTML parse errors (which are never fatal).
    pub on_parse_error: Option<Box<dyn FnMut(Cow<'static, str>)>>,
}

/// Parse an HTML document with html5ever and the default configuration.
pub fn parse_html() -> html5ever::Parser<Sink> {
    parse_html_with_options(ParseOpts::default())
}

/// Parse an HTML document with html5ever with custom configuration.
pub fn parse_html_with_options(opts: ParseOpts) -> html5ever::Parser<Sink> {
    let sink = Sink {
        root_node: NodeRef::new_document(),
        on_parse_error: opts.on_parse_error,
    };
    let html5opts = html5ever::ParseOpts {
        tokenizer: opts.tokenizer,
        tree_builder: opts.tree_builder,
    };
    html5ever::parse_document(sink, html5opts)
}

/// Parse an HTML fragment with html5ever and the default configuration.
pub fn parse_fragment(ctx_name: QualName, ctx_attr: Vec<Attribute>) -> html5ever::Parser<Sink> {
    parse_fragment_with_options(ParseOpts::default(), ctx_name, ctx_attr)
}

/// Parse an HTML fragment with html5ever with custom configuration.
pub fn parse_fragment_with_options(
    opts: ParseOpts,
    ctx_name: QualName,
    ctx_attr: Vec<Attribute>,
) -> html5ever::Parser<Sink> {
    let sink = Sink {
        root_node: NodeRef::new_document(),
        on_parse_error: opts.on_parse_error,
    };
    let html5opts = html5ever::ParseOpts {
        tokenizer: opts.tokenizer,
        tree_builder: opts.tree_builder,
    };
    html5ever::parse_fragment(sink, html5opts, ctx_name, ctx_attr)
}

/// Receives new tree nodes during parsing.
pub struct Sink {
    root_node: Node,
    on_parse_error: Option<Box<dyn FnMut(Cow<'static, str>)>>,
}

impl TreeSink for Sink {
    type Output = Node;

    fn finish(self) -> Node {
        self.root_node
    }

    type Handle = Node;

    #[inline]
    fn parse_error(&mut self, message: Cow<'static, str>) {
        if let Some(ref mut handler) = self.on_parse_error {
            handler(message)
        }
    }

    #[inline]
    fn get_document(&mut self) -> Node {
        self.root_node.clone()
    }

    #[inline]
    fn set_quirks_mode(&mut self, mode: QuirksMode) {
        // self.root_node.as_document().unwrap()._quirks_mode.set(mode);
        self.root_node.as_document().unwrap()._quirks_mode = mode;
    }

    #[inline]
    fn same_node(&self, x: &Node, y: &Node) -> bool {
        x == y
    }

    #[inline]
    fn elem_name<'a>(&self, target: &'a Node) -> ExpandedName<'a> {
        target.as_element().unwrap().name.expanded()
    }

    #[inline]
    fn create_element(
        &mut self,
        name: QualName,
        attrs: Vec<Attribute>,
        _flags: ElementFlags,
    ) -> NodeRef {
        NodeRef::new_element(
            name,
            attrs.into_iter().map(|attr| {
                let Attribute {
                    name: QualName { prefix, ns, local },
                    value,
                } = attr;
                let value = String::from(value);
                (
                    attributes::ExpandedName { ns, local },
                    attributes::Attribute { prefix, value },
                )
            }),
        )
    }

    #[inline]
    fn create_comment(&mut self, text: StrTendril) -> NodeRef {
        NodeRef::new_comment(text)
    }

    #[inline]
    fn create_pi(&mut self, target: StrTendril, data: StrTendril) -> NodeRef {
        NodeRef::new_processing_instruction(target, data)
    }

    #[inline]
    fn append(&mut self, parent: &NodeRef, child: NodeOrText<NodeRef>) {
        match child {
            NodeOrText::AppendNode(node) => parent.append(node),
            NodeOrText::AppendText(text) => {
                if let Some(last_child) = parent.last_child() {
                    if let Some(existing) = last_child.as_text() {
                        existing.borrow_mut().push_str(&text);
                        return;
                    }
                }
                parent.append(NodeRef::new_text(text))
            }
        }
    }

    #[inline]
    fn append_before_sibling(&mut self, sibling: &NodeRef, child: NodeOrText<NodeRef>) {
        match child {
            NodeOrText::AppendNode(node) => sibling.insert_before(node),
            NodeOrText::AppendText(text) => {
                if let Some(previous_sibling) = sibling.previous_sibling() {
                    if let Some(existing) = previous_sibling.as_text() {
                        existing.borrow_mut().push_str(&text);
                        return;
                    }
                }
                sibling.insert_before(NodeRef::new_text(text))
            }
        }
    }

    #[inline]
    fn append_doctype_to_document(
        &mut self,
        name: StrTendril,
        public_id: StrTendril,
        system_id: StrTendril,
    ) {
        self.root_node
            .append(NodeRef::new_doctype(name, public_id, system_id))
    }

    #[inline]
    fn add_attrs_if_missing(&mut self, target: &NodeRef, attrs: Vec<Attribute>) {
        let element = target.as_element().unwrap();
        let mut attributes = element.attributes.borrow_mut();

        for Attribute {
            name: QualName { prefix, ns, local },
            value,
        } in attrs
        {
            attributes
                .map
                .entry(attributes::ExpandedName { ns, local })
                .or_insert_with(|| {
                    let value = String::from(value);
                    attributes::Attribute { prefix, value }
                });
        }
    }

    #[inline]
    fn remove_from_parent(&mut self, target: &NodeRef) {
        target.detach()
    }

    #[inline]
    fn reparent_children(&mut self, node: &NodeRef, new_parent: &NodeRef) {
        // FIXME: Can this be done more effciently in rctree,
        // by moving the whole linked list of children at once?
        for child in node.children() {
            new_parent.append(child)
        }
    }

    #[inline]
    fn mark_script_already_started(&mut self, _node: &NodeRef) {
        // FIXME: Is this useful outside of a browser?
    }

    #[inline]
    fn get_template_contents(&mut self, target: &NodeRef) -> NodeRef {
        target
            .as_element()
            .unwrap()
            .template_contents
            .clone()
            .unwrap()
    }

    fn append_based_on_parent_node(
        &mut self,
        element: &NodeRef,
        prev_element: &NodeRef,
        child: NodeOrText<NodeRef>,
    ) {
        if element.parent().is_some() {
            self.append_before_sibling(element, child)
        } else {
            self.append(prev_element, child)
        }
    }
}
fn main() {}
