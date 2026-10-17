QUIZZES.t4 = {
  questions: [
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
      q: "Your function returns <code>Result&lt;u32, SlipError&gt;</code> and uses <code>?</code> on a <code>Result&lt;u32, ParseIntError&gt;</code>. What makes that compile?",
      options: [
        { t: "<code>impl From&lt;ParseIntError&gt; for SlipError</code>", ok: true },
        { t: "<code>impl Display for SlipError</code>", why: "<code>Display</code> is about printing. <code>?</code> converts errors with <code>From</code>." },
        { t: "<code>#[derive(Debug)]</code> on <code>SlipError</code>", why: "<code>Debug</code> is needed for <code>{:?}</code> and for returning errors from <code>main</code>, but it doesn't convert anything." },
      ],
      explain: "<code>?</code> on <code>Err(e)</code> returns <code>Err(From::from(e))</code>. Use <code>map_err</code> instead when the conversion needs context that <code>From</code> can't know, such as a row number.",
    },
    {
      q: "Which functions can a test in <code>tests/millbrook.rs</code> call?",
      options: [
        { t: "Only the library's <code>pub</code> items, imported by the package name", ok: true },
        { t: "Anything in the project, private or not", why: "That's true of unit tests in a <code>mod tests</code> inside the file, through <code>use super::*</code>. Integration tests are separate programs and see what outside code sees." },
        { t: "Only <code>main</code>", why: "Integration tests can't call a binary's <code>main</code>. That's one reason the logic belongs in <code>lib.rs</code>." },
      ],
      explain: "Integration tests use your library the way a stranger would. If something is hard to test from there, your public interface may be the problem.",
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
