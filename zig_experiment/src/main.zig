const std = @import("std");

const tuile = @import("tuile");
const zaudio = @import("zaudio");

pub fn main() !void {
    var tui = try tuile.Tuile.init(.{});
    defer tui.deinit();

    try tui.add(
        tuile.block(
            .{
                .border = tuile.Border.all(),
                .border_type = .rounded,
                .layout = .{ .flex = 1 },
            },
            tuile.label(.{ .text = "Hello World!" }),
        ),
    );

    try tui.run();
}
