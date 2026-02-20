import * as fs from "node:fs";
import * as path from "node:path";
import * as vscode from "vscode";
import type { PluginManifest } from "../cli/types.js";

/** A detected subproject within the workspace */
export interface SubProject {
  /** Display name (directory name relative to workspace root) */
  name: string;
  /** Absolute path to the subproject directory */
  path: string;
  /** Profile ID installed in this subproject, or null */
  profileId: string | null;
  /** Profile name for display */
  profileName: string | null;
  /** Skill IDs installed (directory names under .claude/skills/) */
  skills: string[];
  /** Whether this is the workspace root */
  isRoot: boolean;
}

/**
 * Detect subprojects in the workspace by scanning for `.claude-plugin/plugin.json`.
 *
 * Scans up to `maxDepth` levels deep, skipping common non-project directories.
 * A "subproject" is any directory containing `.claude-plugin/plugin.json`.
 * The workspace root is always included (even without a plugin.json).
 */
export function detectSubProjects(workspaceRoot: string): SubProject[] {
  const config = vscode.workspace.getConfiguration("rhinolabs");
  const maxDepth = config.get<number>("monorepo.maxDepth", 3);
  const results: SubProject[] = [];

  // Always include the root
  const rootProject = readSubProject(workspaceRoot, workspaceRoot);
  results.push(rootProject);

  // Scan child directories for more subprojects
  scanDirectory(workspaceRoot, workspaceRoot, 1, maxDepth, results);

  return results;
}

/** Directories to skip when scanning for subprojects */
const IGNORED_DIRS = new Set([
  "node_modules",
  ".git",
  ".next",
  "target",
  "dist",
  "build",
  "out",
  ".claude",
  ".claude-plugin",
  ".vscode",
  ".idea",
  "__pycache__",
  "vendor",
  "coverage",
  ".turbo",
  ".cache",
]);

function scanDirectory(
  dir: string,
  workspaceRoot: string,
  currentDepth: number,
  maxDepth: number,
  results: SubProject[],
): void {
  if (currentDepth > maxDepth) {
    return;
  }

  let entries: fs.Dirent[];
  try {
    entries = fs.readdirSync(dir, { withFileTypes: true });
  } catch {
    return; // Permission denied, etc.
  }

  for (const entry of entries) {
    if (!entry.isDirectory() || entry.name.startsWith(".")) {
      continue;
    }
    if (IGNORED_DIRS.has(entry.name)) {
      continue;
    }

    const childPath = path.join(dir, entry.name);
    const pluginJson = path.join(childPath, ".claude-plugin", "plugin.json");

    if (fs.existsSync(pluginJson)) {
      results.push(readSubProject(childPath, workspaceRoot));
    }

    // Keep scanning deeper
    scanDirectory(childPath, workspaceRoot, currentDepth + 1, maxDepth, results);
  }
}

/**
 * Read a subproject's profile and skills from its `.claude-plugin/plugin.json`
 * and `.claude/skills/` directory.
 */
function readSubProject(dir: string, workspaceRoot: string): SubProject {
  const isRoot = dir === workspaceRoot;
  const relativeName = isRoot ? "/" : path.relative(workspaceRoot, dir);
  const pluginJsonPath = path.join(dir, ".claude-plugin", "plugin.json");

  let profileId: string | null = null;
  let profileName: string | null = null;

  // Read profile info from plugin.json
  try {
    if (fs.existsSync(pluginJsonPath)) {
      const raw = fs.readFileSync(pluginJsonPath, "utf-8");
      const manifest: PluginManifest = JSON.parse(raw);
      profileId = manifest.profile?.id ?? null;
      profileName = manifest.profile?.name ?? null;
    }
  } catch {
    // Corrupt or missing — leave null
  }

  // Read installed skills
  const skills = readInstalledSkills(dir);

  return {
    name: relativeName,
    path: dir,
    profileId,
    profileName,
    skills,
    isRoot,
  };
}

/** List skill IDs installed under `.claude/skills/` in a directory */
function readInstalledSkills(dir: string): string[] {
  const skillsDir = path.join(dir, ".claude", "skills");

  if (!fs.existsSync(skillsDir)) {
    return [];
  }

  try {
    return fs
      .readdirSync(skillsDir, { withFileTypes: true })
      .filter((e) => (e.isDirectory() || e.isSymbolicLink()) && !e.name.startsWith("."))
      .map((e) => e.name);
  } catch {
    return [];
  }
}

/**
 * Find which subproject a file belongs to.
 * Used for status bar — given an open file, determine its subproject.
 */
export function findSubProjectForFile(
  filePath: string,
  subProjects: SubProject[],
): SubProject | undefined {
  // Sort by path length descending — longer paths (deeper subprojects) first
  const sorted = [...subProjects].sort((a, b) => b.path.length - a.path.length);

  for (const sp of sorted) {
    if (filePath.startsWith(sp.path)) {
      return sp;
    }
  }

  return undefined;
}
