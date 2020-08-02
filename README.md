# yogawm - the flexible window manager library

Yoga is a felxible library for creating x window managers. You can think of a XMonad like wm in rust.
The goal is to take as much complexity as possible away while offering a full-featured api for programming
your "own" wm.

In the end you will have your own project/crate/code which just uses the interfaces and ideas `yoga` offers you.

While XMonad has a (atleast) some kind of imperative API, `yoga` tries to be as declarative as possible. You
pull in only the features you need. You handle events by yourself. And you can make it everything you want.

## Quickstart

Not possible right now. This lib is pre-alpha. Currently the primary development goal is to fully implement and
test `libx` to have a good starting point for designing the internals of `yoga`.

## Development Roadmap

This is a very very rough overview of the next steps for this library. Things might change, get removed or new stuff
gets added along the way.

### libx

- [x] Establish a connection to X11
- [x] Retrieve data about screens
- [x] Retrieve data about windows
- [ ] Implement a high level interface for working with windows (kill, spawn, move..)
- [ ] Provide a solid event system
- [ ] Provide io interfaces (keyboard/mice)
- [ ] Provide highlevel drawing interfaces

### yoga

- [ ] Provide abstractions over `libx`
- [ ] Focus on window manager tasks (event handling, workspaces, layouts)
- [ ] Provide traits to implement own Layouts
- [ ] Handle internal event loop and interaction with libx (only pass events to client)
- [ ] Have an easy to use API

## Contributing

I'd love to get PR's or accept people as contributors. I'll write a rough technical overview in the next time to make
it easier to get into writing code for this project.
