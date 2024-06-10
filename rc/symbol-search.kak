# ────────────── commands ──────────────
define-command symbol-search -docstring "search for symbols in files in the current working directory" %{
  popup \
    --title 'symbol search' \
    --kak-script %{evaluate-commands "edit-and-goto %opt{popup_output}"} -- \
    kak-symbol-search --config %opt{symbol_search_config}
}

define-command edit-and-goto -hidden -params 3 -docstring "edit a file at a cursor position" %{
  edit %arg{1}
  # go to line %arg{2}, go to column %arg{3} (one-indexed), go back one column (to zero index)
  execute-keys %arg{2} g %arg{3} l h
}

# ────────────── mappings ──────────────
map global normal <c-r> ': symbol-search<ret>'

# ────────────── configuration ──────────────
declare-option str symbol_search_config %{
[rust]
extensions = ["rs"]

[rust.symbols.struct]
type = 'class'
regex = '^\s*(pub(\(([^)]+)\))?\s+)?(struct|enum)\s+(?<item>[A-Za-z0-9_]+)'

[rust.symbols.fn]
type = 'function'
regex = '^\s*(pub(\(([^)]+)\))?\s+)?((const|async|unsafe|extern\s+r?"[^"]*")\s+)*fn\s+(?<item>[A-Za-z0-9_]+)'

[rust.symbols.const]
type = 'global'
regex = '^\s*(pub(\(([^)]+)\))?\s+)?(const|static)\s+(mut\s+)?(?<item>[A-Za-z0-9_]+)'


[python]
extensions = ["py"]

[python.symbols.class]
type = 'class'
regex = '^\s*class\s+(?<item>[A-Za-z0-9_]+)'

[python.symbols.def]
type = 'function'
regex = '^\s*def\s+(?<item>[A-Za-z0-9_]+)'

[python.symbols.global]
type = 'global'
regex = '^(?<item>[A-Za-z0-9_]+)\s+='
}
