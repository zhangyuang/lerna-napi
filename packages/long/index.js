const { loadBinding } = require('@node-rs/helper')

const binding = loadBinding(__dirname, 'long')

module.exports = {
  ...binding
}
