# Frontend Development Standard

A set of principles and patterns for building modular, type-safe SvelteKit applications. These guidelines are framework-opinion where necessary but architecture-general where possible. They apply to any project following this standard.

---

## 1. Module System

### 1.1 Feature Modules

Every feature is a self-contained module. A module owns its state, API calls, utilities, and UI. Nothing leaks.

```
src/lib/client/modules/<module-name>/
├── index.ts      # Barrel — the module's public API
├── store.ts      # Reactive state (stores)
├── remote.ts     # API calls
├── utils.ts      # Internal constants, helpers, serializers
└── ui/
    ├── index.ts  # Component barrel
    └── *.svelte  # Components
```

### 1.2 Encapsulation Rules

- **Consume modules through their barrel.** External code imports from `<module>/index.ts`, never from internal files like `remote.ts` or `utils.ts` directly.
- **`utils.ts` is private by default.** Only promote items to the barrel when another module genuinely depends on them.
- **Modules do not reach into each other's internals.** If Module A needs something from Module B, Module B must export it explicitly.

### 1.3 Foundation Module

One module serves as the project's foundation layer — shared layout, navigation, global stores, API client helpers, formatters, and theme utilities. All other modules depend on it. No other inter-module dependency is assumed unless explicitly designed.

### 1.4 Module Independence

Feature modules are independent by default. Cross-module dependencies (e.g., a shared picker component) should be rare, documented, and flow through barrel exports.

---

## 2. Type Safety

### 2.1 Schema-First Types

Types are the contract. Define them in a schema language (YAML, JSON Schema, OpenAPI, Protobuf) and generate TypeScript from them. Generated code lives in a dedicated directory (`src/generated/`) and is never hand-edited.

- The schema is the source of truth, not the TypeScript.
- Regeneration happens as part of the build pipeline.
- Generated files are gitignored — they're a build artifact.

### 2.2 Runtime Decoding

Every API response is validated at runtime through a decoder function generated alongside the types. Never trust raw JSON. The decoder either returns a typed value or `null`.

```typescript
const data = decodeProductSearchResponse(json);
if (data !== null) {
  // safe to use
}
```

### 2.3 Strict TypeScript

- **`strict: true`** — always.
- **`type` keyword only** — never `interface`. Types are structural; `type` is sufficient and consistent.
- **No type assertions (`as`)** — if you need to assert, the types are wrong. Fix them.
- **No type predicates (`x is T`)** — use decoder functions instead. Type predicates are unchecked lies.
- **No `undefined`** — use `null` for absence. `null` is explicit; `undefined` is accidental. One sentinel value, not two.
- **Type-only imports** — use `import type { ... }` for types to keep the runtime bundle clean.

---

## 3. API Layer

### 3.1 Type-Safe Calls

API calls go through a typed caller that pairs a request with a decoder. The result is a discriminated union — success or failure — not a thrown exception.

```typescript
const result = await APICaller.call(request, decoder);

if (result instanceof APISuccess) {
  // result.response: T
} else {
  // result.error, result.statusCode
}
```

### 3.2 URL Construction

Build API URLs through a single helper that reads the base URL from environment configuration. Never hardcode hostnames.

### 3.3 One File Per Module

Each module has exactly one `remote.ts`. Every API call the module makes lives here. This makes the module's external data dependencies immediately visible by reading a single file.

---

## 4. State Management

### 4.1 Three Tiers of State

| Tier | Where | When |
|------|-------|------|
| **URL state** | Query parameters | User-facing state: search, filters, sort, pagination, selected tab. Must survive refresh, back/forward, bookmarking. |
| **Store state** | Svelte stores | App-level state shared across components: sidebar collapsed, toast queue, auth. Persisted to localStorage where appropriate. |
| **Component state** | `$state` / `$derived` | Local UI concerns: hover, open/closed, input buffer. Dies with the component. |

### 4.2 URL State Serialization

Provide `serialize*` and `deserialize*` utility pairs for converting between typed objects and URL parameters. Deserialize in load functions; serialize on user interaction.

### 4.3 Store Design

- One writable store holds the module's state shape.
- Derived stores expose slices as read-only values.
- Mutation goes through exported functions, not direct store writes. This keeps update logic in one place.
- `localStorage` persistence is wrapped in try/catch for SSR safety.

---

## 5. Svelte Conventions

### 5.1 Svelte 5 Runes

| Rune | Purpose |
|------|---------|
| `$state(initial)` | Mutable reactive state |
| `$derived(expr)` | Read-only computed value |
| `$props()` | Component prop declaration |
| `$bindable()` | Two-way binding prop |

### 5.2 No `$effect`

