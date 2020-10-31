// const { loadBinding } = require('@node-rs/helper')
// const binding = loadBinding(__dirname, 'long')

// module.exports = {
//   ...binding
// }
const Long = require('long')
var longVal = new Long(0xFFFFFFFF, 0x7FFFFFFF)

console.log(longVal.toString())
