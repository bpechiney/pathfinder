#!/usr/bin/env just --justfile

alias t := test

[group: 'dev']
watch +args='test':
  cargo watch --clear --exec '{{ args }}'

 [group: 'misc']
 run:
   cargo run

 # only run tests matching `PATTERN`
[group: 'test']
filter PATTERN:
  cargo test {{PATTERN}}

[group: 'misc']
build:
  cargo build

[group: 'misc']
fmt:
  cargo fmt --all

[group: 'check']
outdated:
  cargo outdated -R