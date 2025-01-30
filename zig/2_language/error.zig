// 1. failingFnCounter() is executed.
// 2. failingFunction returns an error with error.Oops.
// 3. Since failingFnCounter returns an error, the statement that is added after the `errdefer` keyword will be executed; the `problems` variable get increased by one.(if the function did not return an error, the `errdefer` thinging would have not been executed.)
// 4. Since failingFnCounter returns an error, the right hand of the catch binary operation will be executed with the err payload capturing.
// 5. Since the err that the failingFnCounter returns is error.Oops, the first expect would return true.
// 6. Since the problems got increased by one when the failingFnCounter returns an error, it will be equal to 99, not 98.

const std = @import("std");
const expect = std.testing.expect;

fn failingFunction() error{Oops}!void {
    return error.Oops;
}

var problems: u32 = 98;
fn failingFnCounter() error{Oops}!void {
    errdefer problems += 1;
    try failingFunction();
}

test "errdefer" {
    failingFnCounter() catch |err| {
        try expect(err == error.Oops);
        try expect(problems == 99);
        return;
    };
}
