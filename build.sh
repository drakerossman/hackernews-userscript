#!/bin/sh

rm -rf pkg
wasm-pack build --target=no-modules --mode no-install || exit 1

mkdir tmp-target
mkdir tmp-target/firefox
mkdir tmp-target/chrome

cp pkg/* tmp-target/firefox
mv pkg/* tmp-target/chrome
mv tmp-target/* pkg
rm -rf tmp-target

cp firefox_manifest_v3.json pkg/firefox/manifest.json
cp chrome_manifest_v3.json pkg/chrome/manifest.json

cp background-worker.js pkg/firefox/background-worker.js
cp background-worker.js pkg/chrome/background-worker.js

cp icons/* pkg/firefox/
cp icons/* pkg/chrome/

printf "
const runtime = chrome.runtime || browser.runtime;
async function run() {
  await wasm_bindgen(runtime.getURL('hackernews_userscript_bg.wasm'));
}
run();
" >> pkg/firefox/run_wasm.js

printf "
const runtime = chrome.runtime || browser.runtime;
async function run() {
  await wasm_bindgen(runtime.getURL('hackernews_userscript_bg.wasm'));
}
run();
" >> pkg/chrome/run_wasm.js
