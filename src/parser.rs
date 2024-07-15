use std::collections::HashSet;

use anyhow::Context;
use syntect::parsing::{BasicScopeStackOp, ParseState, Scope, ScopeStack, ScopeStackOp, SyntaxReference};

use crate::text::{Loc, Span};

pub struct Parser<'a> {
  scopes: &'a HashSet<Scope>,
  line: usize,
  state: ParseState,
}

impl<'a> Parser<'a> {
  pub fn new(scopes: &'a HashSet<Scope>, syntax: &SyntaxReference) -> Self {
    Self {
      scopes,
      line: 1,
      state: ParseState::new(syntax),
    }
  }

  pub fn parse_line<'b>(&mut self, line: &'b str) -> Result<Vec<(Span, Scope, &'b str)>, anyhow::Error> {
    let scope_ops = self
      .state
      .parse_line(line, &crate::config::SYNTAX_SET)
      .context("parse_line")?;

    let mut stack = Vec::new();
    let mut spans = Vec::new();

    for (column, scope_op) in scope_ops {
      let loc = Loc::new(self.line, column);

      match scope_op {
        ScopeStackOp::Push(scope) => stack.push((loc, scope)),

        ScopeStackOp::Pop(n) => {
          for _ in 0..n {
            let Some((start, scope)) = stack.pop() else {
              continue;
            };

            if self.scopes.contains(&scope) {
              spans.push((Span::new(start, loc), scope, &line[start.column..loc.column]));
            }
          }
        }

        _ => (),
      };
    }

    self.line += 1;

    Ok(spans)
  }
}
