import { execFile } from "node:child_process";
import * as vscode from "vscode";
import type {
  DiagnosticReport,
  Profile,
  ProfileInstallResult,
  ProfileSyncResult,
  ProfileUninstallResult,
  Skill,
  StatusOutput,
  UpdateCheckResult,
} from "./types.js";

/** Error thrown when the CLI process fails */
export class CliError extends Error {
  constructor(
    message: string,
    public readonly exitCode: number | null,
    public readonly stderr: string,
  ) {
    super(message);
    this.name = "CliError";
  }
}

/**
 * Thin wrapper around the `rlai` CLI binary.
 * All methods spawn a child process with `--json` and parse stdout.
 */
export class RlaiCli {
  private binaryPath: string;
  private timeout: number;

  constructor() {
    const config = vscode.workspace.getConfiguration("rhinolabs");
    this.binaryPath = config.get<string>("cli.path", "rlai");
    this.timeout = 30_000; // 30s default
  }

  /** Reload binary path from settings (call after config change) */
  reloadConfig(): void {
    const config = vscode.workspace.getConfiguration("rhinolabs");
    this.binaryPath = config.get<string>("cli.path", "rlai");
  }

  // ── Profile commands ──────────────────────────────────────

  async profileList(): Promise<Profile[]> {
    return this.exec<Profile[]>(["profile", "list"]);
  }

  async profileShow(id: string): Promise<Profile> {
    return this.exec<Profile>(["profile", "show", id]);
  }

  async profileInstall(
    id: string,
    targetPath: string,
    targets?: string[],
  ): Promise<ProfileInstallResult> {
    const args = ["profile", "install", id, "--path", targetPath];
    if (targets && targets.length > 0) {
      for (const t of targets) {
        args.push("--target", t);
      }
    }
    return this.exec<ProfileInstallResult>(args);
  }

  async profileSync(targetPath?: string): Promise<ProfileSyncResult> {
    const args = ["profile", "sync"];
    if (targetPath) {
      args.push("--path", targetPath);
    }
    return this.exec<ProfileSyncResult>(args);
  }

  async profileUninstall(
    targetPath?: string,
    targets?: string[],
  ): Promise<ProfileUninstallResult> {
    const args = ["profile", "uninstall"];
    if (targetPath) {
      args.push("--path", targetPath);
    }
    if (targets && targets.length > 0) {
      for (const t of targets) {
        args.push("--target", t);
      }
    }
    return this.exec<ProfileUninstallResult>(args);
  }

  // ── Skill commands ────────────────────────────────────────

  async skillList(): Promise<Skill[]> {
    return this.exec<Skill[]>(["skill", "list"]);
  }

  async skillShow(id: string): Promise<Skill> {
    return this.exec<Skill>(["skill", "show", id]);
  }

  // ── Diagnostics ───────────────────────────────────────────

  async doctor(): Promise<DiagnosticReport> {
    return this.exec<DiagnosticReport>(["doctor"]);
  }

  // ── Status ────────────────────────────────────────────────

  async status(): Promise<StatusOutput> {
    return this.exec<StatusOutput>(["status"]);
  }

  // ── Update ────────────────────────────────────────────────

  async updateCheck(): Promise<UpdateCheckResult> {
    return this.exec<UpdateCheckResult>(["update", "--check"]);
  }

  // ── Availability ─────────────────────────────────────────

  /**
   * Check if the CLI binary is reachable.
   * Returns true if `rlai --version` succeeds, false otherwise.
   */
  async isAvailable(): Promise<boolean> {
    try {
      await this.execRaw(["--version"]);
      return true;
    } catch {
      return false;
    }
  }

  // ── Internal ──────────────────────────────────────────────

  private exec<T>(args: string[]): Promise<T> {
    // Always append --json
    const fullArgs = [...args, "--json"];

    return new Promise((resolve, reject) => {
      this.execRaw(fullArgs)
        .then((stdout) => {
          try {
            const parsed = JSON.parse(stdout) as T;
            resolve(parsed);
          } catch {
            reject(
              new CliError(
                `Failed to parse CLI output for: rlai ${args.join(" ")}`,
                null,
                stdout,
              ),
            );
          }
        })
        .catch(reject);
    });
  }

  /** Execute a command and return raw stdout */
  private execRaw(args: string[]): Promise<string> {
    const cwd = vscode.workspace.workspaceFolders?.[0]?.uri.fsPath;

    return new Promise((resolve, reject) => {
      execFile(
        this.binaryPath,
        args,
        {
          cwd,
          timeout: this.timeout,
          env: { ...process.env },
        },
        (error, stdout, stderr) => {
          if (error) {
            const exitCode =
              "code" in error ? (error.code as number | null) : null;
            reject(
              new CliError(
                `rlai ${args.join(" ")} failed: ${stderr || error.message}`,
                typeof exitCode === "number" ? exitCode : null,
                stderr,
              ),
            );
            return;
          }
          resolve(stdout);
        },
      );
    });
  }
}
