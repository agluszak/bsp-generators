def _impl(ctx):
    name = ctx.attr.name
    generator_script = ctx.actions.declare_file("%s_gen.sh" % name)

    ctx.actions.run(
        outputs = [generator_script],
        arguments = [name, generator_script.path],
        progress_message = "Generating the %s library" % name,
        executable = ctx.executable.gen_tool,
    )
    return [DefaultInfo(executable = generator_script)]

library_generator = rule(
    executable = True,
    implementation = _impl,
    attrs = {
        "gen_tool": attr.label(
            executable = True,
            allow_files = True,
            cfg = "exec",
        )
    }
)