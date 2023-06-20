# ETKA Life

Dieses Repo enthält die Code-Beispiel für den Vortrag `Web-Development mit Rust/WASM`

## Branches

Es gibt vier Branches:
- `master` hält das fertig implementierte Beispiel, welches sowohl Rust als auch JS verwendet
- `javascript_presentation` wird für den ersten Teil des live Codings verwendet, nach einfügen der Snippets ist der Code identisch zum dem im `master` Branch
- `dioxus` hält das fertig implementierte Beispiel, welches nur Rust verwendet
- `dioxus_presentation` wird für den zweiten Teil des live Codings verwendet, nach einfügen der Snippets ist der Code identisch zum dem im `dioxus` Branch

## Ausführen der Beispiele

Um beide Beispiele ausführen zu können wird folgendes benötigt:
- Rust siehe [hier](https://www.rust-lang.org/tools/install)
- Node.js / npm
- Dioxus cli (cargo add dioxus)

Das Beispiel im `master` / `javascript_presentation` Branch wird nach einem initialen `npm i`, mit `wasm-pack build` gebaut, im Anschluss kann es mit `npm run serve` unter `localhost:8080` bereitgestellt werden.

Das Beispiel im `dioxus` / `dioxus_presentation` Branch wird mit `dioxus serve` gebaut und unter `localhost:8080` bereitgestellt.
