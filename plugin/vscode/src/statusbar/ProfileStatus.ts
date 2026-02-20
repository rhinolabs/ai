import * as vscode from "vscode";
import type { SubProject } from "../workspace/monorepo.js";
import { findSubProjectForFile } from "../workspace/monorepo.js";

/**
 * Status bar item that shows the active profile for the current file.
 *
 * Updates when:
 * - The active editor changes (different file → different subproject)
 * - Subproject data is refreshed (profile installed/removed)
 */
export class ProfileStatus {
  private statusBarItem: vscode.StatusBarItem;
  private subProjects: SubProject[] = [];
  private disposables: vscode.Disposable[] = [];

  constructor() {
    this.statusBarItem = vscode.window.createStatusBarItem(
      vscode.StatusBarAlignment.Left,
      50,
    );
    this.statusBarItem.command = "rhinolabs.refreshInstalled";

    // Listen for active editor changes
    this.disposables.push(
      vscode.window.onDidChangeActiveTextEditor(() => this.updateDisplay()),
    );
  }

  /** Update subproject data and refresh the display */
  update(subProjects: SubProject[]): void {
    this.subProjects = subProjects;
    this.updateDisplay();
  }

  private updateDisplay(): void {
    const editor = vscode.window.activeTextEditor;

    if (!editor || this.subProjects.length === 0) {
      this.statusBarItem.hide();
      return;
    }

    const filePath = editor.document.uri.fsPath;
    const subProject = findSubProjectForFile(filePath, this.subProjects);

    if (!subProject) {
      this.statusBarItem.hide();
      return;
    }

    if (subProject.profileId) {
      const name = subProject.profileName ?? subProject.profileId;
      const skillCount = subProject.skills.length;
      this.statusBarItem.text = `$(zap) ${name} (${skillCount} skills)`;
      this.statusBarItem.tooltip = `Rhinolabs AI Profile: ${name}\nSubproject: ${subProject.name}\nClick to refresh`;
    } else {
      this.statusBarItem.text = "$(warning) No profile";
      this.statusBarItem.tooltip =
        "No Rhinolabs AI profile installed in this subproject.\nClick to refresh.";
    }

    this.statusBarItem.show();
  }

  dispose(): void {
    this.statusBarItem.dispose();
    for (const d of this.disposables) {
      d.dispose();
    }
  }
}
