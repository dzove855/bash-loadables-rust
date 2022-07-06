#!/bin/bash

#declare -A miii
enable -f target/debug/libbash_json_plugin.so json

toto3='{"testing": "hehe", "blob": "triiiiiiiii", "tik": {"muu": "haa", "tut": {"blea": "nek"}}, "mip": "fi", "nik": {"blik": "nek", "lorem": "ipsum", "candy": "handy"}, "michael": "jordan"}'

json -e

json -v exported -d toto3
declare -p exported
