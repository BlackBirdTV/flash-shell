<p align="center">
  <img src="logo.png" />
</p>

# FLASH
Flash is a modern cross-platform shell built for simplicity and ease-of-use. It will definitely run on Windows, Linux and Redox/chronoOS and will most likely have a release for macOS, though untested.

## Changelog
- Added Variables (declare with the `var {name} {value}`, get value by prefixing it with a `$` (e.g. `echo $my_variable`))

## Roadmap
- Math

## Canceled / Moved
- thunder (TUI Editor): Thunder is going to be moved into it's own project. It is too big for including it in flash. However, I will finish Flash 1.0 and maybe another secret project before thunder, it's hard to make somethung better than neovim ;)

## Lua Integration
In order to write commands for flash, you'll have to use Lua. However, not plain lua, you have a few things at your disposal:

- global `command`: This variable is a Table that is similar to the struct flash uses internally. It has a few members:
    - `action`: The command being executed
    - `args`: A table containin all the args given
    - `flags`: A table containin all the flags given
    - `full`: The full command
    - `followedAction`: A Table that describes the action that should happen after the command execution. It consists of a string and a variable that is either nil, a string or a command.
    The value of the second variable is dependent on the first:
        - "PipeFile" : string (Write the stdout to a file)
        - "PipeCommand" : command (Pipe the stdout to another command)
        - "FollowCommand" : command (Run a command after running the current) 
        - "ParallelCommand" : command (Run the current command in parallel with another)
        - "None" : nil
- global `runCommand`: A function that runs a command from a command table.
    