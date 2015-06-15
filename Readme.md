#RustAllegro

[![Build Status](https://travis-ci.org/SiegeLord/RustAllegro.png)](https://travis-ci.org/SiegeLord/RustAllegro)

A very much WIP binding of [Allegro 5](http://liballeg.org/) to the [Rust](http://www.rust-lang.org/) programming language.

## Documentation

See [here](http://siegelord.github.io/RustAllegro/doc/allegro/index.html). Note that it is very incomplete.

## Building

### Via Cargo:

The included packages are:

Wrappers:

* [allegro](https://crates.io/crates/allegro)
* [allegro_acodec](https://crates.io/crates/allegro_acodec)
* [allegro_audio](https://crates.io/crates/allegro_audio)
* [allegro_dialog](https://crates.io/crates/allegro_dialog)
* [allegro_font](https://crates.io/crates/allegro_font)
* [allegro_image](https://crates.io/crates/allegro_image)
* [allegro_primitives](https://crates.io/crates/allegro_primitives)
* [allegro_ttf](https://crates.io/crates/allegro_ttf)

Bindings:

* [allegro-sys](https://crates.io/crates/allegro-sys)
* [allegro_acodec-sys](https://crates.io/crates/allegro_acodec-sys)
* [allegro_audio-sys](https://crates.io/crates/allegro_audio-sys)
* [allegro_dialog-sys](https://crates.io/crates/allegro_dialog-sys)
* [allegro_font-sys](https://crates.io/crates/allegro_font-sys)
* [allegro_image-sys](https://crates.io/crates/allegro_image-sys)
* [allegro_primitives-sys](https://crates.io/crates/allegro_primitives-sys)
* [allegro_ttf-sys](https://crates.io/crates/allegro_ttf-sys)

Examples:

* [allegro_examples](https://crates.io/crates/allegro_examples)

The `allegro-sys` package (and, transitively, the rest of the packages) can be
built to support different Allegro versions by specifying the
`ALLEGRO_INCLUDE_DIR` environment variable when invoking `cargo build`. This
directory should contain the `allegro5` directory with all of the headers
inside it. The build script for that crate will define the following two
metadata entries that the crates that depend on it can use to determine which
version is used:

* sub_version - The sub version of Allegro (e.g. for 5.1.10 the sub version is 1)

* wip_version - The wip version of Allegro (e.g. for 5.1.10 the wip version is 10).

Note that the `Core::init()` will attempt to verify that the binding
corresponds to the version of the library you're linking to, so it is essential
to specify `ALLEGRO_INCLUDE_DIR` more often than not.
