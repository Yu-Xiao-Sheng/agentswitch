# AgentSwitch Provider/Model Separation Design

## Problem

Current `ModelConfig` conflates two distinct concepts:
- **Provider** (API endpoint, authentication, protocol)
- **Model** (specific AI model offered by a provider)

This makes the CLI confusing (`model add` is really "add a provider") and limits multi-model-per-provider workflows.

## Design

### New Data Model

**Provider** (`src/config/provider.rs`):
- `name` — unique identifier (e.g., "zhipu", "jdcloud")
- `base_url` — API endpoint
- `api_key` — encrypted API key
- `protocol` — `OpenAI` or `Anthropic` (default: `OpenAI`)
- `models` — `Vec<String>` of available model names

**ActiveConfig** (`src/config/active.rs`):
- `tool` — agent tool name (e.g., "claude-code")
- `provider` — provider name
- `model` — specific model name

**AppConfig** (`src/config/models.rs` — modified):
- `providers: Vec<Provider>` (renamed from `models`)
- `active: HashMap<String, ActiveConfig>` (renamed from `active_models`)

### CLI Changes

**Remove**: entire `model` subcommand tree.

**Add**: `provider` subcommand with:
- `add` — add provider (optional `--models` for convenience)
- `list` — list all providers
- `show` — show provider details
- `fetch-models` — fetch models from API
- `add-models` — manually add model names
- `test` — test connectivity
- `remove` — delete provider

**Modify** `switch`:
- Full form: `asw switch <tool> <provider> <model>`
- Shorthand: `asw switch <tool> <model>` — auto-resolves provider; errors if ambiguous

**Modify** `status`: unchanged interface, updated display.

### Adapter Changes

`AgentAdapter::apply()` signature changes from `(&ModelConfig)` to `(&Provider, model: &str)`. All 6 adapters updated.

### Backward Compatibility

On `ConfigStore::load_config()`:
1. Detect old format (`[[models]]` / `[active_models]` keys)
2. Migrate `ModelConfig` -> `Provider` (name, base_url, api_key, models, protocol from extra_params)
3. Migrate `active_models: HashMap<String, String>` -> `active: HashMap<String, ActiveConfig>` (provider inferred from model lookup)
4. Save migrated config

### Preserved Features

- API key encryption
- Backup/restore
- Presets (adapted to new types)
- Batch operations (adapted)
- Sync, wizard, doctor, completion

## File Changes

| File | Action |
|------|--------|
| `src/config/provider.rs` | New |
| `src/config/active.rs` | New |
| `src/config/models.rs` | Modify |
| `src/config/mod.rs` | Modify |
| `src/config/store.rs` | Modify |
| `src/cli/args.rs` | Add ProviderCommands |
| `src/cli/mod.rs` | Add Provider subcommand, modify Switch/Status |
| `src/cli/commands.rs` | Implement ProviderCommands, modify Switch/Status |
| `src/agents/adapter.rs` | Modify trait |
| `src/agents/*.rs` | Update all adapters |
| `README.md` | Update |
| `CHANGELOG.md` | Update |
