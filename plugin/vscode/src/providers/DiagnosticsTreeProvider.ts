import * as vscode from "vscode";
import type { DiagnosticCheck, DiagnosticReport } from "../cli/types.js";

// ── Tree item types ─────────────────────────────────────────

type TreeElement = DiagnosticCheckItem | DiagnosticSummaryItem;

class DiagnosticCheckItem extends vscode.TreeItem {
  constructor(check: DiagnosticCheck) {
    super(check.name, vscode.TreeItemCollapsibleState.None);

    this.description = check.message;
    this.tooltip = `${check.name}: ${check.message}`;
    this.iconPath = statusIcon(check.status);
    this.contextValue = "diagnosticCheck";
  }
}

class DiagnosticSummaryItem extends vscode.TreeItem {
  constructor(report: DiagnosticReport) {
    const { passed, warnings, failed } = report;
    const label = `${passed} passed, ${warnings} warnings, ${failed} failed`;

    super(label, vscode.TreeItemCollapsibleState.None);

    this.description = "Summary";
    this.iconPath =
      failed > 0
        ? new vscode.ThemeIcon("error", new vscode.ThemeColor("errorForeground"))
        : warnings > 0
          ? new vscode.ThemeIcon("warning", new vscode.ThemeColor("editorWarning.foreground"))
          : new vscode.ThemeIcon("pass", new vscode.ThemeColor("testing.iconPassed"));
    this.contextValue = "diagnosticSummary";
  }
}

function statusIcon(status: DiagnosticCheck["status"]): vscode.ThemeIcon {
  switch (status) {
    case "Pass":
      return new vscode.ThemeIcon(
        "pass",
        new vscode.ThemeColor("testing.iconPassed"),
      );
    case "Warning":
      return new vscode.ThemeIcon(
        "warning",
        new vscode.ThemeColor("editorWarning.foreground"),
      );
    case "Fail":
      return new vscode.ThemeIcon(
        "error",
        new vscode.ThemeColor("errorForeground"),
      );
  }
}

// ── TreeDataProvider ────────────────────────────────────────

export class DiagnosticsTreeProvider
  implements vscode.TreeDataProvider<TreeElement>
{
  private _onDidChangeTreeData = new vscode.EventEmitter<
    TreeElement | undefined | void
  >();
  readonly onDidChangeTreeData = this._onDidChangeTreeData.event;

  private report: DiagnosticReport | null = null;
  private error: string | null = null;

  /** Update the diagnostics data and refresh the tree */
  update(report: DiagnosticReport): void {
    this.report = report;
    this.error = null;
    this._onDidChangeTreeData.fire();
  }

  /** Show an error state in the tree */
  showError(message: string): void {
    this.report = null;
    this.error = message;
    this._onDidChangeTreeData.fire();
  }

  /** Clear the diagnostics data */
  clear(): void {
    this.report = null;
    this.error = null;
    this._onDidChangeTreeData.fire();
  }

  getTreeItem(element: TreeElement): vscode.TreeItem {
    return element;
  }

  getChildren(element?: TreeElement): TreeElement[] {
    if (element) {
      return [];
    }

    // Error state
    if (this.error) {
      const item = new vscode.TreeItem(
        this.error,
        vscode.TreeItemCollapsibleState.None,
      );
      item.iconPath = new vscode.ThemeIcon(
        "error",
        new vscode.ThemeColor("errorForeground"),
      );
      return [item as TreeElement];
    }

    // No data yet
    if (!this.report) {
      return [];
    }

    // Build the list: summary first, then individual checks
    const items: TreeElement[] = [];

    items.push(new DiagnosticSummaryItem(this.report));

    for (const check of this.report.checks) {
      items.push(new DiagnosticCheckItem(check));
    }

    return items;
  }
}
