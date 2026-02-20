import * as vscode from "vscode";
import type { Profile } from "../cli/types.js";

// ── Tree item types ─────────────────────────────────────────

type TreeElement = AvailableProfileItem | SkillPreviewItem;

export class AvailableProfileItem extends vscode.TreeItem {
  public readonly profile: Profile;

  constructor(profile: Profile) {
    const skillCount = profile.skills.length;

    super(profile.name, vscode.TreeItemCollapsibleState.Collapsed);

    this.profile = profile;
    this.description = profile.id;
    this.tooltip = `${profile.description}\n\n${skillCount} skill${skillCount !== 1 ? "s" : ""}: ${profile.skills.join(", ")}`;
    this.iconPath = new vscode.ThemeIcon("cloud-download");
    this.contextValue = "profile";
  }
}

class SkillPreviewItem extends vscode.TreeItem {
  constructor(skillId: string) {
    super(skillId, vscode.TreeItemCollapsibleState.None);

    this.iconPath = new vscode.ThemeIcon("symbol-package");
    this.contextValue = "skillPreview";
  }
}

// ── TreeDataProvider ────────────────────────────────────────

export class AvailableTreeProvider
  implements vscode.TreeDataProvider<TreeElement>
{
  private _onDidChangeTreeData = new vscode.EventEmitter<
    TreeElement | undefined | void
  >();
  readonly onDidChangeTreeData = this._onDidChangeTreeData.event;

  private profiles: Profile[] = [];

  /** Update the data and refresh the tree */
  update(profiles: Profile[]): void {
    this.profiles = profiles;
    this._onDidChangeTreeData.fire();
  }

  /** Force a visual refresh */
  refresh(): void {
    this._onDidChangeTreeData.fire();
  }

  getTreeItem(element: TreeElement): vscode.TreeItem {
    return element;
  }

  getChildren(element?: TreeElement): TreeElement[] {
    if (!element) {
      // Root level: available profiles
      return this.profiles.map((p) => new AvailableProfileItem(p));
    }

    if (element instanceof AvailableProfileItem) {
      // Profile children: skill previews
      return element.profile.skills.map(
        (skillId) => new SkillPreviewItem(skillId),
      );
    }

    return [];
  }
}
