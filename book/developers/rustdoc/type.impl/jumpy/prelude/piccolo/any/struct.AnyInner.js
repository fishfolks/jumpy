(function() {var type_impls = {
"jumpy":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Collect-for-AnyInner%3CM%3E\" class=\"impl\"><a href=\"#impl-Collect-for-AnyInner%3CM%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;M&gt; Collect for <a class=\"struct\" href=\"jumpy/prelude/piccolo/any/struct.AnyInner.html\" title=\"struct jumpy::prelude::piccolo::any::AnyInner\">AnyInner</a>&lt;M&gt;<div class=\"where\">where\n    M: Collect,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.needs_trace\" class=\"method trait-impl\"><a href=\"#method.needs_trace\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">needs_trace</a>() -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>As an optimization, if this type can never hold a <code>Gc</code> pointer and <code>trace</code> is unnecessary\nto call, you may implement this method and return false. The default implementation returns\ntrue, signaling that <code>Collect::trace</code> must be called.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.trace\" class=\"method trait-impl\"><a href=\"#method.trace\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">trace</a>(&amp;self, cc: &amp;Collection)</h4></section></summary><div class='docblock'><em>Must</em> call <code>Collect::trace</code> on all held <code>Gc</code> pointers. If this type holds inner types that\nimplement <code>Collect</code>, a valid implementation would simply call <code>Collect::trace</code> on all the\nheld values to ensure this.</div></details></div></details>","Collect","jumpy::prelude::piccolo::userdata::UserDataInner"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()