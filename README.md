# Bash loadables builtins in rust

Json Encode/Decode -> bash-json-plugin
Expr -> arithmethic expression 

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

#Output of exported
declare -A exported=([nik.candy]="handy" [tik.muu]="haa" [nik.blik]="nek" [blob]="triiiiiiiii" [michael]="jordan" [mip]="fi" [nik.lorem]="ipsum" [testing]="hehe" [tik.tut.blea]="nek" )
```
Help Example:
```
    Options:
      -e        Encode.
      -d        Decode.
      -v        Variable to assign (Default: JSON)
      -D        Delimeter (Default: :)
```

## Expr
compile:
```
cd expr; cargo build
```
Move the target file where ever you want and then just enable it inside your script:
```
enable -f target/debug/libexpr_builtin.so expr

expr -v result "2/3"
echo $result
```
Help Example:
```
    Options:
      -v        Variable to assign (Default: RESULT)
```
