//! # v_eval
//! Evaluate some expresions with context
//!
//! All are option by default
//!
//! ```rust
//! use v_eval::{Value, Eval};
//!
//!# fn main() -> Result<(), ()> {
//! let e = Eval::default()
//!     .insert("foo", "true")?
//!     .insert("string", "\"foo\"")?
//!     .insert("opt", "Some(true)")?
//!     .insert("bar", "false")?;
//!
//! assert_eq!(e.eval("foo != bar").unwrap(), Value::Bool(true));
//! assert_eq!(
//!     e.eval("true && foo != bar && true").unwrap(),
//!     Value::Bool(true)
//! );
//!
//! assert_eq!(e.eval("1.5.trunc()").unwrap(), Value::Int(1));
//! assert_eq!(e.eval("50.log10().trunc() == 1").unwrap(), Value::Bool(true));
//! assert_eq!(e.eval("Some(1.log10()).unwrap()").unwrap(), Value::Float(1.0f64.log10()));
//!# Ok(())
//!# }
//! ```
//! ## Methods
//! ### By default
//! #### Option
//! - `and`
//! ```rust
//!# use v_eval::{Value, Eval};
//!# fn main() -> Result<(), ()> {
//!# let e = Eval::default()
//!#     .insert("foo", "true")?
//!#     .insert("string", "\"foo\"")?
//!#     .insert("opt", "Some(true)")?
//!#     .insert("bar", "false")?;
//!#
//! assert_eq!(e.eval("foo.and(bar)").unwrap(), Value::Bool(false));
//! assert_eq!(e.eval("not_exist.and(bar)"), None);
//! assert_eq!(e.eval("1.and(2.0)").unwrap(), Value::Float(2.0));
//!# Ok(())
//!# }
//! ```
//! - `is_none`
//! ```rust
//!# use v_eval::{Value, Eval};
//!# fn main() -> Result<(), ()> {
//!# let e = Eval::default()
//!#     .insert("foo", "true")?
//!#     .insert("string", "\"foo\"")?
//!#     .insert("opt", "Some(true)")?
//!#     .insert("bar", "false")?;
//!#
//! assert_eq!(e.eval("foo.is_none()").unwrap(), Value::Bool(false));
//! assert_eq!(e.eval("not_exist.is_none()").unwrap(), Value::Bool(true));
//!# Ok(())
//!# }
//! ```
//! - `is_some`
//! ```rust
//!# use v_eval::{Value, Eval};
//!# fn main() -> Result<(), ()> {
//!# let e = Eval::default()
//!#     .insert("foo", "true")?
//!#     .insert("string", "\"foo\"")?
//!#     .insert("opt", "Some(true)")?
//!#     .insert("bar", "false")?;
//!#
//! assert_eq!(e.eval("foo.is_some()").unwrap(), Value::Bool(true));
//! assert_eq!(e.eval("not_exist.is_some()").unwrap(), Value::Bool(false));
//!# Ok(())
//!# }
//! ```
//! - `or`
//! ```rust
//!# use v_eval::{Value, Eval};
//!# fn main() -> Result<(), ()> {
//!# let e = Eval::default()
//!#     .insert("foo", "true")?
//!#     .insert("string", "\"foo\"")?
//!#     .insert("opt", "Some(true)")?
//!#     .insert("bar", "false")?;
//!#
//! assert_eq!(e.eval("foo.or(bar)").unwrap(), Value::Bool(true));
//! assert_eq!(e.eval("None.or(bar)").unwrap(), Value::Bool(false));
//! assert_eq!(e.eval("None.or(not_exist)"), None);
//!# Ok(())
//!# }
//! ```
//! - `unwrap_or`
//! ```rust
//!# use v_eval::{Value, Eval};
//!# fn main() -> Result<(), ()> {
//!# let e = Eval::default()
//!#     .insert("foo", "true")?
//!#     .insert("string", "\"foo\"")?
//!#     .insert("opt", "Some(true)")?
//!#     .insert("bar", "false")?;
//!#
//! assert_eq!(e.eval("not_exist.unwrap_or(bar)").unwrap(), Value::Bool(false));
//! assert_eq!(e.eval("foo.unwrap_or(bar)").unwrap(), Value::Bool(true));
//! assert_eq!(e.eval("None.unwrap_or(opt)").unwrap(), Value::Bool(true));
//!# Ok(())
//!# }
//! ```
//! - `unwrap`
//! ```rust
//!# use v_eval::{Value, Eval};
//!# fn main() -> Result<(), ()> {
//!# let e = Eval::default()
//!#     .insert("foo", "true")?
//!#     .insert("string", "\"foo\"")?
//!#     .insert("opt", "Some(true)")?
//!#     .insert("bar", "false")?;
//!#
//! assert_eq!(e.eval("foo.unwrap()").unwrap(), Value::Bool(true));
//! assert_eq!(e.eval("not_exist.unwrap()"), None);
//! assert_eq!(e.eval("None.unwrap()"), None);
//!# Ok(())
//!# }
//! ```
//! - `xor`
//! ```rust
//!# use v_eval::{Value, Eval};
//!# fn main() -> Result<(), ()> {
//!# let e = Eval::default()
//!#     .insert("foo", "true")?
//!#     .insert("string", "\"foo\"")?
//!#     .insert("opt", "Some(true)")?
//!#     .insert("bar", "false")?;
//!#
//! assert_eq!(e.eval("not_exist.xor(opt)").unwrap(), Value::Bool(true));
//! assert_eq!(e.eval("not_exist.xor(foo)").unwrap(), Value::Bool(true));
//! assert_eq!(e.eval("bar.xor(None)").unwrap(), Value::Bool(false));
//! assert_eq!(e.eval("bar.xor(foo)"), None);
//!# Ok(())
//!# }
//! ```
//!
//! #### Dynamic type
//! - `is_bool`
//! ```rust
//!# use v_eval::{Value, Eval};
//!# fn main() -> Result<(), ()> {
//!# let e = Eval::default()
//!#     .insert("foo", "true")?
//!#     .insert("string", "\"foo\"")?
//!#     .insert("opt", "Some(true)")?
//!#     .insert("bar", "false")?;
//!#
//! assert_eq!(e.eval("foo.is_bool()").unwrap(), Value::Bool(true));
//! assert_eq!(e.eval("string.is_bool()").unwrap(), Value::Bool(false));
//!# Ok(())
//!# }
//! ```
//! - `is_float`
//! ```rust
//!# use v_eval::{Value, Eval};
//!# fn main() -> Result<(), ()> {
//!# let e = Eval::default()
//!#     .insert("foo", "true")?
//!#     .insert("string", "\"foo\"")?
//!#     .insert("opt", "Some(true)")?
//!#     .insert("bar", "false")?;
//!#
//! assert_eq!(e.eval("1.0.is_float()").unwrap(), Value::Bool(true));
//! assert_eq!(e.eval("bar.is_float()").unwrap(), Value::Bool(false));
//!# Ok(())
//!# }
//! ```
//! - `is_int`
//! ```rust
//!# use v_eval::{Value, Eval};
//!# fn main() -> Result<(), ()> {
//!# let e = Eval::default()
//!#     .insert("foo", "true")?
//!#     .insert("string", "\"foo\"")?
//!#     .insert("opt", "Some(true)")?
//!#     .insert("bar", "false")?;
//!#
//! assert_eq!(e.eval("1.is_int()").unwrap(), Value::Bool(true));
//! assert_eq!(e.eval("foo.is_int()").unwrap(), Value::Bool(false));
//!# Ok(())
//!# }
//! ```
//! - `is_option`
//! ```rust
//!# use v_eval::{Value, Eval};
//!# fn main() -> Result<(), ()> {
//!# let e = Eval::default()
//!#     .insert("foo", "true")?
//!#     .insert("string", "\"foo\"")?
//!#     .insert("opt", "Some(true)")?
//!#     .insert("bar", "false")?;
//!#
//! assert_eq!(e.eval("None.is_option()").unwrap(), Value::Bool(true));
//! assert_eq!(e.eval("not_exist.is_option()").unwrap(), Value::Bool(true));
//! assert_eq!(e.eval("bar.is_option()").unwrap(), Value::Bool(false));
//! assert_eq!(e.eval("opt.is_option()").unwrap(), Value::Bool(true));
//!# Ok(())
//!# }
//! ```
//! - `is_range`
//! ```rust
//!# use v_eval::{Value, Eval};
//!# fn main() -> Result<(), ()> {
//!# let e = Eval::default()
//!#     .insert("foo", "true")?
//!#     .insert("string", "\"foo\"")?
//!#     .insert("opt", "Some(true)")?
//!#     .insert("bar", "false")?;
//!#
//! assert_eq!(e.eval("(0..10).is_range()").unwrap(), Value::Bool(true));
//! assert_eq!(e.eval("bar.is_range()").unwrap(), Value::Bool(false));
//!# Ok(())
//!# }
//! ```
//! - `is_str`
//! ```rust
//!# use v_eval::{Value, Eval};
//!# fn main() -> Result<(), ()> {
//!# let e = Eval::default()
//!#     .insert("foo", "true")?
//!#     .insert("string", "\"foo\"")?
//!#     .insert("opt", "Some(true)")?
//!#     .insert("bar", "false")?;
//!#
//! assert_eq!(e.eval("foo.is_str()").unwrap(), Value::Bool(false));
//! assert_eq!(e.eval("string.is_str()").unwrap(), Value::Bool(true));
//!# Ok(())
//!# }
//! ```
//! - `is_vec`
//! ```rust
//!# use v_eval::{Value, Eval};
//!# fn main() -> Result<(), ()> {
//!# let e = Eval::default()
//!#     .insert("foo", "true")?
//!#     .insert("string", "\"foo\"")?
//!#     .insert("opt", "Some(true)")?
//!#     .insert("bar", "false")?;
//!#
//! assert_eq!(e.eval("[1, 3, 4.0, true, foo].is_vec()").unwrap(), Value::Bool(true));
//! assert_eq!(e.eval("foo.is_vec()").unwrap(), Value::Bool(false));
//!# Ok(())
//!# }
//! ```
//! - `is_same`
//! ```rust
//!# use v_eval::{Value, Eval};
//!# fn main() -> Result<(), ()> {
//!# let e = Eval::default()
//!#     .insert("foo", "true")?
//!#     .insert("string", "\"foo\"")?
//!#     .insert("opt", "Some(true)")?
//!#     .insert("bar", "false")?;
//!#
//! assert_eq!(e.eval("foo.is_same(bar)").unwrap(), Value::Bool(true));
//! assert_eq!(e.eval("foo.is_same(false)").unwrap(), Value::Bool(true));
//! assert_eq!(e.eval("foo.is_same(1)").unwrap(), Value::Bool(false));
//!# Ok(())
//!# }
//! ```
//!
//!### Number (i64 and f64)
//! > See [f64 Rust](https://doc.rust-lang.org/std/primitive.f64.html)
//! - `abs`
//! - `acos`
//! - `acosh`
//! - `asin`
//! - `asinh`
//! - `atan2`
//! - `atan`
//! - `atanh`
//! - `cbrt`
//! - `ceil`
//! - `cos`
//! - `cosh`
//! - `exp2`
//! - `exp_m1`
//! - `exp`
//! - `floor`
//! - `fract`
//! - `hypot`
//! - `ln_1p`
//! - `ln`
//! - `log10`
//! - `log2`
//! - `log`
//! - `max`
//! - `min`
//! - `powf`
//! - `powi`
//! - `recip`
//! - `round`
//! - `signum`
//! - `sin`
//! - `sinh`
//! - `sqrt`
//! - `tan`
//! - `tanh`
//! - `to_degrees`
//! - `to_radians`
//! - `trunc`
//! ```rust
//!# use v_eval::{Value, Eval};
//!# fn main() -> Result<(), ()> {
//!# let e = Eval::default()
//!#     .insert("foo", "true")?
//!#     .insert("string", "\"foo\"")?
//!#     .insert("opt", "Some(true)")?
//!#     .insert("bar", "false")?;
//!#
//! assert_eq!(e.eval("1.5.trunc()").unwrap(), Value::Int(1));
//!# Ok(())
//!# }
//! ```
//!
//!

