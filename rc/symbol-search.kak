# ────────────── commands ──────────────
define-command symbol-search -docstring "search for symbols in files in the current working directory" %{
  popup --title open --kak-script %{
    edit-and-goto %opt{popup_output}
  } -- "
    for ext in $(fd . --type f | sed -e 's/.*\.//'); do
      echo "extension: $ext"

      for symbol_config in $(
        echo $kak_opt_symbol_search_config |
          jq -r '.[] | select(.extension == $ext) | .symbols[] | "\(.type)§\(.regex)"' --arg ext $ext
      ); do
        IFS=§ read -r type regex <<EOF
          $symbol_config
EOF
        rg --only-matching --type $type $regex
      done
    done
  "
}

    define-command edit-and-goto -hidden -params 1 -docstring "edit a file at a cursor position" %{
  echo %arg{1}
}
