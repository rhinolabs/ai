import * as path from "node:path";
import * as vscode from "vscode";
import type { SubProject } from "../workspace/monorepo.js";

// ── Tree item types ─────────────────────────────────────────

type TreeElement = SubProjectItem | ProfileItem | SkillItem;

class SubProjectItem extends vscode.TreeItem {
  public readonly subProject: SubProject;

  constructor(subProject: SubProject) {
    const hasProfile = subProject.profileId !== null;
    const label = subProject.isRoot
      ? `/ (root)`
      : `${subProject.name}/`;

    super(label, vscode.TreeItemCollapsibleState.Expanded);

    this.subProject = subProject;
    this.iconPath = new vscode.ThemeIcon("folder");
    this.contextValue = hasProfile ? "subProject" : "emptySubProject";

    if (!hasProfile) {
      this.description = "No profile installed";
    }
  }
}

class ProfileItem extends vscode.TreeItem {
  public readonly subProject: SubProject;

  constructor(subProject: SubProject) {
    const skillCount = subProject.skills.length;
    const label = subProject.profileName ?? subProject.profileId ?? "unknown";

    super(
      label,
      skillCount > 0
        ? vscode.TreeItemCollapsibleState.Collapsed
        : vscode.TreeItemCollapsibleState.None,
    );

    this.subProject = subProject;
    this.description = `${skillCount} skill${skillCount !== 1 ? "s" : ""}`;
    this.iconPath = new vscode.ThemeIcon("zap");
    this.contextValue = "profile";
    this.tooltip = `Profile: ${label}\nPath: ${subProject.path}\nSkills: ${skillCount}`;
  }
}

class SkillItem extends vscode.TreeItem {
  public readonly skillId: string;
  public readonly subProjectPath: string;

  constructor(skillId: string, subProjectPath: string) {
    super(skillId, vscode.TreeItemCollapsibleState.None);

    this.skillId = skillId;
    this.subProjectPath = subProjectPath;
    this.iconPath = new vscode.ThemeIcon("check");
    this.contextValue = "skill";

    // Clicking opens the SKILL.md file
    const skillFile = path.join(
      subProjectPath,
      ".claude",
      "skills",
      skillId,
      "SKILL.md",
    );
    this.command = {
      command: "vscode.open",
      title: "Open Skill",
      arguments: [vscode.Uri.file(skillFile)],
    };

    this.tooltip = `Skill: ${skillId}\nClick to open SKILL.md`;
  }
}

// ── TreeDataProvider ────────────────────────────────────────

export class InstalledTreeProvider
  implements vscode.TreeDataProvider<TreeElement>
{
  private _onDidChangeTreeData = new vscode.EventEmitter<
    TreeElement | undefined | void
  >();
  readonly onDidChangeTreeData = this._onDidChangeTreeData.event;

  private subProjects: SubProject[] = [];

  /** Update the data and refresh the tree */
  update(subProjects: SubProject[]): void {
    this.subProjects = subProjects;
    this._onDidChangeTreeData.fire();
  }

  /** Force a visual refresh without changing data */
  refresh(): void {
    this._onDidChangeTreeData.fire();
  }

  getTreeItem(element: TreeElement): vscode.TreeItem {
    return element;
  }

  getChildren(element?: TreeElement): TreeElement[] {
    if (!element) {
      // Root level: subprojects
      return this.subProjects.map((sp) => new SubProjectItem(sp));
    }

    if (element instanceof SubProjectItem) {
      const sp = element.subProject;
      if (sp.profileId) {
        // Subproject has a profile — show it as child
        return [new ProfileItem(sp)];
      }
      // No profile — no children (install button shown via context menu)
      return [];
    }

    if (element instanceof ProfileItem) {
      // Profile children: individual skills
      return element.subProject.skills.map(
        (skillId) => new SkillItem(skillId, element.subProject.path),
      );
    }

    return [];
  }

  /** Get the SubProject paths currently loaded */
  getSubProjects(): SubProject[] {
    return this.subProjects;
  }
}
