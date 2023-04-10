fn main() {
    // 論理演算
    let short_circuit_evaluation_and: bool = true && false;
    let short_circuit_evaluation_or: bool = true || false;
    let non_short_circuit_evaluation_and: bool = true & false;
    let non_short_circuit_evaluation_or: bool = true | false;
    let exclusive_or: bool = false ^ true;
    let logical_not: bool = !true;

    println!("true && false = {}", short_circuit_evaluation_and);
    println!("true || false = {}", short_circuit_evaluation_or);
    println!("true & false  = {}", non_short_circuit_evaluation_and);
    println!("true | false  = {}", non_short_circuit_evaluation_or);
    println!("false ^ true  = {}", exclusive_or);
    println!("!true         = {}", logical_not);

    // 短絡評価と非短絡評価
    println!("短絡評価");
    println!("{}", a() || b());

    println!("非短絡評価");
    println!("{}", a() | b());

    // 整数の加減乗除
    let addition_u: u64 = 1234 + 567;
    let subtraction_u: u64 = 678 - 168;
    let multiplication_u: u64 = 56 * 146;
    let division_u: u64 = 572 / 43;
    let remainder_u: u64 = 145 % 23;
    println!("1234 + 567 = {}", addition_u);
    println!("678 - 168  = {}", subtraction_u);
    println!("56 * 146   = {}", multiplication_u);
    println!("572 / 43   = {}", division_u);
    println!("145 % 23   = {}", remainder_u);

    // 整数の比較
    let lt_u: bool = 1234 < 567;
    let lteq_u: bool = 678 <= 168;
    let gt_u: bool = 56 > 146;
    let gteq_u: bool = 572 >= 43;
    let eq_u: bool = 145 == 23;
    println!("1234 < 567 = {}", lt_u);
    println!("678 <= 168 = {}", lteq_u);
    println!("56 > 146   = {}", gt_u);
    println!("572 >= 43  = {}", gteq_u);
    println!("145 == 23  = {}", eq_u);

    // 浮動小数点数の加減乗除
    let addition_f: f64 = 1.245 + 0.57;
    let subtraction_f: f64 = 2.5 - 123.98;
    let multiplication_f: f64 = 5.6 * 14.6;
    let division_f: f64 = 57.2 / 43.2;
    let remainder_f: f64 = 145.6 % 23.4;
    println!("1.245 + 0.57 = {}", addition_f);
    println!("2.5 - 123.98 = {}", subtraction_f);
    println!("5.6 * 14.6   = {}", multiplication_f);
    println!("57.2 / 43.2  = {}", division_f);
    println!("145.6 % 23.4 = {}", remainder_f);

    // 浮動小数点数の比較
    let lt_f: bool = 1.245 < 0.57;
    let lteq_f: bool = 2.5 <= 123.98;
    let gt_f: bool = 5.6 > 14.6;
    let gteq_f: bool = 57.2 >= 43.2;
    let eq_f: bool = 145.6 == 23.4;
    println!("1.245 < 0.57  = {}", lt_f);
    println!("2.5 <= 123.98 = {}", lteq_f);
    println!("5.6 > 14.6    = {}", gt_f);
    println!("57.2 >= 43.2  = {}", gteq_f);
    println!("145.6 == 23.4 = {}", eq_f);

    // ビットシフトの例
    let bit_shift_n: u8 = 0b0001_1000;
    let bit_shift_m: u8 = bit_shift_n << 2; // 2ビット左シフト
    let bit_shift_k: u8 = bit_shift_n >> 2; // 2ビット右シフト
    println!("0b0001_1000 << 2 = 0b{:08b}", bit_shift_m);
    println!("0b0001_1000 >> 2 = 0b{:08b}", bit_shift_k);

    // 算術シフトの例
    let arithmetic_shift_p: i8 = -64; // 0b1100_0000
    let arithmetic_shift_k: i8 = arithmetic_shift_p >> 2; // 2ビット右算術シフト
    let arithmetic_shift_m: i8 = arithmetic_shift_p << 2; // 2ビット左算術シフト
    println!("0b1100_0000 >> 2 = 0b{:08b}", arithmetic_shift_k);
    println!("0b1100_0000 << 2 = 0b{:08b}", arithmetic_shift_m);
}

fn a() -> bool {
    println!("call a");
    true
}

fn b() -> bool {
    println!("call b");
    true
}
