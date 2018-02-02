# les

A project configurable `ls`

sensible defaults ( ls -lah by default )
configurable with .les
- glob level config
  - colour
  - hidden
  - annotate
- consider TOML syntax

## Why?
`ls` is a great multi-purpose tool but does little to address varied  contexts 
and hierachies in different projects. Adding visual information to a file list 
gives us additional structure through which to view a project.

Allowing a user to configure `les` on a per-directory basis acknowledges the 
fact that emphasis of particular files is dependant on the environment in which
you are working.
