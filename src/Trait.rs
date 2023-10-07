// Rust traits ~ Haskell Type Classes

trait Show {
    fn show(&self) -> String;
}

// Box (smart pointer to a heap allocated value ) to avoid "recursive type `Expr` has infinite size"
#[derive(PartialEq, Eq)] // like deriving (Eq)
enum Expr {
    Val(i64),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
}

impl Show for Expr {
    fn show(&self) -> String {
        match self {
            Expr::Val(i) => i.to_string(),
            Expr::Add(l, r) => format!("({} + {})", l.show(), r.show()),
            Expr::Mul(l, r) => format!("({} * {})", l.show(), r.show()),
        }
    }
}

// show_Values :: (Eq t, Show t) => t -> t -> IO ()
fn show_Values<T: Eq + Show>(a: &T, b: &T) { 
    if a != b {
        println!("{} {}", a.show(), b.show());
    } else {
        println!("Same: {}", a.show())
    }
}

fn eval(expr: &Expr) -> i64 {
    match expr {
        Expr::Val(i) => *i,
        Expr::Add(l, r) => eval(l) + eval(r),
        Expr::Mul(l, r) => eval(l) * eval(r),
    }
}

fn main() {
    let e = Expr::Add(
        Box::new(Expr::Val(1)),
        Box::new(Expr::Val(2)),
    );

    println!("{}", eval(&e));
    println!("{}", e.show());

    show_Values(&e, &e);
}
