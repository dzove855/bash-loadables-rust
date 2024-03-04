# Bash loadables builtins in rust

Json Encode/Decode -> bash-json-plugin


NOTE: It's all still experimental to learn Rust

## Json Parser

NOTE: For the moment only decode is working.

compile:
```
cd json-builtin; cargo build
```
Move the target file where ever you want and then just enable it inside your script:
```
enable -f target/debug/libjson_builtin.so json 

test='{"testing": "hehe", "blob": "triiiiiiiii", "tik": {"muu": "haa", "tut": {"blea": "nek"}}, "mip": "fi", "nik": {"blik": "nek", "lorem": "ipsum", "candy": "handy"}, "michael": "jordan"}'

json -v exported -d test -D .
```
Help Example:
```
    Options:
      -e        Encode.
      -d        Decode.
      -v        Variable to assign (Default: JSON)
      -D        Delimeter (Default: :)
```
