# symbol-search.kak

> [!NOTE]
> This plugin is still undergoing development, but is usable.

Search symbols across various languages among files in your current working directory.

![demo.gif][2]

Symbols are extracted using [syntect][1] which parses source code using Sublime Text
syntax files.

## Installation

1. Install the [requirements][3].
2. Install [kak-symbol-search][4] create either with cargo or with nix.
3. Place [`symbol-search.kak`][5] where kakoune will load it.

## Configuration

Two things can be configured: the fzf appearance, and the list of symbols that are searchable.
Configuration is done through a [TOML][6] string, on a per-language basis. Set the
`symbol_search_config` kakoune option to your configuration. See [`symbol-search.kak`][7]
or [`example-config.toml`][8] for an example basic configuration.

## Requirements

- [popup.kak][9]
- [fd][10]

## TODO
- [ ] vendor sublime syntax files
- [ ] script to generate serialized `SyntaxSet`
- [ ] fzf modes for searching specific symbol kinds

[1]: https://github.com/trishume/syntect
[2]: demo.gif
[3]: #requirements
[4]: https://crates.io/crates/kak-symbol-search
[5]: ./rc/symbol-search.kak
[6]: https://toml.io/en/
[7]: ./rc/symbol-search.kak
[8]: ./example-config.toml
[9]: https://github.com/enricozb/popup.kak
[10]: https://github.com/sharkdp/fd
