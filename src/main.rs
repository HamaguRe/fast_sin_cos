// sin,cosの高速（になりそう）な実装
//
// 組み込み関数めっちゃ早い...(3倍以上)

use std::time::Instant;

const PI: f64 = std::f64::consts::PI;
const TWO_PI: f64 = 2.0 * PI;

// sinの分母の逆数を16進数表記で保持(n = 0 to 24)
// 200Byte
const SIN_DENOMS_RECIP: [u64; 25] = [
    0x3FF0000000000000,
    0x3FC5555555555555,
    0x3F81111111111111,
    0x3F2A01A01A01A01A,
    0x3EC71DE3A556C734,
    0x3E5AE64567F544E4,
    0x3DE6124613A86D09,
    0x3D6AE7F3E733B81F,
    0x3CE952C77030AD4A,
    0x3C62F49B46814157,
    0x3BD71B8EF6DCF572,
    0x3B4761B413163819,
    0x3AB3F3CCDD165FAA,
    0x3A1D1AB1C2DCCEA3,
    0x398259F98B4358AE,
    0x38E434D2E783F5BD,
    0x3843981254DD0D52,
    0x37A0DC59C716D91E,
    0x36F9EC8D1C94E85B,
    0x3651E99449A4BACE,
    0x35A65E61C39D0241,
    0x34F95DB45257E511,
    0x344A3CB872220649,
    0x3398DA8E0A127EB9,
    0x32E5A42F0DFEB089,
];

const COS_DENOMS_RECIP: [u64; 25] = [
    0x3FF0000000000000,
    0x3FE0000000000000,
    0x3FA5555555555555,
    0x3F56C16C16C16C17,
    0x3EFA01A01A01A01A,
    0x3E927E4FB7789F5C,
    0x3E21EED8EFF8D898,
    0x3DA93974A8C07C9D,
    0x3D2AE7F3E733B81F,
    0x3CA6827863B97D97,
    0x3C1E542BA4020225,
    0x3B90CE396DB7F853,
    0x3AFF2CF01972F578,
    0x3A688E85FC6A4E5B,
    0x39D0A18A2635085D,
    0x3933932C5047D60E,
    0x389434D2E783F5BD,
    0x37F2710231C0FD7B,
    0x374DF983290C2CA8,
    0x36A5D4ACB9C0C3AB,
    0x35FCA8ED42A12AE0,
    0x35510AF527530DEA,
    0x34A272B1B03FEC6B,
    0x33F240804F659510,
    0x334091B406B6FF28,
];

fn main() {
    println!("sin(3.14): {}", fast_sin(2.0 * PI));

    // ----- 実行速度計測 ----- //
    // 組み込み sin()
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
        num += fast_sin(i as f64);
    }
    let end = start.elapsed();
    let second = end.as_secs();
    let nano_second = end.subsec_nanos();
    let time = (second as f64) + ( (nano_second as f64) * 0.000_000_001);
    println!("result num: {}", num);
    println!("fast_sin time: {}[s]", time);

    // fast_sin_table()
    let mut num = 0.0;
    let start = Instant::now();
    for i in 0..1000000 {
        num += fast_sin_table(i as f64);
    }
    let end = start.elapsed();
    let second = end.as_secs();
    let nano_second = end.subsec_nanos();
    let time = (second as f64) + ( (nano_second as f64) * 0.000_000_001);
    println!("result num: {}", num);
    println!("fast_sin_table() time: {}[s]", time);

    // 組み込み sin_cos()
    let mut num0 = 0.0;
    let mut num1 = 0.0;
    let start = Instant::now();
    for i in 0..1000000 {
        let f = (i as f64).sin_cos();
        num0 += f.0;
        num1 += f.1;
    }
    let end = start.elapsed();
    let second = end.as_secs();
    let nano_second = end.subsec_nanos();
    let time = (second as f64) + ( (nano_second as f64) * 0.000_000_001);
    println!("result num0: {}, num1: {}", num0, num1);
    println!("f64 method sin_cos() time: {}[s]", time);

    // fast_sin_cos()
    let mut num0 = 0.0;
    let mut num1 = 0.0;
    let start = Instant::now();
    for i in 0..1000000 {
        let f = fast_sin_cos(i as f64);
        num0 += f.0;
        num1 += f.1;
    }
    let end = start.elapsed();
    let second = end.as_secs();
    let nano_second = end.subsec_nanos();
    let time = (second as f64) + ( (nano_second as f64) * 0.000_000_001);
    println!("result num0: {}, num1: {}", num0, num1);
    println!("fast_sin_cos() time: {}[s]", time);

    // fast_sin_cos_table()
    let mut num0 = 0.0;
    let mut num1 = 0.0;
    let start = Instant::now();
    for i in 0..1000000 {
        let f = fast_sin_cos_table(i as f64);
        num0 += f.0;
        num1 += f.1;
    }
    let end = start.elapsed();
    let second = end.as_secs();
    let nano_second = end.subsec_nanos();
    let time = (second as f64) + ( (nano_second as f64) * 0.000_000_001);
    println!("result num0: {}, num1: {}", num0, num1);
    println!("fast_sin_cos_table() time: {}[s]", time);
}

