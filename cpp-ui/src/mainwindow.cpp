#include "mainwindow.h"
#include "codeeditor.h"
#include <QFileDialog>
#include <QMessageBox>
#include <QFile>
#include <QTextStream>
#include <QAction>
#include <QMenu>
#include <QTimer>

MainWindow::MainWindow(QWidget *parent)
    : QMainWindow(parent),
      editor_(nullptr),
      isModified_(false),
      parser_(std::make_unique<CyberMD::Parser>()),
      highlighter_(std::make_unique<CyberMD::Highlighter>(CyberMD::Highlighter::Theme::Dark))
{
    setupUI();
    setupMenuBar();
    setupToolBar();
    setupStatusBar();
    createConnections();

    setWindowTitle("CyberMD - Markdown Editor");
    currentFile_ = QString();
}

MainWindow::~MainWindow() {
    // Unique pointers handle cleanup automatically
}

void MainWindow::setupUI() {
    editor_ = new CodeEditor(this);
    setCentralWidget(editor_);

    // Set monospace font
    QFont font("Consolas", 11);
    font.setStyleHint(QFont::Monospace);
    editor_->setFont(font);

    // Set tab width to 4 spaces
    QFontMetrics metrics(font);
    editor_->setTabStopDistance(4 * metrics.horizontalAdvance(' '));
}

void MainWindow::setupMenuBar() {
    // File menu
    QMenu *fileMenu = menuBar()->addMenu("&File");

    QAction *newAction = fileMenu->addAction("&New");
    newAction->setShortcut(QKeySequence::New);
    connect(newAction, &QAction::triggered, this, &MainWindow::newFile);

    QAction *openAction = fileMenu->addAction("&Open...");
    openAction->setShortcut(QKeySequence::Open);
    connect(openAction, &QAction::triggered, this, &MainWindow::openFile);

    QAction *saveAction = fileMenu->addAction("&Save");
    saveAction->setShortcut(QKeySequence::Save);
    connect(saveAction, &QAction::triggered, this, &MainWindow::saveFile);

    QAction *saveAsAction = fileMenu->addAction("Save &As...");
    saveAsAction->setShortcut(QKeySequence::SaveAs);
    connect(saveAsAction, &QAction::triggered, this, &MainWindow::saveFileAs);

    fileMenu->addSeparator();

    QAction *quitAction = fileMenu->addAction("&Quit");
    quitAction->setShortcut(QKeySequence::Quit);
    connect(quitAction, &QAction::triggered, this, &QWidget::close);

    // Help menu
    QMenu *helpMenu = menuBar()->addMenu("&Help");

    QAction *aboutAction = helpMenu->addAction("&About");
    connect(aboutAction, &QAction::triggered, this, &MainWindow::about);
}

void MainWindow::setupToolBar() {
    QToolBar *toolbar = addToolBar("Main Toolbar");
    toolbar->setMovable(false);

    // Add toolbar actions here if needed
}

void MainWindow::setupStatusBar() {
    statusBar()->showMessage("Ready");
}

void MainWindow::createConnections() {
    connect(editor_, &QPlainTextEdit::textChanged, this, &MainWindow::textChanged);
}

void MainWindow::newFile() {
    if (isModified_) {
        QMessageBox::StandardButton reply;
        reply = QMessageBox::question(this, "Unsaved Changes",
                                     "Do you want to save your changes?",
                                     QMessageBox::Save | QMessageBox::Discard | QMessageBox::Cancel);

        if (reply == QMessageBox::Save) {
            saveFile();
        } else if (reply == QMessageBox::Cancel) {
            return;
        }
    }

    editor_->clear();
    currentFile_ = QString();
    isModified_ = false;
    setWindowTitle("CyberMD - Markdown Editor");
    statusBar()->showMessage("New file created");
}

void MainWindow::openFile() {
    QString fileName = QFileDialog::getOpenFileName(this,
        "Open Markdown File", QString(),
        "Markdown Files (*.md *.markdown);;All Files (*)");

    if (fileName.isEmpty())
        return;

    QFile file(fileName);
    if (!file.open(QIODevice::ReadOnly | QIODevice::Text)) {
        QMessageBox::critical(this, "Error", "Could not open file: " + fileName);
        return;
    }

    QTextStream in(&file);
    editor_->setPlainText(in.readAll());
    file.close();

    currentFile_ = fileName;
    isModified_ = false;
    setWindowTitle("CyberMD - " + QFileInfo(fileName).fileName());
    statusBar()->showMessage("File opened: " + fileName);
}

void MainWindow::saveFile() {
    if (currentFile_.isEmpty()) {
        saveFileAs();
        return;
    }

    QFile file(currentFile_);
    if (!file.open(QIODevice::WriteOnly | QIODevice::Text)) {
        QMessageBox::critical(this, "Error", "Could not save file: " + currentFile_);
        return;
    }

    QTextStream out(&file);
    out << editor_->toPlainText();
    file.close();

    isModified_ = false;
    statusBar()->showMessage("File saved: " + currentFile_);
}

void MainWindow::saveFileAs() {
    QString fileName = QFileDialog::getSaveFileName(this,
        "Save Markdown File", QString(),
        "Markdown Files (*.md);;All Files (*)");

    if (fileName.isEmpty())
        return;

    currentFile_ = fileName;
    saveFile();
    setWindowTitle("CyberMD - " + QFileInfo(fileName).fileName());
}

void MainWindow::about() {
    QMessageBox::about(this, "About CyberMD",
        "<h2>CyberMD Markdown Editor</h2>"
        "<p>Version 0.1.0</p>"
        "<p>A modern markdown editor with Rust core and Qt UI.</p>"
        "<p><b>Features:</b></p>"
        "<ul>"
        "<li>Fast Rust-based parsing</li>"
        "<li>Semantic highlighting</li>"
        "<li>Line numbers</li>"
        "<li>Modern dark theme</li>"
        "</ul>"
        "<p>Built with ❤️ using Rust and C++/Qt6</p>");
}

void MainWindow::textChanged() {
    isModified_ = true;

    // Update status bar
    statusBar()->showMessage("Modified");

    // Trigger highlighting update (debounced)
    static QTimer* timer = nullptr;
    if (!timer) {
        timer = new QTimer(this);
        timer->setSingleShot(true);
        connect(timer, &QTimer::timeout, this, &MainWindow::updateHighlighting);
    }

    timer->start(300); // 300ms debounce
}

void MainWindow::updateHighlighting() {
    // Parse the document
    QString text = editor_->toPlainText();

    try {
        CAST* ast = parser_->parse(text.toStdString());
        if (ast) {
            CyberMD::AST astWrapper(ast);

            // Get highlight ranges
            auto ranges = highlighter_->highlight(astWrapper.get());

            // TODO: Apply highlights to editor
            // This would require implementing a custom syntax highlighter
            // For now, just update the status
            statusBar()->showMessage(
                QString("Parsed successfully - %1 highlight ranges")
                    .arg(ranges.size()));
        }
    } catch (const std::exception& e) {
        statusBar()->showMessage(QString("Parse error: %1").arg(e.what()));
    }
}

void MainWindow::updateOutline() {
    // TODO: Implement outline update
    // This would populate a sidebar with document structure
}
