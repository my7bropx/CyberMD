//! # CyberMD FFI - C Foreign Function Interface
//!
//! This module provides C-compatible bindings for the Rust core library.
//! The C++ UI layer calls these functions.
//!
//! ## Safety
//!
//! All exported functions use `unsafe` and require careful memory management.
//! The C++ side must free resources using the provided free functions.

use cybermd_core::{DocumentAnalyzer, ASTWalker};
use cybermd_highlighter::{SemanticHighlighter, ColorTheme, HighlightRange, TokenType};
use cybermd_parser::MarkdownParser;
use cybermd_ast::ASTNode;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;

// ============================================================================
// OPAQUE TYPES
// ============================================================================

/// Opaque parser handle
pub struct CParser {
    parser: MarkdownParser,
}

/// Opaque AST handle
pub struct CAST {
    ast: ASTNode,
}

/// Opaque analyzer handle
pub struct CAnalyzer {
    analyzer: DocumentAnalyzer<'static>,
    // Need to box the AST to make it 'static
    _ast_box: Box<ASTNode>,
}

/// Opaque highlighter handle
pub struct CHighlighter {
    highlighter: SemanticHighlighter,
}

// ============================================================================
// C-COMPATIBLE STRUCTURES
// ============================================================================

/// Outline item for C
#[repr(C)]
pub struct COutlineItem {
    pub level: u8,
    pub text: *mut c_char,
    pub line: usize,
}

/// Foldable region for C
#[repr(C)]
pub struct CFoldableRegion {
    pub start_line: usize,
    pub end_line: usize,
    pub region_type: *mut c_char,
    pub level: u8,
}

/// Highlight range for C
#[repr(C)]
pub struct CHighlightRange {
    pub start_line: usize,
    pub start_col: usize,
    pub end_line: usize,
    pub end_col: usize,
    pub token_type: u32, // TokenType as integer
}

/// Array of outline items
#[repr(C)]
pub struct COutlineArray {
    pub items: *mut COutlineItem,
    pub count: usize,
}

/// Array of foldable regions
#[repr(C)]
pub struct CFoldableArray {
    pub items: *mut CFoldableRegion,
    pub count: usize,
}

/// Array of highlight ranges
#[repr(C)]
pub struct CHighlightArray {
    pub ranges: *mut CHighlightRange,
    pub count: usize,
}

// ============================================================================
// PARSER API
// ============================================================================

/// Create a new parser
#[no_mangle]
pub extern "C" fn cybermd_parser_new() -> *mut CParser {
    let parser = CParser {
        parser: MarkdownParser::new(),
    };
    Box::into_raw(Box::new(parser))
}

/// Parse markdown text into AST
#[no_mangle]
pub unsafe extern "C" fn cybermd_parser_parse(
    parser: *mut CParser,
    text: *const c_char,
) -> *mut CAST {
    if parser.is_null() || text.is_null() {
        return ptr::null_mut();
    }

    let c_str = match CStr::from_ptr(text).to_str() {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };

    let parser = &mut *parser;
    let ast = parser.parser.parse(c_str);

    let cast = CAST { ast };
    Box::into_raw(Box::new(cast))
}

/// Free parser
#[no_mangle]
pub unsafe extern "C" fn cybermd_parser_free(parser: *mut CParser) {
    if !parser.is_null() {
        let _ = Box::from_raw(parser);
    }
}

/// Free AST
#[no_mangle]
pub unsafe extern "C" fn cybermd_ast_free(ast: *mut CAST) {
    if !ast.is_null() {
        let _ = Box::from_raw(ast);
    }
}

// ============================================================================
// ANALYZER API
// ============================================================================

/// Create a new document analyzer
#[no_mangle]
pub unsafe extern "C" fn cybermd_analyzer_new(ast: *mut CAST) -> *mut CAnalyzer {
    if ast.is_null() {
        return ptr::null_mut();
    }

    // Box the AST to make it 'static
    let ast_ref = &(*ast).ast;
    let ast_box = Box::new(ast_ref.clone());
    let ast_static: &'static ASTNode = Box::leak(ast_box.clone());

    let analyzer = CAnalyzer {
        analyzer: DocumentAnalyzer::new(ast_static),
        _ast_box: ast_box,
    };

    Box::into_raw(Box::new(analyzer))
}

/// Analyze the document
#[no_mangle]
pub unsafe extern "C" fn cybermd_analyzer_analyze(analyzer: *mut CAnalyzer) {
    if !analyzer.is_null() {
        (*analyzer).analyzer.analyze();
    }
}

/// Get outline
#[no_mangle]
pub unsafe extern "C" fn cybermd_analyzer_get_outline(
    analyzer: *mut CAnalyzer,
) -> *mut COutlineArray {
    if analyzer.is_null() {
        return ptr::null_mut();
    }

    let outline = (*analyzer).analyzer.get_outline();
    let mut c_items = Vec::with_capacity(outline.len());

    for item in outline.iter() {
        let text = match CString::new(item.text.clone()) {
            Ok(s) => s.into_raw(),
            Err(_) => continue,
        };

        c_items.push(COutlineItem {
            level: item.level,
            text,
            line: item.line,
        });
    }

    let array = COutlineArray {
        items: c_items.as_mut_ptr(),
        count: c_items.len(),
    };

    std::mem::forget(c_items);
    Box::into_raw(Box::new(array))
}

