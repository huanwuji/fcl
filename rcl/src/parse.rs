enum AnyVal {
    Str(String),
    Boolean(bool),
    Num(f64),
}

enum Expr {
    Val(AnyVal),
    Var { name: String },
    Func { args: Vec<Expr> },
    FlowFunc { exprs: Vec<Expr> },
    CurryingFunc { exprs: Vec<Vec<Expr>> },
}

fn main() {
    let a = vec![1, 2, 3];
    println!("Hello, world!");
}
