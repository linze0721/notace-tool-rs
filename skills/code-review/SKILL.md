# Skill: code-review

Use when reviewing code changes before committing, merging, or submitting a PR. This skill analyzes diffs for risks, style consistency, and potential issues using project context and team preferences.

## When to Use

- Before committing significant changes
- Reviewing a PR or merge request
- After refactoring to check for regressions
- When unsure if a change follows project conventions
- After AI-generated code to verify quality

## How to Use

### With MCP (if not-ace-tool is configured)

Call the `code_review` MCP tool:
```
code_review({
  diff: "<paste unified diff or code changes>",
  project_root_path: "/path/to/project",  // optional, for code context
  context: "Adding rate limiting to the API"  // optional, PR description
})
```

To get a diff for review:
```bash
git diff                    # unstaged changes
git diff --cached           # staged changes
git diff main..HEAD         # branch changes vs main
```

### With CLI

```bash
# Review staged changes
git diff --cached | not-ace-tool-rs --tool code_review --input '{"diff": "'"$(git diff --cached)"'", "project_root_path": "."}'

# Or save diff to file first
git diff --cached > /tmp/changes.diff
not-ace-tool-rs --tool code_review --input "{\"diff\": $(cat /tmp/changes.diff | jq -Rs .), \"project_root_path\": \".\"}"
```

### Without MCP or CLI

If the tool is not available, do a manual review:

1. **Get code context** around the changed files:
   ```
   search_context({ query: "<changed function or module>", project_root_path: "..." })
   ```

2. **Check team style preferences**:
   ```
   taste_profile({ format: "markdown" })
   ```

3. **Search for past issues** with similar changes:
   ```
   recall({ query: "<what you changed>" })
   ```

4. Review the diff against this context manually.

## What It Does

1. Searches the codebase for context around changed files
2. Loads project memory for past decisions, known issues, and patterns
3. Loads team coding style preferences (taste profile)
4. Uses LLM to analyze the diff and generate a structured review
5. Automatically saves the review to memory

## Output Format

The review includes:
- **Risk Level** — HIGH / MEDIUM / LOW with justification
- **Issues** — each tagged [CRITICAL], [IMPORTANT], or [MINOR] with file location and fix suggestion
- **Style Consistency** — whether the change follows team conventions
- **Summary** — overall assessment in 2-3 sentences

## Tips

- Include `context` (PR description) for better review quality
- Provide `project_root_path` so the reviewer can see surrounding code
- CRITICAL issues should be fixed before merging
- The review auto-saves to memory, building a knowledge base of past decisions
