QUIZZES.t0 = {
  questions: [
    {
      q: "<code>fn total(a: i64, b: i64) -&gt; i64 { a + b; }</code> doesn't compile. Why?",
      options: [
        { t: "Rust functions need an explicit <code>return</code>", why: "They don't. The last expression of a block, with no semicolon after it, is its value." },
        { t: "The semicolon turns <code>a + b</code> into a statement, so the body produces <code>()</code>, not an <code>i64</code>", ok: true },
        { t: "<code>i64</code> values can't be added without a cast", why: "Two <code>i64</code>s add fine. Casts are for mixing different number types." },
      ],
      explain: "Delete the semicolon and <code>a + b</code> becomes the function's value. The compiler's help says exactly that.",
    },
    {
      q: "<code>total</code> is an <code>f64</code> and <code>lines.len()</code> is a <code>usize</code>. Which line computes the average?",
      options: [
        { t: "<code>total / lines.len()</code>", why: "Rust never converts between number types on its own, so <code>f64 / usize</code> doesn't compile." },
        { t: "<code>total / lines.len() as f64</code>", ok: true },
        { t: "<code>(total as usize) / lines.len()</code>", why: "That compiles, but it throws away the fraction twice: once in the cast, and again in integer division." },
      ],
      explain: "Convert the count with <code>as f64</code>. Going from a whole number to a float loses nothing at these sizes; the other direction would.",
    },
    {
      q: "Python's <code>skus[-1]</code> gets the last item. In Rust, the safest equivalent is:",
      options: [
        { t: "<code>skus[-1]</code>", why: "Positions are <code>usize</code>, which can't be negative, so this doesn't compile." },
        { t: "<code>skus.last()</code>", ok: true },
        { t: "<code>skus[skus.len()]</code>", why: "Positions start at 0, so the last one is <code>len() - 1</code>. This one is past the end, and it panics." },
      ],
      explain: "<code>last()</code> returns an <code>Option</code>, so an empty list gives <code>None</code> instead of a panic. <code>skus[skus.len() - 1]</code> works too, until the list is empty.",
    },
    {
      q: "After <code>for sku in skus { ... }</code>, the line <code>println!(\"{}\", skus.len());</code> fails to compile. What happened?",
      options: [
        { t: "The loop took the vector and used it up, so <code>skus</code> no longer exists", ok: true },
        { t: "<code>skus</code> was changed during the loop and must be declared <code>mut</code>", why: "Nothing changed it. The loop moved it." },
        { t: "<code>len</code> can't be called on a vector of <code>String</code>s", why: "It can. Any vector has a length, as long as you still have the vector." },
      ],
      explain: "Loop over <code>&amp;skus</code> to borrow the items instead. Ticket 1 explains moves properly.",
    },
    {
      q: "What does <code>\"café\".len()</code> return?",
      options: [
        { t: "4", why: "That's Python's answer, which counts code points. Rust's <code>len</code> counts bytes." },
        { t: "5", ok: true },
        { t: "8, two bytes per character", why: "UTF-8 is variable width: ASCII letters take one byte each, and only the <code>é</code> takes two." },
      ],
      explain: "<code>c</code>, <code>a</code> and <code>f</code> are one byte each and <code>é</code> is two. <code>.chars().count()</code> gives 4.",
    },
    {
      q: "A method adds stock to a bin and changes its <code>on_hand</code> field. Its first parameter should be:",
      options: [
        { t: "<code>&amp;self</code>", why: "<code>&amp;self</code> only reads. The compiler refuses to let it change a field." },
        { t: "<code>&amp;mut self</code>", ok: true },
        { t: "<code>self</code>", why: "That takes the whole bin and uses it up, so the caller couldn't use it afterward." },
      ],
      explain: "<code>&amp;mut self</code> borrows the value to change it, and the caller's variable must be <code>let mut</code>. The signature alone tells a reader which methods can change things.",
    },
  ],
  tryit: "In your <code>week0.rs</code>, change the method that returns a line's value from <code>&amp;self</code> to <code>self</code>, and read the errors at each call. Then change it back, and in your journal write what the difference between the two is.",
};
