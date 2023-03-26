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
}

fn a() -> bool {
    println!("call a");
    true
}

fn b() -> bool {
    println!("call b");
    true
}
