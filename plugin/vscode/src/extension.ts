import * as vscode from "vscode";
import { RlaiCli } from "./cli/runner.js";
import {
  AvailableTreeProvider,
  AvailableProfileItem,
} from "./providers/AvailableTreeProvider.js";
import { DiagnosticsTreeProvider } from "./providers/DiagnosticsTreeProvider.js";
import { InstalledTreeProvider } from "./providers/InstalledTreeProvider.js";
import { ProfileStatus } from "./statusbar/ProfileStatus.js";
import { syncOnOpen } from "./startup/syncOnOpen.js";
import { checkForUpdates } from "./startup/updateCheck.js";
import { ProfileWatcher } from "./watchers/ProfileWatcher.js";
import { detectSubProjects } from "./workspace/monorepo.js";

let cli: RlaiCli;
let installedProvider: InstalledTreeProvider;
let availableProvider: AvailableTreeProvider;
let diagnosticsProvider: DiagnosticsTreeProvider;
let profileStatus: ProfileStatus;
let profileWatcher: ProfileWatcher;

export function activate(context: vscode.ExtensionContext): void {
  cli = new RlaiCli();
  installedProvider = new InstalledTreeProvider();
  availableProvider = new AvailableTreeProvider();
  diagnosticsProvider = new DiagnosticsTreeProvider();
  profileStatus = new ProfileStatus();

  // ── Set initial context keys ──────────────────────────────

  const hasWorkspace = (vscode.workspace.workspaceFolders?.length ?? 0) > 0;
  void vscode.commands.executeCommand(
    "setContext",
    "rhinolabs.hasWorkspace",
    hasWorkspace,
  );
  // CLI availability is set asynchronously in runStartupTasks
  void vscode.commands.executeCommand(
    "setContext",
    "rhinolabs.cliUnavailable",
    false,
  );

  // ── Register TreeViews ──────────────────────────────────

  const installedView = vscode.window.createTreeView("rhinolabs.installed", {
    treeDataProvider: installedProvider,
    showCollapseAll: true,
  });
  context.subscriptions.push(installedView);

  const availableView = vscode.window.createTreeView("rhinolabs.available", {
    treeDataProvider: availableProvider,
    showCollapseAll: true,
  });
  context.subscriptions.push(availableView);

  const diagnosticsView = vscode.window.createTreeView(
    "rhinolabs.diagnostics",
    {
      treeDataProvider: diagnosticsProvider,
    },
  );
  context.subscriptions.push(diagnosticsView);

  // ── Register commands ───────────────────────────────────

  context.subscriptions.push(
    vscode.commands.registerCommand("rhinolabs.refreshInstalled", () =>
      refreshInstalled(),
    ),
  );

  context.subscriptions.push(
    vscode.commands.registerCommand("rhinolabs.refreshAvailable", () =>
      refreshAvailable(),
    ),
  );

  context.subscriptions.push(
    vscode.commands.registerCommand(
      "rhinolabs.installProfile",
      (item?: AvailableProfileItem | { subProject?: { path: string } }) =>
        installProfile(item),
    ),
  );

  context.subscriptions.push(
    vscode.commands.registerCommand(
      "rhinolabs.uninstallProfile",
      (item?: { subProject?: { path: string; profileName: string | null } }) =>
        uninstallProfile(item?.subProject?.path, item?.subProject?.profileName),
    ),
  );

  context.subscriptions.push(
    vscode.commands.registerCommand(
      "rhinolabs.toggleSkill",
      (item?: { skillId?: string }) => toggleSkill(item?.skillId),
    ),
  );

  context.subscriptions.push(
    vscode.commands.registerCommand(
      "rhinolabs.openSkillFile",
      (item?: { command?: { arguments?: vscode.Uri[] } }) => {
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
    vscode.commands.registerCommand("rhinolabs.runDiagnostics", () =>
      runDiagnostics(),
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

  // ── File watchers (post-pull auto-sync) ─────────────────

  profileWatcher = new ProfileWatcher(cli, {
    onSyncComplete: async () => {
      await refreshInstalled();
      await refreshAvailable();
    },
  });
  context.subscriptions.push(profileWatcher);

  // ── Cleanup ─────────────────────────────────────────────

  context.subscriptions.push(profileStatus);

  // ── Initial load ────────────────────────────────────────

  refreshInstalled();
  refreshAvailable();

  // ── Startup tasks (run in background, don't block activation) ──

  void runStartupTasks();
}

/**
 * Startup tasks run in the background after activation.
 * They are fire-and-forget — errors are silently swallowed.
 */
async function runStartupTasks(): Promise<void> {
  const workspaceRoot = vscode.workspace.workspaceFolders?.[0]?.uri.fsPath;
  if (!workspaceRoot) {
    return;
  }

  // Check CLI availability and set context key for welcome views
  const cliAvailable = await cli.isAvailable();
  void vscode.commands.executeCommand(
    "setContext",
    "rhinolabs.cliUnavailable",
    !cliAvailable,
  );

  if (!cliAvailable) {
    return;
  }

  // Sync profiles for subprojects that have plugin.json
  const subProjects = detectSubProjects(workspaceRoot);
  await syncOnOpen(cli, subProjects);

  // Refresh views after sync (in case sync changed installed skills)
  await refreshInstalled();

  // Check for plugin updates
  await checkForUpdates(cli);
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

async function refreshAvailable(): Promise<void> {
  try {
    const profiles = await cli.profileList();
    availableProvider.update(profiles);
  } catch {
    // CLI not available — show empty view, don't spam errors on startup
    availableProvider.update([]);
  }
}

async function installProfile(
  item?: AvailableProfileItem | { subProject?: { path: string } },
): Promise<void> {
  try {
    // Determine profile ID — from Available TreeView item or via QuickPick
    let profileId: string | undefined;

    if (item instanceof AvailableProfileItem) {
      profileId = item.profile.id;
    }

    if (!profileId) {
      const profiles = await cli.profileList();
      if (profiles.length === 0) {
        vscode.window.showInformationMessage(
          "No profiles available. Create one with: rlai profile create",
        );
        return;
      }

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
      profileId = picked.profileId;
    }

    // Determine target path — from context menu or via QuickPick
    let installPath: string | undefined;

    if (item && "subProject" in item && item.subProject?.path) {
      installPath = item.subProject.path;
    }

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

    // Install
    const result = await cli.profileInstall(profileId, installPath);

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

    // Refresh both views
    await refreshInstalled();
    await refreshAvailable();
  } catch (error) {
    const msg = error instanceof Error ? error.message : String(error);
    vscode.window.showErrorMessage(`Failed to install profile: ${msg}`);
  }
}

async function uninstallProfile(
  targetPath?: string,
  profileName?: string | null,
): Promise<void> {
  try {
    if (!targetPath) {
      vscode.window.showErrorMessage(
        "No target path. Right-click a profile in the Installed view to uninstall.",
      );
      return;
    }

    // Confirmation dialog
    const displayName = profileName ?? "profile";
    const confirm = await vscode.window.showWarningMessage(
      `Uninstall "${displayName}" from ${targetPath}? This removes skills, instructions, and the plugin manifest.`,
      { modal: true },
      "Uninstall",
    );

    if (confirm !== "Uninstall") {
      return;
    }

    const result = await cli.profileUninstall(targetPath);

    const name = result.profileName ?? result.profileId ?? "Profile";
    vscode.window.showInformationMessage(`"${name}" uninstalled successfully.`);

    await refreshInstalled();
  } catch (error) {
    const msg = error instanceof Error ? error.message : String(error);
    vscode.window.showErrorMessage(`Failed to uninstall profile: ${msg}`);
  }
}

async function toggleSkill(skillId?: string): Promise<void> {
  if (!skillId) {
    return;
  }

  try {
    const skill = await cli.skillShow(skillId);
    const action = skill.enabled ? "disable" : "enable";

    // The CLI doesn't expose `skill toggle --json` yet.
    // Show informational message until it's implemented.
    vscode.window.showInformationMessage(
      `Toggle "${skillId}" → ${action}. (CLI command pending — use GUI or edit .skills-config.json)`,
    );
  } catch (error) {
    const msg = error instanceof Error ? error.message : String(error);
    vscode.window.showErrorMessage(`Failed to toggle skill: ${msg}`);
  }
}

async function runDiagnostics(): Promise<void> {
  try {
    const report = await cli.doctor();
    diagnosticsProvider.update(report);

    const { passed, failed, warnings } = report;
    const summary = `Diagnostics: ${passed} passed, ${warnings} warnings, ${failed} failed`;

    if (failed > 0) {
      vscode.window.showErrorMessage(summary);
    } else if (warnings > 0) {
      vscode.window.showWarningMessage(summary);
    } else {
      vscode.window.showInformationMessage(summary);
    }
  } catch (error) {
    const msg = error instanceof Error ? error.message : String(error);
    diagnosticsProvider.showError(msg);
    vscode.window.showErrorMessage(`Diagnostics failed: ${msg}`);
  }
}
