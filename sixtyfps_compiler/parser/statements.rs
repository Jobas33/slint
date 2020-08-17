/* LICENSE BEGIN

    This file is part of the Sixty FPS Project

    Copyright (c) 2020 Olivier Goffart <olivier.goffart@sixtyfps.io>
    Copyright (c) 2020 Simon Hausmann <simon.hausmann@sixtyfps.io>

    SPDX-License-Identifier: GPL-3.0-only

LICENSE END */
use super::document::parse_code_block;
use super::expressions::parse_expression;
use super::prelude::*;

#[cfg_attr(test, parser_test)]
/// ```test
/// expression
/// expression += expression
/// expression.expression *= 45.2
/// expression = "hello"
/// if (true) { foo = bar; } else { bar = foo;  }
/// ```
pub fn parse_statement(p: &mut impl Parser) -> bool {
    if p.nth(0) == SyntaxKind::RBrace {
        return false;
    }
    if p.test(SyntaxKind::Semicolon) {
        return true;
    }
    let checkpoint = p.checkpoint();

    if p.peek().as_str() == "if" && p.nth(1) == SyntaxKind::LParent {
        let mut p = p.start_node(SyntaxKind::Expression);
        parse_if_statement(&mut *p);
        return true;
    }

    parse_expression(p);
    if matches!(
        p.nth(0),
        SyntaxKind::MinusEqual
            | SyntaxKind::PlusEqual
            | SyntaxKind::StarEqual
            | SyntaxKind::DivEqual
            | SyntaxKind::Equal
    ) {
        let mut p = p.start_node_at(checkpoint.clone(), SyntaxKind::Expression);
        let mut p = p.start_node_at(checkpoint, SyntaxKind::SelfAssignment);
        p.consume();
        parse_expression(&mut *p);
    }
    p.test(SyntaxKind::Semicolon)
}

#[cfg_attr(test, parser_test)]
/// ```test,ConditionalExpression
/// if (true) { foo = bar; } else { bar = foo;  }
/// if (true) { foo += bar; }
/// if (true) { } else { ; }
/// ```
fn parse_if_statement(p: &mut impl Parser) {
    let mut p = p.start_node(SyntaxKind::ConditionalExpression);
    debug_assert_eq!(p.peek().as_str(), "if");
    p.expect(SyntaxKind::Identifier);
    p.expect(SyntaxKind::LParent);
    parse_expression(&mut *p);
    p.expect(SyntaxKind::RParent);
    {
        let mut p = p.start_node(SyntaxKind::Expression);
        parse_code_block(&mut *p);
    }
    if p.peek().as_str() == "else" {
        p.expect(SyntaxKind::Identifier);
        let mut p = p.start_node(SyntaxKind::Expression);
        parse_code_block(&mut *p);
    } else {
        // We need an expression so fake an empty block.
        // FIXME: this shouldn't be needed
        let mut p = p.start_node(SyntaxKind::Expression);
        let _ = p.start_node(SyntaxKind::CodeBlock);
    }
}
