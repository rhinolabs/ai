# MCP Integration Guide

This document explains the technical aspects of Model Context Protocol (MCP) integration in the Rhinolabs Claude plugin.

**For centralized MCP configuration management using mcp-toolkit**, see [MCP_CENTRALIZED_CONFIG.md](MCP_CENTRALIZED_CONFIG.md).

## Overview

The plugin integrates with the Model Context Protocol (MCP) to provide enhanced capabilities for repository operations, file management, and development workflows.

## What is MCP?

Model Context Protocol (MCP) is a standardized protocol that allows AI assistants to interact with external tools and services in a consistent way. It provides:

- **Standardized Communication**: Consistent interface for tool interactions
- **Extensibility**: Easy addition of new capabilities
- **Security**: Controlled access to system resources
- **Performance**: Efficient resource usage through lazy loading

## MCP Configuration

The plugin's MCP configuration is defined in `.mcp.json`:

```json
{
  "mcpServers": {
    "git": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-git"]
    }
  },
  "settings": {
    "defaultTimeout": 30000,
    "retryAttempts": 3,
    "logLevel": "info"
  }
}
```

## Available MCP Servers

### Git Server

**Purpose**: Provides Git repository operations

**Capabilities**:
- Repository status checking
- Branch management
- Commit operations
- Diff viewing
- Log inspection

**Usage Example**:
```typescript
// Claude can now perform Git operations
// "Show me the current git status"
// "Create a new branch called feature/new-skill"
// "Show me the diff for the last commit"
```

## Configuration Options

### Server Configuration

Each MCP server is configured with:

- **command**: The executable to run (e.g., `npx`, `node`)
- **args**: Arguments passed to the command
- **env**: Environment variables (optional)
- **cwd**: Working directory (optional)

### Global Settings

#### defaultTimeout
- **Type**: Number (milliseconds)
- **Default**: 30000 (30 seconds)
- **Purpose**: Maximum time to wait for MCP operations

#### retryAttempts
- **Type**: Number
- **Default**: 3
- **Purpose**: Number of retry attempts for failed operations

#### logLevel
- **Type**: String
- **Options**: `debug`, `info`, `warn`, `error`
- **Default**: `info`
- **Purpose**: Controls logging verbosity

## Lazy Loading

The plugin uses lazy loading for MCP servers to improve performance:

```json
{
  "mcp": {
    "configFile": ".mcp.json",
    "lazyLoad": true
  }
}
```

**Benefits**:
- Faster plugin initialization
- Reduced memory usage
- Servers loaded only when needed
- Better resource management

## Adding New MCP Servers

To add a new MCP server:

1. **Update `.mcp.json`**:
```json
{
  "mcpServers": {
    "git": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-git"]
    },
    "filesystem": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-filesystem"]
    }
  }
}
```

2. **Test the configuration**:
   - Restart Claude Code
   - Verify the server loads correctly
   - Test server capabilities

3. **Document the server**:
   - Add to this documentation
   - Include usage examples
   - Note any special requirements

## Common MCP Operations

### Git Operations

**Check repository status**:
```
"What's the current status of the repository?"
```

**View recent commits**:
```
"Show me the last 5 commits"
```

**Create a branch**:
```
"Create a new branch called feature/authentication"
```

**View changes**:
```
"Show me what changed in the last commit"
```

## Troubleshooting

### Server Not Loading

**Symptoms**: MCP server doesn't respond or isn't available

**Solutions**:
1. Check `.mcp.json` syntax
2. Verify the command is available (`npx` installed)
3. Check Claude Code logs
4. Increase timeout if operations are slow

### Timeout Errors

**Symptoms**: Operations fail with timeout errors

**Solutions**:
1. Increase `defaultTimeout` in settings
2. Check network connectivity
3. Verify server is responding
4. Review system resources

### Permission Errors

**Symptoms**: MCP operations fail with permission errors

**Solutions**:
1. Check file/directory permissions
2. Ensure Claude Code has necessary access
3. Run with appropriate user privileges
4. Review security settings

## Security Considerations

### Access Control

MCP servers have controlled access to:
- File system (limited to project directories)
- Git operations (repository only)
- Environment variables (explicitly configured)

### Best Practices

1. **Limit server capabilities**: Only enable needed servers
2. **Use minimal permissions**: Grant least privilege access
3. **Review logs regularly**: Monitor for unusual activity
4. **Keep servers updated**: Use latest versions
5. **Validate configurations**: Test in development first

## Performance Optimization

### Lazy Loading

Enable lazy loading to improve startup time:
```json
{
  "mcp": {
    "lazyLoad": true
  }
}
```

### Timeout Tuning

Adjust timeouts based on operation types:
- Quick operations: 10000ms (10 seconds)
- Standard operations: 30000ms (30 seconds)
- Long operations: 60000ms (60 seconds)

### Retry Strategy

Configure retries for reliability:
```json
{
  "settings": {
    "retryAttempts": 3,
    "retryDelay": 1000
  }
}
```

## Monitoring and Logging

### Log Levels

**debug**: Detailed information for troubleshooting
```json
{ "logLevel": "debug" }
```

**info**: General operational information (default)
```json
{ "logLevel": "info" }
```

**warn**: Warning messages for potential issues
```json
{ "logLevel": "warn" }
```

**error**: Error messages only
```json
{ "logLevel": "error" }
```

### Log Location

Logs are stored in Claude Code's log directory:
- **Linux**: `~/.config/claude-code/logs/`
- **macOS**: `~/Library/Logs/Claude Code/`
- **Windows**: `%APPDATA%\Claude Code\logs\`

## Advanced Configuration

### Environment Variables

Pass environment variables to MCP servers:
```json
{
  "mcpServers": {
    "custom": {
      "command": "node",
      "args": ["server.js"],
      "env": {
        "API_KEY": "${API_KEY}",
        "DEBUG": "true"
      }
    }
  }
}
```

### Working Directory

Set custom working directory:
```json
{
  "mcpServers": {
    "custom": {
      "command": "node",
      "args": ["server.js"],
      "cwd": "/path/to/directory"
    }
  }
}
```

## Future MCP Servers

Potential servers for future integration:

- **Database**: SQL query execution
- **API**: REST API interactions
- **Docker**: Container management
- **Cloud**: Cloud provider operations
- **Testing**: Test execution and reporting

## Resources

- [MCP Specification](https://modelcontextprotocol.io)
- [MCP Server Git](https://github.com/modelcontextprotocol/servers/tree/main/src/git)
- [Claude Code MCP Documentation](https://code.claude.com/docs/en/mcp)

## Support

For MCP-related issues:
- Check troubleshooting section
- Review Claude Code logs
- Verify MCP server versions
- Contact DevOps team (internal)

---

**Last Updated**: 2026-01-22  
**Version**: 1.0.0
