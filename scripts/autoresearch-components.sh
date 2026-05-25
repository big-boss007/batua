#!/bin/bash
set -euo pipefail

# Autoresearch: Count custom HTML elements that should use @juspay/svelte-ui-components
# Metric direction: DOWN (fewer = better)

cd "$(dirname "$0")/.."

count=0

# Count raw <button> tags (excluding modal-close, dropdown items, and nav)
buttons=$(grep -rn '<button ' frontend/src/lib/client/modules/*/ui/*.svelte frontend/src/routes/admin/**/*.svelte 2>/dev/null | grep -v 'node_modules' | grep -v '.svelte-kit' | wc -l | tr -d ' ')

# Count raw <input> tags
inputs=$(grep -rn '<input ' frontend/src/lib/client/modules/*/ui/*.svelte frontend/src/routes/admin/**/*.svelte 2>/dev/null | grep -v 'node_modules' | grep -v '.svelte-kit' | wc -l | tr -d ' ')

# Count raw <select> tags
selects=$(grep -rn '<select ' frontend/src/lib/client/modules/*/ui/*.svelte frontend/src/routes/admin/**/*.svelte 2>/dev/null | grep -v 'node_modules' | grep -v '.svelte-kit' | wc -l | tr -d ' ')

total=$((buttons + inputs + selects))

echo "=== Component Audit ==="
echo "  Raw <button>: $buttons"
echo "  Raw <input>:  $inputs"
echo "  Raw <select>: $selects"
echo "METRIC custom_elements=$total"
