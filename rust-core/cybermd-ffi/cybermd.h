/*
 * CyberMD FFI - C Header File
 *
 * This header provides C-compatible declarations for the Rust core library.
 * Include this file in your C++ project to use CyberMD functionality.
 *
 * Example usage:
 *
 *   // Parse markdown
 *   CParser* parser = cybermd_parser_new();
 *   CAST* ast = cybermd_parser_parse(parser, "# Hello World");
 *
 *   // Analyze document
 *   CAnalyzer* analyzer = cybermd_analyzer_new(ast);
 *   cybermd_analyzer_analyze(analyzer);
 *   COutlineArray* outline = cybermd_analyzer_get_outline(analyzer);
 *
 *   // Cleanup
 *   cybermd_outline_array_free(outline);
 *   cybermd_analyzer_free(analyzer);
 *   cybermd_ast_free(ast);
 *   cybermd_parser_free(parser);
 */

#ifndef CYBERMD_H
#define CYBERMD_H

#ifdef __cplusplus
extern "C" {
#endif

#include <stdint.h>
#include <stddef.h>

// ============================================================================
// OPAQUE TYPES
// ============================================================================

typedef struct CParser CParser;
typedef struct CAST CAST;
typedef struct CAnalyzer CAnalyzer;
typedef struct CHighlighter CHighlighter;

// ============================================================================
// STRUCTURES
// ============================================================================

typedef struct {
    uint8_t level;
    char* text;
    size_t line;
} COutlineItem;

typedef struct {
    size_t start_line;
    size_t end_line;
    char* region_type;
    uint8_t level;
} CFoldableRegion;

typedef struct {
    size_t start_line;
    size_t start_col;
    size_t end_line;
    size_t end_col;
    uint32_t token_type;
} CHighlightRange;

typedef struct {
    COutlineItem* items;
    size_t count;
} COutlineArray;

typedef struct {
    CFoldableRegion* items;
    size_t count;
} CFoldableArray;

typedef struct {
    CHighlightRange* ranges;
    size_t count;
} CHighlightArray;

// ============================================================================
// TOKEN TYPES (for highlighting)
// ============================================================================

#define TOKEN_HEADING1 1
#define TOKEN_HEADING2 2
#define TOKEN_HEADING3 3
#define TOKEN_HEADING4 4
#define TOKEN_HEADING5 5
#define TOKEN_HEADING6 6
#define TOKEN_PARAGRAPH 7
#define TOKEN_CODE_BLOCK 8
#define TOKEN_INLINE_CODE 9
#define TOKEN_BOLD 10
#define TOKEN_ITALIC 11
#define TOKEN_LINK 12
#define TOKEN_LIST_MARKER 13

// ============================================================================
// PARSER API
// ============================================================================

/**
 * Create a new markdown parser
 * @return Parser handle (must be freed with cybermd_parser_free)
 */
CParser* cybermd_parser_new(void);

/**
 * Parse markdown text into an AST
 * @param parser Parser handle
 * @param text Markdown text (UTF-8 encoded, null-terminated)
 * @return AST handle (must be freed with cybermd_ast_free) or NULL on error
 */
CAST* cybermd_parser_parse(CParser* parser, const char* text);

/**
 * Free a parser
 * @param parser Parser handle to free
 */
void cybermd_parser_free(CParser* parser);

/**
 * Free an AST
 * @param ast AST handle to free
 */
void cybermd_ast_free(CAST* ast);

// ============================================================================
// ANALYZER API
// ============================================================================

/**
 * Create a new document analyzer
 * @param ast AST handle
 * @return Analyzer handle (must be freed with cybermd_analyzer_free) or NULL on error
 */
CAnalyzer* cybermd_analyzer_new(CAST* ast);

/**
 * Analyze the document (computes outline, foldable regions, etc.)
 * @param analyzer Analyzer handle
 */
void cybermd_analyzer_analyze(CAnalyzer* analyzer);

/**
 * Get document outline (table of contents)
 * @param analyzer Analyzer handle
 * @return Outline array (must be freed with cybermd_outline_array_free) or NULL on error
 */
COutlineArray* cybermd_analyzer_get_outline(CAnalyzer* analyzer);

/**
 * Get foldable regions
 * @param analyzer Analyzer handle
 * @return Foldable regions array (must be freed with cybermd_foldable_array_free) or NULL on error
 */
CFoldableArray* cybermd_analyzer_get_foldable_regions(CAnalyzer* analyzer);

/**
 * Free an analyzer
 * @param analyzer Analyzer handle to free
 */
void cybermd_analyzer_free(CAnalyzer* analyzer);

/**
 * Free an outline array
 * @param array Outline array to free
 */
void cybermd_outline_array_free(COutlineArray* array);

/**
 * Free a foldable regions array
 * @param array Foldable regions array to free
 */
void cybermd_foldable_array_free(CFoldableArray* array);

// ============================================================================
// HIGHLIGHTER API
// ============================================================================

/**
 * Create a new semantic highlighter with dark theme
 * @return Highlighter handle (must be freed with cybermd_highlighter_free)
 */
CHighlighter* cybermd_highlighter_new_dark(void);

/**
 * Create a new semantic highlighter with light theme
 * @return Highlighter handle (must be freed with cybermd_highlighter_free)
 */
CHighlighter* cybermd_highlighter_new_light(void);

/**
 * Get highlight ranges for an AST
 * @param highlighter Highlighter handle
 * @param ast AST handle
 * @return Highlight ranges array (must be freed with cybermd_highlight_array_free) or NULL on error
 */
CHighlightArray* cybermd_highlighter_highlight(CHighlighter* highlighter, CAST* ast);

/**
 * Free a highlighter
 * @param highlighter Highlighter handle to free
 */
void cybermd_highlighter_free(CHighlighter* highlighter);

/**
 * Free a highlight ranges array
 * @param array Highlight ranges array to free
 */
void cybermd_highlight_array_free(CHighlightArray* array);

// ============================================================================
// VERSION INFO
// ============================================================================

/**
 * Get library version string
 * @return Version string (static, do not free)
 */
const char* cybermd_version(void);

#ifdef __cplusplus
}
#endif

#endif // CYBERMD_H
