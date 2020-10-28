use std::convert::TryFrom;

use napi::{register_module, CallContext, JsNumber, JsObject, Module, Result};
use napi_derive::js_function;

#[cfg(all(unix, not(target_env = "musl")))]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[cfg(windows)]
#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

register_module!(example, init);

fn init(module: &mut Module) -> Result<()> {
  module.create_named_method("quickSort", quick_sort)?;
  Ok(())
}

#[js_function(1)]
fn quick_sort(ctx: CallContext) -> Result<JsObject> {
  let arr = ctx.get::<JsObject>(0)?;
  let len = arr.get_array_length()?;
  let mut arr_vec: Vec<u32> = vec![];
  for i in 0..len {
    // jsObject -> vec
    let val_u32 = u32::try_from(arr.get_element::<JsNumber>(i)?)?;
    arr_vec.push(val_u32)
  }
  arr_vec = quick_sort_rust(arr_vec);
  let mut arr_res = ctx.env.create_array_with_length(len as usize)?;
  for i in 0..len {
    arr_res.set_element(i, ctx.env.create_uint32(arr_vec[i as usize])?)?;
  }
  Ok(arr_res)
}

fn quick_sort_rust(mut arr: Vec<u32>) -> Vec<u32> {
  // 快速排序 by 阮一峰版本，需要开辟额外空间不是最佳答案
  let len = arr.len();
  if len == 1 || len == 0 {
    return arr;
  }
  let mut left_vec = vec![];
  let mut right_vec = vec![];
  let sign_val = arr[len / 2];
  arr.remove(len / 2);
  for i in 0..arr.len() {
    let val = arr[i];
    if val < sign_val {
      left_vec.push(val);
    } else {
      right_vec.push(val)
    }
  }
  [
    quick_sort_rust(left_vec),
    vec![sign_val],
    quick_sort_rust(right_vec),
  ]
  .concat()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn tests() {
    quick_sort_rust(vec![2, 1, 3, 4, 2]);
  }
}
