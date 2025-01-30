const std = @import("std");
const expect = std.testing.expect;
const print = std.debug.print;

test "assignment" {
    const immutable_variable: i32 = -100;
    var mutable_variable: u32 = 40;

    std.debug.print("{}\n", .{immutable_variable});
    std.debug.print("{}\n", .{mutable_variable});

    mutable_variable += 20;

    std.debug.print("{}\n", .{mutable_variable});

    const inferred_constant: i16 = @as(i16, immutable_variable);
    var inferred_variable = @as(u16, 300);

    std.debug.print("{}\n", .{inferred_constant});
    std.debug.print("{}\n", .{inferred_variable});

    inferred_variable -= 20;

    std.debug.print("{}\n", .{inferred_variable});

    // DO NOT USE UNDEFINED...
    const undefined_immutable_variable: i32 = undefined; // this does not make any sense because when a variable is defined with an undefined value, they will have to be re-assigned to get the value before they are used. But in the case of `const`, they cannot be mutated, so...
    var undefined_mutable_variable: u32 = undefined;

    std.debug.print("{}\n", .{undefined_immutable_variable});
    std.debug.print("{}\n", .{undefined_mutable_variable});

    undefined_mutable_variable = 2000;

    std.debug.print("{}\n", .{undefined_immutable_variable});
    std.debug.print("{}\n", .{undefined_mutable_variable});
}

test "array" {
    const a = [_]u8{ 'a', 'b' };
    const b = [5]u8{
        'h',
        'e',
        'l',
        'l',
        'o',
    };
    const c = [5]u8{ 'w', 'o', 'r', 'l', 'd' }; // string?

    print("{s}\n", .{a});
    print("{s}, {s}\n", .{ b, c });
    print("{}\n", .{b.len});
}

test "if expression" {
    const a = true;
    var x: u8 = 1;

    if (a) {
        x += 1;
    } else {
        x -= 1;
    }

    try expect(x == 2);
}

test "inline if expression" {
    const a = true;
    var x: u8 = 1;
    x += if (a) 1 else -1;
    try expect(x == 2);
}

test "while" {
    var i: u8 = 1;

    while (i < 100) {
        i *= 2;
    }

    var sum: u8 = 0;
    var a: u8 = 0;
    while (a <= 10) : (a += 1) {
        if (a == 2) continue;
        if (a == 10) break;

        sum += a;
    }

    try expect(i == 128);
    try expect(sum == 43);
}

test "for loops" {
    const strings = [_]u8{ 'a', 'b', 'c', 'e' };

    for (strings, 0..) |current_character, current_index| { // you can just put `_` for the variables that you will not use
        print("{} | {}\n", .{ current_character, current_index });
    }

    print("{s}\n", .{strings});
}

test "functions" {
    const variable = addFive(300);
    const fibonacci_num = fibonacci(10);

    // when you do not need the result that comes out of the function.
    // but zig foces you to get the value of the function.
    // you do not need to use the keywords like `const` or `var`
    _ = fibonacci(20); // ignored

    print("{} | {}\n", .{ variable, fibonacci_num });

    try expect(@TypeOf(variable) == u32);
    try expect(variable == 305);
    try expect(fibonacci_num == 55);
}

// camelCase
fn addFive(a: u32) u32 {
    return a + 5;
}

fn fibonacci(n: u16) u16 {
    if (n == 0 or n == 1) return n;
    return fibonacci(n - 1) + fibonacci(n - 2);
}

test "defer" {
    var x: i32 = 10;
    {
        defer x += 1; // the statement that is added after the defer keyword is always executed by the end of the scope always no matter what the scope returns.
        try expect(x == 10);
    }
    try expect(x == 11);
}

test "other defer" {
    var x: i32 = 10;
    {
        x += 1;
        try expect(x == 11);
    }
    try expect(x == 11);
}

test "multi defer" {
    var x: f32 = 5;

    {
        // if there are multiple defers in the isngle block, they will be executed in reverse order... why?
        defer x += 2;
        defer x /= 2;
    }

    try expect(x == 4.5);
}

const FileOpenError = error{ AccessDenied, OutOfMemory, FileNotFound };

const AllocationError = error{OutOfMemory};

test "coerce error from a subset to a superset" {
    const err: FileOpenError = AllocationError.OutOfMemory;
    try expect(err == FileOpenError.OutOfMemory);
}

test "error union" {
    const maybe_error: AllocationError!u16 = AllocationError.OutOfMemory; // FileOpenError.OutOfMemory also works but other enums than OutOfMemory does not work.
    const no_error: u16 = maybe_error catch 0; // catch binary operation returns the right hand value when the left hand value returns an error. In this case, since the maybe_error is hardcoded to be an error all the time, it will 100% return 0, which is the right hand value, but... catch operation is not meant to be used in the cases like this.

    print("{!} | {}\n", .{ maybe_error, no_error });
    try expect(@TypeOf(no_error) == u16);
    try expect(no_error == 0);
}

fn faillingFunction() error{Oops}!u32 {
    return error.Oops; // always returns the error...
    // did not know it can return an error like this.
}

test "returning an error" {
    print("HAHAHAH\n", .{});

    const something: u32 = faillingFunction() catch |err| { // payload capturing
        try expect(err == error.Oops); // try x is short for x catch |err| return err;
        return; // this return statement literally break everyhting and finishes the whole test. so any statements everything below it won't be executed.
    };

    print("{}\n", .{something});
    // print("AHHA\n", .{});
    // try expect(something == 10);
}

fn failingCreatFileFn() !void {
    return error.AccessDenied;
}

test "inferred error set" {
    const x: error{AccessDenied}!void = failingCreatFileFn();

    // Zig does not let us ignore ERRORs through _ = x;
    // we must  unwrap it with "try", "catch", or "if".... why..?
    _ = x catch {};
}

// errors can be merged
const A = error{ AErrorOne, AErrorTwo };
const B = error{ BErrorOne, BErrorTwo };
const C = A || B;

fn failingCFn() C!void {
    return C.AErrorOne; // why doesnt zls give me warnings or errors when I put error.Oops or something that is NOT supposed to be put here.
}

test "something testing failing c fn" {
    failingCFn() catch |err| {
        try expect(err == C.AErrorOne);
    };
}

test "switch" {
    var x: u8 = 2;
    switch (x) {
        1...4 => { // like case {} in Rust
            // ranges can be expressed like {}...{}
            // -1...1 =>;;; -1 cannot be a part of u8 number range, so the compile will give you an error.
            // x = -x; // x = -x cannot happen with the variable that has the type of u8.
            x -= 1;
            defer print("AHHHH!\n", .{});
        },
        10, 100 => { // or can be expressed like {}, {}, {}
            x = @divExact(x, 10); // what is.. @divExact?
        },
        else => {}, // like default in Rust; it must be provided.
    }

    try expect(x == 1);
}

test "former switch statement" {
    var x: u8 = 10;
    // this switch statement is better...
    x = switch (x) {
        1...4 => x - 1,
        10, 100 => @divExact(x, 10),
        else => x, // no warnings or erros from lsp when I do NOT put this...
    };

    try expect(x == 1);
}
