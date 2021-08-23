#![deny(rust_2018_idioms)]

use std::fmt;
use std::ops::Range;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Span {
    Eoi,
    Range(Range<usize>),
}

#[derive(Debug)]
pub struct Error<'a> {
    message: &'a str,
    source: Option<(&'a str, Span)>,
    file_name: Option<&'a str>,
}

impl<'a> Error<'a> {
    pub fn new(message: &'a str) -> Self {
        Self { message, source: None, file_name: None }
    }

    pub fn with_source(mut self, source: &'a str, span: Span) -> Self {
        self.source = Some((source, span));
        self
    }

    pub fn with_file_name(mut self, file_name: &'a str) -> Self {
        self.file_name = Some(file_name);
        self
    }
}

impl fmt::Display for Error<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (locations, indent) = match &self.source {
            None => (None, 2),
            Some((source, span)) => {
                let locations = locations(source, span.clone());
                let indent = locations.iter().map(|loc| digits(loc.line) + 1).max().unwrap();
                (Some(locations), indent)
            }
        };

        if self.source.is_some() {
            for _ in 0..indent - 1 {
                write!(f, " ")?;
            }
            write!(f, "--> ")?;
            if let Some(file_name) = self.file_name {
                write!(f, "{}", file_name)?;
                write!(f, ":")?;
            }

            match locations {
                None => write!(f, "EOI\n")?,
                Some(locations) => {
                    write!(f, "{}:{}\n", locations[0].line, locations[0].column)?;

                    pre(f, "", indent)?;
                    write!(f, "\n")?;

                    for loc in locations {
                        pre(f, &loc.line.to_string(), indent)?;
                        write!(f, "    {}\n", loc.line_slice)?;

                        pre(f, "", indent)?;
                        write!(f, "    ")?;
                        for i in 0..loc.column_last {
                            if i < loc.column - 1 {
                                write!(f, " ")?;
                            } else {
                                write!(f, "^")?;
                            }
                        }
                        write!(f, "\n")?;
                    }

                    pre(f, "", indent)?;
                    write!(f, "\n")?;
                }
            }
        }

        for _ in 0..indent {
            write!(f, " ")?;
        }
        write!(f, "= {}", self.message)?;

        Ok(())
    }
}

fn pre(result: &mut impl fmt::Write, s: &str, len: usize) -> fmt::Result {
    write!(result, "{}", s)?;
    for _ in 0..len - s.len() {
        write!(result, " ")?;
    }
    write!(result, "|")?;

    Ok(())
}

fn digits(mut num: usize) -> usize {
    let mut digits = 0;
    loop {
        num /= 10;
        digits += 1;
        if num == 0 {
            break digits;
        }
    }
}

#[derive(Debug)]
struct ErrLoc<'a> {
    line: usize,
    column: usize,
    column_last: usize,
    line_slice: &'a str,
    range: Range<usize>,
}

fn locations(source: &str, span: Span) -> Vec<ErrLoc<'_>> {
    let span = match span {
        Span::Eoi => source.len()..source.len(),
        Span::Range(range) => range,
    };

    let mut result = Vec::new();
    let mut result_start_new = false;

    let mut pos = 0;
    let mut line_start = 0;
    let mut line_slice = "";

    let mut line = 1;
    let mut column = 1;

    let mut chars = source.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '\r' if chars.peek() == Some(&'\n') => {
                chars.next().unwrap();

                if span.contains(&pos) {
                    result_start_new = true;
                }

                pos += 2;
                line_start = pos;
                line_slice = "";

                line += 1;
                column = 1;

                if pos >= span.end {
                    break;
                }
            }
            '\n' => {
                if span.contains(&pos) {
                    result_start_new = true;
                }

                pos += 1;
                line_start = pos;
                line_slice = "";

                line += 1;
                column = 1;

                if pos >= span.end {
                    break;
                }
            }
            c => {
                line_slice = &source[line_start..pos + c.len_utf8()];

                if span.start == pos || (span.contains(&pos) && result_start_new) {
                    result.push(ErrLoc {
                        line,
                        column,
                        column_last: column,
                        line_slice,
                        range: pos..pos + c.len_utf8(),
                    });
                } else if span.contains(&pos) {
                    result.last_mut().unwrap().column_last = column;
                    result.last_mut().unwrap().range.end = pos + c.len_utf8();
                }
                result_start_new = false;

                if let Some(curr) = result.last_mut() {
                    curr.line_slice = line_slice;
                }

                pos += c.len_utf8();

                column += 1;
            }
        }
    }

    if span.start == pos {
        result.push(ErrLoc { line, column, column_last: column, line_slice, range: pos..pos });
    }

    result
}
