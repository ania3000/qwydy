use crate::token::{Token, TokenKind};
use std::collections::HashMap;

#[derive(Debug)]
struct Function {
    //name: String,
    body: Vec<Stmt>,
}

#[derive(Debug)]
enum Stmt {
    Let(String, Expr),
    If(Expr, Vec<Stmt>, Vec<Stmt>),
    Expr(Expr),
}

#[derive(Debug)]
enum Expr {
    Number(i64),
    Variable(String),
    Unary(char, Box<Expr>),
    Binary(Box<Expr>, char, Box<Expr>),
    Call(String, Vec<Expr>),
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn current(&self) -> &Token {
        &self.tokens[self.pos.min(self.tokens.len() - 1)]
    }

    fn at(&self, kind: &TokenKind) -> bool {
        &self.current().kind == kind
    }

    fn advance(&mut self) -> Token {
        let token = self.current().clone();
        if !self.at(&TokenKind::Eof) {
            self.pos += 1;
        }
        token
    }

    fn error(&self, message: &str) -> String {
        format!(
            "{message} (строка {}, столбец {})",
            self.current().line,
            self.current().column
        )
    }

    fn expect(&mut self, kind: TokenKind) -> Result<Token, String> {
        if self.at(&kind) {
            Ok(self.advance())
        } else {
            Err(self.error(&format!(
                "Ожидался токен {kind:?}, получен {:?}",
                self.current().kind
            )))
        }
    }

    fn take_identifier(&mut self) -> Result<String, String> {
        match self.advance().kind {
            TokenKind::Identifier(name) => Ok(name),
            other => Err(format!("Ожидалось имя, получено {other:?}")),
        }
    }

    // Разбирает всю программу, затем запускает главную функцию.
    pub fn parse(&mut self) -> Result<(), String> {
        let mut functions = HashMap::new();

        while !self.at(&TokenKind::Eof) {
            self.expect(TokenKind::Function)?;
            let name = self.take_identifier()?;

            self.expect(TokenKind::LeftParen)?;
            self.expect(TokenKind::RightParen)?;

            let body = self.parse_block()?;

            if functions.insert(name.clone(), Function { body }).is_some() {
                return Err(format!("Функция «{name}» объявлена повторно"));
            }
        }

        let entry = if functions.contains_key("сӕйраг") {
            "сӕйраг"
        } else if functions.contains_key("main") {
            "main"
        } else {
            return Err("Не найдена главная функция: объяви «функци сӕйраг() { ... }»".to_string());
        };

        execute_function(entry, &functions, 0)?;
        Ok(())
    }

    fn parse_block(&mut self) -> Result<Vec<Stmt>, String> {
        self.expect(TokenKind::LeftBrace)?;
        let mut statements = Vec::new();

        while !self.at(&TokenKind::RightBrace) && !self.at(&TokenKind::Eof) {
            statements.push(self.parse_statement()?);
        }

        self.expect(TokenKind::RightBrace)?;
        Ok(statements)
    }

    fn parse_statement(&mut self) -> Result<Stmt, String> {
        if self.at(&TokenKind::Let) {
            self.advance();
            let name = self.take_identifier()?;
            self.expect(TokenKind::Equal)?;
            let value = self.parse_expression()?;
            self.expect(TokenKind::Semicolon)?;
            return Ok(Stmt::Let(name, value));
        }

        if self.at(&TokenKind::If) {
            self.advance();
            let condition = self.parse_expression()?;
            let then_body = self.parse_block()?;

            let else_body = if self.at(&TokenKind::Else) {
                self.advance();
                self.parse_block()?
            } else {
                Vec::new()
            };

            return Ok(Stmt::If(condition, then_body, else_body));
        }

        let expression = self.parse_expression()?;
        self.expect(TokenKind::Semicolon)?;
        Ok(Stmt::Expr(expression))
    }

    fn parse_expression(&mut self) -> Result<Expr, String> {
        self.parse_equality()
    }

    fn parse_equality(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_additive()?;

        while self.at(&TokenKind::EqualEqual) {
            self.advance();
            let right = self.parse_additive()?;
            expr = Expr::Binary(Box::new(expr), '=', Box::new(right));
        }

        Ok(expr)
    }

    fn parse_additive(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_multiplicative()?;

        loop {
            let op = if self.at(&TokenKind::Plus) {
                '+'
            } else if self.at(&TokenKind::Minus) {
                '-'
            } else {
                break;
            };

            self.advance();
            let right = self.parse_multiplicative()?;
            expr = Expr::Binary(Box::new(expr), op, Box::new(right));
        }

        Ok(expr)
    }

    fn parse_multiplicative(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_unary()?;

        loop {
            let op = if self.at(&TokenKind::Star) {
                '*'
            } else if self.at(&TokenKind::Slash) {
                '/'
            } else {
                break;
            };

            self.advance();
            let right = self.parse_unary()?;
            expr = Expr::Binary(Box::new(expr), op, Box::new(right));
        }

        Ok(expr)
    }

