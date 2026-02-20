import * as vscode from "vscode";
import type { RlaiCli } from "../cli/runner.js";
import type { ProfileSyncResult } from "../cli/types.js";
import type { SubProject } from "../workspace/monorepo.js";

/**
 * Auto-sync profiles for all subprojects that have a `.claude-plugin/plugin.json`.
 *
 * Runs once on startup (when the setting is enabled).
 * For each subproject with an installed profile, calls `rlai profile sync --path <dir> --json`.
 * Shows a summary notification only when changes are detected.
 */
export async function syncOnOpen(
  cli: RlaiCli,
  subProjects: SubProject[],
): Promise<void> {
  const config = vscode.workspace.getConfiguration("rhinolabs");
  if (!config.get<boolean>("autoSync.onStartup", true)) {
    return;
  }

  // Only sync subprojects that already have a profile installed
  const withProfile = subProjects.filter((sp) => sp.profileId !== null);

  if (withProfile.length === 0) {
    return;
  }

  const results: { sp: SubProject; result: ProfileSyncResult }[] = [];

  for (const sp of withProfile) {
    try {
      const result = await cli.profileSync(sp.path);
      results.push({ sp, result });
    } catch {
      // CLI not available or sync failed — silently skip
    }
  }

  // Collect subprojects that had actual changes
  const updated = results.filter((r) => r.result.status === "updated");

  if (updated.length === 0) {
    // Everything already in sync — no notification needed
    return;
  }

  // Build summary message
  const lines = updated.map((u) => {
    const added = u.result.added.length;
    const removed = u.result.removed.length;
    const parts: string[] = [];
    if (added > 0) {
      parts.push(`+${added} added`);
    }
    if (removed > 0) {
      parts.push(`-${removed} removed`);
    }
    const name = u.sp.isRoot ? "/" : u.sp.name;
    return `${name}: ${parts.join(", ")}`;
  });

  const message =
    updated.length === 1
      ? `Profile synced in ${lines[0]}`
      : `Profiles synced:\n${lines.join("\n")}`;

  vscode.window.showInformationMessage(message);
}
