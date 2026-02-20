import * as fs from "node:fs";
import * as os from "node:os";
import * as path from "node:path";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

// Mock the vscode module before importing monorepo
vi.mock("vscode", () => ({
  workspace: {
    getConfiguration: () => ({
      get: (_key: string, defaultValue: unknown) => defaultValue,
    }),
  },
}));

import {
  detectSubProjects,
  findSubProjectForFile,
  type SubProject,
} from "./monorepo.js";

// ── Helpers ──────────────────────────────────────────────────

let tmpDir: string;

function createDir(...segments: string[]): string {
  const dir = path.join(tmpDir, ...segments);
  fs.mkdirSync(dir, { recursive: true });
  return dir;
}

function writeFile(filePath: string, content: string): void {
  const dir = path.dirname(filePath);
  fs.mkdirSync(dir, { recursive: true });
  fs.writeFileSync(filePath, content, "utf-8");
}

function writePluginJson(
  subDir: string,
  profileId: string,
  profileName: string,
): void {
  const manifest = {
    name: "test-plugin",
    description: "Test",
    version: "0.1.0",
    author: { name: "test" },
    profile: { id: profileId, name: profileName },
  };
  writeFile(
    path.join(tmpDir, subDir, ".claude-plugin", "plugin.json"),
    JSON.stringify(manifest),
  );
}

function writeSkills(subDir: string, skillIds: string[]): void {
  for (const id of skillIds) {
    createDir(subDir, ".claude", "skills", id);
  }
}

// ── Setup / Teardown ─────────────────────────────────────────

beforeEach(() => {
  tmpDir = fs.mkdtempSync(path.join(os.tmpdir(), "monorepo-test-"));
});

afterEach(() => {
  fs.rmSync(tmpDir, { recursive: true, force: true });
});

// ── detectSubProjects ────────────────────────────────────────

describe("detectSubProjects", () => {
  it("should include root even without plugin.json", () => {
    const results = detectSubProjects(tmpDir, 3);

    expect(results).toHaveLength(1);
    expect(results[0].name).toBe("/");
    expect(results[0].path).toBe(tmpDir);
    expect(results[0].isRoot).toBe(true);
    expect(results[0].profileId).toBeNull();
    expect(results[0].skills).toEqual([]);
  });

  it("should detect root with plugin.json", () => {
    writePluginJson("", "react-stack", "React Stack");

    const results = detectSubProjects(tmpDir, 3);

    expect(results).toHaveLength(1);
    expect(results[0].profileId).toBe("react-stack");
    expect(results[0].profileName).toBe("React Stack");
  });

  it("should detect subproject with plugin.json", () => {
    createDir("web");
    writePluginJson("web", "frontend", "Frontend Profile");
    writeSkills("web", ["react-patterns", "tailwind-4"]);

    const results = detectSubProjects(tmpDir, 3);

    expect(results).toHaveLength(2);

    const web = results.find((sp) => sp.name === "web");
    expect(web).toBeDefined();
    expect(web!.profileId).toBe("frontend");
    expect(web!.profileName).toBe("Frontend Profile");
    expect(web!.skills).toContain("react-patterns");
    expect(web!.skills).toContain("tailwind-4");
    expect(web!.isRoot).toBe(false);
  });

  it("should detect multiple subprojects", () => {
    writePluginJson("web", "frontend", "Frontend");
    writePluginJson("api", "backend", "Backend");

    const results = detectSubProjects(tmpDir, 3);

    expect(results).toHaveLength(3); // root + web + api
    const names = results.map((sp) => sp.name);
    expect(names).toContain("/");
    expect(names).toContain("web");
    expect(names).toContain("api");
  });

  it("should respect maxDepth", () => {
    // Create a deeply nested subproject at depth 3
    writePluginJson("a/b/deep", "deep-profile", "Deep");

    const shallow = detectSubProjects(tmpDir, 2);
    const deep = detectSubProjects(tmpDir, 3);

    // Shallow scan (depth 2) should NOT find it (a/b/deep is at depth 3)
    expect(shallow.find((sp) => sp.name === path.join("a", "b", "deep"))).toBeUndefined();

    // Deep scan (depth 3) should find it
    expect(deep.find((sp) => sp.name === path.join("a", "b", "deep"))).toBeDefined();
  });

  it("should skip ignored directories", () => {
    createDir("node_modules", "some-pkg");
    writeFile(
      path.join(tmpDir, "node_modules", "some-pkg", ".claude-plugin", "plugin.json"),
      JSON.stringify({
        name: "ignored",
        description: "test",
        version: "0.1.0",
        author: { name: "test" },
        profile: { id: "should-not-find", name: "Ignored" },
      }),
    );

    const results = detectSubProjects(tmpDir, 3);

    expect(results).toHaveLength(1); // only root
    expect(results.find((sp) => sp.profileId === "should-not-find")).toBeUndefined();
  });

  it("should skip dot directories", () => {
    writePluginJson(".hidden", "hidden-profile", "Hidden");

    const results = detectSubProjects(tmpDir, 3);

    expect(results).toHaveLength(1); // only root
  });

  it("should handle corrupt plugin.json gracefully", () => {
    createDir("broken", ".claude-plugin");
    writeFile(
      path.join(tmpDir, "broken", ".claude-plugin", "plugin.json"),
      "not valid json{{{",
    );

    const results = detectSubProjects(tmpDir, 3);

    const broken = results.find((sp) => sp.name === "broken");
    expect(broken).toBeDefined();
    expect(broken!.profileId).toBeNull();
    expect(broken!.profileName).toBeNull();
  });

  it("should read skills and ignore dotfiles", () => {
    writePluginJson("", "test-profile", "Test");
    writeSkills("", ["skill-a", "skill-b"]);
    // Add a dotfile/dotdir that should be ignored
    createDir(".claude", "skills", ".git");

    const results = detectSubProjects(tmpDir, 3);

    expect(results[0].skills).toEqual(["skill-a", "skill-b"]);
    expect(results[0].skills).not.toContain(".git");
  });
});

