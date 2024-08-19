# ────────────── commands ──────────────
define-command symbol-search -docstring "search for symbols in files in the current working directory" %{
  popup \
    --title 'symbol search' \
    --kak-script %{evaluate-commands "edit-and-goto %opt{popup_output}"} -- \
    kak-symbol-search --config %opt{symbol_search_config} --cache-dir "/tmp/kak-symbol-search/%val{session}"
}

define-command edit-and-goto -hidden -params 3 -docstring "edit a file at a cursor position" %{
  edit %arg{1}
  # go to line %arg{2}, go to column %arg{3} (one-indexed), go back one column (to zero index)
  execute-keys %arg{2} g %arg{3} l h
}

# ────────────── mappings ──────────────
map global normal <c-r> ': symbol-search<ret>'

# ────────────── configuration ──────────────
declare-option str symbol_search_config
