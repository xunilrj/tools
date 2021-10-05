use crate::format_token::{GroupToken, LineToken};
use crate::{format_token::FormatToken, format_tokens};
use rslint_parser::ast::{ArrayExpr, GroupingExpr, Literal, ObjectExpr, ObjectProp, UnaryExpr};
use rslint_parser::{parse_text, AstNode, SyntaxKind, SyntaxNode, SyntaxToken};

fn tokenize_token(token: SyntaxToken) -> FormatToken {
	match token.kind() {
		SyntaxKind::NULL_KW => FormatToken::string("null"),
		SyntaxKind::TRUE_KW => FormatToken::string("true"),
		SyntaxKind::FALSE_KW => FormatToken::string("false"),
		SyntaxKind::STRING => FormatToken::string(token.text().as_str()),
		SyntaxKind::NUMBER => FormatToken::string(token.text().as_str()),
		SyntaxKind::MINUS => FormatToken::string("-"),
		_ => panic!("Unsupported JSON token {:?}", token),
	}
}

fn tokenize_node(node: SyntaxNode) -> FormatToken {
	match node.kind() {
		// Used as the root of a JSON
		SyntaxKind::GROUPING_EXPR => {
			let grouping = GroupingExpr::cast(node).unwrap();
			format_tokens![
				tokenize_node((*grouping.inner().unwrap().syntax()).clone()),
				LineToken::hard()
			]
		}
		SyntaxKind::LITERAL => {
			let literal = Literal::cast(node).unwrap();
			tokenize_token(literal.token())
		}
		SyntaxKind::UNARY_EXPR => {
			let expr = UnaryExpr::cast(node).unwrap();
			format_tokens![
				tokenize_token(expr.op_token().unwrap()),
				tokenize_node(expr.expr().unwrap().syntax().clone())
			]
		}
		SyntaxKind::OBJECT_EXPR => {
			let object = ObjectExpr::cast(node).unwrap();

			let separator = FormatToken::concat(vec![
				FormatToken::string(","),
				FormatToken::Line(LineToken::soft_or_space()),
			]);

			let properties_list: Vec<FormatToken> = object
				.props()
				.map(|prop| match prop {
					ObjectProp::LiteralProp(prop) => FormatToken::concat(vec![
						tokenize_node(prop.key().unwrap().syntax().clone()),
						FormatToken::string(":"),
						FormatToken::Space,
						tokenize_node(prop.value().unwrap().syntax().clone()),
					]),
					_ => panic!("Unsupported prop type {:?}", prop),
				})
				.collect();

			let properties = vec![
				FormatToken::Line(LineToken::soft()),
				FormatToken::join(separator, properties_list),
			];

			FormatToken::Group(GroupToken::new(vec![
				FormatToken::string("{"),
				FormatToken::indent(properties),
				FormatToken::Line(LineToken::soft()),
				FormatToken::string("}"),
			]))
		}
		SyntaxKind::ARRAY_EXPR => {
			let array = ArrayExpr::cast(node).unwrap();

			let separator = FormatToken::concat(vec![
				FormatToken::string(","),
				FormatToken::Line(LineToken::soft_or_space()),
			]);

			let elements = vec![
				FormatToken::Line(LineToken::soft()),
				FormatToken::join(
					separator,
					array
						.elements()
						.map(|element| tokenize_node(element.syntax().clone())),
				),
			];

			FormatToken::Group(GroupToken::new(vec![
				FormatToken::string("["),
				FormatToken::indent(elements),
				FormatToken::Line(LineToken::soft()),
				FormatToken::string("]"),
			]))
		}
		_ => panic!("Unsupported JSON kind: {:?}", node.kind()),
	}
}

pub fn tokenize_json(content: &str) -> FormatToken {
	let script = parse_text(format!("({})", content).as_str(), 0);

	// script -> expression statement -> grouping expr -> then take first node
	let grouping = script
		.syntax()
		.descendants()
		.find(|e| e.kind() == SyntaxKind::GROUPING_EXPR)
		.unwrap();

	tokenize_node(grouping)
}

#[cfg(test)]
mod test {
	use crate::{FormatToken, ListToken};

	use super::tokenize_json;
	use crate::format_token::{GroupToken, LineToken};

	#[test]
	fn tokenize_number() {
		let result = tokenize_json("6.45");

		assert_eq!(
			FormatToken::List(ListToken::concat(vec![
				FormatToken::string("6.45"),
				FormatToken::Line(LineToken::hard())
			])),
			result
		);
	}

	#[test]
	fn tokenize_string() {
		let result = tokenize_json(r#""foo""#);

		assert_eq!(
			FormatToken::List(ListToken::concat(vec![
				FormatToken::string(r#""foo""#),
				FormatToken::Line(LineToken::hard())
			])),
			result
		);
	}

	#[test]
	fn tokenize_boolean_false() {
		let result = tokenize_json("false");

		assert_eq!(
			FormatToken::List(ListToken::concat(vec![
				FormatToken::string("false"),
				FormatToken::Line(LineToken::hard())
			])),
			result
		);
	}

	#[test]
	fn tokenize_boolean_true() {
		let result = tokenize_json("true");

		assert_eq!(
			FormatToken::List(ListToken::concat(vec![
				FormatToken::string("true"),
				FormatToken::Line(LineToken::hard())
			])),
			result
		);
	}

	#[test]
	fn tokenize_boolean_null() {
		let result = tokenize_json("null");

		assert_eq!(
			FormatToken::List(ListToken::concat(vec![
				FormatToken::string("null"),
				FormatToken::Line(LineToken::hard())
			])),
			result
		);
	}

	#[test]
	fn tokenize_object() {
		let input = r#"{ "foo": "bar", "num": 5 }"#;
		let expected = FormatToken::List(ListToken::concat(vec![
			FormatToken::Group(GroupToken::new(vec![
				FormatToken::string("{"),
				FormatToken::indent(FormatToken::concat(vec![
					FormatToken::Line(LineToken::soft()),
					FormatToken::string("\"foo\""),
					FormatToken::string(":"),
					FormatToken::Space,
					FormatToken::string("\"bar\""),
					FormatToken::string(","),
					FormatToken::Line(LineToken::soft_or_space()),
					FormatToken::string("\"num\""),
					FormatToken::string(":"),
					FormatToken::Space,
					FormatToken::string("5"),
				])),
				FormatToken::Line(LineToken::soft()),
				FormatToken::string("}"),
			])),
			FormatToken::Line(LineToken::hard()),
		]));

		let result = tokenize_json(input);

		assert_eq!(expected, result);
	}

	#[test]
	fn tokenize_array() {
		let input = r#"[ "foo", "bar", 5 ]"#;
		let expected = FormatToken::List(ListToken::concat(vec![
			FormatToken::Group(GroupToken::new(vec![
				FormatToken::string("["),
				FormatToken::indent(FormatToken::concat(vec![
					FormatToken::Line(LineToken::soft()),
					FormatToken::string("\"foo\""),
					FormatToken::string(","),
					FormatToken::Line(LineToken::soft_or_space()),
					FormatToken::string("\"bar\""),
					FormatToken::string(","),
					FormatToken::Line(LineToken::soft_or_space()),
					FormatToken::string("5"),
				])),
				FormatToken::Line(LineToken::soft()),
				FormatToken::string("]"),
			])),
			FormatToken::Line(LineToken::hard()),
		]));

		let result = tokenize_json(input);

		assert_eq!(expected, result);
	}
}
