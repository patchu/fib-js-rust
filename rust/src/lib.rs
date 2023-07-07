#[macro_use]
extern crate neon;
use neon::prelude::*;

fn generate_fibonacci(n: u32) -> Vec<(usize, u64)> {
    let mut fib = vec![(0, 0), (1, 1)];

    for i in 2..n as usize {
        let next = fib[i-1].1 + fib[i-2].1;
        fib.push((i, next));
    }

    fib
}

fn add_commas(num: u64) -> String {
    let num_str = num.to_string();
    let mut result = String::new();
    let len = num_str.len();
    for (i, digit) in num_str.chars().enumerate() {
        if (len - i) % 3 == 0 && i != 0 {
            result.push(',');
        }
        result.push(digit);
    }
    result
}


fn fibonacci(mut cx: FunctionContext) -> JsResult<JsObject> {
    let count: u32 = cx.argument::<JsNumber>(0)?.value(&mut cx) as u32;
    let fib_numbers = generate_fibonacci(count);
    let js_obj = JsObject::new(&mut cx);

    for (index, number) in fib_numbers.iter() {
        let key = cx.string(format!("{}", index));
        let value = cx.string(add_commas(*number));

        js_obj.set(&mut cx, key, value)?;
    }

    Ok(js_obj)
}

register_module!(mut cx, {
    cx.export_function("fibonacci", fibonacci)
});
