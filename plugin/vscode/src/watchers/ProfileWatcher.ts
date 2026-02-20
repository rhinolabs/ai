import * as vscode from "vscode";
import type { RlaiCli } from "../cli/runner.js";

/** Callbacks invoked by the watcher after a sync operation */
export interface ProfileWatcherCallbacks {
  /** Called after a successful sync to refresh TreeViews and status bar */
  onSyncComplete: () => void | Promise<void>;
}

/**
 * Watches for profile changes and triggers auto-sync.
 *
 * Monitors two sources:
 * 1. `.claude-plugin/plugin.json` — direct changes (edit, pull, install)
 * 2. `.git/FETCH_HEAD` — updated after git fetch/pull (catch-all)
 *
 * Uses a debounce timer to avoid multiple rapid syncs from a single git pull
 * that modifies several files.
 */
export class ProfileWatcher implements vscode.Disposable {
  private disposables: vscode.Disposable[] = [];
  private debounceTimer: ReturnType<typeof setTimeout> | undefined;
  private readonly debounceMs = 2_000;

  constructor(
    private readonly cli: RlaiCli,
    private readonly callbacks: ProfileWatcherCallbacks,
  ) {
    const config = vscode.workspace.getConfiguration("rhinolabs");
    if (!config.get<boolean>("autoSync.onPull", true)) {
      return;
    }

    this.setupWatchers();
  }

  private setupWatchers(): void {
    // Watch for plugin.json changes (create, change, delete)
    const pluginWatcher = vscode.workspace.createFileSystemWatcher(
      "**/.claude-plugin/plugin.json",
    );
    pluginWatcher.onDidChange(() => this.onPluginChange());
    pluginWatcher.onDidCreate(() => this.onPluginChange());
    pluginWatcher.onDidDelete(() => this.onPluginDelete());
    this.disposables.push(pluginWatcher);

    // Watch for .git/FETCH_HEAD changes (git fetch/pull completed)
    const fetchHeadWatcher = vscode.workspace.createFileSystemWatcher(
      "**/.git/FETCH_HEAD",
    );
    fetchHeadWatcher.onDidChange(() => this.onGitFetchComplete());
    fetchHeadWatcher.onDidCreate(() => this.onGitFetchComplete());
    this.disposables.push(fetchHeadWatcher);

    // Watch for skill directory deletions
    const skillsWatcher = vscode.workspace.createFileSystemWatcher(
      "**/.claude/skills/*",
    );
    skillsWatcher.onDidDelete(() => this.onSkillDeleted());
    this.disposables.push(skillsWatcher);
  }

  /** plugin.json was modified or created — someone installed/changed a profile */
  private onPluginChange(): void {
    this.debouncedSync("Profile configuration changed.");
  }

  /** plugin.json was deleted — profile was removed externally */
  private onPluginDelete(): void {
    // Just refresh the views, no sync needed
    this.debounced(async () => {
      await this.callbacks.onSyncComplete();
    });
  }

  /** git fetch/pull completed — profile may have changed in upstream */
  private onGitFetchComplete(): void {
    this.debouncedSync("Git pull detected.");
  }

  /** A skill directory was deleted — may need to re-sync */
  private onSkillDeleted(): void {
    this.debouncedSync("Skill files removed.");
  }

  /**
   * Debounced sync: waits for activity to settle, then runs
   * `rlai profile sync` and prompts the user.
   */
  private debouncedSync(reason: string): void {
    this.debounced(async () => {
      const workspaceRoot =
        vscode.workspace.workspaceFolders?.[0]?.uri.fsPath;
      if (!workspaceRoot) {
        return;
      }

      try {
        const result = await this.cli.profileSync(workspaceRoot);

        if (result.status === "updated") {
          const added = result.added.length;
          const removed = result.removed.length;
          const parts: string[] = [];
          if (added > 0) {
            parts.push(`${added} added`);
          }
          if (removed > 0) {
            parts.push(`${removed} removed`);
          }
          const detail = parts.join(", ");

          const action = await vscode.window.showInformationMessage(
            `${reason} Profile synced: ${detail}.`,
            "View Changes",
            "Dismiss",
          );

          if (action === "View Changes") {
            vscode.commands.executeCommand("rhinolabs.installed.focus");
          }
        }

        // Always refresh views after any sync attempt
        await this.callbacks.onSyncComplete();
      } catch {
        // CLI not available — silently ignore
      }
    });
  }

  /** Generic debounce helper — cancels pending timer and sets a new one */
  private debounced(fn: () => Promise<void>): void {
    if (this.debounceTimer) {
      clearTimeout(this.debounceTimer);
    }
    this.debounceTimer = setTimeout(() => {
      this.debounceTimer = undefined;
      fn().catch(() => {
        /* swallow errors — already handled in fn */
      });
    }, this.debounceMs);
  }

  dispose(): void {
    if (this.debounceTimer) {
      clearTimeout(this.debounceTimer);
    }
    for (const d of this.disposables) {
      d.dispose();
    }
  }
}
