//! Extensions for various things you might find useful from other languages.

const std = @import("std");
const assert = std.debug.assert;

pub fn closure(bindings: anytype) ClosureInternal(@TypeOf(bindings)) {
    return ClosureInternal(@TypeOf(bindings)){ .ctx = bindings };
}

fn ClosureInternal(comptime Spec: type) type {
    comptime {
        const spec_tinfo = @typeInfo(Spec);
        assert(spec_tinfo == .Struct);

        for (spec_tinfo.Struct.fields) |field| {
            assert(field.default_value == null);
        }

        assert(spec_tinfo.Struct.decls.len == 1);
        const call_decl = spec_tinfo.Struct.decls[0];
        assert(call_decl.is_pub);
        assert(std.mem.eql(u8, call_decl.name, "call"));

        const call = Spec.call;
        const call_tinfo = @typeInfo(@TypeOf(call));
        assert(call_tinfo == .Fn);
        assert(!call_tinfo.Fn.is_generic);
        assert(call_tinfo.Fn.params.len >= 1);
        assert(call_tinfo.Fn.params[0].type.? == *const Spec);

        var arg_types: [call_tinfo.Fn.params.len - 1]type = undefined;
        for (call_tinfo.Fn.params[1..], 0..) |arg, i| {
            arg_types[i] = arg.type.?;
        }

        const RetType = call_tinfo.Fn.return_type.?;

        return Closure(Spec, arg_types[0..], RetType);
    }
}

pub fn Closure(comptime Ctx: type, comptime arg_types: []type, comptime RetType: type) type {
    return struct {
        ctx: Ctx,

        pub fn call(self: *const @This(), args: anytype) RetType {
            comptime {
                assert(args.len == arg_types.len);
                for (args, 0..) |_, i| {
                    assert(@TypeOf(args[i]) == arg_types[i]);
                }
            }
            return @call(.auto, Ctx.call, .{&self.ctx} ++ args);
        }
    };
}

test "closures" {
    var x: i32 = 60;

    const foo = closure(struct {
        x: *i32,
        pub fn call(self: *const @This(), y: i32) i32 {
            self.x.* += y;
            return 420;
        }
    }{ .x = &x });

    assert(foo.call(.{@as(i32, 9)}) == 420);
    assert(x == 69);
}
