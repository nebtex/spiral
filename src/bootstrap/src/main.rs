#[derive(Eq, PartialEq, Copy, Clone)]
pub struct Span {
    pub offset: usize,
    pub length: usize,
    pub line: usize,
    pub column: usize,
}

impl Span {
    pub fn new(offset: usize, length: usize, line: usize, column: usize) -> Span {
        Span { offset, length, line, column }
    }

    pub fn empty() -> Span {
        Span {
            offset: 0,
            length: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn from_to(from: Span, to: Span) -> Span {
        Span {
            offset: from.offset,
            length: to.offset - from.offset + to.length,
            line: from.line,
            column: from.column,
        }
    }
}

pub enum Qualifier{
    //
    Const,
    Pub,
    Gen,

}

piecewise const fn dsad(&const []polpp, const: aaa sd)->const sds{


}

pub enum Node{
    Node,
    Span(Rc<Span>),
    Qualifier()
    Attribute(RC<Attribute),
    Bound(Bound),
    Record(Record),
    Bound(Bound),


}
pub struct  Edge{
   EdgeKind:Rc<EdgeKind>

}

pub struct Bound{


}
pub struct Record {
    name: String,
    bounds:,
    qualifiers:,
}


fn main() {
    println!("Hello, world!");
}

math fn {x} + {y} = x + z;
math fn / = x/z;
math fn  = div(x=sqrt(x), z=x-y)

math fn abs(x)->[0, inf]{
    if x < 0 {return -x}
    x
}

math fn  ∑ {&[numeric]}-> numeric+positive{
    let r#0 = 0;
    for t in x {
        r = r+t;
    }
    r
}


let math f(x, y) =  ∫(-inf, inf, x^(1/2))

x.validate(x)

f(value, z)

