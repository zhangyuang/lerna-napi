# Napi + Rust + Lerna

Develop napi by rust
Manage and deploy crates by lerna

## Getting Start

NPM Script

```bash
$ yarn build # build all napi crate
```

## How to use

Run `yarn build` will generate [filename].node file which you can require in Node.js directly

## Publish To Npm

Github CI will publish all crates to NPM automaticlly

```bash
$ lerna version # custom version or patch|minor|major
$ git push origin master
```