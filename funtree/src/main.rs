use funtree::expression::Expression;
use funtree::funtree_data::FuntreeIndividualData;
use funtree::leaf::LeafType;
use funtree::operation::OperationType;
use funtree::val::ValVec;
use revo::config::Config;

/*
// Implement the ToString trait for the Expr enum
impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Value(n) => write!(f, "{}", n),
            Expr::Var() => write!(f, "x"),
            Expr::Add(e1, e2) => write!(f, "({} + {})", e1, e2),
            Expr::Sub(e1, e2) => write!(f, "({} - {})", e1, e2),
            Expr::Mul(e1, e2) => write!(f, "({} * {})", e1, e2),
            Expr::Div(e1, e2) => write!(f, "({} / {})", e1, e2),
        }
    }
}
*/

fn main() {
    // Create a math expression tree: -(1+2)
    let expr = Expression::new_operation(
        Expression::new_leaf(1.0, LeafType::Constant),
        Expression::new_leaf(2.0, LeafType::Constant),
        OperationType::Addition,
        true,
    );

    // Evaluate the expression and print the result
    let result = expr.evaluate(10.0);
    println!("{} = {}", expr, result); // Output: 49

    let config = Config::new("config.json");
    let ind_data = FuntreeIndividualData::from_config(&config);

    println!("{}", ValVec::from_vec(ind_data.vals));
}
