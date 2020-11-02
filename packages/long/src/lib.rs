// use std::convert::TryFrom;

// use napi::{register_module, CallContext, JsNumber, JsObject, Module, Result};
// use napi_derive::js_function;

// #[cfg(all(unix, not(target_env = "musl")))]
// #[global_allocator]
// static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

// #[cfg(windows)]
// #[global_allocator]
// static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

// register_module!(example, init);

// fn init(module: &mut Module) -> Result<()> {
//   // module.create_named_method("quickSort", quick_sort)?;
//   Ok(())
// }

// // #[js_function(1)]
// // fn long(ctx: CallContext) -> Result<JsObject> {
// // }

// #[cfg(test)]
// mod tests {
//   use super::*;

//   #[test]
//   fn tests() {
//     // quick_sort_rust(vec![2, 1, 3, 4, 2]);
//   }
// }

pub struct Long {
  // 分别用 高低位 32 位有符号数 组成 64 位双精度整数
  low: i32,       // 低 32 位有符号数
  high: i32,      // 高 32 位有符号数
  unsigned: bool, // 生成的结果是否是无符号的
}

impl Long {
  fn is_zero(&self) -> bool {
    self.low == 0 && self.high == 0
  }
  // fn fromNumber(&self, val: i64, unsigned: bool) -> {
  //   //
  // }
  fn get_zero(&self) -> Long {
    self.from_int(0, true)
  }
  fn get_uzero(&self) -> Long {
    self.from_int(0, false)
  }

  fn from_int(&self, value: i32, unsigned: bool) -> Long {
    if unsigned {
      // 无符号数
    } else {
      // 有符号数
    }
    Long {
      low: 1,
      high: 1,
      unsigned: false,
    }
  }
  fn from_bits(&self, lowBits: i32, highBits: i32, unsigned: bool) -> Long {
    Long {
      low: lowBits,
      high: highBits,
      unsigned: unsigned,
    }
  }
}
