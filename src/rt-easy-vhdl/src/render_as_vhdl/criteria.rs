use super::RenderAsVhdl;
use crate::*;
use std::fmt::{Display, Formatter, Result};

impl Display for RenderAsVhdl<&Or<And<Criterion>>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let or = self.0;

        for (idx, and) in or.0.iter().enumerate() {
            if idx != 0 {
                write!(f, " OR ")?;
            }

            if or.0.len() > 1 && and.0.len() > 1 {
                write!(f, "({})", RenderAsVhdl(and))?;
            } else {
                write!(f, "{}", RenderAsVhdl(and))?;
            }
        }

        Ok(())
    }
}

impl Display for RenderAsVhdl<&And<Criterion>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let and = self.0;

        for (idx, criterion) in and.0.iter().enumerate() {
            if idx != 0 {
                write!(f, " AND ")?;
            }

            match criterion {
                Criterion::True(id) => write!(f, "k({}) = '1'", id.0)?,
                Criterion::False(id) => write!(f, "k({}) = '0'", id.0)?,
            }
        }

        Ok(())
    }
}