    fn parse_unary(&mut self) -> Result<Expr, String> {
        if self.at(&TokenKind::Minus) {
            self.advance();
            return Ok(Expr::Unary('-', Box::new(self.parse_unary()?)));
        }

        if self.at(&TokenKind::Plus) {
            self.advance();
            return Ok(Expr::Unary('+', Box::new(self.parse_unary()?)));
        }

        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<Expr, String> {
        if matches!(self.current().kind, TokenKind::Number(_)) {
            let token = self.advance();

            if let TokenKind::Number(number) = token.kind {
                let value = number
                    .parse::<i64>()
                    .map_err(|_| format!("Некорректное число: {number}"))?;
                return Ok(Expr::Number(value));
            }
        }

        if self.at(&TokenKind::LeftParen) {
            self.advance();
            let expr = self.parse_expression()?;
            self.expect(TokenKind::RightParen)?;
            return Ok(expr);
        }

        if matches!(self.current().kind, TokenKind::Identifier(_)) {
            let name = self.take_identifier()?;

            if self.at(&TokenKind::LeftParen) {
                self.advance();
                let mut args = Vec::new();

                if !self.at(&TokenKind::RightParen) {
                    loop {
                        args.push(self.parse_expression()?);

                        if self.at(&TokenKind::RightParen) {
                            break;
                        }

                        self.expect(TokenKind::Semicolon).or_else(|_| {
                            Err(self.error(
                                "В этой версии аргументы функций пока не поддерживаются через запятую",
                            ))
                        })?;
                    }
                }

                self.expect(TokenKind::RightParen)?;
                return Ok(Expr::Call(name, args));
            }

            return Ok(Expr::Variable(name));
        }

        Err(self.error("Ожидалось число, имя переменной или выражение в скобках"))
    }
}

fn execute_function(
    name: &str,
    functions: &HashMap<String, Function>,
    depth: usize,
) -> Result<(), String> {
    if depth > 128 {
        return Err("Превышена максимальная глубина вызовов функций".to_string());
    }

    let function = functions
        .get(name)
        .ok_or_else(|| format!("Функция «{name}» не найдена"))?;

    let mut variables = HashMap::new();
    execute_block(&function.body, &mut variables, functions, depth)
}

fn execute_block(
    statements: &[Stmt],
    variables: &mut HashMap<String, i64>,
    functions: &HashMap<String, Function>,
    depth: usize,
) -> Result<(), String> {
    for statement in statements {
        match statement {
            Stmt::Let(name, expr) => {
                let value = evaluate(expr, variables, functions, depth)?;
                variables.insert(name.clone(), value);
            }

            Stmt::If(condition, then_body, else_body) => {
                let value = evaluate(condition, variables, functions, depth)?;
                let branch = if value != 0 { then_body } else { else_body };

                // У каждой ветви собственное окружение переменных.
                let mut local_variables = variables.clone();
                execute_block(branch, &mut local_variables, functions, depth)?;
            }

            Stmt::Expr(expr) => {
                evaluate(expr, variables, functions, depth)?;
            }
        }
    }

    Ok(())
}

fn evaluate(
    expr: &Expr,
    variables: &mut HashMap<String, i64>,
    functions: &HashMap<String, Function>,
    depth: usize,
) -> Result<i64, String> {
    match expr {
        Expr::Number(value) => Ok(*value),

        Expr::Variable(name) => variables
            .get(name)
            .copied()
            .ok_or_else(|| format!("Переменная «{name}» не объявлена")),

        Expr::Unary(op, expr) => {
            let value = evaluate(expr, variables, functions, depth)?;

            match op {
                '-' => value
                    .checked_neg()
                    .ok_or_else(|| "Переполнение числа".to_string()),
                '+' => Ok(value),
                _ => Err(format!("Неизвестный унарный оператор {op}")),
            }
        }

        Expr::Binary(left, op, right) => {
            let a = evaluate(left, variables, functions, depth)?;
            let b = evaluate(right, variables, functions, depth)?;

            match op {
                '+' => a
                    .checked_add(b)
                    .ok_or_else(|| "Переполнение числа".to_string()),
                '-' => a
                    .checked_sub(b)
                    .ok_or_else(|| "Переполнение числа".to_string()),
                '*' => a
                    .checked_mul(b)
                    .ok_or_else(|| "Переполнение числа".to_string()),
                '/' => {
                    if b == 0 {
                        return Err("Деление на ноль".to_string());
                    }
                    a.checked_div(b)
                        .ok_or_else(|| "Переполнение числа".to_string())
                }
                '=' => Ok(i64::from(a == b)),
                _ => Err(format!("Неизвестный оператор {op}")),
            }
        }

        Expr::Call(name, args) => {
            if name == "мыхуыр" || name == "output" {
                if args.len() != 1 {
                    return Err(format!("Функция «{name}» принимает ровно один аргумент"));
                }

                let value = evaluate(&args[0], variables, functions, depth)?;
                println!("{value}");
                return Ok(value);
            }

            if !args.is_empty() {
                return Err(format!("Функция «{name}» пока не поддерживает параметры"));
            }

            execute_function(name, functions, depth + 1)?;
            Ok(0)
        }
    }
}
