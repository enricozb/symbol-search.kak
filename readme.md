# symbol-search.kak

> [!WARNING]
> This plugin is not ready to be used yet.

Search symbols across various languages among files in your current working directory.

![demo.gif][1]

## Installation

1. Install the [requirements][2].
2. Install [kak-symbol-search][3] create either with cargo or with nix.
3. Place [`symbol-search.kak`][4] where kakoune will load it.

## Configuration

Configuration is done through a [TOML][5] string, on a per-language basis. Set the
`symbol_search_config` kakoune option to your configuration. See [`symbol-search.kak`][6]
for an example basic configuration.

## Requirements
- [popup.kak][7]
- [ripgrep][8]
- [fd][9]

[1]: demo.gif
[2]: #requirements
[3]: https://crates.io/crates/kak-symbol-search
[4]: ./rc/symbol-search.kak
[5]: https://toml.io/en/
[6]: ./rc/symbol-search.kak
[7]: https://github.com/enricozb/popup.kak
[8]: https://github.com/BurntSushi/ripgrep
[9]: https://github.com/sharkdp/fd
