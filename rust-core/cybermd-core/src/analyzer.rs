//! Document Analyzer - High-level document structure analysis
//!
//! Analyzes the AST to extract:
//! - Document outline (table of contents)
//! - Foldable regions
//! - Heading hierarchy
//! - Statistics

use crate::walker::ASTWalker;
use cybermd_ast::ASTNode;
use std::collections::HashMap;

/// Outline item for table of contents
#[derive(Debug, Clone)]
pub struct OutlineItem {
    pub level: u8,
    pub text: String,
    pub line: usize,
    pub children: Vec<OutlineItem>,
}

impl OutlineItem {
    pub fn new(level: u8, text: String, line: usize) -> Self {
        Self {
            level,
            text,
            line,
            children: Vec::new(),
        }
    }
}

/// Foldable region in the document
#[derive(Debug, Clone)]
pub struct FoldableRegion {
    pub start_line: usize,
    pub end_line: usize,
    pub region_type: String,
    pub level: u8, // For headings
}

impl FoldableRegion {
    pub fn new(start_line: usize, end_line: usize, region_type: String, level: u8) -> Self {
        Self {
            start_line,
            end_line,
            region_type,
            level,
        }
    }
}

/// Document analyzer for extracting high-level structure
pub struct DocumentAnalyzer<'a> {
    ast: &'a ASTNode,
    analyzed: bool,

    // Cached results
    outline: Vec<OutlineItem>,
    foldable_regions: Vec<FoldableRegion>,
    heading_hierarchy: HashMap<usize, Vec<usize>>,
    stats: HashMap<String, usize>,
}

impl<'a> DocumentAnalyzer<'a> {
    /// Create new analyzer for an AST
    pub fn new(ast: &'a ASTNode) -> Self {
        Self {
            ast,
            analyzed: false,
            outline: Vec::new(),
            foldable_regions: Vec::new(),
            heading_hierarchy: HashMap::new(),
            stats: HashMap::new(),
        }
    }

    /// Perform full document analysis
    pub fn analyze(&mut self) {
        self.outline = self.build_outline();
        self.foldable_regions = self.find_foldable_regions();
        self.heading_hierarchy = self.build_heading_hierarchy();
        self.stats = self.calculate_statistics();
        self.analyzed = true;
    }

    /// Build document outline from headings
    fn build_outline(&self) -> Vec<OutlineItem> {
        let mut walker = ASTWalker::new(self.ast);
        let headings = walker.find_all_headings();

        if headings.is_empty() {
            return Vec::new();
        }

        // Convert headings to outline items
        let mut items = Vec::new();
        for heading in headings {
            if let ASTNode::Heading {
                level,
                text,
                start_pos,
                ..
            } = heading
            {
                let line = start_pos.map(|p| p.line).unwrap_or(0);
                items.push(OutlineItem::new(*level, text.clone(), line));
            }
        }

        // For now, return flat list (full hierarchy building is complex)
        // TODO: Implement proper hierarchy building
        items
    }

    /// Find all foldable regions
    fn find_foldable_regions(&self) -> Vec<FoldableRegion> {
        let mut regions = Vec::new();
        let mut walker = ASTWalker::new(self.ast);

        // Find heading regions
        let headings = walker.find_all_headings();
        for (i, heading) in headings.iter().enumerate() {
            if let ASTNode::Heading {
                level,
                start_pos: Some(start),
                ..
            } = heading
            {
                let start_line = start.line;

                // Find end line (next heading of same or higher level)
                let mut end_line = start_line + 100; // Default large value

                for next_heading in &headings[i + 1..] {
                    if let ASTNode::Heading {
                        level: next_level,
                        start_pos: Some(next_start),
                        ..
                    } = next_heading
                    {
                        if *next_level <= *level {
                            end_line = next_start.line - 1;
                            break;
                        }
                    }
                }

                if end_line > start_line {
                    regions.push(FoldableRegion::new(
                        start_line,
                        end_line,
                        "heading".to_string(),
                        *level,
                    ));
                }
            }
        }

        // Find code block regions
        walker.reset_counter();
        let code_blocks = walker.find_all_code_blocks();
        for block in code_blocks {
            if let (Some(start), Some(end)) = (block.start_pos(), block.end_pos()) {
                regions.push(FoldableRegion::new(
                    start.line,
                    end.line,
                    "code_block".to_string(),
                    0,
                ));
            }
        }

        // Find list regions
        walker.reset_counter();
        let lists = walker.find_all_lists();
        for list in lists {
            if let (Some(start), Some(end)) = (list.start_pos(), list.end_pos()) {
                // Only fold if list has multiple items
                if list.children().len() > 1 {
                    regions.push(FoldableRegion::new(
                        start.line,
                        end.line,
                        "list".to_string(),
                        0,
                    ));
                }
            }
        }

        // Sort by start line
        regions.sort_by_key(|r| r.start_line);

        regions
    }

