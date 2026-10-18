QUIZZES.t5 = {
  questions: [
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
      q: "You moved <code>money.rs</code> out of <code>main.rs</code>. <code>cargo run &amp;&amp; diff -r out_before out</code> prints nothing. What do you know?",
      options: [
        { t: "On today's data, the program's output didn't change", ok: true },
        { t: "The program is correct", why: "The saved output was never proven right. A golden file checks that nothing moved, not that it was right to begin with." },
        { t: "The move can't have broken anything", why: "Only for inputs like today's. A path that today's data never reaches could still be broken. That's what unit tests are for." },
      ],
      explain: "A golden file is a characterization test: cheap, catches any change, and exactly the question a refactor asks. It can't tell a fix from a mistake.",
    },
    {
      q: "What's the difference between <code>mod money;</code> and <code>use crate::money::Cents;</code>?",
      options: [
        { t: "<code>mod</code> declares that the module exists, once, in its parent. <code>use</code> makes a short name for something, in any file", ok: true },
        { t: "They're the same; <code>use</code> is the newer spelling", why: "They do different jobs. Without the <code>mod</code> line, <code>money.rs</code> is never compiled, and no <code>use</code> can reach it." },
        { t: "<code>mod</code> is for files and <code>use</code> is for crates", why: "<code>use</code> works on any path: a module, a type, a function, in your crate or another." },
      ],
      explain: "Python's <code>import</code> does both jobs at once. Rust splits them: <code>mod</code> builds the tree, <code>use</code> gives names to things in it.",
    },
    {
      q: "Next year Millbrook sends a CSV file instead of a table. In a well-split project, what changes?",
      options: [
        { t: "The module that reads Millbrook's rows, plus one new loader. The parsers and the reconciliation don't notice", ok: true },
        { t: "Every module that handles Millbrook data", why: "That's the layout Parnas warned about, split by the steps of the program, where every step knows how the data arrives." },
        { t: "Only <code>main.rs</code>", why: "<code>main.rs</code> should be wiring. If reading a file format lives there, it's doing a module's job." },
      ],
      explain: "Information hiding: each module hides one decision that's likely to change. \"Where the rows come from\" is one of those decisions.",
    },
    {
      q: "A function in <code>src/reconcile.rs</code> is marked <code>pub(crate)</code>. Who can call it?",
      options: [
        { t: "Code anywhere inside the library, but not <code>main.rs</code> or the tests in <code>tests/</code>", ok: true },
        { t: "Everything in the package, <code>main.rs</code> included", why: "<code>main.rs</code> is a separate crate that links to the library. To the compiler it's outside code." },
        { t: "Only code in <code>reconcile.rs</code>", why: "That's what no <code>pub</code> at all gives you: private to the module (and the modules inside it)." },
      ],
      explain: "Each file in <code>tests/</code> and your <code>main.rs</code> are crates of their own. They see the library's <code>pub</code> items and nothing else.",
    },
  ],
  tryit: "Delete the <code>pub</code> in front of the function your integration test calls and run <code>cargo test</code>. Read which crate the error comes from. Then put it back as <code>pub(crate)</code> and predict, before you run it, which crates will build.",
};