extern crate quote_impersonated as quote;
extern crate syn_impersonated as syn;

use std::collections::BTreeMap;

use syn::parse_str;

mod method;
mod operator;
mod reflect;
mod value;

pub use self::{reflect::eval, value::Value};

/// Evaluator with context
pub struct Eval(BTreeMap<String, syn::Expr>);

impl Default for Eval {
    fn default() -> Self {
        Self(BTreeMap::new())
    }
}

impl Eval {
    pub fn new(c: BTreeMap<String, syn::Expr>) -> Self {
        Self(c)
    }

    /// Parse and insert in context name - syn::Expr
    pub fn insert(mut self, k: &str, v: &str) -> Result<Self, ()> {
        let e = parse_str::<syn::Expr>(v).map_err(|_| ())?;
        self.0.insert(k.to_owned(), e);

        Ok(self)
    }

    /// Remove key in context
    pub fn remove(mut self, k: &str) -> Self {
        self.0.remove(k);

        self
    }

    /// Evaluate expression with current context
    pub fn eval(&self, src: &str) -> Option<Value> {
        parse_str::<syn::Expr>(src)
            .ok()
            .and_then(|src| eval(&self.0, &src))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[allow(clippy::cognitive_complexity)]
    #[test]
    fn test() -> Result<(), ()> {
        let e = Eval::default()
            .insert("foo", "true")?
            .insert("fon", "1")?
            .insert("s", r#""foo""#)?
            .insert("arr", "[1, 2]")?
            .insert("bar", "false")?;

        assert_eq!(e.eval("foo != bar").unwrap(), Value::Bool(true));
        assert_eq!(
            e.eval("true && foo != bar && true").unwrap(),
            Value::Bool(true)
        );
        assert_eq!(e.eval("1 == 1 != bar").unwrap(), Value::Bool(true));
        assert_eq!(e.eval("1 == 1 + 1 == bar").unwrap(), Value::Bool(true));
        assert_eq!(e.eval("0..1").unwrap(), Value::Range(0..1));
        assert_eq!(e.eval("(0..1) == (0..1)").unwrap(), Value::Bool(true));
        assert_eq!(e.eval("0..2 * (1 + 1)").unwrap(), Value::Range(0..4));
        assert_eq!(
            e.eval("fon + 1..fon + 2 * (1 + 1)").unwrap(),
            Value::Range(2..5)
        );
        assert_eq!(
            e.eval("fon + 1..fon + 2 * (1 + 1)").unwrap(),
            Value::Range(2..5)
        );
        assert_eq!(e.eval(r#""foo" == s"#).unwrap(), Value::Bool(true));
        assert_eq!(e.eval(r#""bar" != s"#).unwrap(), Value::Bool(true));
        assert_eq!(e.eval("s").unwrap(), Value::Str("foo".into()));
        assert_eq!(
            e.eval("[foo, true]").unwrap(),
            Value::Vec(vec![Value::Bool(true), Value::Bool(true)])
        );
        assert_eq!(
            e.eval("[foo, 1]").unwrap(),
            Value::Vec(vec![Value::Bool(true), Value::Int(1)])
        );
        assert_eq!(
            e.eval("[foo, [1, 2]]").unwrap(),
            Value::Vec(vec![
                Value::Bool(true),
                Value::Vec(vec![Value::Int(1), Value::Int(2)])
            ])
        );
        assert_eq!(
            e.eval(r#""foo" * 2 * 2"#).unwrap(),
            Value::Str("foofoofoofoo".into())
        );
        assert_eq!(
            e.eval(r#""foo" * (2 * 2 - 1 + 1)"#).unwrap(),
            Value::Str("foofoofoofoo".into())
        );
        assert_eq!(
            e.eval(r#"("foo" + "bar") * 2"#).unwrap(),
            Value::Str("foobarfoobar".into())
        );
        assert_eq!(
            e.eval(r#"("bar" + s * 2) * 2"#).unwrap(),
            Value::Str("barfoofoobarfoofoo".into())
        );

        assert_eq!(e.eval("[foo, 1][1]").unwrap(), Value::Int(1));
        assert_eq!(e.eval("&[0, 1][1]").unwrap(), Value::Int(1));

        assert_eq!(e.eval("arr[1]").unwrap(), Value::Int(2));

        assert_eq!(e.eval("arr[1] + 1").unwrap(), Value::Int(3));

        assert_eq!(e.eval("2 * arr[1] + 1").unwrap(), Value::Int(5));

        assert!(e.eval("arr[2]").is_none());
        assert!(e.eval("[foo, 1][2]").is_none());

        assert_eq!(e.eval(r#""bar"[0..1]"#).unwrap(), Value::Str("b".into()));

        assert_eq!(
            e.eval(r#"("bar" * 2)[3..4]"#).unwrap(),
            Value::Str("b".into())
        );

        assert_eq!(
            e.eval("[foo, [1, 2]]").unwrap().to_string(),
            "[true,[1,2,],]"
        );

        assert_eq!(e.eval(r#""foo""#).unwrap().to_string(), r#""foo""#,);
        assert_eq!(e.eval("0..1").unwrap().to_string(), "0..1");

        assert_eq!(e.eval("1.log10()").unwrap(), Value::Float(1.0f64.log10()));

        assert_eq!(
            e.eval("1.log10() + 2.0").unwrap(),
            Value::Float(1.0f64.log10() + 2.0)
        );

        assert_eq!(e.eval("&[true, not_exist]"), None);
        assert_eq!(e.eval("&[true, None]"), None);
        assert_eq!(e.eval("&[\"foo\", None, None]"), None);
        assert_eq!(e.eval("not_exist"), None);
        assert_eq!(e.eval(r#"&[ "foo", self.s]"#), None);

        Ok(())
    }

    #[test]
    fn test_opt() {
        let e = Eval::default();
        assert_eq!(
            e.eval("Some(1.log10()).unwrap()").unwrap(),
            Value::Float(1.0f64.log10())
        );
        assert_eq!(
            e.eval("None.unwrap_or(1.log10())"),
            Some(Value::Float(1.0f64.log10()))
        );
    }
}