// ── findSubProjectForFile ────────────────────────────────────

describe("findSubProjectForFile", () => {
  const mockSubProjects: SubProject[] = [
    {
      name: "/",
      path: "/workspace",
      profileId: "root-profile",
      profileName: "Root",
      skills: [],
      isRoot: true,
    },
    {
      name: "web",
      path: "/workspace/web",
      profileId: "frontend",
      profileName: "Frontend",
      skills: ["react-patterns"],
      isRoot: false,
    },
    {
      name: "api",
      path: "/workspace/api",
      profileId: "backend",
      profileName: "Backend",
      skills: ["rust"],
      isRoot: false,
    },
  ];

  it("should find the correct subproject for a file", () => {
    const result = findSubProjectForFile(
      "/workspace/web/src/App.tsx",
      mockSubProjects,
    );

    expect(result).toBeDefined();
    expect(result!.name).toBe("web");
  });

  it("should match the deepest subproject", () => {
    const withNested: SubProject[] = [
      ...mockSubProjects,
      {
        name: "web/packages/ui",
        path: "/workspace/web/packages/ui",
        profileId: "ui-lib",
        profileName: "UI Library",
        skills: [],
        isRoot: false,
      },
    ];

    const result = findSubProjectForFile(
      "/workspace/web/packages/ui/src/Button.tsx",
      withNested,
    );

    expect(result).toBeDefined();
    expect(result!.name).toBe("web/packages/ui");
  });

  it("should fall back to root for files not in a subproject", () => {
    const result = findSubProjectForFile(
      "/workspace/README.md",
      mockSubProjects,
    );

    expect(result).toBeDefined();
    expect(result!.name).toBe("/");
    expect(result!.isRoot).toBe(true);
  });

  it("should return undefined for files outside workspace", () => {
    const result = findSubProjectForFile(
      "/other/project/file.ts",
      mockSubProjects,
    );

    expect(result).toBeUndefined();
  });

  it("should handle empty subProjects array", () => {
    const result = findSubProjectForFile("/workspace/file.ts", []);

    expect(result).toBeUndefined();
  });
});
