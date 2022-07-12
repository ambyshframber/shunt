#!/bin/sh

dir=$( cd -- "$( dirname -- "$0" )" &> /dev/null && pwd )
cd $dir

lex < shunt.l
gcc -Wall lex.yy.c -o shunt_lex
cargo build --release

cat shunt_src.sh | sed "s@SHUNT_INSTALL_DIR@$dir@g" > shunt
chmod +x shunt
