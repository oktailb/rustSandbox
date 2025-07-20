use crate::add::add_strings;
use crate::div::div_strings;
use crate::div::mod_strings;
use crate::eval_expr::eval;
use crate::mul::mul_strings;
use crate::sub::sub_strings;

type ArithmeticOp = fn(&str, &str, &str) -> Result<String, String>;

struct TestCaseAtomic<'a> {
    name: &'a str,
    op: ArithmeticOp,
    base: &'a str,
    a: &'a str,
    b: &'a str,
    expected: &'a str,
}

struct TestCaseExpr<'a> {
    name: &'a str,
    base: &'a str,
    expression: &'a str,
    expected: &'a str,
}

fn get_test_cases<'a>() -> Vec<TestCaseAtomic<'a>> {
    let base10 = "0123456789";
    let base2 = "01";
    let base16 = "0123456789ABCDEF";

    vec![
        TestCaseAtomic {
            op: add_strings,
            a: "999",
            b: "1",
            expected: "1000",
            name: "Carry waterfall.",
            base: base10,
        },
        TestCaseAtomic {
            op: add_strings,
            a: "1",
            b: "999",
            expected: "1000",
            name: "Carry commutativity.,",
            base: base10,
        },
        TestCaseAtomic {
            op: add_strings,
            a: "12345",
            b: "678",
            expected: "13023",
            name: "different size operand.",
            base: base10,
        },
        TestCaseAtomic {
            op: add_strings,
            a: "0",
            b: "123",
            expected: "123",
            name: "Test with 0.",
            base: base10,
        },
        TestCaseAtomic {
            op: add_strings,
            a: "123",
            b: "0",
            expected: "123",
            name: "Commutativity of 0.",
            base: base10,
        },
        TestCaseAtomic {
            op: sub_strings,
            a: "1000",
            b: "1",
            expected: "999",
            name: "Borrowing waterfall.",
            base: base10,
        },
        TestCaseAtomic {
            op: sub_strings,
            a: "123",
            b: "45",
            expected: "78",
            name: "Simple borrowing.",
            base: base10,
        },
        TestCaseAtomic {
            op: sub_strings,
            a: "45",
            b: "123",
            expected: "-78",
            name: "Negative result.",
            base: base10,
        },
        TestCaseAtomic {
            op: sub_strings,
            a: "123",
            b: "123",
            expected: "0",
            name: "Null result.",
            base: base10,
        },
        TestCaseAtomic {
            op: sub_strings,
            a: "12345",
            b: "12300",
            expected: "45",
            name: "Big numbers with small result.",
            base: base10,
        },
        TestCaseAtomic {
            op: sub_strings,
            a: "123",
            b: "0",
            expected: "123",
            name: "Substraction of 0.",
            base: base10,
        },
        TestCaseAtomic {
            op: mul_strings,
            a: "999",
            b: "999",
            expected: "998001",
            name: "Large intermediate values.",
            base: base10,
        },
        TestCaseAtomic {
            op: mul_strings,
            a: "12345",
            b: "0",
            expected: "0",
            name: "Multiplication by 0.",
            base: base10,
        },
        TestCaseAtomic {
            op: mul_strings,
            a: "0",
            b: "12345",
            expected: "0",
            name: "Commutativity of 0.",
            base: base10,
        },
        TestCaseAtomic {
            op: mul_strings,
            a: "12345",
            b: "1",
            expected: "12345",
            name: "Multiplication by 1.",
            base: base10,
        },
        TestCaseAtomic {
            op: mul_strings,
            a: "100",
            b: "543",
            expected: "54300",
            name: "Multiplication by base power.",
            base: base10,
        },
        TestCaseAtomic {
            op: div_strings,
            a: "123",
            b: "0",
            expected: "Err",
            name: "Division by zero.",
            base: base10,
        },
        TestCaseAtomic {
            op: div_strings,
            a: "45",
            b: "123",
            expected: "0",
            name: "Dividend < Divisor.",
            base: base10,
        },
        TestCaseAtomic {
            op: div_strings,
            a: "123",
            b: "123",
            expected: "1",
            name: "Self division.",
            base: base10,
        },
        TestCaseAtomic {
            op: div_strings,
            a: "123",
            b: "1",
            expected: "123",
            name: "Division by one.",
            base: base10,
        },
        TestCaseAtomic {
            op: div_strings,
            a: "5535",
            b: "45",
            expected: "123",
            name: "Division without remain.",
            base: base10,
        },
        TestCaseAtomic {
            op: div_strings,
            a: "5536",
            b: "45",
            expected: "123",
            name: "Division with remain.",
            base: base10,
        },
        TestCaseAtomic {
            op: div_strings,
            a: "0",
            b: "123",
            expected: "0",
            name: "Division of zero.",
            base: base10,
        },
        TestCaseAtomic {
            op: mod_strings,
            a: "123",
            b: "0",
            expected: "Err",
            name: "Modulo by zero.",
            base: base10,
        },
        TestCaseAtomic {
            op: mod_strings,
            a: "45",
            b: "123",
            expected: "45",
            name: "Dividend < Divisor.",
            base: base10,
        },
        TestCaseAtomic {
            op: mod_strings,
            a: "123",
            b: "123",
            expected: "0",
            name: "Self modulo.",
            base: base10,
        },
        TestCaseAtomic {
            op: mod_strings,
            a: "123",
            b: "1",
            expected: "0",
            name: "Modulo par un.",
            base: base10,
        },
        TestCaseAtomic {
            op: mod_strings,
            a: "5535",
            b: "45",
            expected: "0",
            name: "Void modulo.",
            base: base10,
        },
        TestCaseAtomic {
            op: mod_strings,
            a: "5536",
            b: "45",
            expected: "1",
            name: "Modulo return 1.",
            base: base10,
        },
        TestCaseAtomic {
            op: mod_strings,
            a: "100",
            b: "33",
            expected: "1",
            name: "Classic modulo.",
            base: base10,
        },
        TestCaseAtomic {
            name: "ADD: bignum base 16",
            op: add_strings,
            base: base16,
            a: "FFFFFFFFFFFFFFFFFFFFFF",
            b: "1",
            expected: "10000000000000000000000",
        },
        TestCaseAtomic {
            name: "ADD: size differs base 16",
            op: add_strings,
            base: base16,
            a: "123456789ABC",
            b: "DEF",
            expected: "12345678A8AB",
        },
        TestCaseAtomic {
            name: "SUB: big-num",
            op: sub_strings,
            base: base2,
            a: "100000000000000000000000",
            b: "1",
            expected: "11111111111111111111111",
        },
    ]
}

