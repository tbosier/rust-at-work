QUIZZES.t4 = {
  questions: [
    {
      q: "Why does <code>Shelf</code> have a <code>new(aisle, number)</code> that checks, as well as <code>parse(text)</code>?",
      options: [
        { t: "So code that already has a letter and a number gets the same check as code that has text", ok: true },
        { t: "Because <code>parse</code> can't return a <code>Result</code>", why: "It does return one. It just leaves the rule about each aisle's size to <code>new</code>." },
        { t: "So tests can build a shelf without any check", why: "Tests go through <code>new</code> too. The private fields mean nothing outside the module can skip it." },
      ],
      explain: "With the fields private and every road through <code>new</code>, \"a <code>Shelf</code> exists\" means \"that shelf exists in the building\". A date type works the same way.",
    },
    {
      q: "What does <code>\"+42\".parse::&lt;u32&gt;()</code> return?",
      options: [
        { t: "<code>Ok(42)</code>", ok: true },
        { t: "<code>Err(InvalidDigit)</code>", why: "That's what you might hope. A leading plus sign is legal for Rust's integer parser, so it parses." },
        { t: "It panics", why: "<code>parse</code> never panics. It returns a <code>Result</code>, and you choose what happens on <code>Err</code>." },
      ],
      explain: "So <code>parse</code> alone doesn't prove the text was only digits. Check with <code>bytes().all(|b| b.is_ascii_digit())</code> first, plus an emptiness check, if digits are the rule.",
    },
    {
      q: "Why parse <code>\"1,204.50\"</code> straight into integer cents instead of into an <code>f64</code> first?",
      options: [
        { t: "Most decimal amounts have no exact binary form, so the float route can land a cent off, and <code>as i64</code> truncates", ok: true },
        { t: "Integers are faster to add", why: "They are, a little, but that isn't the reason. A wrong cent is the problem, not speed." },
        { t: "<code>f64</code> can't hold numbers as large as an invoice", why: "An <code>f64</code> holds integers exactly up to about 9 quadrillion. The trouble is fractions like 0.29, not size." },
      ],
      explain: "<code>0.29 * 100.0</code> is <code>28.999999999999996</code>, and <code>as i64</code> makes it 28. The text already contains the exact digits, so read them as integers.",
    },
    {
      q: "A Millbrook row has <code>TBD</code> as its amount. What should the program do?",
      options: [
        { t: "Write the row to rejects with the field and a reason, and carry on with the next row", ok: true },
        { t: "Panic, so someone notices", why: "Dana's email rules this out. A panic stops the whole month-end run for one bad row, and loses everything after it." },
        { t: "Treat it as $0.00", why: "That turns bad data into a confident wrong answer: the PO would show up as underbilled, and nobody would know why." },
      ],
      explain: "Bad input is an expected outcome, not a bug in your program. It belongs in a <code>Result</code>, and from there in the rejects file.",
    },
    {
      q: "A column loads as <code>Option&lt;String&gt;</code> and your parser takes <code>&amp;str</code>. Which call gets you there without copying the text?",
      options: [
        { t: "<code>row.vendor.as_deref().unwrap_or(\"\")</code>", ok: true },
        { t: "<code>row.vendor.clone().unwrap_or_default()</code>", why: "That gives a <code>String</code>, not a <code>&amp;str</code>, so you'd still need <code>&amp;</code> in front of it to call the parser. And <code>clone</code> copies the whole text first. <code>as_deref</code> borrows it." },
        { t: "<code>row.vendor.unwrap().as_str()</code>", why: "<code>unwrap</code> panics on <code>NULL</code>, which the schema allows. It also moves the <code>String</code> out of the row." },
      ],
      explain: "<code>as_deref</code> turns <code>&amp;Option&lt;String&gt;</code> into <code>Option&lt;&amp;str&gt;</code>. <code>unwrap_or(\"\")</code> then can't panic: a missing value becomes empty text, which your parser already rejects.",
    },
    {
      q: "What does <code>\"  a   b \".split_whitespace().collect::&lt;Vec&lt;_&gt;&gt;().join(\" \")</code> give?",
      options: [
        { t: "<code>\"a b\"</code>", ok: true },
        { t: "<code>\" a b \"</code>", why: "<code>split_whitespace</code> skips the empty pieces at both ends, so nothing is left to join there." },
        { t: "<code>\"a   b\"</code>", why: "Splitting throws the spaces away entirely. <code>join</code> puts back exactly one between each pair of pieces." },
      ],
      explain: "Split, then join, is the usual way to trim the ends and squeeze every run of spaces down to one, in a single line.",
    },
    {
      q: "Every one of your 40 tests passes. What do you now know?",
      options: [
        { t: "Every case you thought to write behaves the way you expected", ok: true },
        { t: "The parser has no bugs", why: "Dijkstra: testing shows the presence of bugs, never their absence. The inputs you didn't imagine are untested." },
        { t: "The parser will accept every valid row in next month's file", why: "Only if next month's valid rows look like the ones you tested. New clerks find new ways to type things." },
      ],
      explain: "That's still worth a lot, especially when the cases come from equivalence classes and their boundaries. Just don't mistake it for a proof.",
    },
  ],
  tryit: "In the wiki's <code>parse_duration</code>, delete the <code>if number.is_empty()</code> check and run the tests. Before you look: which tests fail, and what error does <code>\"fortnight\"</code> get now? Work out why from the code, then put the check back.",
};
