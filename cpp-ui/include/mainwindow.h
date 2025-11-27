#ifndef MAINWINDOW_H
#define MAINWINDOW_H

#include <QMainWindow>
#include <QPlainTextEdit>
#include <QMenuBar>
#include <QToolBar>
#include <QStatusBar>
#include <QString>
#include <memory>
#include "rustbridge.h"

class CodeEditor;

class MainWindow : public QMainWindow {
    Q_OBJECT

public:
    explicit MainWindow(QWidget *parent = nullptr);
    ~MainWindow();

private slots:
    void newFile();
    void openFile();
    void saveFile();
    void saveFileAs();
    void about();
    void textChanged();

private:
    void setupUI();
    void setupMenuBar();
    void setupToolBar();
    void setupStatusBar();
    void createConnections();

    void updateHighlighting();
    void updateOutline();

    CodeEditor *editor_;
    QString currentFile_;
    bool isModified_;

    // Rust core components
    std::unique_ptr<CyberMD::Parser> parser_;
    std::unique_ptr<CyberMD::Highlighter> highlighter_;
};

#endif // MAINWINDOW_H
