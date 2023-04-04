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

    // 加減乗除
    let addition: u64 = 1234 + 567;
    let subtraction: u64 = 678 - 168;
    let multiplication: u64 = 56 * 146;
    let division: u64 = 572 / 43;
    let remainder: u64 = 145 % 23;
    println!("1234 + 567 = {}", addition);
    println!("678 - 168  = {}", subtraction);
    println!("56 * 146   = {}", multiplication);
    println!("572 / 43   = {}", division);
    println!("145 % 23   = {}", remainder);

    // 比較
    let lt: bool = 1234 < 567;
    let lteq: bool = 678 <= 168;
    let gt: bool = 56 > 146;
    let gteq: bool = 572 >= 43;
    let eq: bool = 145 == 23;
    println!("1234 < 567 = {}", lt);
    println!("678 <= 168 = {}", lteq);
    println!("56 > 146   = {}", gt);
    println!("572 >= 43  = {}", gteq);
    println!("145 == 23  = {}", eq);
}

fn a() -> bool {
    println!("call a");
    true
}

fn b() -> bool {
    println!("call b");
    true
}
