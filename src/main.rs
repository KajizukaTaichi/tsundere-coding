use sila_transpiler_infrastructure::{
    transpile_javascript, transpile_python, transpile_ruby, Block, Expr, Instruction, Operator,
    Type,
};
use std::{
    fs::File,
    io::{self, Write},
};

fn main() {
    let program = ask_program();
    let program = ask_lang()(program);
    println!("プログラムが出来たわよ！\nで、どのファイルに書き込めば良いのよ？");
    File::create(input(">>> "))
        .expect("ごめん、ファイルを開くのミスっちゃった")
        .write_all(program.as_bytes())
        .expect("ごめん、ファイルの書き込みミスっちゃった");
    println!("書き込んだわ、これで実行できるわね！\n...あんたとプログラミングするの、少しは楽しいのかもしれない")
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
    println!("何言語のプログラムを出力したいのよ？");
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

fn ask_instruction(prompt: &str) -> Instruction {
    println!("{prompt}に何の命令を追加したいのよ？");
    println!("1: 標準出力\n2: 変数宣言\n3: 変数定義\n4: 条件分岐\n5: 繰り返し\n6: エラー処理\n7: 関数定義\n8: 値を返す");
    let answer = input(">>> ");
    if answer == "1" {
        Instruction::Print(ask_expr("引数"))
    } else if answer == "2" {
        Instruction::Let(
            {
                println!("変数名は何にするのよ？");
                input(">>> ")
            },
            ask_expr("変数の値"),
        )
    } else if answer == "3" {
        Instruction::Let(
            {
                println!("何の変数の値を定義するのよ？");
                input(">>> ")
            },
            ask_expr("変数の値"),
        )
    } else if answer == "4" {
        Instruction::If(ask_expr("条件"), ask_block("trueの場合のコード"), {
            println!("Elseのコードは付けるの？");
            println!("1: はい\n2: いいえ\n(デフォルトは2よ)");
            let answer = input(">>> ");
            if answer == "1" {
                Some(ask_block("falseの場合のコード"))
            } else {
                None
            }
        })
    } else if answer == "5" {
        Instruction::While(ask_expr("継続する条件"), ask_block("繰り返すコード"))
    } else if answer == "6" {
        Instruction::TryError(
            ask_block("エラーが起きそうなコード"),
            ask_block("エラー発生時に実行するコード"),
        )
    } else if answer == "7" {
        Instruction::Function(
            {
                println!("関数名は何にするのよ？");
                input(">>> ")
            },
            {
                let mut args = vec![];
                if input("これって引数は取るの？\n1: はい\n2: いいえ\n(デフォルトは2よ)\n>>> ")
                    == "1"
                {
                    loop {
                        args.push(input("引数名を入力してよね\n>>> "));
                        if input(
                            "引数はこれだけよね？\n1: はい\n2: いいえ\n(デフォルトは2よ)\n>>> ",
                        ) == "1"
                        {
                            break;
                        }
                    }
                } else {
                    println!("わかったわ、ただのサブルーチン的な関数なのね")
                }
                args
            },
            ask_block("関数の処理するコード"),
        )
    } else if answer == "8" {
        Instruction::Return(Some(ask_expr("返す値")))
    } else {
        println!("ちょっとぉ！しっかり入力しなさいよ！");
        ask_instruction(prompt)
    }
}

fn ask_program() -> Block {
    println!("さあ、プログラミングを始めるわよ！");
    let mut block: Block = vec![];
    loop {
        block.push(ask_instruction("プログラム"));
        println!("まだプログラミングを続けるわよね？");
        println!("1: もちろん\n2: もうおしまい\n(デフォルトは1よ)");
        let answer = input(">>> ");
        if answer == "2" {
            println!("わかったわ、お疲れ様\nべっ、別にあんたなんかを労って言ってるわけじゃないんだからね！");
            break;
        }
    }
    block
}

fn ask_block(prompt: &str) -> Block {
    println!("さあ、{prompt}を書いていくわよ！");
    let mut block: Block = vec![];
    loop {
        block.push(ask_instruction(prompt));
        println!("まだコードブロックを続けるわよね？");
        println!("1: もちろん\n2: もうおしまい\n(デフォルトは1よ)");
        let answer = input(">>> ");
        if answer == "2" {
            println!("わかったわ、お疲れ様\nべっ、別にあんたなんかを労って言ってるわけじゃないんだからね！");
            break;
        }
    }
    block
}

fn ask_expr(prompt: &str) -> Expr {
    println!("{prompt}の式ぐらい自分で入力しなさいよね！");
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
        } else if token == "==" {
            expr.push(Expr::Operator(Operator::Equal))
        } else if token == ">" {
            expr.push(Expr::Operator(Operator::Greater))
        } else if token == "<" {
            expr.push(Expr::Operator(Operator::Less))
        } else {
            expr.push(Expr::Literal(Type::Symbol(token)))
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
