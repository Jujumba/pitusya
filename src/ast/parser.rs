use super::Proto;

use crate::ast::Ast;
use crate::input::CursoredFile;
use crate::lexer::next_token;
use crate::lexer::tokens::{BinaryOperatorKind, KeywordKind, OperatorKind, TokenKind};

macro_rules! abort_syntax_analysis {
    ($token:expr, $input:expr, $help:expr) => {
        $crate::abort!("{}", $crate::ast::parser::error::construct_error_message(&$token, &$input, $help))
    };
}

pub fn parse(input: &mut CursoredFile) -> Vec<Ast> {
    let mut ast = Vec::new();
    loop {
        let token = next_token(input);
        match &token.kind {
            TokenKind::Keyword(KeywordKind::Fn) => ast.push(Ast::FunctionNode {
                proto: parse_prototype(input, true),
                body: parse_block(input),
            }),
            TokenKind::Keyword(KeywordKind::Extern) => {
                ast.push(Ast::ExternNode(parse_prototype(input, true)));
            }
            TokenKind::EOF => break,
            _ => abort_syntax_analysis!(token, input, "expected `extern` or `fn`"),
        }
    }
    ast
}
fn parse_prototype(input: &mut CursoredFile, definition: bool) -> Proto {
    let name_token = next_token(input);
    let name = match name_token.kind {
        TokenKind::Identifier(name) => name,
        _ => abort_syntax_analysis!(name_token, input, "expected function's name in it's definition"),
    };

    let paren_token = next_token(input);
    match paren_token.kind {
        TokenKind::Operator(OperatorKind::LParen) => (),
        _ => abort_syntax_analysis!(paren_token, input, "expected `(`"),
    }

    let mut args = Vec::<Ast>::new();
    let mut t = next_token(input);

    while t.kind != TokenKind::Operator(OperatorKind::RParen) {
        match t.kind {
            TokenKind::Identifier(_) if name == "main" => abort_syntax_analysis!(t, input, "main function accepts no parameters"),
            TokenKind::Identifier(param) if definition => args.push(Ast::IdentifierNode(param)),
            _ if !definition => {
                input.move_back_cursor(t.len);
                args.push(parse_expression(input));
            }
            _ => abort_syntax_analysis!(t, input, "expected an identifier"),
        }
        let next = next_token(input);
        match next.kind {
            TokenKind::Operator(OperatorKind::Coma) => {
                t = next_token(input);
                continue;
            }
            TokenKind::Operator(OperatorKind::RParen) => break,
            _ => abort_syntax_analysis!(next, input, "expected `,` or `)`"),
        }
    }
    let semicol = next_token(input);
    match semicol.kind {
        TokenKind::Operator(OperatorKind::Semicol) if definition => (),
        _ => input.move_back_cursor(semicol.len),
    }
    Proto { name, args }
}
fn parse_block(input: &mut CursoredFile) -> Vec<Ast> {
    let curly = next_token(input);
    if curly.kind != TokenKind::Operator(OperatorKind::LCurly) {
        abort_syntax_analysis!(curly, input, "`{`");
    }
    let mut body = vec![];
    loop {
        let t = next_token(input);
        match t.kind {
            TokenKind::Keyword(KeywordKind::If) => body.push(Ast::IfNode {
                condition: Box::new(parse_expression(input)),
                body: parse_block(input),
            }),
            TokenKind::Keyword(KeywordKind::While) => body.push(Ast::WhileNode {
                condition: Box::new(parse_expression(input)),
                body: parse_block(input),
            }),
            TokenKind::Keyword(KeywordKind::Let) => body.push(parse_let_expr(input)),
            TokenKind::Identifier(_) | TokenKind::Literal(_) | TokenKind::Operator(OperatorKind::LParen) => {
                input.move_back_cursor(t.len);
                body.push(parse_expression(input));
            }
            TokenKind::Keyword(KeywordKind::Ret) => body.push(Ast::RetNode(Box::new(parse_expression(input)))),
            TokenKind::Operator(OperatorKind::RCurly) => break,
            _ => abort_syntax_analysis!(t, input, "expected `}` or an expression"),
        }
    }
    body
}
fn parse_expression(input: &mut CursoredFile) -> Ast {
    let ast = fetch_lhs(input);
    let token = next_token(input);
    if let TokenKind::Operator(op) = &token.kind {
        match op {
            OperatorKind::Binary(BinaryOperatorKind::Assigment) if matches!(ast, Ast::ValueNode(_)) => {
                abort_syntax_analysis!(token, input, format!("function parameters are immutable"))
            }
            OperatorKind::Binary(op) => Ast::BinaryNode {
                left: Box::new(ast),
                right: Box::new(parse_expression(input)),
                op: *op,
            },
            OperatorKind::Semicol => ast,
            _ => {
                input.move_back_cursor(token.len);
                ast
            }
        }
    } else {
        input.move_back_cursor(token.len);
        ast
    }
}
fn parse_unit_expr(input: &mut CursoredFile) -> Ast {
    let ast = fetch_lhs(input);
    let token = next_token(input);
    match token.kind {
        TokenKind::Operator(op) => match op {
            OperatorKind::Binary(op) => Ast::BinaryNode {
                left: Box::new(ast),
                right: Box::new(parse_unit_expr(input)),
                op,
            },
            OperatorKind::RParen => ast,
            _ => abort_syntax_analysis!(token, input, "expected a binary operator or `)`"),
        },
        _ => abort_syntax_analysis!(token, input, "expected `)`"),
    }
}
fn fetch_lhs(input: &mut CursoredFile) -> Ast {
    let lhs_token = next_token(input);
    match lhs_token.kind {
        TokenKind::Identifier(_) => {
            input.move_back_cursor(lhs_token.len); // todo: !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
            fetch_ident_or_call(input)
        }
        TokenKind::Literal(l) => Ast::ValueNode(l),
        TokenKind::Operator(OperatorKind::LParen) => Ast::UnitNode(Box::new(parse_unit_expr(input))),
        _ => abort_syntax_analysis!(lhs_token, input, "expected an identifier or literal"),
    }
}
fn fetch_ident_or_call(input: &mut CursoredFile) -> Ast {
    let name_token = next_token(input);
    let name = match name_token.kind {
        TokenKind::Identifier(i) => i,
        _ => abort_syntax_analysis!(name_token, input, "expected an identifier"),
    };
    let paren = next_token(input);
    input.move_back_cursor(paren.len);
    if !matches!(paren.kind, TokenKind::Operator(OperatorKind::LParen)) {
        return Ast::IdentifierNode(name);
    };
    input.move_back_cursor(name_token.len);
    Ast::CallNode(parse_prototype(input, false))
}
fn parse_let_expr(input: &mut CursoredFile) -> Ast {
    let token = next_token(input);
    match token.kind {
        TokenKind::Identifier(assignee) => {
            let token = next_token(input);
            match token.kind {
                TokenKind::Operator(OperatorKind::Binary(BinaryOperatorKind::Assigment)) => Ast::LetNode {
                    assignee,
                    value: Box::new(parse_expression(input)),
                },
                _ => abort_syntax_analysis!(token, input, "expected `=`"),
            }
        }
        _ => abort_syntax_analysis!(token, input, "expected an identifier"),
    }
}

