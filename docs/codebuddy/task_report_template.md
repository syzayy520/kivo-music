# Task Report Template

Every CodeBuddy ticket execution must produce a final report using this template. Incomplete reports are not accepted.

## Report Format

```
<ticket-id> Report

1. Base Gate
2. Files Changed
3. Content Summary Per File
4. Self Audit
5. Genealogy / Folder Fan-out Gate
6. Side Effect / Temporary Artifact Gate
7. Commit Result
8. Final Clean Gate
9. Primary Final Classification
10. Recommended Follow-up
11. Push Status
```

## Section Requirements

### 1. Base Gate

| Check | Expected | Actual | Status |
|-------|----------|--------|--------|
| Branch | (from ticket) | (from git) | PASS/FAIL |
| Local HEAD | (from ticket) | (from git rev-parse) | PASS/FAIL |
| Remote HEAD | (from ticket) | (from git ls-remote) | PASS/FAIL |
| Working tree | clean | (from git status) | PASS/FAIL |

### 2. Files Changed

List every file created, modified, or deleted:

| File | Action | Lines Added | Lines Removed |
|------|--------|-------------|---------------|
| path/to/file.rs | Modified | +10 | -5 |

If no files changed: "0 files changed (DESIGN_ONLY / DOC ticket)"

### 3. Content Summary Per File

For each changed file, provide:

- File path
- Single responsibility statement
- What changed (specific, not vague)
- Line count after change

### 4. Self Audit

| Checklist Item | Status |
|---------------|--------|
| Genealogy compliance (8 items) | PASS/FAIL |
| Dependency direction (4 items) | PASS/FAIL |
| Single responsibility (4 items) | PASS/FAIL |
| Side effects documented (4 items) | PASS/FAIL |
| Evidence captured (4 items) | PASS/FAIL |
| Genealogy (4 items) | PASS/FAIL |

### 5. Genealogy / Folder Fan-out Gate

**Required for every task. No exceptions.**

#### 5.1 Touched Folders

| Folder Path | Sibling File Count | Sibling Files |
|-------------|-------------------|---------------|
| (path) | (count) | (list) |

#### 5.2 Folder Fan-out Result

| Folder | Business Files | Hard Limit (<=9) | Target (<=7) | Total w/ mod.rs | Result |
|--------|---------------|------------------|--------------|-----------------|--------|
| (path) | (count) | PASS/BLOCKED | PASS/WARN | (count) | PASS/WARN/BLOCKED |

#### 5.3 Subfamily Split Check

| Folder | Can Split? | Should Split? | Violation? |
|--------|-----------|---------------|------------|
| (path) | YES/NO | YES/NO | YES/NO |

#### 5.4 Single Responsibility Check

| File | Responsibility Count | Violation? |
|------|---------------------|------------|
| (path) | (count) | YES/NO |

#### 5.5 mod.rs Check

| File | Contains Logic? | Violation? |
|------|----------------|------------|
| (path) | YES/NO | YES/NO |

#### 5.6 Forbidden Bucket File Check

| File | Forbidden Name? | Violation? |
|------|----------------|------------|
| (path) | YES/NO | YES/NO |

**If any violation**: Final Classification must be `BLOCKED_FOLDER_FANOUT_GATE` or `STOPPED_GENEALOGY_VIOLATION`. Do NOT continue implementation. Do NOT use test passage to mask structural violations.

### 6. Side Effect / Temporary Artifact Gate

| Check | Result |
|-------|--------|
| `git status --short` | (output) |
| Temporary files found | 0 / N |
| Unexpected files | 0 / N |

If unexpected files found: STOP_SIDE_EFFECT, report paths.

### 7. Commit Result

| Item | Value |
|------|-------|
| Committed? | YES / NO |
| Commit hash | (hash or N/A) |
| Commit message | (message or N/A) |
| Files in commit | (list or N/A) |

### 8. Final Clean Gate

| Check | Expected | Actual | Status |
|-------|----------|--------|--------|
| Working tree | clean | (from git status) | PASS/FAIL |
| Local-ahead | (expected) | (from git rev-list) | PASS/FAIL |
| Remote HEAD | (expected) | (from git ls-remote) | PASS/FAIL |
| HEAD after commit | (new hash) | (from git rev-parse) | PASS/FAIL |

### 9. Primary Final Classification

Format: `<ticket-id> <classification>`

**Valid classifications**:
- `DESIGN_READY_NO_FILES_CHANGED` — design-only ticket, no files changed
- `DOCS_IMPLEMENTED_LOCAL_COMMIT_NO_PUSH` — docs created, committed locally, no push
- `IMPLEMENTED_LOCAL_COMMIT_NO_PUSH` — code implemented, committed locally, no push
- `BLOCKED_BASE_GATE` — stopped at base gate
- `BLOCKED_SCOPE_EXPANSION` — stopped due to scope expansion
- `BLOCKED_SIDE_EFFECT` — stopped due to unexpected side effects
- `BLOCKED_FOLDER_FANOUT_GATE` — stopped due to folder fan-out violation
- `STOPPED_GENEALOGY_VIOLATION` — stopped due to genealogy violation
- `VALIDATION_FAILED_NO_COMMIT` — validation failed, no commit made

**Rules**:
- Classification describes THIS ticket only
- Must not conflate with follow-up recommendation
- Must not describe expected future state

### 10. Recommended Follow-up

Format: `<classification>`

**Valid follow-up classifications**:
- `FOLLOW_UP_NONE` — no further action needed
- `FOLLOW_UP_IMMEDIATE` — next ticket required before other work
- `FOLLOW_UP_SCHEDULED` — next ticket can be deferred
- `FOLLOW_UP_SCRIPTED_VALIDATION_IMPLEMENTATION` — scripts implementation next
- `FOLLOW_UP_DOCS_THEN_SCRIPTED_VALIDATION` — docs first, then scripts
- `BLOCKED_HUMAN_DECISION` — human input required

**Rules**:
- Follow-up is a recommendation, not an instruction
- CodeBuddy must NOT execute follow-up without user instruction
- Must list specific ticket IDs if known

### 11. Push Status

```
Push Status = no push
```

Or if push was authorized and executed:

```
Push Status = PUSHED
Remote HEAD = <new-hash>
```

## Honesty Requirements

The report MUST NOT:

1. Write "completed" without evidence
2. Hide failures that were fixed on re-run (must note: "first run failed, re-run passed")
3. Describe advisory findings as "fixed" (must note: "advisory, not addressed in this ticket")
4. Describe accepted gaps as "fully covered" (must note: "accepted gap, documented")
5. Describe "ready" as "pushed" (must note: "ready for push, Push Status = no push")
6. Omit gate results (every gate must appear in Evidence Ledger)
7. Estimate line counts (must measure with `wc -l` or equivalent)
8. Skip validation (every required gate must be run and reported)