/// Get foldable regions
#[no_mangle]
pub unsafe extern "C" fn cybermd_analyzer_get_foldable_regions(
    analyzer: *mut CAnalyzer,
) -> *mut CFoldableArray {
    if analyzer.is_null() {
        return ptr::null_mut();
    }

    let regions = (*analyzer).analyzer.get_foldable_regions();
    let mut c_items = Vec::with_capacity(regions.len());

    for region in regions.iter() {
        let region_type = match CString::new(region.region_type.clone()) {
            Ok(s) => s.into_raw(),
            Err(_) => continue,
        };

        c_items.push(CFoldableRegion {
            start_line: region.start_line,
            end_line: region.end_line,
            region_type,
            level: region.level,
        });
    }

    let array = CFoldableArray {
        items: c_items.as_mut_ptr(),
        count: c_items.len(),
    };

    std::mem::forget(c_items);
    Box::into_raw(Box::new(array))
}

/// Free analyzer
#[no_mangle]
pub unsafe extern "C" fn cybermd_analyzer_free(analyzer: *mut CAnalyzer) {
    if !analyzer.is_null() {
        let _ = Box::from_raw(analyzer);
    }
}

/// Free outline array
#[no_mangle]
pub unsafe extern "C" fn cybermd_outline_array_free(array: *mut COutlineArray) {
    if !array.is_null() {
        let array = Box::from_raw(array);
        let items = Vec::from_raw_parts(array.items, array.count, array.count);
        for item in items {
            if !item.text.is_null() {
                let _ = CString::from_raw(item.text);
            }
        }
    }
}

/// Free foldable array
#[no_mangle]
pub unsafe extern "C" fn cybermd_foldable_array_free(array: *mut CFoldableArray) {
    if !array.is_null() {
        let array = Box::from_raw(array);
        let items = Vec::from_raw_parts(array.items, array.count, array.count);
        for item in items {
            if !item.region_type.is_null() {
                let _ = CString::from_raw(item.region_type);
            }
        }
    }
}

// ============================================================================
// HIGHLIGHTER API
// ============================================================================

/// Create a new semantic highlighter (dark theme)
#[no_mangle]
pub extern "C" fn cybermd_highlighter_new_dark() -> *mut CHighlighter {
    let highlighter = CHighlighter {
        highlighter: SemanticHighlighter::dark(),
    };
    Box::into_raw(Box::new(highlighter))
}

/// Create a new semantic highlighter (light theme)
#[no_mangle]
pub extern "C" fn cybermd_highlighter_new_light() -> *mut CHighlighter {
    let highlighter = CHighlighter {
        highlighter: SemanticHighlighter::light(),
    };
    Box::into_raw(Box::new(highlighter))
}

/// Get highlight ranges for AST
#[no_mangle]
pub unsafe extern "C" fn cybermd_highlighter_highlight(
    highlighter: *mut CHighlighter,
    ast: *mut CAST,
) -> *mut CHighlightArray {
    if highlighter.is_null() || ast.is_null() {
        return ptr::null_mut();
    }

    let ranges = (*highlighter).highlighter.highlight(&(*ast).ast);
    let mut c_ranges = Vec::with_capacity(ranges.len());

    for range in ranges {
        let token_type = match range.token_type {
            TokenType::Heading1 => 1,
            TokenType::Heading2 => 2,
            TokenType::Heading3 => 3,
            TokenType::Heading4 => 4,
            TokenType::Heading5 => 5,
            TokenType::Heading6 => 6,
            TokenType::Paragraph => 7,
            TokenType::CodeBlock => 8,
            TokenType::InlineCode => 9,
            TokenType::Bold => 10,
            TokenType::Italic => 11,
            TokenType::Link => 12,
            TokenType::ListMarker => 13,
        };

        c_ranges.push(CHighlightRange {
            start_line: range.start_line,
            start_col: range.start_col,
            end_line: range.end_line,
            end_col: range.end_col,
            token_type,
        });
    }

    let array = CHighlightArray {
        ranges: c_ranges.as_mut_ptr(),
        count: c_ranges.len(),
    };

    std::mem::forget(c_ranges);
    Box::into_raw(Box::new(array))
}

/// Free highlighter
#[no_mangle]
pub unsafe extern "C" fn cybermd_highlighter_free(highlighter: *mut CHighlighter) {
    if !highlighter.is_null() {
        let _ = Box::from_raw(highlighter);
    }
}

/// Free highlight array
#[no_mangle]
pub unsafe extern "C" fn cybermd_highlight_array_free(array: *mut CHighlightArray) {
    if !array.is_null() {
        let array = Box::from_raw(array);
        let _ = Vec::from_raw_parts(array.ranges, array.count, array.count);
    }
}

// ============================================================================
// VERSION INFO
// ============================================================================

/// Get library version
#[no_mangle]
pub extern "C" fn cybermd_version() -> *const c_char {
    static VERSION: &str = "0.1.0\0";
    VERSION.as_ptr() as *const c_char
}
