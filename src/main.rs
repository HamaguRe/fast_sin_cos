// sin,cosの高速（になりそう）な実装
//
// 組み込み関数めっちゃ早い...(3倍以上)

use std::time::Instant;

const PI: f64 = std::f64::consts::PI;
const TWO_PI: f64 = 2.0 * PI;

fn main() {
    println!("sin(3.14): {}", fast_sin(2.0 * PI));

    // ----- 実行速度計測 ----- //
    // 組み込み関数
    let mut num = 0.0;
    let start = Instant::now();
    for i in 0..1000000 {
        num += (i as f64).sin();
    }
    let end = start.elapsed();
    let second = end.as_secs();
    let nano_second = end.subsec_nanos();
    let time = (second as f64) + ( (nano_second as f64) * 0.000_000_001);
    println!("result num: {}", num);
    println!("f64 method sin time: {}[s]", time);

    // fast_sin()
    let mut num = 0.0;
    let start = Instant::now();
    for i in 0..1000000 {
        num += fast_sin( i as f64 );
    }
    let end = start.elapsed();
    let second = end.as_secs();
    let nano_second = end.subsec_nanos();
    let time = (second as f64) + ( (nano_second as f64) * 0.000_000_001);
    println!("result num: {}", num);
    println!("fast_sin time: {}[s]", time);

}

// 一つ前の値を使いまわして高速化
#[inline(always)]
fn fast_sin(x: f64) -> f64 {
    let x = x % TWO_PI;  // 入力値の範囲を制限
    let pow_2 = x*x;
    let mut numer = x;  // 分子
    let mut denom = 1.0;  // 分母
    let mut sin_x = x;
    for i in 1..25u64 {
        numer *= pow_2;
        let two_i = i << 1;
        denom *= ( two_i * (two_i + 1) ) as f64;
        let tmp = numer / denom;
        if (i & 1) == 1 {  // iが奇数
            sin_x -= tmp;
        } else {
            sin_x += tmp;
        }
    }
    sin_x
}