mod error {
    use crate::{input::CursoredFile, lexer::tokens::Token};
    use colored::Colorize;

    pub fn construct_error_message<A: AsRef<str>>(token: &Token, file: &CursoredFile, help: A) -> String {
        let help = help.as_ref();
        let chars: &[char] = file.as_ref();
        let start = token.start
            - chars[..token.start]
                .iter()
                .rev()
                .position(|c| *c == '\n')
                .unwrap_or(token.start);
        let end = token.start + chars[token.start..].iter().position(|c| *c == '\n').unwrap_or(file.content.len());
        let line: String = chars[start..end].iter().collect();
        let line_number = chars[..token.start].iter().filter(|c| **c == '\n').count();
        let span_start = token.start - start;
        let span_len = token.len;

        if span_start == 0 {
            format!(
                "{error} in {file_name} on line {line_number}:\n\t{line}\n\t{sep:^>span_len$}\n{col_help}: {actual_help}",
                file_name = file.name.display().to_string().bright_cyan().bold(), // I'm sorry
                line_number = line_number,
                error = "error".bright_red(),
                sep = "^".bright_red(),
                col_help = "note".bright_cyan(),
                actual_help = help
            )
        } else {
            format!(
                "{error} in {file_name} on line {line_number}:\n\t{line}\n\t{space:>span_start$}{sep:^>span_len$}\n{col_help}: {actual_help}",
                file_name = file.name.display().to_string().bright_cyan().bold(),
                line_number = line_number,
                error = "error".bright_red(),
                space = ' ',
                sep = "^".bright_red(),
                col_help = "note".bright_cyan(),
                actual_help = help
            )
        }
    }
}
