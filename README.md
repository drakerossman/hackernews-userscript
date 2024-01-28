Available for Firefox and Chrome:

![Firefox Add-Onn](./firefox-store.webp) ![Chrome Extension](./chrome-store.webp)

# Description
Hacker News Userscript - a Browser Extension to Play Hide and Seek!

The Usescript is written in Rust with Web Assembly compile target. Based on the [Mubelotix's Wasm Extension Template](https://github.com/Mubelotix/wasm-extension-template).

The extension provides features, which allow users to filter submissions based on a text or a regular expression:
- Filter submissions by text in title or link
- Filter comments by text
- Hide submissions and comments from a specific user
- Get visual feedback on how many items were hidden by a particular filter
- Toggle hide of all the children for a hidden parent comment
- Show `[hidden]` placeholder in place of hidden items
- Hide an individual submission, without auto-loading the next one. You can hide everything on the page, and it will stay blank!

Uses [Rust's Regex](https://github.com/rust-lang/regex) to enable expression matching. Regardless of regex usage, matches by text as a substring in an item

All the filtering data and settings are stored in the localstorage.

See the videos below for the features showcase!

# Features Showcase
https://github.com/drakerossman/hackernews-userscript/assets/97120319/fc6d9e68-b897-4f78-b2e0-172dce9f8e6b

https://github.com/drakerossman/hackernews-userscript/assets/97120319/0a18c470-61aa-43ca-bdd5-e9079af97375

# Local Development and Installation
For the local development, a convenient `flake.nix` with devshell is provided.

1. Clone the repo:
```sh
git clone https://github.com/drakerossman/hackernews-userscript
```

2. If not already, install the nix package manager via Determinate Systems' nix installer:
```sh
curl --proto '=https' --tlsv1.2 -sSf -L https://install.determinate.systems/nix | sh -s -- install
```
It will walk you through the installation process via easy-to-follow prompts. You will also be prompted for sudo.

3. Navigate to the cloned repo's directory and invoke the devshell:
```sh
cd hackernews-userscript
nix develop
```

After fetching the files, all the tools required for compilation and development should be made ready-available in the new shell.

4. Build the extension for chrome and firefox:
```sh
./build.sh
```

To load extension in Chrome:
- navigate to [chrome://extensions/](chrome://extensions/)
- enable Developer Mode by clicking the toggle switch next to Developer mode.
- use Load unpacked and select the extension directory - pkg/chrome

To load extension in Firefox:
- navigate to [about:debugging](about:debugging)
- click "This Firefox"
- use Load Temporary Add-on, navigate to the extension directory, and select any file from the root of the directory - pkg/firefox/background-worker.js

# For Contributors
PRs are always welcome, especially if:
- you can make the interface prettier and more erogonomic
- you know how to optimize regex so it won't spend a full second on parsing one
- you are eager to refactor the codebase

Please mind, that I have been developing this extension on NixOS via the provided devShell in `flake.nix`.

For whatever reason, should you also use NixOS, and should the path to the folder of this project contain spaces, the compilation via `build.sh` would fail with the following error:
```shell
error: linking with `cc` failed: exit status: 1
```

# Coming Soon
- [ ] Highlighting by topic
- [ ] Endless scrolling
- [ ] Thread auto-update
- [ ] Background thread watcher
- [ ] Codebase refactoring
- [ ] Replies, subscriptions
- [ ] Group items by topics
- [ ] Chronological comments with reply links
- [ ] Easy point-and-click to filter an item
- [ ] Collapsible sidebar
- [ ] Better UI
- [ ] Favorites functionality
- [ ] Automatically mark already-read submissions and comments
- [ ] Export and import filters and settings
- [ ] Infotips on hover

# In the Future
- [ ] Support for other websites
- [ ] Social collaborative filtering

# Sponsor This Project
You can sponsor this work, as well as other projects of mine via Liberapay and Patreon:

<a href="https://liberapay.com/drakerossman/donate"><img alt="Donate using Liberapay" src="https://liberapay.com/assets/widgets/donate.svg" style="height: 2rem;"></a> <a href="https://patreon.com/DrakeRossman"><img alt="Donate using Patreon" src="./support-on-patreon.png" style="height: 2rem;"></a>
