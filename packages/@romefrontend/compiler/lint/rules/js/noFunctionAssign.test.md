# `noFunctionAssign.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test packages/@romefrontend/compiler/lint/rules/js/noFunctionAssign.test.ts --update-snapshots` to update.

## `no function reassignment`

### `0`

```

 unknown:1:19 lint/js/noFunctionAssign ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Do not reassign a function declaration.

    function foo() {}; foo = bar;
                       ^^^

  ℹ Use a local variable instead.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

✖ Found 1 problem

```

### `0: formatted`

```
function foo() {}
foo = bar;

```

### `1`

```

 unknown:1:17 lint/js/noFunctionAssign ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Do not reassign a function declaration.

    function foo() { foo = bar; }
                     ^^^

  ℹ Use a local variable instead.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

✖ Found 1 problem

```

### `1: formatted`

```
function foo() {
	foo = bar;
}

```

### `2`

```

 unknown:1 lint/js/noFunctionAssign ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Do not reassign a function declaration.

    foo = bar; function foo() { };
    ^^^

  ℹ Use a local variable instead.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

✖ Found 1 problem

```

### `2: formatted`

```
foo = bar;
function foo() {}

```

### `3`

```

 unknown:1:1 lint/js/noFunctionAssign ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Do not reassign a function declaration.

    [foo] = bar; function foo() { };
     ^^^

  ℹ Use a local variable instead.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

✖ Found 1 problem

```

### `3: formatted`

```
[foo] = bar;
function foo() {}

```

### `4`

```

 unknown:1:5 lint/js/noFunctionAssign ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Do not reassign a function declaration.

    ({x: foo = 0} = bar); function foo() { };
         ^^^

  ℹ Use a local variable instead.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

✖ Found 1 problem

```

### `4: formatted`

```
({x: foo = 0} = bar);
function foo() {}

```

### `5`

```

 unknown:1:18 lint/js/noFunctionAssign ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Do not reassign a function declaration.

    function foo() { [foo] = bar; }
                      ^^^

  ℹ Use a local variable instead.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

✖ Found 1 problem

```

### `5: formatted`

```
function foo() {
	[foo] = bar;
}

```

### `6`

```

 unknown:1:19 lint/js/noFunctionAssign ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Do not reassign a function declaration.

    (function() { ({x: foo = 0} = bar); function foo() { }; })();
                       ^^^

  ℹ Use a local variable instead.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

✖ Found 1 problem

```

### `6: formatted`

```
(function() {
	({x: foo = 0} = bar);
	function foo() {}
})();

```

### `7`

```
✔ No known problems!

```

### `7: formatted`

```
function foo() {
	var foo = bar;
}

```

### `8`

```
✔ No known problems!

```

### `8: formatted`

```
function foo(foo) {
	foo = bar;
}

```

### `9`

```
✔ No known problems!

```

### `9: formatted`

```
function foo() {
	var foo;
	foo = bar;
}

```

### `10`

```
✔ No known problems!

```

### `10: formatted`

```
var foo = () => {};
foo = bar;

```

### `11`

```
✔ No known problems!

```

### `11: formatted`

```
var foo = function() {};
foo = bar;

```

### `12`

```
✔ No known problems!

```

### `12: formatted`

```
var foo = function() {
	foo = bar;
};

```

### `13`

```
✔ No known problems!

```

### `13: formatted`

```
import bar from "bar";
function foo() {
	var foo = bar;
}

```