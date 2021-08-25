
extern crate cpython;

use cpython::{py_fn, py_module_initializer, PyResult, Python};

py_module_initializer!(mylib, |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust.")?;
    m.add(py, "get_result", py_fn!(py, get_result(val: &str)))?;
    m.add(py, "do_something", py_fn!(py, do_something()))?;
    m.add(py, "count_doubles", py_fn!(py, count_doubles(val: &str)))?;
    m.add(py, "count_int", py_fn!(py, count_int(val:i32,val2:i32)))?;
    Ok(())
});

fn get_result(_py: Python, val: &str) -> PyResult<String> {
    Ok("Rust says: ".to_owned() + val)}
fn do_something(_py: Python) -> PyResult<String>{

    Ok("Return something".parse().unwrap())
    }

fn count_doubles(_py: Python, val: &str) -> PyResult<u64> {
    let mut total = 0u64;

    // There is an improved version later on this post
    for (c1, c2) in val.chars().zip(val.chars().skip(1)) {
        if c1 == c2 {
            total += 1;
        }
    }

    Ok(total)
}

fn count_int (_py:Python, val:i32, val2:i32) -> PyResult<i32> {
    let x = internal_count(val, val2);
    Ok(x)
}


fn internal_count (val:i32, val2:i32) -> i32{
    let r:i32 = val + val2;
    return r;

}
