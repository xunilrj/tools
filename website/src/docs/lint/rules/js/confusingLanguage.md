---
title: Lint Rule js/confusingLanguage
layout: layouts/page.njk
description: MISSING DOCUMENTATION
eleventyNavigation:
	key: lint-rules/js/confusingLanguage
	parent: lint-rules
	title: js/confusingLanguage
---

# js/confusingLanguage

MISSING DOCUMENTATION

<!-- EVERYTHING BELOW IS AUTOGENERATED. SEE SCRIPTS FOLDER FOR UPDATE SCRIPTS hash(c3f05a1c0550fcb0a3914eb97f4a42c31f27c77b) -->

## Examples
### Invalid
<pre class="language-text"><code class="language-text"><span class="token comment">//	the	blacklist</span></code></pre>
<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dotted;">file.ts:1:5</span> <strong>lint/js/confusingLanguage</strong> <span style="color: white; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">The word </span><span style="color: Tomato;"><strong>blacklist</strong></span><span style="color: Tomato;"> can be considered racially charged language.</span>

    <span class="token comment">//</span><span class="token comment">  </span><span class="token comment">the</span><span class="token comment">  </span><span class="token comment">blacklist</span>
          <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">See </span><span style="color: DodgerBlue;"><a href="https://www.ncbi.nlm.nih.gov/pmc/articles/PMC6148600/">https://www.ncbi.nlm.nih.gov/pmc/articles/PMC6148600/</a></span><span style="color: DodgerBlue;"> for a more</span>
    <span style="color: DodgerBlue;">detailed explanation.</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Consider using </span><span style="color: DodgerBlue;"><strong>denylist</strong></span><span style="color: DodgerBlue;"> instead</span>

</code></pre>

---------------

<pre class="language-text"><code class="language-text"><span class="token comment">/*	the</span>
<span class="token comment">blacklist	*/</span></code></pre>
<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dotted;">file.ts:2</span> <strong>lint/js/confusingLanguage</strong> <span style="color: white; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">The word </span><span style="color: Tomato;"><strong>blacklist</strong></span><span style="color: Tomato;"> can be considered racially charged language.</span>

  <strong>  1</strong><strong> │ </strong><span class="token comment">/*</span><span class="token comment">  </span><span class="token comment">the</span>
  <strong><span style="color: Tomato;">&gt;</span></strong><strong> 2</strong><strong> │ </strong><span class="token comment">blacklist</span><span class="token comment">  </span><span class="token comment">*/</span>
     <strong> │ </strong><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">See </span><span style="color: DodgerBlue;"><a href="https://www.ncbi.nlm.nih.gov/pmc/articles/PMC6148600/">https://www.ncbi.nlm.nih.gov/pmc/articles/PMC6148600/</a></span><span style="color: DodgerBlue;"> for a more</span>
    <span style="color: DodgerBlue;">detailed explanation.</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Consider using </span><span style="color: DodgerBlue;"><strong>denylist</strong></span><span style="color: DodgerBlue;"> instead</span>

</code></pre>

---------------

<pre class="language-text"><code class="language-text"><span class="token variable">blacklist</span><span class="token punctuation">;</span></code></pre>
<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dotted;">file.ts:1</span> <strong>lint/js/confusingLanguage</strong> <span style="color: white; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">The word </span><span style="color: Tomato;"><strong>blacklist</strong></span><span style="color: Tomato;"> can be considered racially charged language.</span>

    <span class="token variable">blacklist</span><span class="token punctuation">;</span>
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">See </span><span style="color: DodgerBlue;"><a href="https://www.ncbi.nlm.nih.gov/pmc/articles/PMC6148600/">https://www.ncbi.nlm.nih.gov/pmc/articles/PMC6148600/</a></span><span style="color: DodgerBlue;"> for a more</span>
    <span style="color: DodgerBlue;">detailed explanation.</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Consider using </span><span style="color: DodgerBlue;"><strong>denylist</strong></span><span style="color: DodgerBlue;"> instead</span>

</code></pre>

---------------

<pre class="language-text"><code class="language-text"><span class="token variable">BLACKLIST</span><span class="token punctuation">;</span></code></pre>
<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dotted;">file.ts:1</span> <strong>lint/js/confusingLanguage</strong> <span style="color: white; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">The word </span><span style="color: Tomato;"><strong>blacklist</strong></span><span style="color: Tomato;"> can be considered racially charged language.</span>

    <span class="token variable">BLACKLIST</span><span class="token punctuation">;</span>
    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">See </span><span style="color: DodgerBlue;"><a href="https://www.ncbi.nlm.nih.gov/pmc/articles/PMC6148600/">https://www.ncbi.nlm.nih.gov/pmc/articles/PMC6148600/</a></span><span style="color: DodgerBlue;"> for a more</span>
    <span style="color: DodgerBlue;">detailed explanation.</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Consider using </span><span style="color: DodgerBlue;"><strong>DENYLIST</strong></span><span style="color: DodgerBlue;"> instead</span>

</code></pre>

---------------

<pre class="language-text"><code class="language-text"><span class="token variable">someBlacklist</span><span class="token punctuation">;</span></code></pre>
<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dotted;">file.ts:1:4</span> <strong>lint/js/confusingLanguage</strong> <span style="color: white; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">The word </span><span style="color: Tomato;"><strong>blacklist</strong></span><span style="color: Tomato;"> can be considered racially charged language.</span>

    <span class="token variable">someBlacklist</span><span class="token punctuation">;</span>
        <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">See </span><span style="color: DodgerBlue;"><a href="https://www.ncbi.nlm.nih.gov/pmc/articles/PMC6148600/">https://www.ncbi.nlm.nih.gov/pmc/articles/PMC6148600/</a></span><span style="color: DodgerBlue;"> for a more</span>
    <span style="color: DodgerBlue;">detailed explanation.</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Consider using </span><span style="color: DodgerBlue;"><strong>Denylist</strong></span><span style="color: DodgerBlue;"> instead</span>

</code></pre>

---------------

<pre class="language-text"><code class="language-text"><span class="token variable">SOME_BLACKLIST</span><span class="token punctuation">;</span></code></pre>
<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dotted;">file.ts:1:5</span> <strong>lint/js/confusingLanguage</strong> <span style="color: white; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">The word </span><span style="color: Tomato;"><strong>blacklist</strong></span><span style="color: Tomato;"> can be considered racially charged language.</span>

    <span class="token variable">SOME_BLACKLIST</span><span class="token punctuation">;</span>
         <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">See </span><span style="color: DodgerBlue;"><a href="https://www.ncbi.nlm.nih.gov/pmc/articles/PMC6148600/">https://www.ncbi.nlm.nih.gov/pmc/articles/PMC6148600/</a></span><span style="color: DodgerBlue;"> for a more</span>
    <span style="color: DodgerBlue;">detailed explanation.</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Consider using </span><span style="color: DodgerBlue;"><strong>DENYLIST</strong></span><span style="color: DodgerBlue;"> instead</span>

</code></pre>