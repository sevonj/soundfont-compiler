# SoundFont Compiler

## What?

SoundFont Compiler is a command-line tool for creating soundfonts. Unlike most other tools, it isn't designed around a workflow of editing soundfonts. Instead, the binary soundfont file is generated from "source code" comprising text and audio files.

Presets, instruments, etc. are defined in TOML-files.

## Why?

Breaking the soundfont into separate files and defining parameters in a text-based format makes collaboration and version control much easier.

## Current state

The project in its current state is a barely working proof of concept. The output from compiling the example project is _good enough_ to be parsed by [RustySynth](https://github.com/sinshu/rustysynth/).
