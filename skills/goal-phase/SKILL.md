# Skill: goal-phase

Use when managing individual phases within a goal. Phases support 3-strike self-healing on failure.

## When to Use

- Starting execution of a goal phase
- Verifying a phase completed successfully
- Reporting a phase failure
- Healing a failed phase with a new approach

## How to Use

### With MCP
```
goal_phase({ action: "list", goal_id: "..." })
goal_phase({ action: "start", goal_id: "...", phase_id: "..." })
goal_phase({ action: "verify", goal_id: "...", phase_id: "...", evidence: {"tests_passed": true} })
goal_phase({ action: "fail", goal_id: "...", phase_id: "...", error_detail: "test timeout" })
goal_phase({ action: "heal", goal_id: "...", phase_id: "...", notes: "increased timeout", spec: {} })
```

### With CLI
```bash
not-ace-tool-rs --tool goal_phase --input '{"action": "list", "goal_id": "..."}'
```

## Parameters

| Parameter | Required | Description |
|-----------|----------|-------------|
| `action` | Yes | `start`, `verify`, `fail`, `heal`, `list` |
| `goal_id` | Yes | Goal ID |
| `phase_id` | Most actions | Phase ID |
| `evidence` | `verify` | Verification evidence |
| `error_detail` | `fail` | Failure description |
| `spec` | `heal` | Healing specification |
| `notes` | `heal` | Healing notes |

## Phase Lifecycle
`pending → in_progress → verifying → done`

On failure: strike 1 → retry suggestion → strike 2 → heal spec required → strike 3 → escalated
