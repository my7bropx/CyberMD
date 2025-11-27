// RustBridge - C++ wrapper for Rust FFI
// Provides RAII and C++ convenience for the Rust core library

#ifndef RUSTBRIDGE_H
#define RUSTBRIDGE_H

#include "../../rust-core/cybermd-ffi/cybermd.h"
#include <string>
#include <vector>
#include <memory>

namespace CyberMD {

// Forward declarations
class Parser;
class AST;
class Analyzer;
class Highlighter;

// ============================================================================
// RAII Wrappers
// ============================================================================

class Parser {
public:
    Parser() : handle_(cybermd_parser_new()) {}
    ~Parser() { if (handle_) cybermd_parser_free(handle_); }

    // Prevent copying
    Parser(const Parser&) = delete;
    Parser& operator=(const Parser&) = delete;

    // Allow moving
    Parser(Parser&& other) noexcept : handle_(other.handle_) {
        other.handle_ = nullptr;
    }

    Parser& operator=(Parser&& other) noexcept {
        if (this != &other) {
            if (handle_) cybermd_parser_free(handle_);
            handle_ = other.handle_;
            other.handle_ = nullptr;
        }
        return *this;
    }

    CAST* parse(const std::string& text) {
        return cybermd_parser_parse(handle_, text.c_str());
    }

private:
    CParser* handle_;
};

class AST {
public:
    explicit AST(CAST* handle) : handle_(handle) {}
    ~AST() { if (handle_) cybermd_ast_free(handle_); }

    // Prevent copying
    AST(const AST&) = delete;
    AST& operator=(const AST&) = delete;

    // Allow moving
    AST(AST&& other) noexcept : handle_(other.handle_) {
        other.handle_ = nullptr;
    }

    AST& operator=(AST&& other) noexcept {
        if (this != &other) {
            if (handle_) cybermd_ast_free(handle_);
            handle_ = other.handle_;
            other.handle_ = nullptr;
        }
        return *this;
    }

    CAST* get() const { return handle_; }

private:
    CAST* handle_;
};

struct OutlineItem {
    uint8_t level;
    std::string text;
    size_t line;
};

struct FoldableRegion {
    size_t start_line;
    size_t end_line;
    std::string region_type;
    uint8_t level;
};

struct HighlightRange {
    size_t start_line;
    size_t start_col;
    size_t end_line;
    size_t end_col;
    uint32_t token_type;
};

class Analyzer {
public:
    explicit Analyzer(CAST* ast) : handle_(cybermd_analyzer_new(ast)) {}
    ~Analyzer() { if (handle_) cybermd_analyzer_free(handle_); }

    // Prevent copying
    Analyzer(const Analyzer&) = delete;
    Analyzer& operator=(const Analyzer&) = delete;

    void analyze() {
        cybermd_analyzer_analyze(handle_);
    }

    std::vector<OutlineItem> get_outline() {
        std::vector<OutlineItem> result;
        COutlineArray* array = cybermd_analyzer_get_outline(handle_);

        if (array) {
            for (size_t i = 0; i < array->count; ++i) {
                result.push_back({
                    array->items[i].level,
                    std::string(array->items[i].text),
                    array->items[i].line
                });
            }
            cybermd_outline_array_free(array);
        }

        return result;
    }

    std::vector<FoldableRegion> get_foldable_regions() {
        std::vector<FoldableRegion> result;
        CFoldableArray* array = cybermd_analyzer_get_foldable_regions(handle_);

        if (array) {
            for (size_t i = 0; i < array->count; ++i) {
                result.push_back({
                    array->items[i].start_line,
                    array->items[i].end_line,
                    std::string(array->items[i].region_type),
                    array->items[i].level
                });
            }
            cybermd_foldable_array_free(array);
        }

        return result;
    }

private:
    CAnalyzer* handle_;
};

class Highlighter {
public:
    enum class Theme { Dark, Light };

    explicit Highlighter(Theme theme = Theme::Dark) {
        handle_ = (theme == Theme::Dark)
            ? cybermd_highlighter_new_dark()
            : cybermd_highlighter_new_light();
    }

    ~Highlighter() { if (handle_) cybermd_highlighter_free(handle_); }

    // Prevent copying
    Highlighter(const Highlighter&) = delete;
    Highlighter& operator=(const Highlighter&) = delete;

    std::vector<HighlightRange> highlight(CAST* ast) {
        std::vector<HighlightRange> result;
        CHighlightArray* array = cybermd_highlighter_highlight(handle_, ast);

        if (array) {
            for (size_t i = 0; i < array->count; ++i) {
                result.push_back({
                    array->ranges[i].start_line,
                    array->ranges[i].start_col,
                    array->ranges[i].end_line,
                    array->ranges[i].end_col,
                    array->ranges[i].token_type
                });
            }
            cybermd_highlight_array_free(array);
        }

        return result;
    }

private:
    CHighlighter* handle_;
};

} // namespace CyberMD

#endif // RUSTBRIDGE_H
