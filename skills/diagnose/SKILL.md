# Skill: diagnose

Use when encountering any error, exception, stack trace, or unexpected behavior. This skill diagnoses the root cause using project memory, codebase search, and web research, then auto-saves the solution for future reference.

## When to Use

- Build failure or compilation error
- Runtime exception or crash
- Test failure with unclear cause
- Unexpected behavior that doesn't match expectations
- Error messages from third-party libraries or APIs

## How to Use

### With MCP (if not-ace-tool is configured)

Call the `diagnose` MCP tool:
```
diagnose({
  error_message: "<paste the full error/stack trace here>",
  project_root_path: "/path/to/project"  // optional, for code context
})
```

### With CLI

```bash
not-ace-tool-rs --tool diagnose --input '{"error_message": "TypeError: Cannot read properties of undefined", "project_root_path": "/path/to/project"}'
```

### Without MCP or CLI

If the tool is not available, follow this manual process:

1. **Search memory** for similar past errors:
   ```
   recall({ query: "<error message>" })
   ```

2. **Search codebase** for the error location:
   ```
   search_context({ query: "<error message or function name>", project_root_path: "..." })
   ```

3. **Search the web** if the error is from a library:
   ```
   web_search({ query: "<error message>" })
   ```

4. **Save the solution** once you've fixed it:
   ```
   memory({ content: "Error: <summary>\nSolution: <what fixed it>" })
   ```

## What It Does

1. Searches project memory for similar past errors and known solutions
2. Searches the codebase for code related to the error (if project path provided)
3. Searches the web for community solutions
4. Uses LLM to synthesize a diagnosis: problem location, root cause, suggested fix
5. Automatically saves the error-solution pair to memory for future use

## Output Format

The diagnosis includes:
- **Problem Location** — file, function, line
- **Root Cause Analysis** — why the error occurs
- **Suggested Fix** — concrete steps with code examples
- **References** — links or past solutions

## Tips

- Include the **full stack trace**, not just the error message — more context = better diagnosis
- Provide `project_root_path` when possible — code context dramatically improves accuracy
- If the diagnosis suggests a fix, verify it works before moving on
- The tool auto-saves to memory, so the same error won't need diagnosis twice
