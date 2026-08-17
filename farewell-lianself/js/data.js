const CODE_DATA = {
    DEFAULT: [
        { line: 1, html: '<span class="header-pre">pre</span> <span class="header-cur">cur</span> <span class="sep">|</span>', source: '// git<diff header @HEAD~1>' },
        { line: 2, html: ' <span class="diff-neutral">·</span>   <span class="diff-neutral">·</span>  <span class="sep">|</span> <span class="attr">#[allow<span class="paren">(</span>dead_code<span class="paren">)</span>]</span>', source: '// Rust<src/lib.rs:allow>' },
        { line: 3, html: ' <span class="diff-del">-</span>      <span class="sep">|</span> <span class="kw-storage">static</span> <span class="kw-def">struct</span> <span class="type">Lian</span> <span class="brace">{</span>', source: '// C<src/lian.h:storage>' },
        { line: 4, html: ' <span class="diff-add">+</span>   <span class="diff-neutral">·</span>  <span class="sep">|</span> <span class="kw-storage">static</span> <span class="kw-storage">final</span> <span class="kw-def">class</span> <span class="type">Lian</span> <span class="brace">{</span>', source: '// Java<src/Lian.java:class>' },
        { line: 5, html: ' <span class="diff-add">+</span>   <span class="diff-neutral">·</span>  <span class="sep">|</span>     <span class="kw-access">private</span> <span class="type">Lian</span><span class="paren">()</span> <span class="brace">{</span>', source: '// Java<src/Lian.java:ctor>' },
        { line: 6, html: ' <span class="diff-neutral">·</span>   <span class="diff-neutral">·</span>  <span class="sep">|</span>         <span class="self-kw">self</span>.<span class="self-prop">_attr</span><span class="semicolon">;</span>', source: '// Python<lian.py:_attr>' },
        { line: 7, html: ' <span class="diff-neutral">·</span>   <span class="diff-neutral">·</span>  <span class="sep">|</span>         <span class="self-kw">self</span>.<span class="self-prop">__M</span><span class="semicolon">;</span>', source: '// Python<lian.py:__M>' },
        { line: 8, html: ' <span class="diff-add">+</span>   <span class="diff-neutral">·</span>  <span class="sep">|</span>     <span class="brace">}</span><span class="semicolon">;</span>', source: '// Java<src/Lian.java:end>' },
        { line: 9, html: ' <span class="diff-neutral">·</span>   <span class="diff-del">-</span>  <span class="sep">|</span> <span class="brace">}</span> <span class="comment">/* ill-formed by design */</span> <span class="semicolon">;</span>', source: '// Java<src/Lian.java:comment>' },
        { line: 10, html: '     <span class="diff-add">+</span>  <span class="sep">|</span> <span class="brace">}</span><span class="semicolon">;</span>', source: '// Java<src/Lian.java:end>' },
        { line: 11, html: '     <span class="diff-add">+</span>  <span class="sep">|</span> <span class="macro-error">#error</span> <span class="string">"Class is not meant to be used"</span>', source: '// C++<lian.hpp:#error>' }
    ],
    AURORA: [
        { line: 1, html: '<span class="attr">#[allow<span class="paren">(</span>dead_code<span class="paren">)</span>]</span>', source: '// Attribute' },
        { line: 2, html: '<span class="kw-storage">static</span> <span class="kw-def">struct</span> <span class="type">Lian</span> <span class="brace">{</span>', source: '// Struct Definition' },
        { line: 3, html: '    <span class="self-kw">self</span>.<span class="self-prop">_attr</span><span class="semicolon">;</span>', source: '// Member Access' },
        { line: 4, html: '    <span class="self-kw">self</span>.<span class="self-prop">__M</span><span class="semicolon">;</span>', source: '// Internal State' },
        { line: 5, html: '<span class="brace">}</span> <span class="comment">/* ill-formed by design */</span> <span class="semicolon">;</span>', source: '// Design Constraint' }
    ],
    VITRIMURA: [
        { line: 1, html: '<span class="attr">#[allow<span class="paren">(</span>dead_code<span class="paren">)</span>]</span>', source: '// Ignored Warning' },
        { line: 2, html: '<span class="kw-storage">static</span> <span class="kw-storage">final</span> <span class="kw-def">class</span> <span class="type">Lian</span> <span class="brace">{</span>', source: '// Final Class' },
        { line: 3, html: '    <span class="kw-access">private</span> <span class="type">Lian</span><span class="paren">()</span> <span class="brace">{</span>', source: '// Private Constructor' },
        { line: 4, html: '        <span class="self-kw">self</span>.<span class="self-prop">_attr</span><span class="semicolon">;</span>', source: '// Attribute' },
        { line: 5, html: '        <span class="self-kw">self</span>.<span class="self-prop">__M</span><span class="semicolon">;</span>', source: '// Memory' },
        { line: 6, html: '    <span class="brace">}</span><span class="semicolon">;</span>', source: '// End Scope' },
        { line: 7, html: '<span class="brace">}</span> <span class="comment">/* ill-formed by design */</span> <span class="semicolon">;</span>', source: '// Fatal Flaw' }
    ],
    NOSTOFOBIA: [
        { line: 1, html: '<span class="attr">#[allow<span class="paren">(</span>dead_code<span class="paren">)</span>]</span>', source: '' },
        { line: 2, html: '<span class="kw-storage">static</span> <span class="kw-storage">final</span> <span class="kw-def">class</span> <span class="type">Lian</span> <span class="brace">{</span>', source: '' },
        { line: 3, html: '    <span class="kw-access">private</span> <span class="type">Lian</span><span class="paren">()</span> <span class="brace">{</span>', source: '' },
        { line: 4, html: '        <span class="self-kw">self</span>.<span class="self-prop">_attr</span><span class="semicolon">;</span>', source: '' },
        { line: 5, html: '        <span class="self-kw">self</span>.<span class="self-prop">__M</span><span class="semicolon">;</span>', source: '' },
        { line: 6, html: '    <span class="brace">}</span><span class="semicolon">;</span>', source: '' },
        { line: 7, html: '<span class="brace">}</span><span class="semicolon">;</span>', source: '' },
        { line: 8, html: '<span class="macro-error">#error</span> <span class="string">"Class is not meant to be used"</span>', source: '' }
    ]
};

const STATE_NAMES = {
    DEFAULT: "lian.inc",
    AURORA: "State_Aurora",
    VITRIMURA: "State_Vitrimura",
    NOSTOFOBIA: "State_Nostofobia"
};
