(function() {var type_impls = {
"jumpy":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Clone-for-ContactManifold%3CManifoldData,+ContactData%3E\" class=\"impl\"><a class=\"src rightside\" href=\"http://docs.rs/parry/0.1.1/src/parry2d/query/contact_manifolds/contact_manifold.rs.html#73\">source</a><a href=\"#impl-Clone-for-ContactManifold%3CManifoldData,+ContactData%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;ManifoldData, ContactData&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"http://docs.rs/parry/0.1.1/parry2d/query/contact_manifolds/contact_manifold/struct.ContactManifold.html\" title=\"struct parry2d::query::contact_manifolds::contact_manifold::ContactManifold\">ContactManifold</a>&lt;ManifoldData, ContactData&gt;<div class=\"where\">where\n    ManifoldData: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,\n    ContactData: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"http://docs.rs/parry/0.1.1/src/parry2d/query/contact_manifolds/contact_manifold.rs.html#73\">source</a><a href=\"#method.clone\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html#tymethod.clone\" class=\"fn\">clone</a>(&amp;self) -&gt; <a class=\"struct\" href=\"http://docs.rs/parry/0.1.1/parry2d/query/contact_manifolds/contact_manifold/struct.ContactManifold.html\" title=\"struct parry2d::query::contact_manifolds::contact_manifold::ContactManifold\">ContactManifold</a>&lt;ManifoldData, ContactData&gt;</h4></section></summary><div class='docblock'>Returns a copy of the value. <a href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html#tymethod.clone\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone_from\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.81.0/src/core/clone.rs.html#172\">source</a></span><a href=\"#method.clone_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html#method.clone_from\" class=\"fn\">clone_from</a>(&amp;mut self, source: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.reference.html\">&amp;Self</a>)</h4></section></summary><div class='docblock'>Performs copy-assignment from <code>source</code>. <a href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html#method.clone_from\">Read more</a></div></details></div></details>","Clone","jumpy::core::physics::rapier::ContactManifold"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-ContactManifold%3CManifoldData,+ContactData%3E\" class=\"impl\"><a class=\"src rightside\" href=\"http://docs.rs/parry/0.1.1/src/parry2d/query/contact_manifolds/contact_manifold.rs.html#110\">source</a><a href=\"#impl-ContactManifold%3CManifoldData,+ContactData%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;ManifoldData, ContactData&gt; <a class=\"struct\" href=\"http://docs.rs/parry/0.1.1/parry2d/query/contact_manifolds/contact_manifold/struct.ContactManifold.html\" title=\"struct parry2d::query::contact_manifolds::contact_manifold::ContactManifold\">ContactManifold</a>&lt;ManifoldData, ContactData&gt;<div class=\"where\">where\n    ContactData: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.new\" class=\"method\"><a class=\"src rightside\" href=\"http://docs.rs/parry/0.1.1/src/parry2d/query/contact_manifolds/contact_manifold.rs.html#112-114\">source</a><h4 class=\"code-header\">pub fn <a href=\"http://docs.rs/parry/0.1.1/parry2d/query/contact_manifolds/contact_manifold/struct.ContactManifold.html#tymethod.new\" class=\"fn\">new</a>() -&gt; <a class=\"struct\" href=\"http://docs.rs/parry/0.1.1/parry2d/query/contact_manifolds/contact_manifold/struct.ContactManifold.html\" title=\"struct parry2d::query::contact_manifolds::contact_manifold::ContactManifold\">ContactManifold</a>&lt;ManifoldData, ContactData&gt;<div class=\"where\">where\n    ManifoldData: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a>,</div></h4></section></summary><div class=\"docblock\"><p>Create a new empty contact-manifold.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.with_data\" class=\"method\"><a class=\"src rightside\" href=\"http://docs.rs/parry/0.1.1/src/parry2d/query/contact_manifolds/contact_manifold.rs.html#120\">source</a><h4 class=\"code-header\">pub fn <a href=\"http://docs.rs/parry/0.1.1/parry2d/query/contact_manifolds/contact_manifold/struct.ContactManifold.html#tymethod.with_data\" class=\"fn\">with_data</a>(\n    subshape1: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.u32.html\">u32</a>,\n    subshape2: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.u32.html\">u32</a>,\n    data: ManifoldData,\n) -&gt; <a class=\"struct\" href=\"http://docs.rs/parry/0.1.1/parry2d/query/contact_manifolds/contact_manifold/struct.ContactManifold.html\" title=\"struct parry2d::query::contact_manifolds::contact_manifold::ContactManifold\">ContactManifold</a>&lt;ManifoldData, ContactData&gt;</h4></section></summary><div class=\"docblock\"><p>Create a new empty contact-manifold with the given associated data.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.take\" class=\"method\"><a class=\"src rightside\" href=\"http://docs.rs/parry/0.1.1/src/parry2d/query/contact_manifolds/contact_manifold.rs.html#137-139\">source</a><h4 class=\"code-header\">pub fn <a href=\"http://docs.rs/parry/0.1.1/parry2d/query/contact_manifolds/contact_manifold/struct.ContactManifold.html#tymethod.take\" class=\"fn\">take</a>(&amp;mut self) -&gt; <a class=\"struct\" href=\"http://docs.rs/parry/0.1.1/parry2d/query/contact_manifolds/contact_manifold/struct.ContactManifold.html\" title=\"struct parry2d::query::contact_manifolds::contact_manifold::ContactManifold\">ContactManifold</a>&lt;ManifoldData, ContactData&gt;<div class=\"where\">where\n    ManifoldData: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,</div></h4></section></summary><div class=\"docblock\"><p>Clones <code>self</code> and then remove all contact points from <code>self</code>.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.contacts\" class=\"method\"><a class=\"src rightside\" href=\"http://docs.rs/parry/0.1.1/src/parry2d/query/contact_manifolds/contact_manifold.rs.html#175\">source</a><h4 class=\"code-header\">pub fn <a href=\"http://docs.rs/parry/0.1.1/parry2d/query/contact_manifolds/contact_manifold/struct.ContactManifold.html#tymethod.contacts\" class=\"fn\">contacts</a>(&amp;self) -&gt; &amp;[<a class=\"struct\" href=\"jumpy/core/physics/rapier/struct.TrackedContact.html\" title=\"struct jumpy::core::physics::rapier::TrackedContact\">TrackedContact</a>&lt;ContactData&gt;]</h4></section></summary><div class=\"docblock\"><p>The slice of all the contacts, active or not, on this contact manifold.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.try_update_contacts\" class=\"method\"><a class=\"src rightside\" href=\"http://docs.rs/parry/0.1.1/src/parry2d/query/contact_manifolds/contact_manifold.rs.html#181\">source</a><h4 class=\"code-header\">pub fn <a href=\"http://docs.rs/parry/0.1.1/parry2d/query/contact_manifolds/contact_manifold/struct.ContactManifold.html#tymethod.try_update_contacts\" class=\"fn\">try_update_contacts</a>(\n    &amp;mut self,\n    pos12: &amp;<a class=\"struct\" href=\"jumpy/core/physics/rapier/nalgebra/struct.Isometry.html\" title=\"struct jumpy::core::physics::rapier::nalgebra::Isometry\">Isometry</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.f32.html\">f32</a>, <a class=\"struct\" href=\"jumpy/core/physics/rapier/nalgebra/struct.Unit.html\" title=\"struct jumpy::core::physics::rapier::nalgebra::Unit\">Unit</a>&lt;<a class=\"struct\" href=\"jumpy/core/physics/rapier/nalgebra/struct.Complex.html\" title=\"struct jumpy::core::physics::rapier::nalgebra::Complex\">Complex</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.f32.html\">f32</a>&gt;&gt;, 2&gt;,\n) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.bool.html\">bool</a></h4></section></summary><div class=\"docblock\"><p>Attempts to use spatial coherence to update contacts points.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.try_update_contacts_eps\" class=\"method\"><a class=\"src rightside\" href=\"http://docs.rs/parry/0.1.1/src/parry2d/query/contact_manifolds/contact_manifold.rs.html#191-196\">source</a><h4 class=\"code-header\">pub fn <a href=\"http://docs.rs/parry/0.1.1/parry2d/query/contact_manifolds/contact_manifold/struct.ContactManifold.html#tymethod.try_update_contacts_eps\" class=\"fn\">try_update_contacts_eps</a>(\n    &amp;mut self,\n    pos12: &amp;<a class=\"struct\" href=\"jumpy/core/physics/rapier/nalgebra/struct.Isometry.html\" title=\"struct jumpy::core::physics::rapier::nalgebra::Isometry\">Isometry</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.f32.html\">f32</a>, <a class=\"struct\" href=\"jumpy/core/physics/rapier/nalgebra/struct.Unit.html\" title=\"struct jumpy::core::physics::rapier::nalgebra::Unit\">Unit</a>&lt;<a class=\"struct\" href=\"jumpy/core/physics/rapier/nalgebra/struct.Complex.html\" title=\"struct jumpy::core::physics::rapier::nalgebra::Complex\">Complex</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.f32.html\">f32</a>&gt;&gt;, 2&gt;,\n    angle_dot_threshold: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.f32.html\">f32</a>,\n    dist_sq_threshold: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.f32.html\">f32</a>,\n) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.bool.html\">bool</a></h4></section></summary><div class=\"docblock\"><p>Attempts to use spatial coherence to update contacts points, using user-defined tolerances.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.match_contacts\" class=\"method\"><a class=\"src rightside\" href=\"http://docs.rs/parry/0.1.1/src/parry2d/query/contact_manifolds/contact_manifold.rs.html#232\">source</a><h4 class=\"code-header\">pub fn <a href=\"http://docs.rs/parry/0.1.1/parry2d/query/contact_manifolds/contact_manifold/struct.ContactManifold.html#tymethod.match_contacts\" class=\"fn\">match_contacts</a>(&amp;mut self, old_contacts: &amp;[<a class=\"struct\" href=\"jumpy/core/physics/rapier/struct.TrackedContact.html\" title=\"struct jumpy::core::physics::rapier::TrackedContact\">TrackedContact</a>&lt;ContactData&gt;])</h4></section></summary><div class=\"docblock\"><p>Copy data associated to contacts from <code>old_contacts</code> to the new contacts in <code>self</code>\nbased on matching their feature-ids.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.match_contacts_using_positions\" class=\"method\"><a class=\"src rightside\" href=\"http://docs.rs/parry/0.1.1/src/parry2d/query/contact_manifolds/contact_manifold.rs.html#245-249\">source</a><h4 class=\"code-header\">pub fn <a href=\"http://docs.rs/parry/0.1.1/parry2d/query/contact_manifolds/contact_manifold/struct.ContactManifold.html#tymethod.match_contacts_using_positions\" class=\"fn\">match_contacts_using_positions</a>(\n    &amp;mut self,\n    old_contacts: &amp;[<a class=\"struct\" href=\"jumpy/core/physics/rapier/struct.TrackedContact.html\" title=\"struct jumpy::core::physics::rapier::TrackedContact\">TrackedContact</a>&lt;ContactData&gt;],\n    dist_threshold: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.f32.html\">f32</a>,\n)</h4></section></summary><div class=\"docblock\"><p>Copy data associated to contacts from <code>old_contacts</code> to the new contacts in <code>self</code>\nbased on matching the contact positions.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clear\" class=\"method\"><a class=\"src rightside\" href=\"http://docs.rs/parry/0.1.1/src/parry2d/query/contact_manifolds/contact_manifold.rs.html#264\">source</a><h4 class=\"code-header\">pub fn <a href=\"http://docs.rs/parry/0.1.1/parry2d/query/contact_manifolds/contact_manifold/struct.ContactManifold.html#tymethod.clear\" class=\"fn\">clear</a>(&amp;mut self)</h4></section></summary><div class=\"docblock\"><p>Removes all the contacts from <code>self</code>.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.find_deepest_contact\" class=\"method\"><a class=\"src rightside\" href=\"http://docs.rs/parry/0.1.1/src/parry2d/query/contact_manifolds/contact_manifold.rs.html#269\">source</a><h4 class=\"code-header\">pub fn <a href=\"http://docs.rs/parry/0.1.1/parry2d/query/contact_manifolds/contact_manifold/struct.ContactManifold.html#tymethod.find_deepest_contact\" class=\"fn\">find_deepest_contact</a>(&amp;self) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.81.0/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;&amp;<a class=\"struct\" href=\"jumpy/core/physics/rapier/struct.TrackedContact.html\" title=\"struct jumpy::core::physics::rapier::TrackedContact\">TrackedContact</a>&lt;ContactData&gt;&gt;</h4></section></summary><div class=\"docblock\"><p>Returns the contact with the smallest distance (i.e. the largest penetration depth).</p>\n</div></details></div></details>",0,"jumpy::core::physics::rapier::ContactManifold"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-ContactManifold%3CManifoldData,+ContactData%3E\" class=\"impl\"><a class=\"src rightside\" href=\"http://docs.rs/parry/0.1.1/src/parry2d/query/contact_manifolds/contact_manifold.rs.html#73\">source</a><a href=\"#impl-Debug-for-ContactManifold%3CManifoldData,+ContactData%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;ManifoldData, ContactData&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"http://docs.rs/parry/0.1.1/parry2d/query/contact_manifolds/contact_manifold/struct.ContactManifold.html\" title=\"struct parry2d::query::contact_manifolds::contact_manifold::ContactManifold\">ContactManifold</a>&lt;ManifoldData, ContactData&gt;<div class=\"where\">where\n    ManifoldData: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>,\n    ContactData: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"http://docs.rs/parry/0.1.1/src/parry2d/query/contact_manifolds/contact_manifold.rs.html#73\">source</a><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.81.0/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/1.81.0/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.81.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.81.0/core/fmt/struct.Error.html\" title=\"struct core::fmt::Error\">Error</a>&gt;</h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/1.81.0/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","jumpy::core::physics::rapier::ContactManifold"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Default-for-ContactManifold%3CManifoldData,+ContactData%3E\" class=\"impl\"><a class=\"src rightside\" href=\"http://docs.rs/parry/0.1.1/src/parry2d/query/contact_manifolds/contact_manifold.rs.html#73\">source</a><a href=\"#impl-Default-for-ContactManifold%3CManifoldData,+ContactData%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;ManifoldData, ContactData&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"http://docs.rs/parry/0.1.1/parry2d/query/contact_manifolds/contact_manifold/struct.ContactManifold.html\" title=\"struct parry2d::query::contact_manifolds::contact_manifold::ContactManifold\">ContactManifold</a>&lt;ManifoldData, ContactData&gt;<div class=\"where\">where\n    ManifoldData: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a>,\n    ContactData: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.default\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"http://docs.rs/parry/0.1.1/src/parry2d/query/contact_manifolds/contact_manifold.rs.html#73\">source</a><a href=\"#method.default\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.81.0/core/default/trait.Default.html#tymethod.default\" class=\"fn\">default</a>() -&gt; <a class=\"struct\" href=\"http://docs.rs/parry/0.1.1/parry2d/query/contact_manifolds/contact_manifold/struct.ContactManifold.html\" title=\"struct parry2d::query::contact_manifolds::contact_manifold::ContactManifold\">ContactManifold</a>&lt;ManifoldData, ContactData&gt;</h4></section></summary><div class='docblock'>Returns the “default value” for a type. <a href=\"https://doc.rust-lang.org/1.81.0/core/default/trait.Default.html#tymethod.default\">Read more</a></div></details></div></details>","Default","jumpy::core::physics::rapier::ContactManifold"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()