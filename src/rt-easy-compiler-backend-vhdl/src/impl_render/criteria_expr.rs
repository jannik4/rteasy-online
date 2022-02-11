use crate::vhdl;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct RenderCriteriaExpr<'a>(pub &'a vhdl::Or<vhdl::And<vhdl::Criterion>>);

impl Display for RenderCriteriaExpr<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let or = self.0;

        for (idx, and) in or.0.iter().enumerate() {
            if idx != 0 {
                write!(f, " OR ")?;
            }

            let parentheses_needed = or.0.len() > 1 && and.0.len() > 1;
            if parentheses_needed {
                write!(f, "(")?;
            }
            for (idx, criterion) in and.0.iter().enumerate() {
                if idx != 0 {
                    write!(f, " AND ")?;
                }
                match criterion {
                    vhdl::Criterion::True(id) => write!(f, "k({}) = '1'", id.0)?,
                    vhdl::Criterion::False(id) => write!(f, "k({}) = '0'", id.0)?,
                }
            }
            if parentheses_needed {
                write!(f, ")")?;
            }
        }

        Ok(())
    }
}
