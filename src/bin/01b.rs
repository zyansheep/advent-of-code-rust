use std::io::{self, BufRead};

use chumsky::{prelude::*, error::Cheap, Span, text::int};
use lender::{Lending, Lender};

struct LinesStr<B> {
    buf: B,
    line: String,
}
impl<B: io::BufRead> LinesStr<B> {
	fn new(buf: B) -> LinesStr<B> {
		LinesStr { buf, line: String::with_capacity(32) }
	}
}
impl<'lend, B: io::BufRead> Lending<'lend> for LinesStr<B> {
    type Lend = io::Result<&'lend str>;
}
impl<B: io::BufRead> Lender for LinesStr<B> {
    fn next<'lend>(&'lend mut self) -> Option<io::Result<&'lend str>> {
        self.line.clear();
        match self.buf.read_line(&mut self.line) {
            Err(e) => return Some(Err(e)),
            Ok(0) => return None,
            Ok(_nread) => (),
        };
        if self.line.ends_with('\n') {
            self.line.pop();
            if self.line.ends_with('\r') {
                self.line.pop();
            }
        }
        Some(Ok(&self.line))
    }
}

fn digit_parser() -> impl Parser<char, Option<u32>, Error = Cheap<char>> {
	filter::<_, _, Cheap<char>>(char::is_ascii_digit).map(|c|Some(c.to_digit(10).unwrap())) // digit or
	.or(choice(( // text
		just("one").to(1),
		just("two").to(2),
		just("three").to(3),
		just("four").to(4),
		just("five").to(5),
		just("six").to(6),
		just("seven").to(7),
		just("eight").to(8),
		just("nine").to(9),
	)).map(|v|Some(v)).rewind().then_ignore(any())).or(any().to(None))
}
fn parser() -> impl Parser<char, (Option<u32>, Option<u32>), Error = Cheap<char>> {
	(digit_parser().or(end().to(None)).then(digit_parser().or(end().to(None))))
	.then(digit_parser().repeated()).foldl(|acc: (Option<u32>, Option<u32>), v: Option<u32>|{
		if acc.0.is_none() {
			(acc.1, v)
		} else if v.is_none() {
			acc
		} else { (acc.0, v) }
	})
}

fn main() {
	let line_buf = LinesStr::new(io::stdin().lock());
	let parser = parser();
	let mut num = 0;
	let sum = line_buf.map(|line: io::Result<&str>| {
		let line = line.unwrap();
		num += 1;
		let parse = parser.parse(line).unwrap();
		let ret = match parse {
			(Some(first), Some(last)) => first * 10 + last,
			(Some(first), None) | (None, Some(first)) => first * 10 + first,
			(None, _) => 0
		};
		println!("{line} -> {ret}");
		ret
		
	}).fold(0, |acc, v| acc + v);
	println!("{sum}");
	
}