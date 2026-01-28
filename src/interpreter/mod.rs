use log::debug;
use crate::parser::expr::Expr;
use crate::lexer::token::Token;

pub fn evaluate(expr: &Expr, depth: usize) -> Result<f64, String> {
    let indent = "  ".repeat(depth);

    match expr {
        Expr::Int(i) => {
            debug!("{}-> Integer: {}", indent, i);
            Ok(*i as f64)
        }
        Expr::Float(f) => {
            debug!("{}-> Float: {}", indent, f);
            Ok(*f)
        }
        Expr::Unary { op, right } => {
            debug!("{}-> Unaire {:?}", indent, op);
            let val = evaluate(right, depth + 1)?;
            let res = match op {
                Token::Minus => -val,
                _ => unreachable!(),
            };
            debug!("{}   = Resultat Unaire: {}", indent, res);
            Ok(res)
        }
        Expr::Binary { left, op, right } => {
            debug!("{}-> Binaire {:?}", indent, op);
            let l = evaluate(left, depth + 1)?;
            let r = evaluate(right, depth + 1)?;

            let res = match op {
                Token::Plus => Ok(l + r),
                Token::Minus => Ok(l - r),
                Token::Star => Ok(l * r),
                Token::Slash => {
                    if r == 0.0 {
                        debug!("{}   ! ERREUR: Division par zéro", indent);
                        Err("Division par zéro")
                    } else {
                        Ok(l / r)
                    }
                },
                _ => unreachable!(),
            }?;
            debug!("{}   = Resultat {:?}: {}", indent, op, res);
            Ok(res)
        }
        Expr::Grouping(e) => {
            debug!("{}-> Groupe ( )", indent);
            let res = evaluate(e, depth + 1)?;
            debug!("{}   = Resultat Groupe: {}", indent, res);
            Ok(res)
        }
    }
}