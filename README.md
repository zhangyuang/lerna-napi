# Napi + Rust + Lerna

![CI](https://github.com/zhangyuang/lerna-napi/workflows/CI/badge.svg)
![coverage](https://codecov.io/gh/zhangyuang/learn-node/graph/badge.svg)

Develop napi by rust  
Manage and deploy crates by lerna

## Getting Start

NPM Script

```bash
$ yarn build # build all napi crates
```

## How to use

Run `yarn build` will generate `[filename].node` file which you can require in Node.js directly

## Publish in NPM

Github CI will publish all crates to NPM automaticlly

```bash
$ npx lerna version --conventional-commits # custom version or patch|minor|major
$ git push origin master
```