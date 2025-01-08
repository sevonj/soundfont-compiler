# SoundFont Compiler

## What?

SoundFont Compiler is a command-line tool for creating soundfonts. Unlike most other tools, it isn't designed around a workflow of editing soundfonts. Instead, the binary soundfont file is generated from "source code" comprising text and audio files.

## Why?

Breaking the soundfont into separate files and defining parameters in a text-based format makes collaboration and version control much easier.

## Current state

The project in its current state is a barely working proof of concept. The output from compiling the example project is _good enough_ to be parsed by [RustySynth](https://github.com/sinshu/rustysynth/).

## More details

You should be somewhat familiar with the SoundFont 2 format.

<details>
  <summary>tl;dr</summary>
  
  `preset -> instrument -> sample`
  
  - A soundfont contains one or more presets.
  - A preset contains one or more instruments.
  - An instrument contains one or more samples.

  The names can be confusing. The preset is the unit visible outside. In this context, when you choose a "sound," you're choosing a preset, not an instrument. Instruments are _internal_ to the soundfont. A preset may layer multiple instruments over each other to create a specific sound.

</details>

Example project structure:
```
.
├── instruments/
│   └── AcousticGuitar.toml
├── presets/
│   └── 025_AcousticGuitar.toml
├── samples/
│   ├── guitarpluck_l.wav
│   └── guitarpluck_r.wav
└── SoundFont.toml
```


Presets, instruments, and samples 
Samples are loose WAVE file. Presets, instruments and parameters are defined in TOML-files. See the example project.
