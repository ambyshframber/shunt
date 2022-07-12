#!/bin/sh

cd SHUNT_INSTALL_DIR
cat - | ./shunt_lex | target/release/shunt
