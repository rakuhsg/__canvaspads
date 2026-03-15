const builtin = @import("builtin");

const backend = switch (builtin.rhi_backend) {
    .vulkan => @import("backend/vulkan.zig"),
    .d3d12 => @import("backend/d3d12.zig"),
    .metal => @import("backend/metal.zig"),
};
