//! Bash builtin to implement a counter.

use bash_builtins::variables::{self};
use bash_builtins::{builtin_metadata, Args, Builtin, BuiltinOptions, Result};
use std::io::{self, BufWriter};
use mexprp::{Expression};

builtin_metadata!(
    name = "expr",
    create = Expr::default,
    short_doc = "expr 'string'",
    long_doc = "
        Arithmetic expression

        Options:
          -v\tVariable to assign (Default: JSON)
    ",
);

#[derive(BuiltinOptions)]
enum Opt {
    #[opt = 'v']
    Var(String),

}

#[derive(Default)]
struct Expr;

impl Builtin for Expr {
    fn call(&mut self, args: &mut Args) -> Result<()> {
        // No options: print the current value and increment it.
        let mut var = "EXPR".to_string();
        let stdout_handle = io::stdout();
        let _output = BufWriter::new(stdout_handle.lock());
        // Parse options. They can change the value of the counter, but the
        // updated value is stored only if we don't get any error.
        for opt in args.options() {
            match opt? {
                Opt::Var(v) => { 
                    var = v;
                }
            }
        }
        
        let expr = args.string_arguments().next().unwrap();
        let expr: Expression<f64> = Expression::parse(expr?).unwrap();
        let res = expr.eval(); // Ok(Answer::Single(9.0))
        let _ = variables::set(&var, String::from(res.unwrap().to_string()));
        Ok(())
    }
}

