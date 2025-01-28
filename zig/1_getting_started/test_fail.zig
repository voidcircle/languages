const std = @import("std");
const expect = std.testing.expect;

test "always failed?" {
    try expect(false);
}
