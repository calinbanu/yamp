# yamp - Yet Another Map Parser

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
![CircleCI](https://img.shields.io/circleci/build/github/calinbanu/yamp/develop?label=develop)
![CircleCI](https://img.shields.io/circleci/build/github/calinbanu/yamp/develop?label=main)
[![codecov](https://codecov.io/gh/calinbanu/yamp/branch/main/graph/badge.svg?token=CKMTMTA5W1)](https://codecov.io/gh/calinbanu/yamp)
![GitHub issues](https://img.shields.io/github/issues/calinbanu/yamp)

## Description

yamp is a tool that parses .map files, generated during compilation, that contain memory information used by data and code.\
The tool is intended to be used by embedded developers to analyze the impact of various code changes.\
This is inspired by [amap](https://www.sikorskiy.net/info/prj/amap/) tool. I have used this tool along the years and felt the need for some extra features.

## Features

### Mapfile structure

The parser will try to split the mapfile into **Sections** when it matches the following strings:
 - *Archive member included to satisfy reference by file (symbol)*
 - *Allocating common symbols*
 - *Discarded input sections*
 - *Memory Configuration*
 - *Linker script and memory map*

Only the "*Linker script and memory map*" **Section** will be further parsed and split into **Segments** (bss/rodata/text/etc.) and then into **Entries**:

- Section "*Linker script and memory map*"
	- Segment 0
		- Entry 0
		- Entry 1
		- ...
		- Entry M
	- Segment 1
	- ...
	- Segment N

### XML Output

Using *--xlsfile[=\<PATH\>]* option, we can output the parsed information into XML format.\
If no *PATH* is provided, data will be saved into *mapfile.xml*. If *PATH* is *stdout*, output will be printed in console.\
The structure of the XML file will be the following:

```xml
<mapfile datetime="<data and time>" source="<file name>">
    <section name="MemoryMap">
        <segments count="<number>">
			<segment name="<string>" address="<hex address>" size="<number>">
                <entry name="<string>" address="<hex address>" size="<number>" fill_size="<number>" fill_overlaps="<true/false>" />
				...
            </segment>
			...
		</segments>
        <objects count="<number>">
			<object name="<string>">
                <segments count="<number>">
                    <segment name="<string>" size="<number>" />
					...
                </segments>
            </object>
			...
		</objects>
    </section>
</mapfile>
```

### XLSX Output

Using *--xlsfile[=\<PATH\>]* option, we can output the parsed information into an XLSX file.\
If no *PATH* is provided, data will be saved into *mapfile.xlsx*.\
The XLSX file will have the following worksheets:
- Segments : Contains segment name, start address and size
- Entries : Contains segment name into which is places, entry name, start address and size
- Objects : Contains object name, segment name where part of the object is placed and size

### Loglevel

After parsing all the **Entries** in a **Segment**, it will sum the sizes and compare the value with the **Segment** size. If the values are not equal it will report a *warning* in console. For some **Sections** this might can be ignored (ex: debug).\
If it fails to parse **Segment**/**Entry** information, it will print an *error* in console.\
Lines that are not getting parsed will be reported as *info* in console.

By default only errors are reported. Rest of them can be enabled using *--loglevel <LEVEL>* option.

## Install

Binary [releases](https://github.com/calinbanu/yamp/releases) comes as is and does not require installation, nor do they have special requirements.\
For now only x86_64-unknown-linux-gnu target is supported.

## Usage

```bash
Usage: parser [OPTIONS] --mapfile <PATH>

Options:
  -m, --mapfile <PATH>    Path to input Map file
      --xlsfile[=<PATH>]  Path to output XLSX file. If not specified, outputs to "mapfile.xlsx"
      --xmlfile[=<PATH>]  Path to output XML file. If not specified, outputs to "mapfile.xml"
  -l, --loglevel <LEVEL>  Set log level [default: error] [possible values: off, 0, error, 1, warn, 2, info, 3, debug, 4, trace, 5]
  -h, --help              Print help
  -V, --version           Print version
```

## Tested Compilers

|       SDK        |  GCC |
|------------------|------|
|Zephyr 0.14.1     |10.3.0|

## Future Development

- Support windows
- Support macos
- Improve parsing time using parallelization
- Add diff feature (to see size changes between 2 mapfiles)
- Add option to filter sections
- Parse libs information
- Support as many compilers as possible