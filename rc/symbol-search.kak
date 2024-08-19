# ────────────── commands ──────────────
define-command symbol-search -docstring "search for symbols in files in the current working directory" %{
  popup \
    --title 'symbol search' \
    --kak-script %{evaluate-commands "edit %opt{popup_output}"} -- \
    kak-symbol-search --config %opt{symbol_search_config} --cache-dir "/tmp/kak-symbol-search/%val{session}"
}

# ────────────── mappings ──────────────
map global normal <c-r> ': symbol-search<ret>'

# ────────────── configuration ──────────────
declare-option str symbol_search_config
