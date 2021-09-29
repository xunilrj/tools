use crate::format_token::NodeToken;
use crate::{
	format_tokens, FormatContext, FormatToken, FormatValue, GroupToken, LineToken, ListToken,
};

use rslint_parser::{GreenNode, SyntaxKind};
use rslint_rowan::GreenToken;
use serde_json::Value;

#[inline]
fn create_green_node(kind: SyntaxKind) -> GreenNode {
	GreenNode::new(rslint_rowan::SyntaxKind(kind.into()), std::iter::empty())
}

#[inline]
fn create_literal_token(token: GreenToken) -> FormatToken {
	FormatToken::from(NodeToken::new(
		create_green_node(SyntaxKind::LITERAL),
		token,
	))
}

impl FormatValue for Value {
	fn format(&self, context: &mut FormatContext) -> FormatToken {
		match self {
			Value::String(string) => string_token(string, context),

			Value::Number(number) => {
				let number = number.as_f64().unwrap();
				let number_token = context
					.tokens
					.get(SyntaxKind::NUMBER, number.to_string().as_str());

				create_literal_token(number_token)
			}

			Value::Bool(value) => {
				let token = match value {
					true => context.tokens.get(SyntaxKind::TRUE_KW, "true"),
					false => context.tokens.get(SyntaxKind::FALSE_KW, "false"),
				};
				create_literal_token(token)
			}

			Value::Object(value) => {
				let separator = format_tokens![context.tokens.comma(), LineToken::soft_or_space(),];

				let properties_list: Vec<FormatToken> = value
					.iter()
					.map(|(key, value)| {
						format_tokens![
							string_token(key, context),
							context.tokens.colon(),
							FormatToken::Space,
							value.format(context),
						]
					})
					.collect();

				let properties = format_tokens![
					LineToken::soft(),
					ListToken::join(separator, properties_list),
				];

				NodeToken::new(
					create_green_node(SyntaxKind::OBJECT_EXPR),
					GroupToken::new(format_tokens![
						context.tokens.left_brace(),
						FormatToken::indent(properties),
						LineToken::soft(),
						context.tokens.right_brace(),
					]),
				)
				.into()
			}
			Value::Null => context.tokens.null().into(),

			Value::Array(array) => {
				let separator = format_tokens![context.tokens.comma(), LineToken::soft_or_space(),];

				let elements = format_tokens![
					LineToken::soft(),
					ListToken::join(
						separator,
						array.iter().map(|element| element.format(context))
					),
				];

				NodeToken::new(
					create_green_node(SyntaxKind::ARRAY_EXPR),
					GroupToken::new(format_tokens![
						context.tokens.left_bracket(),
						FormatToken::indent(elements),
						LineToken::soft(),
						context.tokens.right_bracket(),
					]),
				)
				.into()
			}
		}
	}
}

fn string_token(string: &str, context: &mut FormatContext) -> FormatToken {
	let escaped = string
		.replace("\\", "\\\\")
		.replace('"', "\\\"")
		.replace('\r', "\\r")
		.replace('\t', "\\t")
		.replace('\n', "\\n");

	create_literal_token(context.tokens.double_quoted_string(&escaped))
}

pub fn json_to_tokens(content: &str) -> FormatToken {
	let json: Value = serde_json::from_str(content).expect("cannot convert json to tokens");
	format_tokens![
		json.format(&mut FormatContext::default()),
		LineToken::hard()
	]
}

#[cfg(test)]
mod test {
	use crate::format_json::create_green_node;
	use crate::{format_tokens, FormatToken, IndentToken, NodeToken, Tokens};

	use super::json_to_tokens;
	use crate::format_token::{GroupToken, LineToken};
	use rslint_parser::SyntaxKind;

	#[test]
	fn tokenize_number() {
		let result = json_to_tokens("6.45");
		let mut tokens = Tokens::default();

		assert_eq!(
			format_tokens![
				NodeToken::new(
					create_green_node(SyntaxKind::LITERAL),
					tokens.get(SyntaxKind::NUMBER, "6.45")
				),
				LineToken::hard()
			],
			result
		);
	}

