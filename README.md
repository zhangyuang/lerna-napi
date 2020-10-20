# Napi + Rust + Lerna

Develop napi by rust  
Manage and deploy crates by lerna

## Getting Start

NPM Script

```bash
$ yarn build # build all napi crates
```

## How to use

Run `yarn build` will generate [filename].node file which you can require in Node.js directly

## Publish To NPM

Github CI will publish all crates to NPM automaticlly

```bash
$ yarn version # custom version or patch|minor|major
$ git push origin master
```