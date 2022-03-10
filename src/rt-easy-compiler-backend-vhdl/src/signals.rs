pub(crate) use self::fmt::Fmt;

#[derive(Debug)]
pub struct Signals {
    pub condition_signals: Vec<String>,
    pub control_signals: Vec<String>,
}

impl Signals {
    pub(crate) fn new(vhdl: &crate::Vhdl<'_>) -> Self {
        Self {
            condition_signals: vhdl
                .criteria
                .iter()
                .map(|expression| Fmt(expression).to_string())
                .collect(),
            control_signals: vhdl
                .operations
                .iter()
                .map(|operation| Fmt(operation).to_string())
                .collect(),
        }
    }
}

mod fmt {
    use crate::vhdl::*;
    use rtcore::util;
    use std::fmt::{Display, Formatter, Result};

    #[derive(Debug)]
    pub struct Fmt<T>(pub T);

    impl Display for Fmt<&Operation<'_>> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            use Operation::*;
            match &self.0 {
                Write(op) => write!(f, "{}", Fmt(op)),
                Read(op) => write!(f, "{}", Fmt(op)),
                Assignment(op) => write!(f, "{}", Fmt(op)),
            }
        }
    }

    impl Display for Fmt<&Write<'_>> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "write {}", self.0.ident.0)
        }
    }

    impl Display for Fmt<&Read<'_>> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "read {}", self.0.ident.0)
        }
    }

    impl Display for Fmt<&Assignment<'_>> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "{} <- {}", Fmt(&self.0.lhs), Fmt(&self.0.rhs))
        }
    }

    impl Display for Fmt<&Lvalue<'_>> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            use Lvalue::*;
            match &self.0 {
                Register(lvalue) => write!(f, "{}", Fmt(lvalue)),
                Bus(lvalue) => write!(f, "{}", Fmt(lvalue)),
                RegisterArray(lvalue) => write!(f, "{}", Fmt(lvalue)),
                ConcatClocked(lvalue) => write!(f, "{}", Fmt(lvalue)),
                ConcatUnclocked(lvalue) => write!(f, "{}", Fmt(lvalue)),
            }
        }
    }

    impl Display for Fmt<&Expression<'_>> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "{}", Fmt(&self.0.kind))
        }
    }

    impl Display for Fmt<&ExpressionKind<'_>> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            use ExpressionKind::*;
            match &self.0 {
                Atom(expr) => write!(f, "{}", Fmt(expr)),
                BinaryTerm(expr) => write!(f, "{}", Fmt(&**expr)),
                UnaryTerm(expr) => write!(f, "{}", Fmt(&**expr)),
            }
        }
    }

    impl Display for Fmt<&Atom<'_>> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            use Atom::*;
            match &self.0 {
                Concat(atom) => write!(f, "{}", Fmt(atom)),
                Register(atom) => write!(f, "{}", Fmt(atom)),
                Bus(atom) => write!(f, "{}", Fmt(atom)),
                RegisterArray(atom) => write!(f, "{}", Fmt(atom)),
                Number(atom) => write!(f, "{}", Fmt(atom)),
            }
        }
    }

    impl Display for Fmt<&BinaryTerm<'_>> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            if util::parentheses_binary_left(
                self.0.operator.precedence(),
                precedence(&self.0.lhs),
                self.0.operator.associativity(),
            ) {
                write!(f, "({})", Fmt(&self.0.lhs))?;
            } else {
                write!(f, "{}", Fmt(&self.0.lhs))?;
            }

            write!(f, " {} ", self.0.operator)?;

            if util::parentheses_binary_right(
                self.0.operator.precedence(),
                precedence(&self.0.rhs),
                self.0.operator.associativity(),
            ) {
                write!(f, "({})", Fmt(&self.0.rhs))?;
            } else {
                write!(f, "{}", Fmt(&self.0.rhs))?;
            }

            Ok(())
        }
    }

    impl Display for Fmt<&UnaryTerm<'_>> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            let ws = match self.0.operator {
                UnaryOperator::Sign => "",
                UnaryOperator::Neg | UnaryOperator::Not | UnaryOperator::Sxt => " ",
            };

            if util::parentheses_unary(self.0.operator.precedence(), precedence(&self.0.expression))
            {
                write!(f, "{}{}({})", self.0.operator, ws, Fmt(&self.0.expression))
            } else {
                write!(f, "{}{}{}", self.0.operator, ws, Fmt(&self.0.expression))
            }
        }
    }

    impl Display for Fmt<&Register<'_>> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            match self.0.range {
                Some(range) => write!(f, "{}{}", self.0.ident.0, range),
                None => write!(f, "{}", self.0.ident.0),
            }
        }
    }

    impl Display for Fmt<&Bus<'_>> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            match self.0.range {
                Some(range) => write!(f, "{}{}", self.0.ident.0, range),
                None => write!(f, "{}", self.0.ident.0),
            }
        }
    }

    impl Display for Fmt<&RegisterArray<'_>> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "{}[{}]", self.0.ident.0, Fmt(&*self.0.index))
        }
    }

    impl Display for Fmt<&Number> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "\"{}\"", self.0.value.as_bin(true))
        }
    }

    impl<P> Display for Fmt<&Concat<P>>
    where
        for<'a> Fmt<&'a P>: Display,
    {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            if !self.0.parts.is_empty() {
                let mut parts = self.0.parts.iter();
                write!(f, "{}", Fmt(parts.next().unwrap()))?;
                for part in parts {
                    write!(f, ".{}", Fmt(part))?;
                }
            }

            Ok(())
        }
    }

    impl Display for Fmt<&ConcatPartLvalueClocked<'_>> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            use ConcatPartLvalueClocked::*;
            match &self.0 {
                Register(reg, _) => write!(f, "{}", Fmt(reg)),
                RegisterArray(reg_array, _) => write!(f, "{}", Fmt(reg_array)),
            }
        }
    }

    impl Display for Fmt<&ConcatPartLvalueUnclocked<'_>> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            use ConcatPartLvalueUnclocked::*;
            match &self.0 {
                Bus(bus, _) => write!(f, "{}", Fmt(bus)),
            }
        }
    }

    impl Display for Fmt<&ConcatPartExpr<'_>> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            use ConcatPartExpr::*;
            match &self.0 {
                Register(reg) => write!(f, "{}", Fmt(reg)),
                Bus(bus) => write!(f, "{}", Fmt(bus)),
                RegisterArray(reg_array) => write!(f, "{}", Fmt(reg_array)),
                Number(number) => write!(f, "{}", Fmt(number)),
            }
        }
    }

    fn precedence(expression: &Expression<'_>) -> u32 {
        match &expression.kind {
            ExpressionKind::Atom(_) => u32::MAX,
            ExpressionKind::BinaryTerm(binary) => binary.operator.precedence(),
            ExpressionKind::UnaryTerm(unary) => unary.operator.precedence(),
        }
    }
}
