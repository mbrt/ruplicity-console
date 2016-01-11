# ruplicity-console
[![Build Status](https://travis-ci.org/mbrt/ruplicity-console.svg?branch=master)](https://travis-ci.org/mbrt/ruplicity-console)
[![Build status](https://ci.appveyor.com/api/projects/status/nfyy0g8yi782cx52/branch/master?svg=true)](https://ci.appveyor.com/project/mbrt/ruplicity-console/branch/master)

Command line interface for [ruplicity](https://github.com/mbrt/ruplicity).

## Installation

Starting from Rust 1.5:

```
cargo install ruplicity-console
```

Windows user can download pre-built binaries for convenience in the [releases section](https://github.com/mbrt/ruplicity-console/releases). For those, Visual C++ 2015 redistributable binaries are needed. If you don't have them, you can download them in [Visual Studio Downloads](https://www.visualstudio.com/downloads/download-visual-studio-vs), under the "Tools for Visual Studio 2015" section.

## Usage

Only backups present in the local file system are supported for now.

Suppose you have a duplicity backup stored in `/media/foo/backup/`. You can have informations about the backup by using:

```
ruplicity-console info /media/foo/backup
```

And list the files for the last snapshot by using:

```
ruplicity-console ls /media/foo/backup
```

You can specify which snapshot to list by using the `-i` option with the desired snapshot index:

```
ruplicity-console ls -i 0 /media/backup
```

shows the files for the first backup snapshot. If you want to know the index for some snapshot use `ruplicity-console info`.

## License

This crate is licensed through GPL-2.0. Why?
* The core functionality is already licensed under MIT, because it is exposed trough [ruplicity](https://github.com/mbrt/ruplicity) crate, so you can use it in whatever form you want (even closed source projects).
* This crate however provides a "product", and not a library, so I don't want anyone to fork it and close the sources (it could be possible with MIT license). Anyone is still free to use, contribute and modify it in whatever form they want. The only restriction is that they cannot change the license or integrate it in non-GPL projects;
* The GPL license is not compatible with the Rust license, but since there is no reason in integrate this crate into the standard distribution, there is no point in make it compatible.
