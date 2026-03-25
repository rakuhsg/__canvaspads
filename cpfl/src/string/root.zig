const std = @import("std");

const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;

/// Single Unicode character
pub const Char = struct {
    const Self = @This();

    pub fn size() usize {}
};

pub const StringDecoder = struct {
    pub fn next()
};

/// Unicode string
pub const String = struct {
    const Self = @This();

    list: ArrayList(u8),

    pub const empty: Self = .{
        .list = .empty,
    };

    pub fn initCapacity(gpa: Allocator, num: usize) Allocator.Error!Self {
        const list: ArrayList(u8) = try ArrayList.initCapacity(gpa, num);
        return .{
            .list = list,
        };
    }

    pub fn clone(self: Self, gpa: Allocator) Allocator.Error!Self {
        const list = try self.list.clone(gpa);
        return .{
            .list = list,
        };
    }

    pub fn decoder(self: *Self) StringDecoder { 
    }
};

test {
    var a: String = .empty;
    _ = a;
}
