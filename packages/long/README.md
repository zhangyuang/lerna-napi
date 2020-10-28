# long

A Long class for representing a 64 bit two's-complement integer value derived from the Closure Library for stand-alone use and extended with unsigned support.

ref: [long.js](https://www.npmjs.com/package/long)

## IEEE754 

Please read this [article]((https://medium.com/starbugs/see-why-floating-point-error-can-not-be-avoided-from-ieee-754-809720b32175)) for understand [IEEE754](https://zh.wikipedia.org/wiki/IEEE_754)

![](https://res.wx.qq.com/op_res/Es_QOqoJrxq9VdEDWD4vrP34vr73D2xIe8rs-z_5IBY_oDoEgYzwtUiySIfedUb6)

[IEEE 754 Calculator](http://weitz.de/ieee/)

### Why JS MAX_SAFE_INTEGER is 2^53 - 1

As IEEE754 says, double type variable has 1 sign, 11 exponent, 52 fraction。Due to Number 1 is the value beginning default, so the exponent max value is 53. why MAX_SAFE_INTEGER is 2^53 - 1 because if number is more than 2^53 such as 2^53 and 2^53 + 1 whose fraction will be same.