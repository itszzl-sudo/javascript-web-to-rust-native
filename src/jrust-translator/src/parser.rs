use crate::ast::{
    Expression, Identifier, LiteralValue, Program, SourceLocation, SourceType, Statement,
    SwitchCase, VariableDeclarationKind,
};
use crate::error::{Error, Result};

pub struct Parser {
    source: String,
    pos: usize,
    tokens: Vec<Token>,
    token_pos: usize,
}

#[derive(Debug, Clone)]
struct Token {
    kind: TokenKind,
    value: String,
    start: usize,
    end: usize,
}

#[derive(Debug, Clone, PartialEq)]
enum TokenKind {
    Keyword,
    Identifier,
    String,
    Number,
    Template,
    Regex,
    Punctuator,
    Boolean,
    Null,
    Eof,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            source: String::new(),
            pos: 0,
            tokens: Vec::new(),
            token_pos: 0,
        }
    }

    pub fn parse(&mut self, source: &str) -> Result<Program> {
        self.source = source.to_string();
        self.pos = 0;
        self.tokenize();
        self.token_pos = 0;
        self.parse_program()
    }

    fn tokenize(&mut self) {
        let keywords = [
            "var", "let", "const", "function", "if", "else", "for", "while", "do",
            "switch", "case", "default", "break", "continue", "return", "try",
            "catch", "finally", "throw", "new", "delete", "typeof", "void",
            "in", "instanceof", "class", "extends", "super", "this", "import",
            "export", "from", "as", "async", "await", "yield", "static",
            "get", "set", "true", "false", "null", "undefined",
        ];

        let mut tokens = Vec::new();

        while self.pos < self.source.len() {
            let start = self.pos;
            let ch = self.peek();

            if ch.is_whitespace() {
                self.advance();
                continue;
            }

            if ch == '/' {
                if self.peek_next() == '/' {
                    while self.pos < self.source.len() && self.peek() != '\n' {
                        self.advance();
                    }
                    continue;
                }
                if self.peek_next() == '*' {
                    self.advance_n(2);
                    while self.pos < self.source.len() - 1 {
                        if self.peek() == '*' && self.peek_next() == '/' {
                            self.advance_n(2);
                            break;
                        }
                        self.advance();
                    }
                    continue;
                }
            }

            if ch.is_alphabetic() || ch == '_' || ch == '$' {
                let mut value = String::new();
                while self.pos < self.source.len() {
                    let c = self.peek();
                    if c.is_alphanumeric() || c == '_' || c == '$' {
                        value.push(self.advance());
                    } else {
                        break;
                    }
                }
                let kind = if keywords.contains(&value.as_str()) {
                    TokenKind::Keyword
                } else if value == "true" || value == "false" {
                    TokenKind::Boolean
                } else if value == "null" {
                    TokenKind::Null
                } else {
                    TokenKind::Identifier
                };
                tokens.push(Token {
                    kind,
                    value,
                    start,
                    end: self.pos,
                });
                continue;
            }

            if ch.is_numeric() || (ch == '.' && self.peek_next().is_numeric()) {
                let mut value = String::new();
                while self.pos < self.source.len() {
                    let c = self.peek();
                    if c.is_numeric() || c == '.' || c == 'e' || c == 'E' || c == 'x' || c == 'X' {
                        value.push(self.advance());
                    } else {
                        break;
                    }
                }
                tokens.push(Token {
                    kind: TokenKind::Number,
                    value,
                    start,
                    end: self.pos,
                });
                continue;
            }

            if ch == '"' || ch == '\'' {
                let quote = self.advance();
                let mut value = String::new();
                while self.pos < self.source.len() && self.peek() != quote {
                    if self.peek() == '\\' {
                        self.advance();
                        let escaped = self.advance();
                        match escaped {
                            'n' => value.push('\n'),
                            't' => value.push('\t'),
                            'r' => value.push('\r'),
                            '\\' => value.push('\\'),
                            '\'' => value.push('\''),
                            '"' => value.push('"'),
                            _ => value.push(escaped),
                        }
                    } else {
                        value.push(self.advance());
                    }
                }
                self.expect_token(&quote.to_string()).ok();
                tokens.push(Token {
                    kind: TokenKind::String,
                    value,
                    start,
                    end: self.pos,
                });
                continue;
            }

            let punctuators = [
                "===", "!==", "==", "!=", "<=", ">=", "=>", "&&", "||", "??",
                "...", "**", "+=", "-=", "*=", "/=", "%=", "&=", "|=", "^=",
                "<<=", ">>=", ">>>=", "++", "--", "<<", ">>", "<<<", ">>>",
                "::", "?.", "?", ":", ";", ",", ".", "[", "]", "{", "}",
                "(", ")", "+", "-", "*", "/", "%", "=", "<", ">", "&", "|",
                "^", "~", "!", "%", "#", "@", "`",
            ];

            let mut found = false;
            for punc in punctuators {
                if self.source[self.pos..].starts_with(punc) {
                    tokens.push(Token {
                        kind: TokenKind::Punctuator,
                        value: punc.to_string(),
                        start,
                        end: self.pos + punc.len(),
                    });
                    self.advance_n(punc.len());
                    found = true;
                    break;
                }
            }

            if !found {
                tokens.push(Token {
                    kind: TokenKind::Punctuator,
                    value: self.advance().to_string(),
                    start,
                    end: self.pos,
                });
            }
        }

        tokens.push(Token {
            kind: TokenKind::Eof,
            value: String::new(),
            start: self.source.len(),
            end: self.source.len(),
        });

        self.tokens = tokens;
    }

    fn peek_token(&self) -> &Token {
        self.tokens.get(self.token_pos).unwrap_or(self.tokens.last().unwrap())
    }

    fn peek_next_token(&self) -> &Token {
        self.tokens.get(self.token_pos + 1).unwrap_or(self.tokens.last().unwrap())
    }

    fn advance_token(&mut self) -> Token {
        let token = self.tokens.get(self.token_pos).cloned().unwrap_or_else(|| Token {
            kind: TokenKind::Eof,
            value: String::new(),
            start: self.source.len(),
            end: self.source.len(),
        });
        self.token_pos += 1;
        token
    }

    fn expect_token(&mut self, value: &str) -> Result<Token> {
        let token = self.advance_token();
        if token.value != value {
            return Err(Error::ParseError {
                location: format!("line {}", self.get_line()),
                message: format!("Expected '{}', found '{}'", value, token.value),
            });
        }
        Ok(token)
    }

    fn get_line(&self) -> usize {
        self.source[..self.pos.min(self.source.len())]
            .chars()
            .filter(|&c| c == '\n')
            .count()
            + 1
    }

    fn peek(&self) -> char {
        self.source.chars().nth(self.pos).unwrap_or('\0')
    }

    fn peek_next(&self) -> char {
        self.source.chars().nth(self.pos + 1).unwrap_or('\0')
    }

    fn advance(&mut self) -> char {
        let ch = self.source.chars().nth(self.pos).unwrap_or('\0');
        self.pos += 1;
        ch
    }

    fn advance_n(&mut self, n: usize) {
        self.pos += n;
    }

    fn parse_program(&mut self) -> Result<Program> {
        let mut body = Vec::new();
        let source_type = if self.source.contains("import ") || self.source.contains("export ") {
            SourceType::Module
        } else {
            SourceType::Script
        };
        while self.peek_token().kind != TokenKind::Eof {
            if let Some(stmt) = self.parse_statement()? {
                body.push(stmt);
            }
        }
        Ok(Program {
            source_type,
            body,
            loc: SourceLocation::default(),
        })
    }

    fn parse_statement(&mut self) -> Result<Option<Statement>> {
        let token = self.peek_token();

        match token.value.as_str() {
            "var" | "let" | "const" => self.parse_variable_declaration(),
            "async" => {
                self.advance_token();
                if self.peek_token().value == "function" {
                    self.parse_function_declaration()
                } else if self.peek_token().value == "(" {
                    self.parse_expression_statement_or_labeled()
                } else {
                    Err(Error::ParseError {
                        location: format!("line {}", self.get_line()),
                        message: "Unexpected token after async".to_string(),
                    })
                }
            }
            "function" => self.parse_function_declaration(),
            "if" => self.parse_if_statement(),
            "for" => self.parse_for_statement(),
            "while" => self.parse_while_statement(),
            "do" => self.parse_do_while_statement(),
            "switch" => self.parse_switch_statement(),
            "try" => self.parse_try_statement(),
            "return" => self.parse_return_statement(),
            "break" => self.parse_break_statement(),
            "continue" => self.parse_continue_statement(),
            "throw" => self.parse_throw_statement(),
            "with" => self.parse_with_statement(),
            "class" => self.parse_class_declaration(),
            "import" => self.parse_import_statement(),
            "export" => self.parse_export_statement(),
            "debugger" => {
                self.advance_token();
                if self.peek_token().value == ";" {
                    self.advance_token();
                }
                Ok(Some(Statement::DebuggerStatement {
                    loc: SourceLocation::default(),
                }))
            },
            "{" => self.parse_block_or_object_statement(),
            ";" => {
                self.advance_token();
                Ok(Some(Statement::EmptyStatement {
                    loc: SourceLocation::default(),
                }))
            }
            _ => self.parse_expression_statement_or_labeled(),
        }
    }

    fn parse_variable_declaration(&mut self) -> Result<Option<Statement>> {
        let token = self.advance_token();
        let kind = match token.value.as_str() {
            "var" => VariableDeclarationKind::Var,
            "let" => VariableDeclarationKind::Let,
            "const" => VariableDeclarationKind::Const,
            _ => return Err(Error::ParseError {
                location: format!("line {}", self.get_line()),
                message: "Expected var, let, or const".to_string(),
            }),
        };

        let mut declarations = Vec::new();
        loop {
            let id = self.parse_pattern()?;
            let init = if self.peek_token().value == "=" {
                self.advance_token();
                Some(self.parse_expression()?)
            } else {
                None
            };
            declarations.push(crate::ast::VariableDeclarator {
                id,
                init,
                loc: SourceLocation::default(),
            });
            if self.peek_token().value == "," {
                self.advance_token();
            } else {
                break;
            }
        }
        if self.peek_token().value == ";" {
            self.advance_token();
        }
        Ok(Some(Statement::VariableDeclaration {
            declarations,
            kind,
            loc: SourceLocation::default(),
        }))
    }

    fn parse_pattern(&mut self) -> Result<crate::ast::Pattern> {
        let token = self.peek_token();
        
        if token.value == "..." {
            self.advance_token();
            let inner = self.parse_pattern()?;
            Ok(crate::ast::Pattern::RestElement {
                argument: Box::new(inner),
                loc: SourceLocation::default(),
            })
        } else if token.value == "[" {
            self.parse_array_pattern()
        } else if token.value == "{" {
            self.parse_object_pattern()
        } else if token.kind == TokenKind::Identifier {
            let token = self.advance_token();
            Ok(crate::ast::Pattern::Identifier {
                id: Identifier {
                    name: token.value,
                    loc: SourceLocation::default(),
                },
                init: None,
            })
        } else {
            Err(Error::ParseError {
                location: format!("line {}", self.get_line()),
                message: format!("Expected identifier, found '{}'", token.value),
            })
        }
    }

    fn parse_array_pattern(&mut self) -> Result<crate::ast::Pattern> {
        self.advance_token();
        let mut elements = Vec::new();
        while self.peek_token().value != "]" {
            if self.peek_token().value == "," {
                elements.push(None);
                self.advance_token();
            } else if self.peek_token().value == "..." {
                self.advance_token();
                let inner = self.parse_pattern()?;
                elements.push(Some(crate::ast::Pattern::RestElement {
                    argument: Box::new(inner),
                    loc: SourceLocation::default(),
                }));
                if self.peek_token().value == "," {
                    self.advance_token();
                }
            } else {
                elements.push(Some(self.parse_pattern()?));
                if self.peek_token().value == "," {
                    self.advance_token();
                }
            }
        }
        self.expect_token("]")?;
        Ok(crate::ast::Pattern::ArrayPattern {
            elements,
            loc: SourceLocation::default(),
        })
    }

    fn parse_object_pattern(&mut self) -> Result<crate::ast::Pattern> {
        self.advance_token();
        let mut properties = Vec::new();
        while self.peek_token().value != "}" {
            let key = self.parse_assignment()?;
            let value = if self.peek_token().value == ":" {
                self.advance_token();
                Some(self.parse_pattern()?)
            } else {
                if let Expression::Identifier(id) = &key {
                    Some(crate::ast::Pattern::Identifier {
                        id: id.clone(),
                        init: None,
                    })
                } else {
                    None
                }
            };
            properties.push(crate::ast::ObjectPatternProperty {
                key,
                value,
                computed: false,
                loc: SourceLocation::default(),
            });
            if self.peek_token().value == "," {
                self.advance_token();
            }
        }
        self.expect_token("}")?;
        Ok(crate::ast::Pattern::ObjectPattern {
            properties,
            loc: SourceLocation::default(),
        })
    }

    fn parse_function_declaration(&mut self) -> Result<Option<Statement>> {
        self.advance_token();
        
        let async_ = self.peek_token().value == "async";
        if async_ {
            self.advance_token();
        }
        
        let id = if self.peek_token().kind == TokenKind::Identifier {
            let token = self.advance_token();
            Some(Identifier {
                name: token.value,
                loc: SourceLocation::default(),
            })
        } else {
            None
        };
        
        let generator = self.peek_token().value == "*";
        if generator {
            self.advance_token();
        }
        
        self.expect_token("(")?;
        let mut params = Vec::new();
        while self.peek_token().value != ")" {
            params.push(self.parse_pattern()?);
            if self.peek_token().value == "," {
                self.advance_token();
            }
        }
        self.expect_token(")")?;
        self.expect_token("{")?;
        let body = self.parse_block_body()?;
        Ok(Some(Statement::FunctionDeclaration {
            id: id.unwrap_or(Identifier {
                name: String::new(),
                loc: SourceLocation::default(),
            }),
            params,
            body: Box::new(crate::ast::FunctionBody {
                body,
                loc: SourceLocation::default(),
            }),
            generator,
            async_,
            loc: SourceLocation::default(),
        }))
    }

    fn parse_block_body(&mut self) -> Result<Vec<Statement>> {
        let mut body = Vec::new();
        let mut brace_count = 1;
        while brace_count > 0 && self.peek_token().kind != TokenKind::Eof {
            if self.peek_token().value == "{" {
                brace_count += 1;
            } else if self.peek_token().value == "}" {
                brace_count -= 1;
                if brace_count == 0 {
                    self.advance_token();
                    break;
                }
            }
            if let Some(stmt) = self.parse_statement()? {
                body.push(stmt);
            }
        }
        Ok(body)
    }

    fn parse_if_statement(&mut self) -> Result<Option<Statement>> {
        self.advance_token();
        self.expect_token("(")?;
        let test = self.parse_expression()?;
        self.expect_token(")")?;
        let consequent = if self.peek_token().value == "{" {
            self.parse_block_statement()?.unwrap()
        } else {
            self.parse_statement()?.unwrap_or(Statement::EmptyStatement {
                loc: SourceLocation::default(),
            })
        };
        let alternate = if self.peek_token().value == "else" {
            self.advance_token();
            Some(Box::new(if self.peek_token().value == "{" {
                self.parse_block_statement()?.unwrap()
            } else {
                self.parse_statement()?.unwrap_or(Statement::EmptyStatement {
                    loc: SourceLocation::default(),
                })
            }))
        } else {
            None
        };
        Ok(Some(Statement::IfStatement {
            test,
            consequent: Box::new(consequent),
            alternate,
            loc: SourceLocation::default(),
        }))
    }

    fn parse_for_statement(&mut self) -> Result<Option<Statement>> {
        self.advance_token();
        self.expect_token("(")?;

        let init = if self.peek_token().value == "var" || self.peek_token().value == "let" || self.peek_token().value == "const" {
            let token = self.advance_token();
            let kind = match token.value.as_str() {
                "var" => VariableDeclarationKind::Var,
                "let" => VariableDeclarationKind::Let,
                "const" => VariableDeclarationKind::Const,
                _ => VariableDeclarationKind::Var,
            };
            let mut declarations = Vec::new();
            loop {
                let id = self.parse_pattern()?;
                let init = if self.peek_token().value == "=" {
                    self.advance_token();
                    Some(self.parse_expression()?)
                } else {
                    None
                };
                declarations.push(crate::ast::VariableDeclarator {
                    id,
                    init,
                    loc: SourceLocation::default(),
                });
                if self.peek_token().value == "," {
                    self.advance_token();
                } else {
                    break;
                }
            }
            Some(crate::ast::ForStatementInit::Variable(Box::new(Statement::VariableDeclaration {
                declarations,
                kind,
                loc: SourceLocation::default(),
            })))
        } else if self.peek_token().value != ";" {
            Some(crate::ast::ForStatementInit::Expression(self.parse_expression()?))
        } else {
            None
        };

        if self.peek_token().value == "in" {
            self.advance_token();
            let right = self.parse_expression()?;
            self.expect_token(")")?;

            let body = if self.peek_token().value == "{" {
                self.parse_block_statement()?.unwrap()
            } else {
                self.parse_statement()?.unwrap_or(Statement::EmptyStatement {
                    loc: SourceLocation::default(),
                })
            };

            let left = match &init {
                Some(crate::ast::ForStatementInit::Variable(stmt)) => crate::ast::ForStatementLeft::Variable(stmt.clone()),
                Some(crate::ast::ForStatementInit::Expression(expr)) => {
                    if let Expression::Identifier(id) = expr {
                        crate::ast::ForStatementLeft::Pattern(crate::ast::Pattern::Identifier {
                            id: id.clone(),
                            init: None,
                        })
                    } else {
                        crate::ast::ForStatementLeft::Pattern(crate::ast::Pattern::Identifier {
                            id: Identifier {
                                name: "_".to_string(),
                                loc: SourceLocation::default(),
                            },
                            init: None,
                        })
                    }
                }
                None => crate::ast::ForStatementLeft::Pattern(crate::ast::Pattern::Identifier {
                    id: Identifier {
                        name: "_".to_string(),
                        loc: SourceLocation::default(),
                    },
                    init: None,
                }),
            };

            return Ok(Some(Statement::ForInStatement {
                left,
                right,
                body: Box::new(body),
                loc: SourceLocation::default(),
            }));
        }

        if self.peek_token().value == "of" {
            self.advance_token();
            let right = self.parse_expression()?;
            self.expect_token(")")?;

            let body = if self.peek_token().value == "{" {
                self.parse_block_statement()?.unwrap()
            } else {
                self.parse_statement()?.unwrap_or(Statement::EmptyStatement {
                    loc: SourceLocation::default(),
                })
            };

            let left = match &init {
                Some(crate::ast::ForStatementInit::Variable(stmt)) => crate::ast::ForStatementLeft::Variable(stmt.clone()),
                Some(crate::ast::ForStatementInit::Expression(expr)) => {
                    if let Expression::Identifier(id) = expr {
                        crate::ast::ForStatementLeft::Pattern(crate::ast::Pattern::Identifier {
                            id: id.clone(),
                            init: None,
                        })
                    } else {
                        crate::ast::ForStatementLeft::Pattern(crate::ast::Pattern::Identifier {
                            id: Identifier {
                                name: "_".to_string(),
                                loc: SourceLocation::default(),
                            },
                            init: None,
                        })
                    }
                }
                None => crate::ast::ForStatementLeft::Pattern(crate::ast::Pattern::Identifier {
                    id: Identifier {
                        name: "_".to_string(),
                        loc: SourceLocation::default(),
                    },
                    init: None,
                }),
            };

            return Ok(Some(Statement::ForOfStatement {
                left,
                right,
                body: Box::new(body),
                loc: SourceLocation::default(),
            }));
        }

        self.expect_token(";")?;

        let test = if self.peek_token().value != ";" {
            Some(self.parse_expression()?)
        } else {
            None
        };
        self.expect_token(";")?;

        let update = if self.peek_token().value != ")" {
            Some(self.parse_expression()?)
        } else {
            None
        };
        self.expect_token(")")?;

        let body = if self.peek_token().value == "{" {
            self.parse_block_statement()?.unwrap()
        } else {
            self.parse_statement()?.unwrap_or(Statement::EmptyStatement {
                loc: SourceLocation::default(),
            })
        };

        Ok(Some(Statement::ForStatement {
            init,
            test,
            update,
            body: Box::new(body),
            loc: SourceLocation::default(),
        }))
    }

    fn parse_while_statement(&mut self) -> Result<Option<Statement>> {
        self.advance_token();
        self.expect_token("(")?;
        let test = self.parse_expression()?;
        self.expect_token(")")?;
        let body = if self.peek_token().value == "{" {
            self.parse_block_statement()?.unwrap()
        } else {
            self.parse_statement()?.unwrap_or(Statement::EmptyStatement {
                loc: SourceLocation::default(),
            })
        };
        Ok(Some(Statement::WhileStatement {
            test,
            body: Box::new(body),
            loc: SourceLocation::default(),
        }))
    }

    fn parse_do_while_statement(&mut self) -> Result<Option<Statement>> {
        self.advance_token();
        let body = if self.peek_token().value == "{" {
            self.parse_block_statement()?.unwrap()
        } else {
            self.parse_statement()?.unwrap_or(Statement::EmptyStatement {
                loc: SourceLocation::default(),
            })
        };
        self.expect_token("while")?;
        self.expect_token("(")?;
        let test = self.parse_expression()?;
        self.expect_token(")")?;
        if self.peek_token().value == ";" {
            self.advance_token();
        }
        Ok(Some(Statement::DoWhileStatement {
            body: Box::new(body),
            test,
            loc: SourceLocation::default(),
        }))
    }

    fn parse_switch_statement(&mut self) -> Result<Option<Statement>> {
        self.advance_token();
        self.expect_token("(")?;
        let discriminant = self.parse_expression()?;
        self.expect_token(")")?;
        self.expect_token("{")?;

        let mut cases = Vec::new();
        while self.peek_token().value != "}" && self.peek_token().kind != TokenKind::Eof {
            let test = if self.peek_token().value == "case" {
                self.advance_token();
                Some(self.parse_expression()?)
            } else {
                self.expect_token("default")?;
                None
            };
            self.expect_token(":")?;
            let mut consequent = Vec::new();
            while self.peek_token().value != "case" && self.peek_token().value != "default" && self.peek_token().value != "}" {
                if let Some(stmt) = self.parse_statement()? {
                    consequent.push(stmt);
                }
            }
            cases.push(SwitchCase {
                test,
                consequent,
                loc: SourceLocation::default(),
            });
        }
        self.expect_token("}")?;

        Ok(Some(Statement::SwitchStatement {
            discriminant,
            cases,
            loc: SourceLocation::default(),
        }))
    }

    fn parse_try_statement(&mut self) -> Result<Option<Statement>> {
        self.advance_token();
        let block = if self.peek_token().value == "{" {
            self.parse_block_statement()?.unwrap()
        } else {
            self.parse_statement()?.unwrap_or(Statement::EmptyStatement {
                loc: SourceLocation::default(),
            })
        };

        let handler = if self.peek_token().value == "catch" {
            self.advance_token();
            self.expect_token("(")?;
            let param = self.parse_pattern()?;
            self.expect_token(")")?;
            let body = if self.peek_token().value == "{" {
                self.parse_block_statement()?.unwrap()
            } else {
                self.parse_statement()?.unwrap_or(Statement::EmptyStatement {
                    loc: SourceLocation::default(),
                })
            };
            Some(Box::new(crate::ast::CatchClause {
                param,
                body: Box::new(body),
                loc: SourceLocation::default(),
            }))
        } else {
            None
        };

        let finalizer = if self.peek_token().value == "finally" {
            self.advance_token();
            Some(Box::new(if self.peek_token().value == "{" {
                self.parse_block_statement()?.unwrap()
            } else {
                self.parse_statement()?.unwrap_or(Statement::EmptyStatement {
                    loc: SourceLocation::default(),
                })
            }))
        } else {
            None
        };

        Ok(Some(Statement::TryStatement {
            block: Box::new(block),
            handler,
            finalizer,
            loc: SourceLocation::default(),
        }))
    }

    fn parse_return_statement(&mut self) -> Result<Option<Statement>> {
        self.advance_token();
        let argument = if self.peek_token().value == ";" || self.peek_token().value == "}" {
            None
        } else {
            Some(self.parse_expression()?)
        };
        if self.peek_token().value == ";" {
            self.advance_token();
        }
        Ok(Some(Statement::ReturnStatement {
            argument,
            loc: SourceLocation::default(),
        }))
    }

    fn parse_break_statement(&mut self) -> Result<Option<Statement>> {
        self.advance_token();
        let label = if self.peek_token().kind == TokenKind::Identifier {
            let token = self.advance_token();
            Some(Identifier {
                name: token.value,
                loc: SourceLocation::default(),
            })
        } else {
            None
        };
        if self.peek_token().value == ";" {
            self.advance_token();
        }
        Ok(Some(Statement::BreakStatement {
            label,
            loc: SourceLocation::default(),
        }))
    }

    fn parse_continue_statement(&mut self) -> Result<Option<Statement>> {
        self.advance_token();
        let label = if self.peek_token().kind == TokenKind::Identifier {
            let token = self.advance_token();
            Some(Identifier {
                name: token.value,
                loc: SourceLocation::default(),
            })
        } else {
            None
        };
        if self.peek_token().value == ";" {
            self.advance_token();
        }
        Ok(Some(Statement::ContinueStatement {
            label,
            loc: SourceLocation::default(),
        }))
    }

    fn parse_throw_statement(&mut self) -> Result<Option<Statement>> {
        self.advance_token();
        let argument = self.parse_conditional()?;
        if self.peek_token().value == ";" {
            self.advance_token();
        }
        Ok(Some(Statement::ThrowStatement {
            argument,
            loc: SourceLocation::default(),
        }))
    }

    fn parse_with_statement(&mut self) -> Result<Option<Statement>> {
        self.advance_token();
        self.expect_token("(")?;
        let object = self.parse_expression()?;
        self.expect_token(")")?;
        let body = self.parse_statement()?;
        Ok(Some(Statement::WithStatement {
            object,
            body: Box::new(body.unwrap_or(Statement::EmptyStatement {
                loc: SourceLocation::default(),
            })),
            loc: SourceLocation::default(),
        }))
    }

    fn parse_class_declaration(&mut self) -> Result<Option<Statement>> {
        self.advance_token();
        let id = if self.peek_token().kind == TokenKind::Identifier {
            let token = self.advance_token();
            Some(Identifier {
                name: token.value,
                loc: SourceLocation::default(),
            })
        } else {
            None
        };

        let super_class = if self.peek_token().value == "extends" {
            self.advance_token();
            let token = self.advance_token();
            Some(Box::new(Expression::Identifier(Identifier {
                name: token.value,
                loc: SourceLocation::default(),
            })))
        } else {
            None
        };

        self.expect_token("{")?;
        let body = self.parse_class_body()?;

        Ok(Some(Statement::ClassDeclaration {
            id: id.unwrap_or(Identifier {
                name: String::new(),
                loc: SourceLocation::default(),
            }),
            super_class,
            body: Box::new(crate::ast::ClassBody {
                body,
                loc: SourceLocation::default(),
            }),
            loc: SourceLocation::default(),
        }))
    }

    fn parse_class_body(&mut self) -> Result<Vec<crate::ast::ClassElement>> {
        let mut body = Vec::new();
        while self.peek_token().value != "}" && self.peek_token().kind != TokenKind::Eof {
            let method = self.parse_class_method()?;
            if let Some(method) = method {
                body.push(method);
            }
        }
        self.expect_token("}")?;
        Ok(body)
    }

    fn parse_class_method(&mut self) -> Result<Option<crate::ast::ClassElement>> {
        let key = self.parse_expression()?;
        let key_clone = key.clone();
        if self.peek_token().value == "(" {
            let mut params = Vec::new();
            self.expect_token("(")?;
            while self.peek_token().value != ")" {
                params.push(self.parse_pattern()?);
                if self.peek_token().value == "," {
                    self.advance_token();
                }
            }
            self.expect_token(")")?;
            self.expect_token("{")?;
            let method_body = self.parse_block_body()?;
            Ok(Some(crate::ast::ClassElement {
                key: Some(key),
                value: Some(crate::ast::MethodDefinition {
                    key: key_clone,
                    value: Box::new(crate::ast::FunctionExpression::FunctionExpression {
                        id: None,
                        params,
                        body: Box::new(crate::ast::FunctionBody {
                            body: method_body,
                            loc: SourceLocation::default(),
                        }),
                        generator: false,
                        async_: false,
                        loc: SourceLocation::default(),
                    }),
                    kind: crate::ast::ClassElementKind::Method,
                    computed: false,
                    static_: false,
                    loc: SourceLocation::default(),
                }),
                kind: crate::ast::ClassElementKind::Method,
                computed: false,
                static_: false,
                loc: SourceLocation::default(),
            }))
        } else {
            Ok(Some(crate::ast::ClassElement {
                key: Some(key),
                value: None,
                kind: crate::ast::ClassElementKind::Field,
                computed: false,
                static_: false,
                loc: SourceLocation::default(),
            }))
        }
    }

    fn parse_import_statement(&mut self) -> Result<Option<Statement>> {
        self.advance_token();
        let mut specifiers = Vec::new();

        if self.peek_token().kind == TokenKind::Identifier {
            let token = self.advance_token();
            if self.peek_token().value == "," {
                self.advance_token();
            }
            if self.peek_token().value == "{" {
                self.advance_token();
                while self.peek_token().value != "}" {
                    let imported = if self.peek_token().kind == TokenKind::Identifier {
                        let token = self.advance_token();
                        Identifier {
                            name: token.value,
                            loc: SourceLocation::default(),
                        }
                    } else {
                        return Err(Error::ParseError {
                            location: format!("line {}", self.get_line()),
                            message: "Expected identifier in import".to_string(),
                        });
                    };
                    let local = if self.peek_token().value == "as" {
                        self.advance_token();
                        let token = self.advance_token();
                        Identifier {
                            name: token.value,
                            loc: SourceLocation::default(),
                        }
                    } else {
                        imported.clone()
                    };
                    specifiers.push(crate::ast::ImportSpecifier {
                        imported,
                        local,
                        loc: SourceLocation::default(),
                    });
                    if self.peek_token().value == "," {
                        self.advance_token();
                    }
                }
                self.expect_token("}")?;
            }
        }

        self.expect_token("from")?;
        let source = if self.peek_token().kind == TokenKind::String {
            let token = self.advance_token();
            token.value
        } else {
            return Err(Error::ParseError {
                location: format!("line {}", self.get_line()),
                message: "Expected string after from".to_string(),
            });
        };
        if self.peek_token().value == ";" {
            self.advance_token();
        }

        Ok(Some(Statement::ImportDeclaration {
            specifiers,
            source,
            loc: SourceLocation::default(),
        }))
    }

    fn parse_export_statement(&mut self) -> Result<Option<Statement>> {
        self.advance_token();

        if self.peek_token().value == "default" {
            self.advance_token();
            let declaration = self.parse_statement()?;
            return Ok(Some(Statement::ExportDefaultDeclaration {
                declaration: Box::new(declaration.unwrap_or(Statement::EmptyStatement {
                    loc: SourceLocation::default(),
                })),
                loc: SourceLocation::default(),
            }));
        }

        if self.peek_token().value == "*" {
            self.advance_token();
            self.expect_token("from")?;
            let source = if self.peek_token().kind == TokenKind::String {
                let token = self.advance_token();
                token.value
            } else {
                return Err(Error::ParseError {
                    location: format!("line {}", self.get_line()),
                    message: "Expected string after from".to_string(),
                });
            };
            if self.peek_token().value == ";" {
                self.advance_token();
            }
            return Ok(Some(Statement::ExportAllDeclaration {
                source,
                loc: SourceLocation::default(),
            }));
        }

        if self.peek_token().value == "{" {
            self.advance_token();
            let mut specifiers = Vec::new();
            while self.peek_token().value != "}" {
                let local = if self.peek_token().kind == TokenKind::Identifier {
                    let token = self.advance_token();
                    Identifier {
                        name: token.value,
                        loc: SourceLocation::default(),
                    }
                } else {
                    return Err(Error::ParseError {
                        location: format!("line {}", self.get_line()),
                        message: "Expected identifier in export".to_string(),
                    });
                };
                let exported = if self.peek_token().value == "as" {
                    self.advance_token();
                    let token = self.advance_token();
                    Some(Identifier {
                        name: token.value,
                        loc: SourceLocation::default(),
                    })
                } else {
                    None
                };
                specifiers.push(crate::ast::ExportSpecifier {
                    local,
                    exported,
                    loc: SourceLocation::default(),
                });
                if self.peek_token().value == "," {
                    self.advance_token();
                }
            }
            self.expect_token("}")?;
            if self.peek_token().value == ";" {
                self.advance_token();
            }
            return Ok(Some(Statement::ExportNamedDeclaration {
                declaration: None,
                specifiers,
                source: None,
                loc: SourceLocation::default(),
            }));
        }

        let declaration = self.parse_statement()?;
        Ok(Some(Statement::ExportNamedDeclaration {
            declaration: Some(Box::new(declaration.unwrap_or(Statement::EmptyStatement {
                loc: SourceLocation::default(),
            }))),
            specifiers: Vec::new(),
            source: None,
            loc: SourceLocation::default(),
        }))
    }

    fn parse_block_or_object_statement(&mut self) -> Result<Option<Statement>> {
        self.advance_token();
        let next_token = self.peek_token();
        if next_token.value == "}" {
            self.advance_token();
            Ok(Some(Statement::ExpressionStatement {
                expression: Expression::ObjectExpression {
                    properties: Vec::new(),
                    loc: SourceLocation::default(),
                },
                loc: SourceLocation::default(),
            }))
        } else {
            let lookahead = self.peek_next_token();
            if (next_token.kind == TokenKind::Identifier || next_token.kind == TokenKind::Keyword || 
                next_token.kind == TokenKind::String || next_token.kind == TokenKind::Number) && 
               lookahead.value == ":" {
                let mut properties = Vec::new();
                while self.peek_token().value != "}" {
                    if self.peek_token().value == "," {
                        self.advance_token();
                        continue;
                    }
                    if self.peek_token().value == "..." {
                        self.advance_token();
                        let arg = self.parse_conditional()?;
                        let key = Expression::SpreadElement {
                            argument: Box::new(arg),
                            loc: SourceLocation::default(),
                        };
                        properties.push(crate::ast::ObjectProperty {
                            key: key.clone(),
                            value: key,
                            kind: crate::ast::PropertyKind::Init,
                            method: false,
                            shorthand: false,
                            computed: false,
                            loc: SourceLocation::default(),
                        });
                    } else {
                        let key = self.parse_object_key()?;
                        let value = if self.peek_token().value == ":" {
                            self.advance_token();
                            self.parse_object_value()?
                        } else {
                            key.clone()
                        };
                        properties.push(crate::ast::ObjectProperty {
                            key,
                            value,
                            kind: crate::ast::PropertyKind::Init,
                            method: false,
                            shorthand: false,
                            computed: false,
                            loc: SourceLocation::default(),
                        });
                    }
                    if self.peek_token().value == "," {
                        self.advance_token();
                    }
                }
                self.expect_token("}")?;
                Ok(Some(Statement::ExpressionStatement {
                    expression: Expression::ObjectExpression {
                        properties,
                        loc: SourceLocation::default(),
                    },
                    loc: SourceLocation::default(),
                }))
            } else {
                let body = self.parse_block_body_with_brace()?;
                Ok(Some(Statement::BlockStatement {
                    body,
                    loc: SourceLocation::default(),
                }))
            }
        }
    }

    fn parse_block_statement(&mut self) -> Result<Option<Statement>> {
        self.advance_token();
        let body = self.parse_block_body_with_brace()?;
        Ok(Some(Statement::BlockStatement {
            body,
            loc: SourceLocation::default(),
        }))
    }

    fn parse_block_body_with_brace(&mut self) -> Result<Vec<Statement>> {
        let mut body = Vec::new();
        while self.peek_token().value != "}" && self.peek_token().kind != TokenKind::Eof {
            if let Some(stmt) = self.parse_statement()? {
                body.push(stmt);
            }
        }
        self.expect_token("}")?;
        Ok(body)
    }

    fn parse_expression_statement_or_labeled(&mut self) -> Result<Option<Statement>> {
        let expr = self.parse_expression()?;
        if self.peek_token().value == ":" && matches!(&expr, Expression::Identifier(_)) {
            self.advance_token();
            let body = self.parse_statement()?;
            return Ok(Some(Statement::LabeledStatement {
                label: if let Expression::Identifier(id) = expr {
                    id
                } else {
                    Identifier {
                        name: String::new(),
                        loc: SourceLocation::default(),
                    }
                },
                body: Box::new(body.unwrap_or(Statement::EmptyStatement {
                    loc: SourceLocation::default(),
                })),
                loc: SourceLocation::default(),
            }));
        }
        if self.peek_token().value == ";" {
            self.advance_token();
        }
        Ok(Some(Statement::ExpressionStatement {
            expression: expr,
            loc: SourceLocation::default(),
        }))
    }

    fn parse_expression(&mut self) -> Result<Expression> {
        self.parse_assignment()
    }

    fn parse_assignment(&mut self) -> Result<Expression> {
        let left = self.parse_sequence()?;
        if self.peek_token().value == "=" || self.peek_token().value == "+=" ||
           self.peek_token().value == "-=" || self.peek_token().value == "*=" ||
           self.peek_token().value == "/=" || self.peek_token().value == "%=" ||
           self.peek_token().value == "&=" || self.peek_token().value == "|=" ||
           self.peek_token().value == "^=" || self.peek_token().value == "<<=" ||
           self.peek_token().value == ">>=" || self.peek_token().value == ">>>=" {
            let op = self.advance_token();
            let right = self.parse_assignment()?;
            let operator = match op.value.as_str() {
                "=" => crate::ast::AssignmentOperator::Simple,
                "+=" => crate::ast::AssignmentOperator::Plus,
                "-=" => crate::ast::AssignmentOperator::Minus,
                "*=" => crate::ast::AssignmentOperator::Multiply,
                "/=" => crate::ast::AssignmentOperator::Divide,
                "%=" => crate::ast::AssignmentOperator::Modulo,
                "&=" => crate::ast::AssignmentOperator::BitAnd,
                "|=" => crate::ast::AssignmentOperator::BitOr,
                "^=" => crate::ast::AssignmentOperator::BitXor,
                "<<=" => crate::ast::AssignmentOperator::ShiftLeft,
                ">>=" => crate::ast::AssignmentOperator::ShiftRight,
                ">>>=" => crate::ast::AssignmentOperator::ShiftRightUnsigned,
                _ => crate::ast::AssignmentOperator::Simple,
            };
            return Ok(Expression::AssignmentExpression {
                operator,
                left: Box::new(left),
                right: Box::new(right),
                loc: SourceLocation::default(),
            });
        }
        Ok(left)
    }

    fn parse_sequence(&mut self) -> Result<Expression> {
        let mut expressions = vec![self.parse_conditional()?];
        while self.peek_token().value == "," {
            self.advance_token();
            expressions.push(self.parse_conditional()?);
        }
        if expressions.len() == 1 {
            Ok(expressions.pop().unwrap())
        } else {
            Ok(Expression::SequenceExpression {
                expressions,
                loc: SourceLocation::default(),
            })
        }
    }

    fn parse_conditional(&mut self) -> Result<Expression> {
        let test = self.parse_null_coalescing()?;
        if self.peek_token().value == "?" {
            self.advance_token();
            let consequent = self.parse_assignment()?;
            self.expect_token(":")?;
            let alternate = self.parse_assignment()?;
            return Ok(Expression::ConditionalExpression {
                test: Box::new(test),
                consequent: Box::new(consequent),
                alternate: Box::new(alternate),
                loc: SourceLocation::default(),
            });
        }
        Ok(test)
    }

    fn parse_null_coalescing(&mut self) -> Result<Expression> {
        let mut left = self.parse_logical_or()?;
        while self.peek_token().value == "??" {
            self.advance_token();
            let right = self.parse_logical_or()?;
            left = Expression::NullishCoalescingExpression {
                left: Box::new(left),
                right: Box::new(right),
                loc: SourceLocation::default(),
            };
        }
        Ok(left)
    }

    fn parse_logical_or(&mut self) -> Result<Expression> {
        let mut left = self.parse_logical_and()?;
        while self.peek_token().value == "||" {
            self.advance_token();
            let right = self.parse_logical_and()?;
            left = Expression::LogicalExpression {
                operator: crate::ast::LogicalOperator::Or,
                left: Box::new(left),
                right: Box::new(right),
                loc: SourceLocation::default(),
            };
        }
        Ok(left)
    }

    fn parse_logical_and(&mut self) -> Result<Expression> {
        let mut left = self.parse_bitwise_or()?;
        while self.peek_token().value == "&&" {
            self.advance_token();
            let right = self.parse_bitwise_or()?;
            left = Expression::LogicalExpression {
                operator: crate::ast::LogicalOperator::And,
                left: Box::new(left),
                right: Box::new(right),
                loc: SourceLocation::default(),
            };
        }
        Ok(left)
    }

    fn parse_bitwise_or(&mut self) -> Result<Expression> {
        let mut left = self.parse_bitwise_xor()?;
        loop {
            let op = match self.peek_token().value.as_str() {
                "|" => {
                    self.advance_token();
                    crate::ast::BinaryOperator::BitOr
                }
                _ => break,
            };
            let right = self.parse_bitwise_xor()?;
            left = Expression::BinaryExpression {
                operator: op,
                left: Box::new(left),
                right: Box::new(right),
                loc: SourceLocation::default(),
            };
        }
        Ok(left)
    }

    fn parse_bitwise_xor(&mut self) -> Result<Expression> {
        let mut left = self.parse_bitwise_and()?;
        loop {
            let op = match self.peek_token().value.as_str() {
                "^" => {
                    self.advance_token();
                    crate::ast::BinaryOperator::BitXor
                }
                _ => break,
            };
            let right = self.parse_bitwise_and()?;
            left = Expression::BinaryExpression {
                operator: op,
                left: Box::new(left),
                right: Box::new(right),
                loc: SourceLocation::default(),
            };
        }
        Ok(left)
    }

    fn parse_bitwise_and(&mut self) -> Result<Expression> {
        let mut left = self.parse_equality()?;
        loop {
            let op = match self.peek_token().value.as_str() {
                "&" => {
                    self.advance_token();
                    crate::ast::BinaryOperator::BitAnd
                }
                _ => break,
            };
            let right = self.parse_equality()?;
            left = Expression::BinaryExpression {
                operator: op,
                left: Box::new(left),
                right: Box::new(right),
                loc: SourceLocation::default(),
            };
        }
        Ok(left)
    }

    fn parse_equality(&mut self) -> Result<Expression> {
        let mut left = self.parse_relational()?;
        loop {
            let op = match self.peek_token().value.as_str() {
                "===" => {
                    self.advance_token();
                    crate::ast::BinaryOperator::StrictEqual
                }
                "!==" => {
                    self.advance_token();
                    crate::ast::BinaryOperator::StrictNotEqual
                }
                "==" => {
                    self.advance_token();
                    crate::ast::BinaryOperator::Equal
                }
                "!=" => {
                    self.advance_token();
                    crate::ast::BinaryOperator::NotEqual
                }
                _ => break,
            };
            let right = self.parse_relational()?;
            left = Expression::BinaryExpression {
                operator: op,
                left: Box::new(left),
                right: Box::new(right),
                loc: SourceLocation::default(),
            };
        }
        Ok(left)
    }

    fn parse_relational(&mut self) -> Result<Expression> {
        let mut left = self.parse_shift()?;
        loop {
            let op = match self.peek_token().value.as_str() {
                "<" => {
                    self.advance_token();
                    crate::ast::BinaryOperator::LessThan
                }
                "<=" => {
                    self.advance_token();
                    crate::ast::BinaryOperator::LessThanEqual
                }
                ">" => {
                    self.advance_token();
                    crate::ast::BinaryOperator::GreaterThan
                }
                ">=" => {
                    self.advance_token();
                    crate::ast::BinaryOperator::GreaterThanEqual
                }
                "instanceof" => {
                    self.advance_token();
                    crate::ast::BinaryOperator::InstanceOf
                }
                "in" => {
                    self.advance_token();
                    crate::ast::BinaryOperator::In
                }
                _ => break,
            };
            let right = self.parse_shift()?;
            left = Expression::BinaryExpression {
                operator: op,
                left: Box::new(left),
                right: Box::new(right),
                loc: SourceLocation::default(),
            };
        }
        Ok(left)
    }

    fn parse_shift(&mut self) -> Result<Expression> {
        self.parse_additive()
    }

    fn parse_additive(&mut self) -> Result<Expression> {
        let mut left = self.parse_multiplicative()?;
        loop {
            let op = match self.peek_token().value.as_str() {
                "+" => {
                    self.advance_token();
                    crate::ast::BinaryOperator::Add
                }
                "-" => {
                    self.advance_token();
                    crate::ast::BinaryOperator::Subtract
                }
                _ => break,
            };
            let right = self.parse_multiplicative()?;
            left = Expression::BinaryExpression {
                operator: op,
                left: Box::new(left),
                right: Box::new(right),
                loc: SourceLocation::default(),
            };
        }
        Ok(left)
    }

    fn parse_multiplicative(&mut self) -> Result<Expression> {
        let mut left = self.parse_unary()?;
        loop {
            let op = match self.peek_token().value.as_str() {
                "*" => {
                    self.advance_token();
                    crate::ast::BinaryOperator::Multiply
                }
                "/" => {
                    self.advance_token();
                    crate::ast::BinaryOperator::Divide
                }
                "%" => {
                    self.advance_token();
                    crate::ast::BinaryOperator::Modulo
                }
                _ => break,
            };
            let right = self.parse_unary()?;
            left = Expression::BinaryExpression {
                operator: op,
                left: Box::new(left),
                right: Box::new(right),
                loc: SourceLocation::default(),
            };
        }
        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Expression> {
        let value = self.peek_token().value.clone();
        match value.as_str() {
            "new" => {
                self.advance_token();
                let callee = self.parse_call()?;
                let arguments = if self.peek_token().value == "(" {
                    self.advance_token();
                    let mut args = Vec::new();
                    while self.peek_token().value != ")" {
                        args.push(self.parse_assignment()?);
                        if self.peek_token().value == "," {
                            self.advance_token();
                        }
                    }
                    self.expect_token(")")?;
                    args
                } else {
                    Vec::new()
                };
                Ok(Expression::NewExpression {
                    callee: Box::new(callee),
                    arguments,
                    loc: SourceLocation::default(),
                })
            }
            "await" => {
                self.advance_token();
                let arg = self.parse_unary()?;
                Ok(Expression::AwaitExpression {
                    argument: Box::new(arg),
                    loc: SourceLocation::default(),
                })
            }
            "yield" => {
                self.advance_token();
                let delegate = self.peek_token().value == "*";
                if delegate {
                    self.advance_token();
                }
                let argument = if self.peek_token().value != ";" && 
                    self.peek_token().value != "}" && 
                    self.peek_token().value != "," &&
                    self.peek_token().kind != TokenKind::Eof {
                    Some(Box::new(self.parse_assignment()?))
                } else {
                    None
                };
                Ok(Expression::YieldExpression {
                    argument,
                    delegate,
                    loc: SourceLocation::default(),
                })
            }
            "!" | "-" | "+" | "~" | "typeof" | "void" | "delete" => {
                self.advance_token();
                let arg = self.parse_unary()?;
                let operator = match value.as_str() {
                    "!" => crate::ast::UnaryOperator::Not,
                    "-" => crate::ast::UnaryOperator::Minus,
                    "+" => crate::ast::UnaryOperator::Plus,
                    "~" => crate::ast::UnaryOperator::BitNot,
                    "typeof" => crate::ast::UnaryOperator::TypeOf,
                    "void" => crate::ast::UnaryOperator::Void,
                    "delete" => crate::ast::UnaryOperator::Delete,
                    _ => crate::ast::UnaryOperator::Void,
                };
                Ok(Expression::UnaryExpression {
                    operator,
                    argument: Box::new(arg),
                    prefix: true,
                    loc: SourceLocation::default(),
                })
            }
            "++" | "--" => {
                self.advance_token();
                let arg = self.parse_unary()?;
                let operator = if value == "++" {
                    crate::ast::UpdateOperator::Increment
                } else {
                    crate::ast::UpdateOperator::Decrement
                };
                Ok(Expression::UpdateExpression {
                    operator,
                    argument: Box::new(arg),
                    prefix: true,
                    loc: SourceLocation::default(),
                })
            }
            _ => self.parse_update(),
        }
    }

    fn parse_update(&mut self) -> Result<Expression> {
        let arg = self.parse_call()?;
        if self.peek_token().value == "++" || self.peek_token().value == "--" {
            let operator = if self.peek_token().value == "++" {
                self.advance_token();
                crate::ast::UpdateOperator::Increment
            } else {
                self.advance_token();
                crate::ast::UpdateOperator::Decrement
            };
            Ok(Expression::UpdateExpression {
                operator,
                argument: Box::new(arg),
                prefix: false,
                loc: SourceLocation::default(),
            })
        } else {
            Ok(arg)
        }
    }

    fn parse_call(&mut self) -> Result<Expression> {
        let mut expr = self.parse_primary()?;
        loop {
            if self.peek_token().value == "(" {
                self.advance_token();
                let mut arguments = Vec::new();
                while self.peek_token().value != ")" {
                    arguments.push(self.parse_assignment()?);
                    if self.peek_token().value == "," {
                        self.advance_token();
                    }
                }
                self.expect_token(")")?;
                expr = Expression::CallExpression {
                    callee: Box::new(expr),
                    arguments,
                    loc: SourceLocation::default(),
                };
            } else if self.peek_token().value == "." {
                self.advance_token();
                let token = self.advance_token();
                expr = Expression::MemberExpression {
                    object: Box::new(expr),
                    property: Box::new(Expression::Identifier(Identifier {
                        name: token.value,
                        loc: SourceLocation::default(),
                    })),
                    computed: false,
                    loc: SourceLocation::default(),
                };
            } else if self.peek_token().value == "[" {
                self.advance_token();
                let property = self.parse_expression()?;
                self.expect_token("]")?;
                expr = Expression::MemberExpression {
                    object: Box::new(expr),
                    property: Box::new(property),
                    computed: true,
                    loc: SourceLocation::default(),
                };
            } else if self.peek_token().value == "?" && self.peek_next_token().value == "." {
                self.advance_token();
                self.advance_token();
                let token = self.advance_token();
                expr = Expression::OptionalMemberExpression {
                    object: Box::new(expr),
                    property: Box::new(Expression::Identifier(Identifier {
                        name: token.value,
                        loc: SourceLocation::default(),
                    })),
                    computed: false,
                    loc: SourceLocation::default(),
                };
            } else if self.peek_token().value == "?" && self.peek_next_token().value == "[" {
                self.advance_token();
                self.advance_token();
                let property = self.parse_expression()?;
                self.expect_token("]")?;
                expr = Expression::OptionalMemberExpression {
                    object: Box::new(expr),
                    property: Box::new(property),
                    computed: true,
                    loc: SourceLocation::default(),
                };
            } else if self.peek_token().value == "?" && self.peek_next_token().value == "(" {
                self.advance_token();
                self.advance_token();
                let mut arguments = Vec::new();
                while self.peek_token().value != ")" {
                    arguments.push(self.parse_assignment()?);
                    if self.peek_token().value == "," {
                        self.advance_token();
                    }
                }
                self.expect_token(")")?;
                expr = Expression::OptionalCallExpression {
                    callee: Box::new(expr),
                    arguments,
                    loc: SourceLocation::default(),
                };
            } else if self.peek_token().value == "?" {
                self.advance_token();
                let consequent = self.parse_call()?;
                self.expect_token(":")?;
                let alternate = self.parse_assignment()?;
                expr = Expression::ConditionalExpression {
                    test: Box::new(expr),
                    consequent: Box::new(consequent),
                    alternate: Box::new(alternate),
                    loc: SourceLocation::default(),
                };
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Expression> {
        let token = self.peek_token();

        if token.kind == TokenKind::Identifier || token.kind == TokenKind::Keyword {
            let token = self.advance_token();
            if token.value == "function" {
                return self.parse_function_expression();
            }
            if token.value == "class" {
                return self.parse_class_expression();
            }
            if token.value == "this" {
                return Ok(Expression::This {
                    loc: SourceLocation::default(),
                });
            }
            if token.value == "super" {
                return Ok(Expression::Super {
                    loc: SourceLocation::default(),
                });
            }
            if token.value == "import" {
                return Ok(Expression::Import {
                    loc: SourceLocation::default(),
                });
            }
            let next_token = self.peek_token();
            if next_token.value == "=>" {
                return self.parse_arrow_function_from_id(token.value);
            }
            return Ok(Expression::Identifier(Identifier {
                name: token.value,
                loc: SourceLocation::default(),
            }));
        }

        match token.value.as_str() {
            "true" => {
                self.advance_token();
                return Ok(Expression::Literal {
                    value: LiteralValue::Boolean(true),
                    raw: "true".to_string(),
                    loc: SourceLocation::default(),
                });
            }
            "false" => {
                self.advance_token();
                return Ok(Expression::Literal {
                    value: LiteralValue::Boolean(false),
                    raw: "false".to_string(),
                    loc: SourceLocation::default(),
                });
            }
            "null" => {
                self.advance_token();
                return Ok(Expression::Literal {
                    value: LiteralValue::Null,
                    raw: "null".to_string(),
                    loc: SourceLocation::default(),
                });
            }
            "undefined" => {
                self.advance_token();
                return Ok(Expression::Literal {
                    value: LiteralValue::Undefined,
                    raw: "undefined".to_string(),
                    loc: SourceLocation::default(),
                });
            }
            "Infinity" => {
                self.advance_token();
                return Ok(Expression::Literal {
                    value: LiteralValue::Number(f64::INFINITY),
                    raw: "Infinity".to_string(),
                    loc: SourceLocation::default(),
                });
            }
            "NaN" => {
                self.advance_token();
                return Ok(Expression::Literal {
                    value: LiteralValue::Number(f64::NAN),
                    raw: "NaN".to_string(),
                    loc: SourceLocation::default(),
                });
            }
            _ => {}
        }

        if token.kind == TokenKind::String {
            let token = self.advance_token();
            return Ok(Expression::Literal {
                value: LiteralValue::String(token.value.clone()),
                raw: token.value,
                loc: SourceLocation::default(),
            });
        }

        if token.kind == TokenKind::Number {
            let token = self.advance_token();
            let value = if token.value.starts_with("0x") || token.value.starts_with("0X") {
                u64::from_str_radix(&token.value[2..], 16).unwrap_or(0) as f64
            } else {
                token.value.parse::<f64>().unwrap_or(0.0)
            };
            return Ok(Expression::Literal {
                value: LiteralValue::Number(value),
                raw: token.value,
                loc: SourceLocation::default(),
            });
        }

        if token.value == "[" {
            return self.parse_array();
        }

        if token.value == "{" {
            return self.parse_object();
        }

        if token.value == "(" {
            self.advance_token();
            let expr = self.parse_expression()?;
            self.expect_token(")")?;
            if self.peek_token().value == "=>" {
                return self.parse_arrow_function_from_expr(expr);
            }
            return Ok(expr);
        }

        if token.value == "`" {
            return self.parse_template_literal();
        }

        Err(Error::ParseError {
            location: format!("line {}", self.get_line()),
            message: format!("Unexpected token: {}", token.value),
        })
    }

    fn parse_function_expression(&mut self) -> Result<Expression> {
        self.advance_token();
        let id = if self.peek_token().kind == TokenKind::Identifier {
            let token = self.advance_token();
            Some(Identifier {
                name: token.value,
                loc: SourceLocation::default(),
            })
        } else {
            None
        };
        self.expect_token("(")?;
        let mut params = Vec::new();
        while self.peek_token().value != ")" {
            params.push(self.parse_pattern()?);
            if self.peek_token().value == "," {
                self.advance_token();
            }
        }
        self.expect_token(")")?;
        self.expect_token("{")?;
        let body = self.parse_block_body()?;
        Ok(Expression::FunctionExpression {
            id,
            params,
            body: Box::new(crate::ast::FunctionBody {
                body,
                loc: SourceLocation::default(),
            }),
            loc: SourceLocation::default(),
        })
    }

    fn parse_class_expression(&mut self) -> Result<Expression> {
        self.advance_token();
        let id = if self.peek_token().kind == TokenKind::Identifier {
            let token = self.advance_token();
            Some(Identifier {
                name: token.value,
                loc: SourceLocation::default(),
            })
        } else {
            None
        };

        let super_class = if self.peek_token().value == "extends" {
            self.advance_token();
            let token = self.advance_token();
            Some(Box::new(Expression::Identifier(Identifier {
                name: token.value,
                loc: SourceLocation::default(),
            })))
        } else {
            None
        };

        self.expect_token("{")?;
        let body = self.parse_class_body()?;

        Ok(Expression::ClassExpression {
            id,
            super_class,
            body: Box::new(crate::ast::ClassBody {
                body,
                loc: SourceLocation::default(),
            }),
            loc: SourceLocation::default(),
        })
    }

    fn parse_array(&mut self) -> Result<Expression> {
        self.advance_token();
        let mut elements = Vec::new();
        while self.peek_token().value != "]" {
            if self.peek_token().value == "," {
                elements.push(None);
                self.advance_token();
            } else if self.peek_token().value == "..." {
                self.advance_token();
                let arg = self.parse_assignment()?;
                elements.push(Some(Expression::SpreadElement {
                    argument: Box::new(arg),
                    loc: SourceLocation::default(),
                }));
                if self.peek_token().value == "," {
                    self.advance_token();
                }
            } else {
                elements.push(Some(self.parse_assignment()?));
                if self.peek_token().value == "," {
                    self.advance_token();
                }
            }
        }
        self.expect_token("]")?;
        Ok(Expression::ArrayExpression {
            elements,
            loc: SourceLocation::default(),
        })
    }

    fn parse_object(&mut self) -> Result<Expression> {
        self.advance_token();
        let mut properties = Vec::new();
        while self.peek_token().value != "}" {
            if self.peek_token().value == "," {
                self.advance_token();
                continue;
            }
            if self.peek_token().value == "..." {
                self.advance_token();
                let arg = self.parse_assignment()?;
                let key = Expression::SpreadElement {
                    argument: Box::new(arg),
                    loc: SourceLocation::default(),
                };
                properties.push(crate::ast::ObjectProperty {
                    key: key.clone(),
                    value: key,
                    kind: crate::ast::PropertyKind::Init,
                    method: false,
                    shorthand: false,
                    computed: false,
                    loc: SourceLocation::default(),
                });
            } else {
                let key = self.parse_object_key()?;
                let value = if self.peek_token().value == ":" {
                    self.advance_token();
                    self.parse_object_value()?
                } else {
                    key.clone()
                };
                properties.push(crate::ast::ObjectProperty {
                    key,
                    value,
                    kind: crate::ast::PropertyKind::Init,
                    method: false,
                    shorthand: false,
                    computed: false,
                    loc: SourceLocation::default(),
                });
            }
            if self.peek_token().value == "," {
                self.advance_token();
            }
        }
        self.expect_token("}")?;
        Ok(Expression::ObjectExpression {
            properties,
            loc: SourceLocation::default(),
        })
    }

    fn parse_object_value(&mut self) -> Result<Expression> {
        let expr = self.parse_conditional()?;
        if self.peek_token().value == "," || self.peek_token().value == "}" {
            Ok(expr)
        } else {
            Err(Error::ParseError {
                location: format!("line {}", self.get_line()),
                message: format!("Unexpected token in object value: {}", self.peek_token().value),
            })
        }
    }

    fn parse_object_key(&mut self) -> Result<Expression> {
        let token = self.peek_token();
        if token.kind == TokenKind::Identifier || token.kind == TokenKind::Keyword {
            let token = self.advance_token();
            Ok(Expression::Identifier(Identifier {
                name: token.value,
                loc: SourceLocation::default(),
            }))
        } else if token.kind == TokenKind::String {
            let token = self.advance_token();
            Ok(Expression::Literal {
                value: LiteralValue::String(token.value.clone()),
                raw: token.value,
                loc: SourceLocation::default(),
            })
        } else if token.kind == TokenKind::Number {
            let token = self.advance_token();
            let value: f64 = token.value.parse().unwrap_or(0.0);
            Ok(Expression::Literal {
                value: LiteralValue::Number(value),
                raw: token.value,
                loc: SourceLocation::default(),
            })
        } else {
            Err(Error::ParseError {
                location: format!("line {}", self.get_line()),
                message: format!("Unexpected token in object key: {}", token.value),
            })
        }
    }

    fn parse_template_literal(&mut self) -> Result<Expression> {
        self.advance_token();
        let mut quasis = Vec::new();
        let mut expressions = Vec::new();
        let mut raw = String::new();

        while self.peek_token().value != "`" {
            if self.peek_token().value == "${" {
                quasis.push(crate::ast::TemplateElement {
                    value: crate::ast::TemplateElementValue {
                        raw: raw.clone(),
                        cooked: raw.clone(),
                    },
                    tail: false,
                    loc: SourceLocation::default(),
                });
                self.expect_token("${")?;
                expressions.push(self.parse_expression()?);
                self.expect_token("}")?;
                raw = String::new();
            } else {
                let token = self.advance_token();
                raw.push_str(&token.value);
            }
        }
        self.expect_token("`")?;
        quasis.push(crate::ast::TemplateElement {
            value: crate::ast::TemplateElementValue {
                raw: raw.clone(),
                cooked: raw.clone(),
            },
            tail: true,
            loc: SourceLocation::default(),
        });

        Ok(Expression::TemplateLiteral {
            quasis,
            expressions,
            loc: SourceLocation::default(),
        })
    }

    fn parse_arrow_function_from_id(&mut self, name: String) -> Result<Expression> {
        self.expect_token("=>")?;
        let params = vec![crate::ast::Pattern::Identifier {
            id: Identifier {
                name,
                loc: SourceLocation::default(),
            },
            init: None,
        }];
        self.parse_arrow_function_body(params)
    }

    fn parse_arrow_function_from_expr(&mut self, expr: Expression) -> Result<Expression> {
        self.expect_token("=>")?;
        let params = match expr {
            Expression::SequenceExpression { expressions, .. } => expressions
                .into_iter()
                .map(|e| match e {
                    Expression::Identifier(id) => Ok(crate::ast::Pattern::Identifier {
                        id,
                        init: None,
                    }),
                    _ => Err(Error::ParseError {
                        location: format!("line {}", self.get_line()),
                        message: "Expected identifier in arrow function params".to_string(),
                    }),
                })
                .collect::<Result<Vec<_>>>()?,
            Expression::Identifier(id) => vec![crate::ast::Pattern::Identifier {
                id,
                init: None,
            }],
            _ => vec![],
        };
        self.parse_arrow_function_body(params)
    }

    fn parse_arrow_function_body(&mut self, params: Vec<crate::ast::Pattern>) -> Result<Expression> {
        if self.peek_token().value == "{" {
            self.advance_token();
            let body = self.parse_block_body()?;
            Ok(Expression::ArrowFunctionExpression {
                params,
                body: Box::new(crate::ast::ArrowFunctionBody::BlockFunctionBody(
                    crate::ast::FunctionBody {
                        body,
                        loc: SourceLocation::default(),
                    },
                )),
                loc: SourceLocation::default(),
            })
        } else {
            let body = self.parse_assignment()?;
            Ok(Expression::ArrowFunctionExpression {
                params,
                body: Box::new(crate::ast::ArrowFunctionBody::Expression(body)),
                loc: SourceLocation::default(),
            })
        }
    }
}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}
