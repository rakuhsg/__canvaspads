const std = @import("std");

pub fn setupRhi(b: *std.Build, target: std.Build.ResolvedTarget, optimize: std.builtin.OptimizeMode) *std.Build.Step.Compile {
    const mod = b.addModule("cpflrhi", .{
        .root_source_file = b.path("src/rhi/root.zig"),
        .target = target,
        .optimize = optimize,
    });

    const tests = b.addTest(.{
        .root_module = mod,
    });
    const test_step = b.step("rhi-test", "Run RHI module tests");
    const run_tests = b.addRunArtifact(tests);
    test_step.dependOn(&run_tests.step);

    const lib = b.addLibrary(.{
        .name = "cpflrhi",
        .linkage = .static,
        .root_module = mod,
    });

    return lib;
}

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const lib_rhi = setupRhi(b, target, optimize);
    b.installArtifact(lib_rhi);
    //const exe = b.addExecutable(.{
    //    .name = "demo",
    //    .root_module = b.createModule(.{
    //        .root_source_file = b.path("src/demo.zig"),
    //        .target = target,
    //        .optimize = optimize,
    //    }),
    //});
    //exe.root_module.linkLibrary(lib_rhi);
    //const run_step = b.step("run", "Run the app");
    //const run_cmd = b.addRunArtifact(exe);
    //run_step.dependOn(&run_cmd.step);
    //run_cmd.step.dependOn(b.getInstallStep());
    //if (b.args) |args| {
    //    run_cmd.addArgs(args);
    //}
}