    /// Build heading hierarchy (parent-child relationships)
    fn build_heading_hierarchy(&self) -> HashMap<usize, Vec<usize>> {
        let mut walker = ASTWalker::new(self.ast);
        let headings = walker.find_all_headings();
        let mut hierarchy = HashMap::new();

        for (i, heading) in headings.iter().enumerate() {
            if let ASTNode::Heading { level, .. } = heading {
                let mut children = Vec::new();

                for (j, next_heading) in headings[i + 1..].iter().enumerate() {
                    if let ASTNode::Heading {
                        level: next_level, ..
                    } = next_heading
                    {
                        if *next_level == level + 1 {
                            // Direct child
                            children.push(i + 1 + j);
                        } else if *next_level <= *level {
                            // Sibling or parent - stop looking
                            break;
                        }
                    }
                }

                hierarchy.insert(i, children);
            }
        }

        hierarchy
    }

    /// Calculate document statistics
    fn calculate_statistics(&self) -> HashMap<String, usize> {
        let mut walker = ASTWalker::new(self.ast);
        let mut stats = HashMap::new();

        stats.insert("headings".to_string(), walker.count_nodes("heading"));
        walker.reset_counter();
        stats.insert("paragraphs".to_string(), walker.count_nodes("paragraph"));
        walker.reset_counter();
        stats.insert(
            "code_blocks".to_string(),
            walker.count_nodes("code_block"),
        );
        walker.reset_counter();
        stats.insert("lists".to_string(), walker.count_nodes("list"));

        // Find max heading level
        walker.reset_counter();
        let headings = walker.find_all_headings();
        let max_level = headings
            .iter()
            .filter_map(|h| {
                if let ASTNode::Heading { level, .. } = h {
                    Some(*level as usize)
                } else {
                    None
                }
            })
            .max()
            .unwrap_or(0);

        stats.insert("max_heading_level".to_string(), max_level);

        stats
    }

    /// Get document outline (analyzes if not done yet)
    pub fn get_outline(&mut self) -> &Vec<OutlineItem> {
        if !self.analyzed {
            self.analyze();
        }
        &self.outline
    }

    /// Get foldable regions (analyzes if not done yet)
    pub fn get_foldable_regions(&mut self) -> &Vec<FoldableRegion> {
        if !self.analyzed {
            self.analyze();
        }
        &self.foldable_regions
    }

    /// Get statistics (analyzes if not done yet)
    pub fn get_statistics(&mut self) -> &HashMap<String, usize> {
        if !self.analyzed {
            self.analyze();
        }
        &self.stats
    }

    /// Get heading hierarchy (analyzes if not done yet)
    pub fn get_heading_hierarchy(&mut self) -> &HashMap<usize, Vec<usize>> {
        if !self.analyzed {
            self.analyze();
        }
        &self.heading_hierarchy
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cybermd_ast::ASTNode;

    #[test]
    fn test_build_outline() {
        let mut doc = ASTNode::new_document();
        let _ = doc.add_child(ASTNode::new_heading(1, "Title".to_string()));
        let _ = doc.add_child(ASTNode::new_heading(2, "Subtitle".to_string()));

        let mut analyzer = DocumentAnalyzer::new(&doc);
        let outline = analyzer.get_outline();

        assert_eq!(outline.len(), 2);
        assert_eq!(outline[0].level, 1);
        assert_eq!(outline[1].level, 2);
    }

    #[test]
    fn test_find_foldable_regions() {
        let mut doc = ASTNode::new_document();
        let _ = doc.add_child(ASTNode::new_heading(1, "Title".to_string()));
        let _ = doc.add_child(ASTNode::new_code_block("rust".to_string(), "code".to_string()));

        let mut analyzer = DocumentAnalyzer::new(&doc);
        let regions = analyzer.get_foldable_regions();

        assert!(regions.len() >= 1); // At least heading should be foldable
    }

    #[test]
    fn test_statistics() {
        let mut doc = ASTNode::new_document();
        let _ = doc.add_child(ASTNode::new_heading(1, "Title".to_string()));
        let _ = doc.add_child(ASTNode::new_paragraph("Text".to_string()));
        let _ = doc.add_child(ASTNode::new_code_block("rust".to_string(), "code".to_string()));

        let mut analyzer = DocumentAnalyzer::new(&doc);
        let stats = analyzer.get_statistics();

        assert_eq!(stats["headings"], 1);
        assert_eq!(stats["paragraphs"], 1);
        assert_eq!(stats["code_blocks"], 1);
    }
}
