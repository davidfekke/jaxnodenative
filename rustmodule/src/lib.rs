use neon::prelude::*;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}

fn add2numbers(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let a_num: Handle<JsNumber> = cx.argument(0)?;
    let b_num: Handle<JsNumber> = cx.argument(1)?;

    // Extract the numeric values from the Handle objects
    let a_value: f64 = a_num.value(&mut cx);
    let b_value: f64 = b_num.value(&mut cx);

    // Perform the addition
    let result = a_value + b_value;

    // Return the result as a JsNumber
    Ok(cx.number(result))
}

fn fibonacci(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let n = cx.argument::<JsNumber>(0)?.value(&mut cx) as u32;

    let result = calculate_fibonacci(n);
    Ok(cx.number(result as f64))
}

fn calculate_fibonacci(n: u32) -> u64 {
    if n <= 1 {
        return n as u64;
    }

    let mut a = 0;
    let mut b = 1;
    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    b
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("hello", hello)?;
    cx.export_function("add2numbers", add2numbers)?;
    cx.export_function("fibonacci", fibonacci)?;
    Ok(())
}
