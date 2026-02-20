import * as vscode from "vscode";
import { RlaiCli } from "./cli/runner.js";
import { InstalledTreeProvider } from "./providers/InstalledTreeProvider.js";
import { ProfileStatus } from "./statusbar/ProfileStatus.js";
import { detectSubProjects } from "./workspace/monorepo.js";

let cli: RlaiCli;
let installedProvider: InstalledTreeProvider;
let profileStatus: ProfileStatus;

export function activate(context: vscode.ExtensionContext): void {
  cli = new RlaiCli();
  installedProvider = new InstalledTreeProvider();
  profileStatus = new ProfileStatus();

  // ── Register TreeView ───────────────────────────────────

  const installedView = vscode.window.createTreeView("rhinolabs.installed", {
    treeDataProvider: installedProvider,
    showCollapseAll: true,
  });
  context.subscriptions.push(installedView);

  // ── Register commands ───────────────────────────────────

  context.subscriptions.push(
    vscode.commands.registerCommand(
      "rhinolabs.refreshInstalled",
      () => refreshInstalled(),
    ),
  );

  context.subscriptions.push(
    vscode.commands.registerCommand(
      "rhinolabs.installProfile",
      (item?: { subProject?: { path: string } }) =>
        installProfile(item?.subProject?.path),
    ),
  );

  context.subscriptions.push(
    vscode.commands.registerCommand(
      "rhinolabs.openSkillFile",
      (item?: { command?: { arguments?: vscode.Uri[] } }) => {
        // The SkillItem already has a command that opens the file.
        // This handler is for the context menu (manual trigger).
        if (item?.command?.arguments?.[0]) {
          vscode.commands.executeCommand(
            "vscode.open",
            item.command.arguments[0],
          );
        }
      },
    ),
  );

  context.subscriptions.push(
    vscode.commands.registerCommand(
      "rhinolabs.runDiagnostics",
      () => runDiagnostics(),
    ),
  );

  // ── React to config changes ─────────────────────────────

  context.subscriptions.push(
    vscode.workspace.onDidChangeConfiguration((e) => {
      if (e.affectsConfiguration("rhinolabs.cli.path")) {
        cli.reloadConfig();
      }
    }),
  );

  // ── Cleanup ─────────────────────────────────────────────

  context.subscriptions.push(profileStatus);

  // ── Initial load ────────────────────────────────────────

  refreshInstalled();
}

export function deactivate(): void {
  // Cleanup handled by context.subscriptions
}

// ── Command implementations ───────────────────────────────

async function refreshInstalled(): Promise<void> {
  const workspaceRoot = vscode.workspace.workspaceFolders?.[0]?.uri.fsPath;
  if (!workspaceRoot) {
    return;
  }

  const subProjects = detectSubProjects(workspaceRoot);
  installedProvider.update(subProjects);
  profileStatus.update(subProjects);
}

async function installProfile(targetPath?: string): Promise<void> {
  try {
    // 1. Get available profiles
    const profiles = await cli.profileList();
    if (profiles.length === 0) {
      vscode.window.showInformationMessage(
        "No profiles available. Create one with: rlai profile create",
      );
      return;
    }

    // 2. Pick a profile
    const picked = await vscode.window.showQuickPick(
      profiles.map((p) => ({
        label: p.name,
        description: p.id,
        detail: `${p.description} (${p.skills.length} skills)`,
        profileId: p.id,
      })),
      { placeHolder: "Select a profile to install" },
    );
    if (!picked) {
      return;
    }

    // 3. Pick target directory (if not provided from context menu)
    let installPath = targetPath;
    if (!installPath) {
      const subProjects = installedProvider.getSubProjects();
      const items = subProjects.map((sp) => ({
        label: sp.isRoot ? "/ (root)" : sp.name,
        description: sp.profileId ? `Current: ${sp.profileId}` : "No profile",
        path: sp.path,
      }));

      const target = await vscode.window.showQuickPick(items, {
        placeHolder: "Select target directory",
      });
      if (!target) {
        return;
      }
      installPath = target.path;
    }

    // 4. Install
    const result = await cli.profileInstall(picked.profileId, installPath);

    const skillCount = result.skillsInstalled.length;
    const failCount = result.skillsFailed.length;
    let message = `Installed "${result.profileName}" with ${skillCount} skill${skillCount !== 1 ? "s" : ""}`;
    if (failCount > 0) {
      message += ` (${failCount} failed)`;
    }

    if (failCount > 0) {
      vscode.window.showWarningMessage(message);
    } else {
      vscode.window.showInformationMessage(message);
    }

    // 5. Refresh
    await refreshInstalled();
  } catch (error) {
    const msg = error instanceof Error ? error.message : String(error);
    vscode.window.showErrorMessage(`Failed to install profile: ${msg}`);
  }
}

async function runDiagnostics(): Promise<void> {
  try {
    const report = await cli.doctor();
    const { passed, failed, warnings } = report;

    const summary = `Diagnostics: ${passed} passed, ${warnings} warnings, ${failed} failed`;

    if (failed > 0) {
      const details = report.checks
        .filter((c) => c.status === "Fail")
        .map((c) => `  - ${c.name}: ${c.message}`)
        .join("\n");

      vscode.window.showErrorMessage(`${summary}\n${details}`);
    } else if (warnings > 0) {
      vscode.window.showWarningMessage(summary);
    } else {
      vscode.window.showInformationMessage(summary);
    }
  } catch (error) {
    const msg = error instanceof Error ? error.message : String(error);
    vscode.window.showErrorMessage(`Diagnostics failed: ${msg}`);
  }
}
