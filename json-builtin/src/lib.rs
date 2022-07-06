//! Bash builtin to implement a counter.

use bash_builtins::variables::{self};
use bash_builtins::{builtin_metadata, Args, Builtin, BuiltinOptions, Result};
use serde_json::{Value, Map};
use std::io::{self, BufWriter, Write};

builtin_metadata!(
    name = "json",
    create = Json::default,
    short_doc = "json [-d] [-e]",
    long_doc = "
        Encode and Decode json.

        Options:
          -e\tEncode.
          -d\tDecode.
    ",
);

#[derive(BuiltinOptions)]
enum Opt {
    #[opt = 'd']
    Decode(String),

    #[opt = 'e']
    Encode,

    #[opt = 'v']
    Var(String),
}

#[derive(Default)]
struct Json;

impl Builtin for Json {
    fn call(&mut self, args: &mut Args) -> Result<()> {
        // No options: print the current value and increment it.
        let mut var = "JSON".to_string();
        let mut to_decode = None;
        let mut action = "";
        let stdout_handle = io::stdout();
        let mut output = BufWriter::new(stdout_handle.lock());
        // Parse options. They can change the value of the counter, but the
        // updated value is stored only if we don't get any error.
        for opt in args.options() {
            match opt? {
                Opt::Var(v) => { 
                    var = v;
                }
                Opt::Decode(v) => {
                    to_decode = variables::find_as_string(&v);
                    action = "decode";

                }
                Opt::Encode => {
                    writeln!(&mut output, "Encode not implemented yet!")?;
                }
            }
        }

        match action {
            "decode" => {
                if let Some(value) = to_decode.as_ref().and_then(|v| v.to_str().ok()) {
                    let json: Value = serde_json::from_str(value).unwrap();
                    parse_json(&json, "", &var)?;   
                }
            }
            _ => {}
        };

        Ok(())
        
    }
}

fn parse_json(json: &Value, tl: &str, var: &str) -> Result<()>{
    let delimeter = ":";
    'jloop: for (k, t) in json.as_object().unwrap() {
        match t.as_str() {
            Some(value) => {    
                variables::assoc_set(&var, format!("{}{}", tl.clone(), k).as_str(), value)?;
                continue 'jloop;
            }
            None => {}
        };
        if tl.is_empty() {
            parse_json(&json[k], format!("{}{}", k, delimeter).as_str(), &var)?;
        } else {
            parse_json(&json[k], format!("{}{}{}", tl, k, delimeter).as_str(), &var)?;
        }
    }

    Ok(())
}
