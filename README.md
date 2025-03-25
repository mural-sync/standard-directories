# standard-directories

Retrieve the standard directories for common purposes like configuration files
and data files.

This crate was primarily developed for `mural-sync`, but it is completely
standalone and can be used in any Rust project. However, the focus of the crate
will always be on supporting `mural-sync`'s needs. Since `mural-sync` currently
only supports Linux, `standard-directories` also supports Linux exclusively.
Furthermore. `mural-sync` only requires a config directory and a data directory, so
other standard directories like `xdg`'s `XDG_STATE_HOME`, `XDG_CACHE_HOME` and
`XDG_RUNTIME_DIR` are not supported.
