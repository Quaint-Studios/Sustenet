const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const main_module = b.addModule("main", .{
        .root_source_file = b.path("src/main.zig"),
    });

    {
        const options = b.addOptions();
        options.addOption(bool, "main_blocking", false);
        main_module.addOptions("build", options);
    }

    // Sustenet
    const exe = b.addExecutable(.{
        .name = "Sustenet",
        .root_source_file = b.path("src/main.zig"),
        .target = target,
        .optimize = optimize,
    });

    // Tests
    const tests = b.addTest(.{
        .name = "test",
        .target = target,
        .optimize = optimize,
        .root_source_file = b.path("src/main.zig"),
        .test_runner = b.path("test_runner.zig"),
    });

    // build.zig.zon modules
    const DEPS: [1][]const u8 = .{
        "network",
        // "zig_numerics",
    };

    for (DEPS) |name| {
        const dep = b.dependency(name, .{ .target = target, .optimize = optimize });
        exe.root_module.addImport(name, dep.module(name));
        tests.root_module.addImport(name, dep.module(name));
    }

    // exe
    b.installArtifact(exe);
    const run_cmd = b.addRunArtifact(exe);
    run_cmd.step.dependOn(b.getInstallStep());
    if (b.args) |args| {
        run_cmd.addArgs(args);
    }
    const run_step = b.step("run", "Run the app");
    run_step.dependOn(&run_cmd.step);

    // tests
    const run_tests = b.addRunArtifact(tests);
    run_tests.has_side_effects = true;
    const test_step = b.step("test", "Run tests");
    test_step.dependOn(&run_tests.step);
}
