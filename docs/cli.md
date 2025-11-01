# CLI

## Table of contents
- [Introduction](#introduction)
- [Commands](#commands)
  - [help](#mluva-help)
  - [init](#mluva-init)
  - [uninit](#mluva-uninit)
  - [build](#mluva-build)
  - [run](#mluva-run)
- [Project config](#project-config)

## Introduction
The Mluva CLI is the command-line interface for the Mluva programming language. It provides various commands to compile, run, and manage Mluva projects.

## Commands

### mluva help
Displays help information about Mluva CLI and its commands.

### mluva init
Initializes a new Mluva project in the current directory. It creates the necessary project structure and files.
The `.mluva/` directory is managed by the CLI and should not be modified manually if not necessary. It contains compiled bytecode modules and other metadata required for the project.

### mluva uninit
Removes Mluva project files from the current directory. It will not delete any source code files. To use this command, user must confirm the action.

### mluva build
Compiles the Mluva project in the current directory. It generates bytecode files in the `.mluva/modules/` directory. Every module is compiled separately, so only modified modules are recompiled on subsequent builds.

### mluva run
Runs the Mluva project in the current directory. It first builds the project (if necessary) and then executes the main module. The main module is expected to have a `main` function with no parameters. The Return value of the `main` function is printed.

## Project config
Mluva projects can be configured using the `mluva.yaml` file located in the project root directory. This file allows you to specify various settings such as the main module name, compiler options, and dependencies.
Here are the available configuration options:
- `root_module`: Specifies the name of the main module to run, optional, default is `main`.
- `project_name`: Specifies the name of the project.