# quickSort

As this [article](https://zhuanlan.zhihu.com/p/234914336) says, napi call overhead is expensive。it's not apposite to realize sort algorithms

```bash
$ yarn bench
yarn run v1.22.4
$ cross-env NODE_ENV=production node benchmark/quickSort.js
quickSort by js x 145,726,560 ops/sec ±0.40% (90 runs sampled)
quickSort by napi x 2,517,344 ops/sec ±0.34% (89 runs sampled)
Fastest is quickSort by js
✨  Done in 12.98s.
```