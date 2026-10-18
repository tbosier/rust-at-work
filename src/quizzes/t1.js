QUIZZES.t1 = {
  questions: [
    {
      q: "The <code>received_date</code> column is empty until the parts arrive. Which field type fits it?",
      options: [
        { t: "<code>String</code>, and use <code>\"\"</code> for missing", why: "rusqlite refuses to put a <code>NULL</code> into a <code>String</code>, and an empty string that means \"missing\" is a value every later reader has to remember to check." },
        { t: "<code>Option&lt;String&gt;</code>", ok: true },
        { t: "<code>&amp;str</code>", why: "A <code>&amp;str</code> borrows text someone else owns. The row's text needs an owner, and the struct is it." },
      ],
      explain: "<code>Option</code> puts \"might be missing\" in the type, and the compiler makes every reader deal with <code>None</code>.",
    },
    {
      q: "Why does <code>count_open</code> take <code>pos: &amp;[PurchaseOrder]</code> rather than <code>pos: Vec&lt;PurchaseOrder&gt;</code>?",
      options: [
        { t: "Slices are faster to iterate than vectors", why: "Iterating is the same speed. The difference is who owns the data afterward." },
        { t: "So the caller keeps the purchase orders and can use them again", ok: true },
        { t: "Because a <code>Vec</code> can't be passed to a function", why: "It can. It's moved, and the caller loses it, which is exactly the problem." },
      ],
      explain: "A function that only reads should borrow. Taking the <code>Vec</code> by value moves it in, and it's freed when the function returns.",
    },
    {
      q: "<code>let first = &amp;ids[0];</code>, then <code>ids.push(9);</code>, then <code>println!(\"{first}\");</code>. Why won't it compile?",
      options: [
        { t: "<code>push</code> may move the items to a bigger block of memory, and <code>first</code> would point at the old one", ok: true },
        { t: "You can't take a reference to an element of a <code>Vec</code>", why: "You can, and it's common. The trouble is changing the <code>Vec</code> while that reference is still in use." },
        { t: "<code>ids</code> wasn't declared <code>mut</code>", why: "That's a different error. Even with <code>let mut ids</code>, this fails: a value can have many <code>&amp;</code> borrows or one <code>&amp;mut</code>, never both at once." },
      ],
      explain: "Many readers or one writer. Move the <code>println!</code> above the <code>push</code> and it compiles, because a borrow ends at its last use.",
    },
    {
      q: "What does the <code>?</code> in <code>let pos = load(&amp;db)?;</code> do?",
      options: [
        { t: "Ignores the error and carries on with an empty list", why: "Nothing in Rust ignores errors silently. That's the point." },
        { t: "On error, returns that error from the current function; otherwise unwraps the value", ok: true },
        { t: "Panics with the error message", why: "That's <code>.unwrap()</code> or <code>.expect(...)</code>. <code>?</code> hands the error to the caller instead." },
      ],
      explain: "It's an early return for errors. That's why the function using it must itself return a <code>Result</code>. (Ticket 4 shows that <code>?</code> works the same way on an <code>Option</code>, in a function that returns <code>Option</code>.)",
    },
    {
      q: "What does this print?",
      code: "println!(\"{}\", \"8/11/2026\" < \"9/1/2025\");",
      options: [
        { t: "<code>true</code>", ok: true },
        { t: "<code>false</code>, because August 2026 is after September 2025", why: "That's what the dates mean, but these are strings. Text compares character by character, and <code>'8'</code> comes before <code>'9'</code>, so the comparison is decided on the first character." },
        { t: "It doesn't compile: you can't compare strings with <code>&lt;</code>", why: "You can. <code>&amp;str</code> implements <code>Ord</code>, comparing byte by byte." },
      ],
      explain: "August 2026 sorts before September 2025. ISO dates put the biggest unit first at a fixed width, so text order is time order. US dates don't.",
    },
    {
      q: "Your <code>Vec</code> grew to 2,017 items by pushing one at a time. Roughly how many times did its capacity change?",
      options: [
        { t: "2,017, once per push", why: "That would make pushing slow. A <code>Vec</code> grows its capacity by a factor, so it goes to the allocator rarely." },
        { t: "About 10", ok: true },
        { t: "Once, because it knew the size", why: "It can't know the size of a loop's output in advance unless you tell it with <code>Vec::with_capacity</code>." },
      ],
      explain: "Capacity went 4, 8, 16, and so on up to 2,048: ten changes. Growing by a factor makes a push cost a constant amount on average.",
    },
  ],
  tryit: "Change the stale-PO function to take <code>Vec&lt;PurchaseOrder&gt;</code> instead of a slice, and read the error in <code>main</code>. Then change it back, and in your journal say in one sentence what the compiler was protecting.",
};
