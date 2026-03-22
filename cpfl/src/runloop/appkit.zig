/// Apple Core Foundation Library
pub const cf = @cImport({
    @cInclude("CoreFoundation/CoreFoundation.h");
});

test "CoreFoundation" {
    _ = cf;
}
