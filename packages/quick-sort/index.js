const { loadBinding } = require('@node-rs/helper')

const binding = loadBinding(__dirname, 'quick-sort')

module.exports = {
  quickSortByNapi: binding.quickSort
}
