QUIZZES.t2 = {
  questions: [
    {
      q: "What type does <code>names.get(&amp;7)</code> return, for <code>names: HashMap&lt;i64, &amp;str&gt;</code>?",
      options: [
        { t: "<code>&amp;str</code>", why: "If it did, what would it return for a key that isn't there? Rust has no null." },
        { t: "<code>Option&lt;&amp;&amp;str&gt;</code>", ok: true },
        { t: "<code>Result&lt;&amp;str, KeyError&gt;</code>", why: "A missing key isn't an error in the map's eyes, just an absence. That's what <code>Option</code> is for." },
      ],
      explain: "<code>get</code> borrows the value if it's there. The value is a <code>&amp;str</code>, so a borrow of it is <code>&amp;&amp;str</code>, wrapped in <code>Option</code> for the missing case. <code>.copied()</code> removes one layer.",
    },
    {
      q: "What does <code>*totals.entry(id).or_insert(0) += amount;</code> do when <code>id</code> isn't in the map yet?",
      options: [
        { t: "Panics, because the key is missing", why: "That's what <code>totals[&amp;id]</code> would do. The entry API exists so you don't have to check first." },
        { t: "Inserts 0 for <code>id</code>, then adds <code>amount</code> to it", ok: true },
        { t: "Does nothing until you call <code>insert</code>", why: "<code>or_insert</code> does the inserting, and returns a mutable reference to the new value." },
      ],
      explain: "One hash, one lookup: find the slot, fill it if empty, and hand back a reference you can change through with <code>*</code>.",
    },
    {
      q: "Your map is <code>HashMap&lt;&amp;str, u32&gt;</code> with keys borrowed from <code>suppliers</code>. What happens if you <code>drop(suppliers)</code> and then print the map?",
      options: [
        { t: "It prints fine: the map copied the names", why: "A <code>&amp;str</code> key is a view, not a copy. Copying would need <code>String</code> keys." },
        { t: "It compiles, but prints garbage", why: "That's C's behavior. Rust's borrow checker refuses to compile it." },
        { t: "It doesn't compile: <code>suppliers</code> is still borrowed", ok: true },
      ],
      explain: "Error E0505. A borrowed key can't outlive the data it points into. Own the keys with <code>String</code> if the map must outlive the rows.",
    },
    {
      q: "Why won't <code>rows.sort_by_key(|r| r.total)</code> compile when <code>total</code> is an <code>f64</code>?",
      options: [
        { t: "Floats have no total order, because <code>NaN</code> compares as neither less, equal nor greater", ok: true },
        { t: "Sorting floats is too slow, so Rust forbids it", why: "Speed has nothing to do with it. <code>total_cmp</code> sorts floats just as fast." },
        { t: "<code>sort_by_key</code> only works on integers", why: "It works on any key that implements <code>Ord</code>: strings, tuples, your own types." },
      ],
      explain: "<code>f64</code> implements <code>PartialOrd</code> but not <code>Ord</code>. <code>a.total_cmp(&amp;b)</code> supplies a total order on purpose.",
    },
    {
      q: "You write the CSV by iterating the totals <code>HashMap</code> directly. What goes wrong?",
      options: [
        { t: "Nothing, the order is just the insertion order", why: "That's Python's <code>dict</code>. Rust's <code>HashMap</code> promises no order at all." },
        { t: "The row order can change from one run to the next", ok: true },
        { t: "It won't compile", why: "It compiles. Iterating a map is allowed; relying on its order is the mistake." },
      ],
      explain: "Each <code>HashMap</code> gets a randomly seeded hasher, to defend against hash flooding. Sort into a <code>Vec</code> before you write.",
    },
    {
      q: "Python's <code>list.sort()</code> and Rust's <code>sort_by</code> are both stable. What does stable mean?",
      options: [
        { t: "It never panics", why: "Stability is about equal items, not crashes." },
        { t: "Items that compare equal keep their original order", ok: true },
        { t: "It always takes the same amount of time", why: "Run time depends on the input. Stability is about which equal item comes first." },
      ],
      explain: "Sort by name, then stable-sort by total, and ties stay alphabetical. A tie-break with <code>then_with</code> does the same in one pass, and says so explicitly.",
    },
  ],
  tryit: "Print your totals map with <code>{:?}</code> three times, in three separate runs. Then do the same with a <code>BTreeMap</code> (also in <code>std::collections</code>). What's different, and what would it cost you to use one?",
};
