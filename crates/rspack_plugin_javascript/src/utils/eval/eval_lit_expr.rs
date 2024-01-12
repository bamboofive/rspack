use rspack_core::SpanExt;
use swc_core::common::Spanned;
use swc_core::ecma::ast::{Lit, PropName, Str};

use super::BasicEvaluatedExpression;

fn eval_str(str: &Str) -> BasicEvaluatedExpression {
  let mut res = BasicEvaluatedExpression::with_range(str.span().real_lo(), str.span_hi().0);
  res.set_string(str.value.to_string());
  res
}

pub fn eval_lit_expr(expr: &Lit) -> Option<BasicEvaluatedExpression> {
  match expr {
    Lit::Str(str) => Some(eval_str(str)),
    Lit::Regex(regexp) => {
      let mut res =
        BasicEvaluatedExpression::with_range(regexp.span().real_lo(), regexp.span_hi().0);
      res.set_regexp(regexp.exp.to_string(), regexp.flags.to_string());
      Some(res)
    }
    Lit::Null(null) => {
      let mut res = BasicEvaluatedExpression::with_range(null.span.real_lo(), null.span.hi().0);
      res.set_null();
      Some(res)
    }
    // TODO:
    _ => None,
  }
}

pub fn eval_prop_name(prop_name: &PropName) -> Option<BasicEvaluatedExpression> {
  match prop_name {
    PropName::Str(str) => Some(eval_str(str)),
    // TODO:
    PropName::Ident(_) => None,
    PropName::Num(_) => None,
    PropName::Computed(_) => None,
    PropName::BigInt(_) => None,
  }
}
