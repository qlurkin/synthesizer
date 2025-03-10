const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const mode = b.standardOptimizeOption(.{});

    const exe = b.addExecutable("hello", "hello.c");
    exe.setTarget(target);
    exe.setBuildMode(mode);
    exe.install();
}