`$effect` is banned. It creates implicit dependency chains that are hard to trace and debug. Instead:
- Derive values reactively with `$derived`.
- Handle side effects in event callbacks.
- Use Svelte `use:` actions for DOM-level side effects.

### 5.3 Component Design

- Props are destructured from `$props()` with explicit types.
- Callbacks are passed as props (`onSearch`, `onFilterChange`), not dispatched events.
- Content slots use the `Snippet` type and `{@render}`.
- Event handlers use lowercase DOM attributes: `onclick`, `oninput`, `onchange`.
- Every component is a single `.svelte` file. No multi-file components.

### 5.4 Data Loading

- `+page.ts` load functions handle data fetching. They run before the component mounts.
- Load functions deserialize URL state, call API functions, and return typed data to the component.
- Components receive data through `$props()` — they do not fetch.

---

## 6. Styling

### 6.1 Design Tokens via CSS Custom Properties

All visual values are defined as CSS custom properties on `:root`. No Tailwind. No utility classes beyond a few global helpers (`.visually-hidden`, `.truncate`).

Tokens cover:
- **Colors**: background, foreground, surface, muted, border, primary, success, error, warning, info
- **Spacing**: a consistent scale (e.g., 4px grid)
- **Radii**: sm, md, lg, full
- **Shadows**: sm, md, lg
- **Typography**: font families, base size

### 6.2 Theming

Light and dark modes are handled by overriding tokens on a data attribute (`[data-theme="dark"]`). Components that use `var(--token)` get theming for free.

### 6.3 Scoped Styles

CSS lives in `<style>` blocks inside Svelte components. Styles are scoped automatically. Global styles exist only in the root CSS file for resets, tokens, and component library overrides.

### 6.4 Component Library

Use a shared UI component library for standard elements (Table, Input, Modal, Toast, Pill, Select, etc.). Check the library before building something custom. Override library appearance through CSS custom properties, not by forking components.

---

## 7. Code Quality

### 7.1 Formatting

Enforce consistent formatting with Prettier (or equivalent). Key settings:
- Single quotes
- No trailing commas
- Reasonable print width (100)
- Framework-specific plugin (e.g., prettier-plugin-svelte)

### 7.2 Linting

ESLint with strict rules:
- **Curly braces required** on all control flow (`if`, `else`, `for`, `while`). No bracketless one-liners.
- **No unused imports** (auto-removable as errors).
- **Unused variables** are warnings. Prefix intentionally unused params with `_`.
- **No type assertions** (`as`).
- **No type predicates** (`x is T`).
- **No `undefined`**.
- **No `$effect`**.

### 7.3 Pre-Commit Check

Every commit should pass:

```bash
format && lint && type-check
```

---

## 8. Routing

### 8.1 File-Based Routes

SvelteKit's file-based routing. Each route is a directory:

```
src/routes/<feature>/
├── +page.svelte         # Page component
├── +page.ts             # Load function
└── [param]/
    ├── +page.svelte     # Dynamic page
    └── +page.ts         # Dynamic load
```

### 8.2 Navigation

- Use `goto()` for programmatic navigation.
- Pass transient data via `goto(url, { state })` for detail pages that can fallback to a fetch.
- Filters, search, sort, and pagination always go in URL query params — never in navigation state alone.

---

## 9. Imports

### 9.1 Path Aliases

Define path aliases to avoid deep relative paths:

| Alias | Target |
|-------|--------|
| `$lib` | Application code (`src/lib/`) |
| `$generated` | Generated types (`src/generated/`) |

### 9.2 Import Order

Group imports in this order, separated by blank lines:

1. Framework (`svelte`, `$app/*`)
2. Third-party libraries
3. Generated types and decoders (`$generated/*`)
4. Module imports (`$lib/client/modules/*`)
5. Shared components (`$lib/components/*`)
6. Relative imports (`./`, `../`)
7. Assets

### 9.3 Asset Imports

SVG icons are imported with `?raw` for inline rendering. Images and other static assets go in `static/` or are imported through Vite's asset pipeline.

---

## 10. Planning

### 10.1 Use Skulls MCP for Planning

All module planning is driven through the **Skulls MCP** server. It provides language-aware planning templates with structured phases, checklists, and verification steps.

**Workflow:**

1. **`init_planning`** — Start a planning session. Returns a `sessionId` required by all subsequent calls.
2. **`select_language`** — Pick the language/framework (e.g., `sveltekit`). Returns available templates.
3. **`get_template`** — Fetch the full template for your use case (e.g., `client-module`). It returns the exact file names and phase structure to create.
4. **`get_phase`** — Re-read a specific phase during implementation without reloading everything.
5. **`get_quick_reference`** — Lookup placeholders, patterns, and checklists mid-build.
6. **`get_verification_steps`** — Get the verification checklist to confirm the module is complete.
7. **`complete_planning`** — Close the session when planning is done.

