const { arr } = require('../benchmark/arr')
const { quickSortByNapi } = require('../index')

test('the arr can be sorted by quickSortByNapi', () => {
  expect(quickSortByNapi(arr).join('') === arr.sort().join(''))
})