	#[test]
	fn tokenize_string() {
		let result = json_to_tokens(r#""foo""#);
		let mut tokens = Tokens::default();

		assert_eq!(
			format_tokens![
				NodeToken::new(
					create_green_node(SyntaxKind::LITERAL),
					tokens.get(SyntaxKind::STRING, r#""foo""#)
				),
				LineToken::hard()
			],
			result
		);
	}

	#[test]
	fn tokenize_boolean_false() {
		let result = json_to_tokens("false");
		let mut tokens = Tokens::default();

		assert_eq!(
			format_tokens![
				NodeToken::new(
					create_green_node(SyntaxKind::LITERAL),
					tokens.get(SyntaxKind::FALSE_KW, "false")
				),
				LineToken::hard()
			],
			result
		);
	}

	#[test]
	fn tokenize_boolean_true() {
		let result = json_to_tokens("true");
		let mut tokens = Tokens::default();

		assert_eq!(
			format_tokens![
				NodeToken::new(
					create_green_node(SyntaxKind::LITERAL),
					tokens.get(SyntaxKind::TRUE_KW, "true")
				),
				LineToken::hard()
			],
			result
		);
	}

	#[test]
	fn tokenize_boolean_null() {
		let result = json_to_tokens("null");
		let tokens = Tokens::default();

		assert_eq!(format_tokens![tokens.null(), LineToken::hard()], result);
	}

	#[test]
	fn tokenize_object() {
		let input = r#"{ "foo": "bar", "num": 5 }"#;
		let mut tokens = Tokens::default();

		let expected: FormatToken = format_tokens![
			NodeToken::new(
				create_green_node(SyntaxKind::OBJECT_EXPR),
				GroupToken::new(format_tokens![
					tokens.left_brace(),
					IndentToken::new(format_tokens![
						LineToken::soft(),
						NodeToken::new(
							create_green_node(SyntaxKind::LITERAL),
							tokens.get(SyntaxKind::STRING, "\"foo\"")
						),
						tokens.colon(),
						FormatToken::Space,
						NodeToken::new(
							create_green_node(SyntaxKind::LITERAL),
							tokens.get(SyntaxKind::STRING, "\"bar\"")
						),
						tokens.comma(),
						FormatToken::Line(LineToken::soft_or_space()),
						NodeToken::new(
							create_green_node(SyntaxKind::LITERAL),
							tokens.get(SyntaxKind::STRING, "\"num\"")
						),
						tokens.colon(),
						FormatToken::Space,
						NodeToken::new(
							create_green_node(SyntaxKind::LITERAL),
							tokens.get(SyntaxKind::NUMBER, "5")
						),
					]),
					LineToken::soft(),
					tokens.right_brace(),
				]),
			),
			LineToken::hard()
		];

		let result = json_to_tokens(input);

		assert_eq!(expected, result);
	}

	#[test]
	fn tokenize_array() {
		let input = r#"[ "foo", "bar", 5 ]"#;
		let mut tokens = Tokens::default();

		let expected = format_tokens![
			NodeToken::new(
				create_green_node(SyntaxKind::ARRAY_EXPR),
				GroupToken::new(format_tokens![
					tokens.left_bracket(),
					IndentToken::new(format_tokens![
						LineToken::soft(),
						NodeToken::new(
							create_green_node(SyntaxKind::LITERAL),
							tokens.get(SyntaxKind::STRING, "\"foo\"")
						),
						tokens.comma(),
						LineToken::soft_or_space(),
						NodeToken::new(
							create_green_node(SyntaxKind::LITERAL),
							tokens.get(SyntaxKind::STRING, "\"bar\"")
						),
						tokens.comma(),
						LineToken::soft_or_space(),
						NodeToken::new(
							create_green_node(SyntaxKind::LITERAL),
							tokens.get(SyntaxKind::NUMBER, "5")
						),
					]),
					LineToken::soft(),
					tokens.right_bracket()
				])
			),
			LineToken::hard(),
		];

		let result = json_to_tokens(input);

		assert_eq!(expected, result);
	}
}
