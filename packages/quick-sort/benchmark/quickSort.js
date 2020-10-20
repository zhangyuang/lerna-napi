const { Suite } = require('benchmark')
const { arr } = require('./arr')
const { quickSortByNapi } = require('../index.js')

const suite = new Suite()

const quickSort = function (arr) {
  // 快速排序 阮一峰版
  if (arr.length <= 1) { return arr }// 判断数组，一个长度直接返回
  const pivotIndex = Math.floor(arr.length / 2)
  const pivot = arr.splice(pivotIndex, 1)[0]// 找出基准元素
  const left = []
  const right = []
  for (let i = 0; i < arr.length; i++) {
    if (arr[i] < pivot) {
      left.push(arr[i])
    } else {
      right.push(arr[i])
    }
  }
  return quickSort(left).concat([pivot], quickSort(right))
}

suite
  .add('quickSort by js', () => {
    quickSort(arr)
  })
  .add('quickSort by napi', () => {
    quickSortByNapi(arr)
  })
  .on('cycle', function (event) {
    console.log(String(event.target))
  })
  .on('complete', function () {
    console.log('Fastest is ' + this.filter('fastest').map('name'))
  })
  // run async
  .run({ async: true })
