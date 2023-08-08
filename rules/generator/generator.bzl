def _impl(ctx):
    name = ctx.attr.library_name
    generator_script = ctx.actions.declare_file("%s_gen.sh" % name)
    output = ctx.actions.declare_directory("%s_gen" % name)

    ctx.actions.run(
        outputs = [generator_script, output],
        arguments = [name, output.path, generator_script.path],
        progress_message = "Generating the %s library" % name,
        executable = ctx.executable.gen_tool,
    )
    return [DefaultInfo(executable = generator_script)]

library_generator = rule(
    executable = True,
    implementation = _impl,
    attrs = {
        "library_name": attr.string(mandatory = True),
        "gen_tool": attr.label(
            executable = True,
            allow_files = True,
            cfg = "exec",
        )
    }
)
