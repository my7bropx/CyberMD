//! AST Walker - Tree Traversal and Node Extraction
//!
//! Provides utilities for traversing and querying the AST.
//!
//! ## Features
//!
//! - Visitor pattern for AST traversal
//! - Query methods to find specific node types
//! - Pre-order and post-order traversal support
//! - Position-based node lookup

use cybermd_ast::{ASTNode, Position};
use std::collections::HashSet;

/// AST Walker for traversing and querying the tree
pub struct ASTWalker<'a> {
    ast: &'a ASTNode,
    nodes_visited: usize,
}

impl<'a> ASTWalker<'a> {
    /// Create a new AST walker
    pub fn new(ast: &'a ASTNode) -> Self {
        Self {
            ast,
            nodes_visited: 0,
        }
    }

    /// Visit each node in the tree with a visitor function
    ///
    /// # Arguments
    ///
    /// * `node` - Starting node
    /// * `visitor_func` - Function to call for each node
    /// * `post_order` - If true, visit children before node (default: pre-order)
    ///
    /// # Example
    ///
    /// ```ignore
    /// walker.visit(ast, |node| {
    ///     println!("Found: {}", node);
    /// }, false);
    /// ```
    pub fn visit<F>(&mut self, node: &'a ASTNode, mut visitor_func: F, post_order: bool)
    where
        F: FnMut(&'a ASTNode),
    {
        self.visit_internal(node, &mut visitor_func, post_order, &mut HashSet::new(), 0);
    }

    fn visit_internal<F>(
        &mut self,
        node: &'a ASTNode,
        visitor_func: &mut F,
        post_order: bool,
        visited: &mut HashSet<*const ASTNode>,
        depth: usize,
    ) where
        F: FnMut(&'a ASTNode),
    {
        // Prevent infinite recursion
        if depth > 1000 {
            return;
        }

        // Check for circular references
        let node_ptr = node as *const ASTNode;
        if visited.contains(&node_ptr) {
            return;
        }
        visited.insert(node_ptr);

        // Pre-order: visit node first
        if !post_order {
            visitor_func(node);
            self.nodes_visited += 1;
        }

        // Visit all children
        for child in node.children() {
            self.visit_internal(child, visitor_func, post_order, visited, depth + 1);
        }

        // Post-order: visit node after children
        if post_order {
            visitor_func(node);
            self.nodes_visited += 1;
        }
    }

    /// Find all nodes of a specific type
    ///
    /// # Example
    ///
    /// ```ignore
    /// let headings = walker.find_all("heading");
    /// ```
    pub fn find_all(&mut self, node_type: &str) -> Vec<&'a ASTNode> {
        let mut results = Vec::new();

        self.visit(
            self.ast,
            |node| {
                if node.node_type() == node_type {
                    results.push(node);
                }
            },
            false,
        );

        results
    }

    /// Find all heading nodes
    pub fn find_all_headings(&mut self) -> Vec<&'a ASTNode> {
        self.find_all("heading")
    }

    /// Find all code block nodes
    pub fn find_all_code_blocks(&mut self) -> Vec<&'a ASTNode> {
        self.find_all("code_block")
    }

    /// Find all paragraph nodes
    pub fn find_all_paragraphs(&mut self) -> Vec<&'a ASTNode> {
        self.find_all("paragraph")
    }

    /// Find all list nodes
    pub fn find_all_lists(&mut self) -> Vec<&'a ASTNode> {
        self.find_all("list")
    }

    /// Find the node at a specific position in the document
    ///
    /// Useful for click-to-navigate and cursor tracking.
    ///
    /// # Arguments
    ///
    /// * `line` - Line number (0-based)
    /// * `column` - Column number (0-based)
    pub fn find_by_position(&mut self, line: usize, column: usize) -> Option<&'a ASTNode> {
        let mut result = None;

        self.visit(
            self.ast,
            |node| {
                if let (Some(start), Some(end)) = (node.start_pos(), node.end_pos()) {
                    if start.line <= line && line <= end.line {
                        // Check column only if on start or end line
                        let in_range = if line == start.line && line == end.line {
                            start.column <= column && column <= end.column
                        } else if line == start.line {
                            column >= start.column
                        } else if line == end.line {
                            column <= end.column
                        } else {
                            true
                        };

                        if in_range {
                            result = Some(node);
                        }
                    }
                }
            },
            false,
        );

        result
    }

    /// Find all nodes matching a predicate function
    ///
    /// Most flexible query method.
    ///
    /// # Example
    ///
    /// ```ignore
    /// // Find all headings with level >= 2
    /// let subheadings = walker.filter_nodes(|node| {
    ///     matches!(node, ASTNode::Heading { level, .. } if *level >= 2)
    /// });
    /// ```
    pub fn filter_nodes<P>(&mut self, mut predicate: P) -> Vec<&'a ASTNode>
    where
        P: FnMut(&ASTNode) -> bool,
    {
        let mut results = Vec::new();

        self.visit(
            self.ast,
            |node| {
                if predicate(node) {
                    results.push(node);
                }
            },
            false,
        );

        results
    }

    /// Count nodes of a specific type
    pub fn count_nodes(&mut self, node_type: &str) -> usize {
        let mut count = 0;

        self.visit(
            self.ast,
            |node| {
                if node.node_type() == node_type {
                    count += 1;
                }
            },
            false,
        );

        count
    }

    /// Get total number of nodes visited in last traversal
    pub fn nodes_visited(&self) -> usize {
        self.nodes_visited
    }

    /// Reset visit counter
    pub fn reset_counter(&mut self) {
        self.nodes_visited = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cybermd_ast::ASTNode;

    #[test]
    fn test_find_headings() {
        let mut doc = ASTNode::new_document();
        let _ = doc.add_child(ASTNode::new_heading(1, "Title".to_string()));
        let _ = doc.add_child(ASTNode::new_paragraph("Text".to_string()));
        let _ = doc.add_child(ASTNode::new_heading(2, "Subtitle".to_string()));

        let mut walker = ASTWalker::new(&doc);
        let headings = walker.find_all_headings();

        assert_eq!(headings.len(), 2);
    }

    #[test]
    fn test_count_nodes() {
        let mut doc = ASTNode::new_document();
        let _ = doc.add_child(ASTNode::new_heading(1, "Title".to_string()));
        let _ = doc.add_child(ASTNode::new_paragraph("Text".to_string()));

        let mut walker = ASTWalker::new(&doc);
        let count = walker.count_nodes("heading");

        assert_eq!(count, 1);
    }

    #[test]
    fn test_filter_nodes() {
        let mut doc = ASTNode::new_document();
        let _ = doc.add_child(ASTNode::new_heading(1, "H1".to_string()));
        let _ = doc.add_child(ASTNode::new_heading(2, "H2".to_string()));
        let _ = doc.add_child(ASTNode::new_heading(3, "H3".to_string()));

        let mut walker = ASTWalker::new(&doc);
        let subheadings = walker.filter_nodes(|node| {
            matches!(node, ASTNode::Heading { level, .. } if *level >= 2)
        });

        assert_eq!(subheadings.len(), 2);
    }
}
