pub mod scope;

use std::collections::{HashMap, HashSet};

use anyhow::Context;
use syntect::parsing::{ParseState, Scope, ScopeStackOp, SyntaxReference};

use crate::text::{Loc, Span};

pub struct Parser<'a> {
  /// Scopes to search for, mapping to optional scope-level exclusions. See [`ScopeExpr`].
  include: &'a HashMap<Scope, Option<Scope>>,
  /// Symbols that restrict some scopes. See [`LanguageConfig`].
  restrict: &'a HashSet<Scope>,
  /// Scopes within which no symbols are returned.
  exclude: &'a HashSet<Scope>,
  /// The current (unordered) stack of restricted scopes and their counts (nesting depth).
  /// Scopes that are popped resulting in a count of zero are not necessarily removed from the map.
  restricted_scope_counters: HashMap<Scope, usize>,
  /// The current number of excluded scopes in the scope stack. As excluded scopes are pushed / popped,
  /// this number is incremented / decremented respectively. While this number is positive, no scopes
  /// are returned.
  excluded_scope_count: usize,
  /// The current line, 0 before any line is parsed.
  line: usize,
  /// The syntect parser's internal state.
  state: ParseState,
}

impl<'a> Parser<'a> {
  pub fn new(
    include: &'a HashMap<Scope, Option<Scope>>,
    restrict: &'a HashSet<Scope>,
    exclude: &'a HashSet<Scope>,
    syntax: &SyntaxReference,
  ) -> Self {
    Self {
      include,
      restrict,
      exclude,
      restricted_scope_counters: HashMap::new(),
      excluded_scope_count: 0,
      line: 0,
      state: ParseState::new(syntax),
    }
  }

  pub fn parse_line<'b>(&mut self, line: &'b str) -> Result<Vec<(Span, Scope, &'b str)>, anyhow::Error> {
    self.line += 1;

    let scope_ops = self
      .state
      .parse_line(line, &crate::config::SYNTAX_SET)
      .context("parse_line")?;

    let mut stack = Vec::new();
    let mut spans = Vec::new();

    for (column, scope_op) in scope_ops {
      let loc = Loc::new(self.line, column + 1);

      match scope_op {
        ScopeStackOp::Push(scope) => {
          if self.restrict.contains(&scope) {
            self.restricted_scope_counters.entry(scope).or_insert(1);
          }

          if self.exclude.contains(&scope) {
            self.excluded_scope_count += 1;
          }

          stack.push((loc, scope));
        }

        ScopeStackOp::Pop(n) => {
          for _ in 0..n {
            let Some((start, scope)) = stack.pop() else {
              continue;
            };

            if self.restrict.contains(&scope) {
              *self.restricted_scope_counters.get_mut(&scope).unwrap() -= 1;
            }

            if self.exclude.contains(&scope) {
              self.excluded_scope_count -= 1;
            }

            if self.excluded_scope_count > 0 {
              continue;
            }

            match self.include.get(&scope) {
              Some(None) => {
                spans.push((Span::new(start, loc), scope, &line[start.column - 1..loc.column - 1]));
              }

              Some(Some(restrict))
                if self
                  .restricted_scope_counters
                  .get(restrict)
                  .cloned()
                  .unwrap_or_default()
                  == 0 =>
              {
                spans.push((Span::new(start, loc), scope, &line[start.column - 1..loc.column - 1]));
              }

              _ => continue,
            }
          }
        }

        _ => (),
      };
    }

    Ok(spans)
  }
}