fn get_test_cases_expr<'a>() -> Vec<TestCaseExpr<'a>> {
    
    let base42 = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef";
    let base10 = "0123456789";

    vec![
        TestCaseExpr {
            expression: "1+1",
            expected: "2",
            name: "Let's start by the beggining.",
            base: base10,
        },
        TestCaseExpr {
            expression: "(5S4 + Bc) / (4W - 3O) * A",
            expected: "180",
            name: "Let's have fun.",
            base: base42,
        },
    ]
}

pub fn run_tests() {
    let test_cases = get_test_cases();
    let test_cases_expr = get_test_cases_expr();
    let mut failures = 0;

    for test in test_cases {
        let result= (test.op)(test.a, test.b, test.base);

        let outcome: String;
        let is_match: bool;

        match result {
            Ok(res_str) => {
                outcome = res_str;
                is_match = outcome == test.expected;
            }
            Err(_e) => {
                outcome = "Err".to_string();
                is_match = outcome == test.expected;
            }
        }

        if is_match {
            println!("✅ PASS: {}", test.name);
        } else {
            println!(
                "❌ FAIL: {} | Expected: '{}', Got: '{}'",
                test.name, test.expected, outcome
            );
            failures += 1;
        }
    }

    for test in test_cases_expr {
	let result = eval(test.expression, test.base, "+-*/%");
        let outcome: String;
        let is_match: bool;

        match result {
            Ok(res_str) => {
                outcome = res_str;
                is_match = outcome == test.expected;
            }
            Err(_e) => {
                outcome = "Err".to_string();
                is_match = outcome == test.expected;
            }
        }

        if is_match {
            println!("✅ PASS: {}", test.name);
        } else {
            println!(
                "❌ FAIL: {} | Expected: '{}', Got: '{}'",
                test.name, test.expected, outcome
            );
            failures += 1;
        }
    }
    
    println!("\n--- Results ---");
    if failures == 0 {
        println!("All test passed !");
    } else {
        println!("{} test(s) fails.", failures);
    }
}