**Rules:**
- Always use the file names returned by `PLAN_FILES_TO_CREATE` exactly as given. Do not rename them.
- Plan output is a directory per module: `plans/<module-name>/` with files inside.
- Every plan must include `00-overview.md` and `checklist.md` at minimum.

### 10.2 Plan Structure

Plans live in `plans/module-N-<name>/` with numbered files:

```
00-overview.md          # Goal, scope (in/out), success criteria, dependencies
01-planning.md          # Architecture decisions, trade-offs
02-setup.md             # Bootstrap steps
03-type-definitions.md  # Schema/type design
04-state-management.md  # Store and URL state design
05-api-integration.md   # API contracts and remote functions
06-utilities.md         # Helper function specifications
07-ui-components.md     # Component inventory with props and behavior
08-integration.md       # Route wiring and end-to-end flow
checklist.md            # Trackable implementation checklist
```

### 10.3 Plan Principles

- **Scope explicitly.** Every plan states what's in scope and what's not.
- **Define success criteria.** Testable statements that say when the module is done.
- **List dependencies.** Libraries, APIs, and other modules this work depends on.
- **Count files.** Estimate the file count before starting. It constrains scope creep.
- **Reference prior modules.** Completed modules are the template for new ones.

---

## 11. MCP Servers

Three MCP servers are available and must be used in their respective domains. They are the authoritative source — not general knowledge, not guesswork.

### 11.1 Skulls MCP — Planning

Use for all module planning and implementation scaffolding (see Section 10.1).

- Provides language-specific templates with phases, checklists, and verification steps.
- Ensures consistent plan structure across modules and projects.
- Always start with `init_planning` → `select_language` → `get_template`.

### 11.2 Svelte MCP — Framework Guidelines

Use whenever writing or reviewing Svelte/SvelteKit code. This server provides official Svelte 5 and SvelteKit documentation.

**Workflow:**

1. **`list-sections`** — Call FIRST to discover all available documentation sections. Analyze the `use_cases` field to identify what's relevant.
2. **`get-documentation`** — Fetch ALL sections relevant to the task. Accepts single or multiple section names. Be thorough — fetch everything that applies rather than making multiple round trips.
3. **`svelte-autofixer`** — Run on every piece of Svelte code before considering it done. Keep calling until zero issues remain. This is mandatory, not optional.
4. **`playground-link`** — Generate a Svelte Playground link. Only when the user asks for one, and never when code was written to project files.

**Rules:**
- Always consult `list-sections` at the start of any Svelte-related work.
- Always run `svelte-autofixer` before delivering Svelte code. No exceptions.
- Prefer implementing with existing knowledge first, then validate with `svelte-autofixer`. Use `get-documentation` when you need to look up specific APIs or patterns.

### 11.3 Svelte UI Components MCP — Component Library

Use whenever working with `@juspay/svelte-ui-components`. This server documents every component in the library — props, snippets, events, and CSS variables.

**Workflow:**

1. **`list_components`** — Discover all available components with brief descriptions. Call this before building anything custom to check if the library already provides it.
2. **`get_component_docs`** — Get full documentation for a specific component: usage examples, props table, snippet props, event callbacks, and CSS custom properties for theming.

**Rules:**
- Always check `list_components` before creating a custom component. If the library has it, use it.
- Use `get_component_docs` to get the exact prop names, types, and CSS variables. Do not guess component APIs.
- Theme library components through CSS custom properties (defined in `app.css`), not by wrapping or forking them.

---

## 12. General Principles

### 12.1 Convention Over Configuration

Follow established patterns. When in doubt, look at how a completed module does it and do the same thing.

### 12.2 Explicit Over Implicit

- Barrel exports define public API — no wildcard re-exports of everything.
- `null` over `undefined` — one way to express absence.
- Decoders over assertions — prove the shape, don't assume it.
- URL state over hidden state — if the user cares about it, it should be in the URL.

### 12.3 Boundaries

- Validate at the boundary (API responses, URL params, localStorage reads). Trust code inside the boundary.
- Each module is a boundary. Its barrel is its interface.
- Generated code is a boundary. The schema is the contract.

### 12.4 No Premature Abstraction

- Three similar lines are better than a premature helper.
- Build for what you need now. The second module will reveal the real pattern.
- If a utility is used by one module, it stays in that module. Promote to shared only when a second consumer appears.

### 12.5 Delete Over Deprecate

- Remove unused code. Don't comment it out, don't leave `// TODO: remove` markers.
- Git history is the archive.
