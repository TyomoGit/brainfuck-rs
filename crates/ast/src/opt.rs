use crate::inst::{Ast, AstCode};

#[derive(Debug, Default)]
pub struct Optimizer {}

impl Optimizer {
    pub fn new() -> Self {
        Self {}
    }

    #[allow(clippy::let_and_return)]
    pub fn optimize(&self, code: AstCode) -> AstCode {
        let code = run_length_optimize(code);
        let code = replace_patterns(code);
        code
    }
}

fn run_length_optimize(code: AstCode) -> AstCode {
    macro_rules! impl_run_length_optimize {
        ($variant:path, $result:expr, $count:expr) => {
            if let Some($variant(last)) = $result.last_mut() {
                *last += $count;
            } else {
                $result.push($variant(*$count));
            }
        };
    }
    let vec = code.vec();
    let mut result: Vec<Ast> = Vec::new();

    for ast in vec {
        match ast {
            Ast::InclementPointer(count) => {
                impl_run_length_optimize!(Ast::InclementPointer, result, count)
            }
            Ast::DecrementPointer(count) => {
                impl_run_length_optimize!(Ast::DecrementPointer, result, count)
            }
            Ast::InclementValue(count) => {
                impl_run_length_optimize!(Ast::InclementValue, result, count)
            }
            Ast::DecrementValue(count) => {
                impl_run_length_optimize!(Ast::DecrementValue, result, count)
            }
            Ast::Loop(l) => {
                let l = run_length_optimize(l.clone());
                result.push(Ast::Loop(l));
            }
            Ast::Output
            | Ast::Input
            | Ast::Load(_)
            | Ast::SumRight(_)
            | Ast::SumLeft(_)
            | Ast::JumpZeroRight { .. }
            | Ast::JumpZeroLeft { .. } => result.push(ast.clone()),
        }
    }

    AstCode::new(result)
}

fn replace_patterns(mut code: AstCode) -> AstCode {
    for ast in code.vec_mut() {
        if let Ast::Loop(l) = ast {
            let result = replace_loops(l);
            *ast = result;
        }
    }

    code
}

fn replace_loops(loop_code: &AstCode) -> Ast {
    let vec = loop_code.vec();

    if *vec == vec![Ast::DecrementValue(1)] {
        return Ast::Load(0);
    }

    if let [Ast::InclementPointer(count)] = vec[..] {
        return Ast::JumpZeroRight { per: count };
    }

    if let [Ast::DecrementPointer(count)] = vec[..] {
        return Ast::JumpZeroLeft { per: count };
    }

    if let [Ast::DecrementValue(1), Ast::InclementPointer(count1), Ast::InclementValue(1), Ast::DecrementPointer(count2)] =
        vec[..]
    {
        if count1 == count2 {
            return Ast::SumRight(count1);
        }
    }

    if let [Ast::DecrementValue(1), Ast::DecrementPointer(count1), Ast::InclementValue(1), Ast::InclementPointer(count2)] =
        vec[..]
    {
        if count1 == count2 {
            return Ast::SumLeft(count1);
        }
    }

    // 最適化パターンに合わなかった場合は何もしない
    Ast::Loop(loop_code.clone())
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_run_length_optimize() {
        let code = AstCode::new(vec![
            Ast::InclementPointer(1),
            Ast::InclementPointer(1),
            Ast::InclementPointer(1),
            Ast::Loop(AstCode::new(vec![
                Ast::DecrementValue(1),
                Ast::DecrementValue(1),
                Ast::DecrementValue(1),
            ])),
        ]);

        assert_eq!(
            run_length_optimize(code.clone()),
            AstCode::new(vec![
                Ast::InclementPointer(3),
                Ast::Loop(AstCode::new(vec![Ast::DecrementValue(3)])),
            ])
        );
    }

    #[test]
    fn test_replace_loops() {
        let code = AstCode::new(vec![
            Ast::Loop(AstCode::new(vec![Ast::DecrementValue(1)])),
            Ast::Loop(AstCode::new(vec![Ast::DecrementValue(10)])),
            Ast::Loop(AstCode::new(vec![Ast::InclementPointer(10)])),
            Ast::Loop(AstCode::new(vec![
                Ast::DecrementValue(1),
                Ast::InclementPointer(10),
                Ast::InclementValue(1),
                Ast::DecrementPointer(10),
            ])),
            Ast::Loop(AstCode::new(vec![
                Ast::DecrementValue(1),
                Ast::DecrementPointer(10),
                Ast::InclementValue(1),
                Ast::InclementPointer(10),
            ])),
            Ast::Loop(AstCode::new(vec![
                Ast::DecrementValue(2),
                Ast::InclementPointer(10),
                Ast::InclementValue(1),
                Ast::DecrementPointer(10),
            ])),
        ]);

        assert_eq!(
            replace_patterns(code),
            AstCode::new(vec![
                Ast::Load(0),
                Ast::Loop(AstCode::new(vec![Ast::DecrementValue(10),])),
                Ast::JumpZeroRight { per: 10 },
                Ast::SumRight(10),
                Ast::SumLeft(10),
                Ast::Loop(AstCode::new(vec![
                    Ast::DecrementValue(2),
                    Ast::InclementPointer(10),
                    Ast::InclementValue(1),
                    Ast::DecrementPointer(10),
                ])),
            ])
        );
    }
}
