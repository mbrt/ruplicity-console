# ruplicity-console
[![Build Status](https://travis-ci.org/mbrt/ruplicity-console.svg?branch=master)](https://travis-ci.org/mbrt/ruplicity-console)
[![Build status](https://ci.appveyor.com/api/projects/status/nfyy0g8yi782cx52/branch/master?svg=true)](https://ci.appveyor.com/project/mbrt/ruplicity-console/branch/master)

Command line interface for [ruplicity](https://github.com/mbrt/ruplicity).

WARNING: This project is under heavy development.

## Installation

Starting from Rust 1.5:

```
cargo install ruplicity-console
```

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

You can specify which snapshot to list by using the `-i` option, to specify an index (from 0, to the number of snapshots):

```
ruplicity-console ls -i 0 /media/backup
```

shows the files for the first backup snapshot.
