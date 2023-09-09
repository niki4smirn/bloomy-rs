# bloomy-rs

Fast bloom filter implementation. 

Stack-based alternative of [fastbloom-rs](https://github.com/yankun1992/fastbloom).

### Benchmarks

|                             | `bloomy`                  | `fastbloom`                       |
|:----------------------------|:--------------------------|:--------------------------------- |
| **`insert`**                | `16.58 ms` (✅ **1.00x**)  | `19.97 ms` (❌ *1.20x slower*)     |
| **`contains_existing`**     | `14.49 ms` (✅ **1.00x**)  | `19.19 ms` (❌ *1.32x slower*)     |
| **`contains_non_existing`** | `152.97 us` (✅ **1.00x**) | `206.73 us` (❌ *1.35x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

### WARNING

It works only with nightly toolchain, because of #![feature(generic_const_exprs)].
