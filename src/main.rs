use sila_transpiler_infrastructure::{
    transpile_javascript, transpile_python, transpile_ruby, Block, Expr, Instruction, Operator,
    Type,
};
use std::io::{self, Write};

fn main() {
    let program = ask_block();
    println!("ほら、プログラムが出来たわよ！\n{}", ask_lang()(program));
}

/// Get standard input
fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut result = String::new();
    io::stdin().read_line(&mut result).ok();
    result.trim().to_string()
}

fn ask_lang() -> Box<dyn Fn(Block) -> String> {
    println!("これが最後の質問よ。\n何言語のプログラムを出力したいのよ？");
    println!("1: JavaScript\n2: Ruby\n3: Python");
    let answer = input(">>> ");
    if answer == "1" {
        Box::new(transpile_javascript)
    } else if answer == "2" {
        Box::new(transpile_ruby)
    } else if answer == "3" {
        Box::new(transpile_python)
    } else {
        println!("まじめに入力しなさいよね！");
        ask_lang()
    }
}

fn ask_instruction() -> Instruction {
    println!("何の命令を追加したいのよ？");
    println!("1: 標準出力");
    let answer = input(">>> ");
    if answer == "1" {
        Instruction::Print(ask_expr())
    } else {
        println!("まじめに入力しなさいよね！");
        ask_instruction()
    }
}

fn ask_block() -> Block {
    let mut block: Block = vec![];
    loop {
        block.push(ask_instruction());
        println!("まだプログラミングを続けるわよね？");
        println!("1: もちろん\n2: もうおしまい\n(デフォルトは1よ)");
        let answer = input(">>> ");
        if answer == "2" {
            println!("え、もうやめるの？わかったわ。お疲れ様");
            break;
        }
    }
    block
}

fn ask_expr() -> Expr {
    println!("式ぐらい自分で入力しなさいよね！");
    parse_expr(input(">>> "))
}

fn parse_expr(source: String) -> Expr {
    let mut expr = vec![];
    for token in tokenize(source) {
        let chars: Vec<char> = token.trim().chars().collect();
        if let Ok(i) = token.parse::<i64>() {
            expr.push(Expr::Literal(Type::Integer(i)))
        } else if let Ok(f) = token.parse::<f64>() {
            expr.push(Expr::Literal(Type::Float(f)))
        } else if chars[0] == '"' && chars[chars.len() - 1] == '"' {
            let inner_string = String::from_iter(chars[1..chars.len() - 1].iter());
            expr.push(Expr::Literal(Type::String(inner_string)))
        } else if chars[0] == '(' && chars[chars.len() - 1] == ')' {
            let inner_brace = String::from_iter(chars[1..chars.len() - 1].iter());
            expr.push(parse_expr(inner_brace))
        } else if token == "+" {
            expr.push(Expr::Operator(Operator::Add))
        } else if token == "-" {
            expr.push(Expr::Operator(Operator::Sub))
        } else if token == "*" {
            expr.push(Expr::Operator(Operator::Mul))
        } else if token == "/" {
            expr.push(Expr::Operator(Operator::Div))
        } else if token == "%" {
            expr.push(Expr::Operator(Operator::Mod))
        }
    }
    Expr::Expr(expr)
}

fn tokenize(input: String) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current_token = String::new();
    let mut in_parentheses: usize = 0;
    let mut in_quote = false;

    for c in input.chars() {
        match c {
            '(' if !in_quote => {
                if in_parentheses != 0 {
                    in_parentheses += 1;
                    current_token.push(c);
                } else {
                    if !current_token.is_empty() {
                        tokens.push(current_token.clone());
                        current_token.clear();
                    }
                    in_parentheses += 1;
                    current_token.push(c);
                }
            }
            ')' if !in_quote => {
                if in_parentheses != 0 {
                    current_token.push(c);
                    in_parentheses -= 1;
                    if in_parentheses == 0 {
                        tokens.push(current_token.clone());
                        current_token.clear();
                    }
                } else {
                    panic!("Syntax error: invalid end of parentheses")
                }
            }
            '"' => {
                if in_parentheses == 0 {
                    if in_quote {
                        current_token.push(c);
                        in_quote = false;
                        tokens.push(current_token.clone());
                        current_token.clear();
                    } else {
                        in_quote = true;
                        current_token.push(c);
                    }
                } else {
                    current_token.push(c);
                }
            }
            ' ' | '\n' | '\t' | '\r' | '　' => {
                if in_parentheses != 0 || in_quote {
                    current_token.push(c);
                } else {
                    if !current_token.is_empty() {
                        tokens.push(current_token.clone());
                        current_token.clear();
                    }
                }
            }
            _ => {
                current_token.push(c);
            }
        }
    }

    if in_parentheses != 0 {
        panic!("Syntax error: There isn't end of parentheses");
    }
    if in_quote {
        panic!("Syntax error: There isn't end of quote");
    }

    if !current_token.is_empty() {
        tokens.push(current_token);
    }

    tokens
}