// 分母を定数配列に持てばもっと早くなりそう
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

// 分母はあらかじめ計算しておいた値を使う
// 除算が無くなった
fn fast_sin_table(x: f64) -> f64 {
    let x = x % TWO_PI;  // 入力値の範囲を制限
    let pow_2 = x*x;
    let mut numer = x;  // 分子
    let mut sin_x = x;
    for i in 1..25u64 {
        numer *= pow_2;
        let tmp = numer * f64::from_bits( SIN_DENOMS_RECIP[i as usize] );
        if (i & 1) == 1 {  // iが奇数
            sin_x -= tmp;
        } else {
            sin_x += tmp;
        }
    }
    sin_x
}

fn fast_cos(x: f64) -> f64 {
    let x = x % TWO_PI;
    let pow_2 = x*x;
    let mut numer = 1.0;
    let mut denom = 1.0;
    let mut cos_x = 1.0;
    for i in 1..25u64 {
        numer *= pow_2;
        let two_i = i << 1;
        denom *= ( two_i * (two_i - 1) ) as f64;
        let tmp = numer / denom;
        if (i & 1) == 1 {
            cos_x -= tmp;
        } else {
            cos_x += tmp;
        }
    }
    cos_x
}

/// return (sin(x), cos(x))
fn fast_sin_cos(x: f64) -> (f64, f64) {
    let x = x % TWO_PI;
    let pow_2 = x*x;
    let mut sin_numer = x;
    let mut sin_denom = 1.0;
    let mut sin_x = x;
    let mut cos_numer = 1.0;
    let mut cos_denom = 1.0;
    let mut cos_x = 1.0;
    for i in 1..25u64 {
        sin_numer *= pow_2;
        cos_numer *= pow_2;
        let two_i = i << 1;
        sin_denom *= ( two_i * (two_i + 1) ) as f64;
        cos_denom *= ( two_i * (two_i - 1) ) as f64;
        let sin_tmp = sin_numer / sin_denom;
        let cos_tmp = cos_numer / cos_denom;
        if (i & 1) == 1 {
            sin_x -= sin_tmp;
            cos_x -= cos_tmp;
        } else {
            sin_x += sin_tmp;
            cos_x += cos_tmp;
        }
    }
    (sin_x, cos_x)
}

/// return (sin(x), cos(x))
fn fast_sin_cos_table(x: f64) -> (f64, f64) {
    let x = x % TWO_PI;
    let pow_2 = x*x;
    let mut sin_numer = x;
    let mut sin_x = x;
    let mut cos_numer = 1.0;
    let mut cos_x = 1.0;
    for i in 1..25 {
        sin_numer *= pow_2;
        cos_numer *= pow_2;
        let sin_tmp = sin_numer * f64::from_bits( SIN_DENOMS_RECIP[i] );
        let cos_tmp = cos_numer * f64::from_bits( COS_DENOMS_RECIP[i] );
        if (i & 1) == 1 {
            sin_x -= sin_tmp;
            cos_x -= cos_tmp;
        } else {
            sin_x += sin_tmp;
            cos_x += cos_tmp;
        }
    }
    (sin_x, cos_x)
}