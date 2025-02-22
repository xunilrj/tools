use crate::format_element::{join_elements, soft_line_break_or_space};
use crate::{
	format_element::FormatElement, format_elements, group_elements, hard_line_break, soft_indent,
	space_token, token,
};
use rslint_parser::ast::{
	JsArrayExpression, JsBooleanLiteral, JsNullLiteral, JsNumberLiteral, JsParenthesizedExpression,
	JsStringLiteral, JsUnaryExpression, LiteralProp, ObjectExpr, ObjectProp,
};
use rslint_parser::{parse_text, AstNode, SyntaxKind, SyntaxNode, SyntaxNodeExt, SyntaxToken};

fn tokenize_token(syntax_token: SyntaxToken) -> FormatElement {
	match syntax_token.kind() {
		SyntaxKind::NULL_KW => token("null"),
		SyntaxKind::TRUE_KW => token("true"),
		SyntaxKind::FALSE_KW => token("false"),
		SyntaxKind::JS_STRING_LITERAL_TOKEN => token(syntax_token.text()),
		SyntaxKind::JS_NUMBER_LITERAL_TOKEN => token(syntax_token.text()),
		SyntaxKind::MINUS => token("-"),
		_ => panic!("Unsupported JSON token {:?}", syntax_token),
	}
}

fn tokenize_node(node: SyntaxNode) -> FormatElement {
	match node.kind() {
		SyntaxKind::JS_STRING_LITERAL => {
			tokenize_token(node.to::<JsStringLiteral>().value_token().unwrap())
		}
		SyntaxKind::JS_BOOLEAN_LITERAL => {
			tokenize_token(node.to::<JsBooleanLiteral>().value_token().unwrap())
		}
		SyntaxKind::JS_NULL_LITERAL => {
			tokenize_token(node.to::<JsNullLiteral>().value_token().unwrap())
		}
		SyntaxKind::JS_NUMBER_LITERAL => {
			tokenize_token(node.to::<JsNumberLiteral>().value_token().unwrap())
		}
		SyntaxKind::JS_UNARY_EXPRESSION => {
			let expr = JsUnaryExpression::cast(node).unwrap();
			format_elements![
				tokenize_token(expr.operator().unwrap()),
				tokenize_node(expr.argument().unwrap().syntax().clone())
			]
		}

		SyntaxKind::LITERAL_PROP => {
			let prop = LiteralProp::cast(node).unwrap();
			format_elements![
				tokenize_node(prop.key().unwrap().syntax().clone()),
				token(":"),
				space_token(),
				tokenize_node(prop.value().unwrap().syntax().clone()),
			]
		}

		SyntaxKind::OBJECT_EXPR => {
			let object = ObjectExpr::cast(node).unwrap();

			let separator = format_elements![token(","), soft_line_break_or_space()];

			let properties_list: Vec<FormatElement> = object
				.props()
				.iter()
				.map(|prop| match prop {
					ObjectProp::LiteralProp(prop) => {
						format_elements![tokenize_node(prop.syntax().clone())]
					}
					_ => panic!("Unsupported prop type {:?}", prop),
				})
				.collect();

			let properties = join_elements(separator, properties_list);

			group_elements(format_elements![
				token("{"),
				soft_indent(properties),
				token("}"),
			])
		}
		SyntaxKind::JS_ARRAY_EXPRESSION => {
			let array = JsArrayExpression::cast(node).unwrap();

			let separator = format_elements![token(","), soft_line_break_or_space(),];

			let elements = join_elements(
				separator,
				array
					.elements()
					.iter()
					.map(|element| tokenize_node(element.syntax().clone())),
			);

			group_elements(format_elements![
				token("["),
				soft_indent(elements),
				token("]"),
			])
		}
		_ => panic!("Unsupported JSON kind: {:?}", node.kind()),
	}
}

pub fn tokenize_json(content: &str) -> FormatElement {
	let script = parse_text(format!("({})", content).as_str(), 0);

	// Unwrap the grouping to get to the JSON content. The grouping is only used as a trick to parse JSON
	let json_content = JsParenthesizedExpression::cast(
		script
			.syntax()
			.descendants()
			.find(|e| e.kind() == SyntaxKind::JS_PARENTHESIZED_EXPRESSION)
			.unwrap(),
	)
	// TODO: #1725 this should be reviewed for error handling
	.and_then(|grouping| grouping.expression().ok())
	.unwrap();

	let tokenized_content = tokenize_node(json_content.syntax().clone());
	format_elements![tokenized_content, hard_line_break()]
}

#[cfg(test)]
mod test {
	use crate::{
		format_elements, group_elements, hard_line_break, soft_line_break,
		soft_line_break_or_space, space_token, token,
	};

	use super::tokenize_json;
	use crate::format_element::Indent;

	#[test]
	fn tokenize_number() {
		let result = tokenize_json("6.45");

		assert_eq!(format_elements![token("6.45"), hard_line_break()], result);
	}

	#[test]
	fn tokenize_string() {
		let result = tokenize_json(r#""foo""#);

		assert_eq!(
			format_elements![token(r#""foo""#), hard_line_break()],
			result
		);
	}

	#[test]
	fn tokenize_boolean_false() {
		let result = tokenize_json("false");

		assert_eq!(format_elements![token("false"), hard_line_break()], result);
	}

	#[test]
	fn tokenize_boolean_true() {
		let result = tokenize_json("true");

		assert_eq!(format_elements![token("true"), hard_line_break()], result);
	}

	#[test]
	fn tokenize_boolean_null() {
		let result = tokenize_json("null");

		assert_eq!(format_elements![token("null"), hard_line_break()], result);
	}

	#[test]
	fn tokenize_object() {
		let input = r#"{ "foo": "bar", "num": 5 }"#;
		let expected = format_elements![
			group_elements(format_elements![
				token("{"),
				FormatElement::Indent(Indent::new(format_elements![
					soft_line_break(),
					token("\"foo\""),
					token(":"),
					space_token(),
					token("\"bar\""),
					token(","),
					soft_line_break_or_space(),
					token("\"num\""),
					token(":"),
					space_token(),
					token("5"),
				])),
				soft_line_break(),
				token("}"),
			]),
			hard_line_break(),
		];

		let result = tokenize_json(input);

		assert_eq!(expected, result);
	}

	#[test]
	fn tokenize_array() {
		let input = r#"[ "foo", "bar", 5 ]"#;
		let expected = format_elements![
			group_elements(format_elements![
				token("["),
				FormatElement::Indent(Indent::new(format_elements![
					soft_line_break(),
					token("\"foo\""),
					token(","),
					soft_line_break_or_space(),
					token("\"bar\""),
					token(","),
					soft_line_break_or_space(),
					token("5"),
				])),
				soft_line_break(),
				token("]"),
			]),
			hard_line_break(),
		];

		let result = tokenize_json(input);

		assert_eq!(expected, result);
	}
}
