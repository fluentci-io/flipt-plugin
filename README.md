# Flipt Plugin

[![ci](https://github.com/fluentci-io/flipt-plugin/actions/workflows/ci.yml/badge.svg)](https://github.com/fluentci-io/flipt-plugin/actions/workflows/ci.yml)

This plugin sets up your CI/CD pipeline with a specific version of [flipt](https://flipt.io).

## 🚀 Usage

Add the following command to your CI configuration file:

```bash
fluentci run --wasm flipt setup
```

## Functions

| Name     | Description                                   |
| -------- | --------------------------------------------- |
| setup    | Installs a specific version of flipt          |
| bundle   | Manage Flipt bundles                          |
| validate | Validate Flipt flag state (.yaml, .yml) files |

## Code Usage

Add `fluentci-pdk` crate to your `Cargo.toml`:

```toml
[dependencies]
fluentci-pdk = "0.1.9"
```

Use the following code to call the plugin:

```rust
use fluentci_pdk::dag;

// ...

dag().call("https://pkg.fluentci.io/flipt@v0.1.0?wasm=1", "setup", vec!["latest"])?;
```

## 📚 Examples

Github Actions:

```yaml
- name: Setup Fluent CI CLI
  uses: fluentci-io/setup-fluentci@v5
  with:
    wasm: true
    plugin: flipt
    args: |
      setup
    working-directory: example
- name: Show flipt version
  run: |
    type flipt
    flipt --version
```
