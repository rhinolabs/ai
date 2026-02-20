import * as vscode from "vscode";
import type { RlaiCli } from "../cli/runner.js";

/**
 * Check for rhinolabs-ai plugin updates on startup.
 *
 * Calls `rlai update --check --json` and shows a notification
 * when a newer version is available.
 */
export async function checkForUpdates(cli: RlaiCli): Promise<void> {
  const config = vscode.workspace.getConfiguration("rhinolabs");
  if (!config.get<boolean>("autoUpdate.check", true)) {
    return;
  }

  try {
    const result = await cli.updateCheck();

    if (!result.updateAvailable || !result.latestVersion) {
      return;
    }

    const action = await vscode.window.showInformationMessage(
      `Rhinolabs AI v${result.latestVersion} is available (current: v${result.currentVersion}).`,
      "View Details",
      "Dismiss",
    );

    if (action === "View Details") {
      // Open a terminal with the update command for the user to review
      const terminal = vscode.window.createTerminal("Rhinolabs AI Update");
      terminal.show();
      terminal.sendText("rlai update");
    }
  } catch {
    // CLI not available or network error — silently skip
  }
}
