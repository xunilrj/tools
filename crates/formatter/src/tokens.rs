use crate::token_cache::TokensCache;
use lazy_static::lazy_static;
use rowan::GreenToken;
use syntax::SyntaxKind;

/// Helper for building rowan green tree tokens.
///
/// Caches the tokens internally to reduce the memory foot-print.
#[derive(Debug, Clone, Default)]
pub struct Tokens(TokensCache);

#[inline]
fn create_token(kind: SyntaxKind, text: &str) -> GreenToken {
	GreenToken::new(rowan::SyntaxKind(kind.into()), text)
}

impl Tokens {
	/// Creates a token with the specified kind and text or retrieves a cached instance
	pub fn get(&mut self, kind: SyntaxKind, text: &str) -> GreenToken {
		self.0.get(kind, text)
	}

	/// Creates a string token with the specified text
	#[inline]
	pub fn string(&mut self, text: &str) -> GreenToken {
		self.get(SyntaxKind::STRING_TOKEN, text)
	}

	/// Creates a whitespace token
	#[inline]
	pub fn whitespace(&mut self, whitespace: &str) -> GreenToken {
		self.get(SyntaxKind::WHITESPACE, whitespace)
	}

	/// Returns a `,` token
	#[inline]
	pub fn comma(&self) -> GreenToken {
		lazy_static! {
			static ref COMMA: GreenToken = create_token(SyntaxKind::COMMA_TOKEN, ",");
		}

		(*COMMA).clone()
	}

	/// Returns a `"` token
	#[inline]
	pub fn double_quote(&self) -> GreenToken {
		lazy_static! {
			static ref DQUOTE: GreenToken = create_token(SyntaxKind::DQUOTE_TOKEN, "\"");
		}

		(*DQUOTE).clone()
	}

	/// Returns a `'` token
	#[inline]
	pub fn single_quote(&self) -> GreenToken {
		lazy_static! {
			static ref SQUOTE: GreenToken = create_token(SyntaxKind::SQUOTE_TOKEN, "'");
		}

		(*SQUOTE).clone()
	}

	/// Returns a `'` token
	#[inline]
	pub fn colon(&self) -> GreenToken {
		lazy_static! {
			static ref COLON: GreenToken = create_token(SyntaxKind::COLON_TOKEN, ":");
		}

		(*COLON).clone()
	}

	/// Returns a `[` token
	#[inline]
	pub fn left_bracket(&self) -> GreenToken {
		lazy_static! {
			static ref LBRACK: GreenToken = create_token(SyntaxKind::LBRACK_TOKEN, "[");
		}

		(*LBRACK).clone()
	}

	/// Returns a `]` token
	#[inline]
	pub fn right_bracket(&self) -> GreenToken {
		lazy_static! {
			static ref RBRACK: GreenToken = create_token(SyntaxKind::RBRACK_TOKEN, "]");
		}

		(*RBRACK).clone()
	}

	/// Returns a `{` token
	#[inline]
	pub fn left_brace(&self) -> GreenToken {
		lazy_static! {
			static ref LBRACE: GreenToken = create_token(SyntaxKind::LBRACE_TOKEN, "{");
		}

		(*LBRACE).clone()
	}

	/// Returns a `}` token
	#[inline]
	pub fn right_brace(&self) -> GreenToken {
		lazy_static! {
			static ref RBRACE: GreenToken = create_token(SyntaxKind::RBRACE_TOKEN, "}");
		}

		(*RBRACE).clone()
	}

	/// Returns a `null` token
	#[inline]
	pub fn null(&self) -> GreenToken {
		lazy_static! {
			static ref NULL: GreenToken = create_token(SyntaxKind::NULL, "null");
		}

		(*NULL).clone()
	}
}
