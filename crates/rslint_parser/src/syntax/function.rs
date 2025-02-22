use crate::syntax::decl::{is_semi, parameter_list};
use crate::syntax::pat::opt_binding_identifier;
use crate::syntax::stmt::block_impl;
use crate::syntax::typescript::{ts_type_or_type_predicate_ann, ts_type_params};
use crate::{CompletedMarker, Parser, ParserState};
use rslint_syntax::SyntaxKind::{
	ERROR, JS_FUNCTION_BODY, JS_FUNCTION_DECLARATION, JS_FUNCTION_EXPRESSION,
	JS_IDENTIFIER_BINDING, TS_RETURN_TYPE,
};
use rslint_syntax::{SyntaxKind, T};
use std::collections::HashMap;

/// A function declaration, this could be async and or a generator. This takes a marker
/// because you need to first advance over async or start a marker and feed it in.
// test function_decl
// function foo() {}
// function *foo() {}
// function foo(await) {}
// async function *foo() {}
// async function foo() {}
// function *foo() {
//   yield foo;
// }
pub(super) fn function_declaration(p: &mut Parser) -> CompletedMarker {
	function(p, JS_FUNCTION_DECLARATION)
}

pub(super) fn function_expression(p: &mut Parser) -> CompletedMarker {
	function(p, JS_FUNCTION_EXPRESSION)
}

fn function(p: &mut Parser, kind: SyntaxKind) -> CompletedMarker {
	let m = p.start();

	if kind == JS_FUNCTION_DECLARATION {
		// TS function declaration
		p.eat(T![declare]);
	}

	let in_async = p.at(T![ident]) && p.cur_src() == "async";
	if in_async {
		p.bump_remap(T![async]);
	}

	p.expect(T![function]);

	let in_generator = p.eat(T![*]);
	let guard = &mut *p.with_state(ParserState {
		labels: HashMap::new(),
		in_function: true,
		in_async,
		in_generator,
		..p.state.clone()
	});

	let id = opt_binding_identifier(guard);

	if let Some(mut identifier_marker) = id {
		identifier_marker.change_kind(guard, JS_IDENTIFIER_BINDING);
	} else if kind == JS_FUNCTION_DECLARATION {
		let err = guard
			.err_builder(
				"expected a name for the function in a function declaration, but found none",
			)
			.primary(guard.cur_tok().range, "");

		guard.error(err);
	}

	parameter_types(guard);
	parameter_list(guard);
	return_type(guard);

	if kind == JS_FUNCTION_DECLARATION {
		function_body_or_declaration(guard);
	} else {
		function_body(guard);
	}

	m.complete(guard, kind)
}

pub(super) fn function_body(p: &mut Parser) -> Option<CompletedMarker> {
	block_impl(p, JS_FUNCTION_BODY, None)
}

// TODO 1725 This is probably not ideal (same with the `declare` keyword). We should
// use a different AST type for function declarations. For example, a function declaration should
// never have a body but that would be allowed with this approach. Same for interfaces, interface
// methods should never have a body
/// Either parses a typescript declaration body or the function body
pub(crate) fn function_body_or_declaration(p: &mut Parser) {
	// omitting the body is allowed in ts
	if p.typescript() && !p.at(T!['{']) && is_semi(p, 0) {
		p.eat(T![;]);
	} else {
		let mut complete = function_body(p);
		if let Some(ref mut block) = complete {
			if p.state.in_declare {
				let err = p
					.err_builder(
						"function implementations cannot be given in ambient (declare) contexts",
					)
					.primary(block.range(p), "");

				p.error(err);
				block.change_kind(p, ERROR);
			}
		}
	}
}

pub(super) fn args_body(p: &mut Parser) {
	parameter_types(p);
	parameter_list(p);
	return_type(p);
	function_body_or_declaration(p);
}

fn parameter_types(p: &mut Parser) {
	if p.at(T![<]) {
		if let Some(ref mut ty) = ts_type_params(p) {
			ty.err_if_not_ts(p, "type parameters can only be used in TypeScript files");
		}
	}
}

fn return_type(p: &mut Parser) {
	if p.at(T![:]) {
		let return_type = p.start();
		if let Some(ref mut ty) = ts_type_or_type_predicate_ann(p, T![:]) {
			ty.err_if_not_ts(p, "return types can only be used in TypeScript files");
		}
		return_type.complete(p, TS_RETURN_TYPE);
	}
}
