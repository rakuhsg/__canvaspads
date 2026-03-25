/// Apple Core Foundation Library
const cf = @cImport({
    @cInclude("CoreFoundation/CoreFoundation.h");
});

pub const RunLoopImpl = struct {
    loop: cf.CFRunLoopRef,
};

test "CoreFoundation" {
    _ = cf;
}
