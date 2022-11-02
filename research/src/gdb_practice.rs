/*
 * This file serves as pratice to use GDB to debug applications
 *
 * things you can pratice:
 * - break at initilisation of "value_in_main"
 * - break at new value assignment of "mut_value_in_main"
 * - break at function call of "somefunc"
 * - check all arguments passed into "somefunc"
 * - view the return value of "somefunc"
 * - break at line 9 and check all assigned values
 *
 * Feel free to add code to this example
 */

pub fn start_gdb_practice() {
    let value_in_main = 10;
    let mut mut_value_in_main = 5;

    mut_value_in_main = mut_value_in_main + 20;

    let somefunc_result = somefunc(value_in_main, mut_value_in_main as u64);

    println!("{}", somefunc_result);
}

fn somefunc(x: i32, y: u64) -> String {
    let result = x + y as i32;

    format!("{}", result)
}
