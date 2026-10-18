QUIZZES.t6 = {
  questions: [
    {
      q: "In <code>trait TableRow { fn headers() -&gt; &amp;'static [&amp;'static str]; fn cells(&amp;self) -&gt; Vec&lt;String&gt;; }</code>, why does <code>headers</code> have no <code>self</code>?",
      options: [
        { t: "The column names belong to the type, not to any one row, so there's no row to ask", ok: true },
        { t: "Methods without <code>self</code> run faster", why: "Speed has nothing to do with it. The question is whether the answer depends on a particular value. Every book has the same columns." },
        { t: "So the trait can be used as <code>dyn TableRow</code>", why: "The opposite. A function with no <code>self</code> is exactly what stops a trait being dyn compatible: a trait object has a value but has forgotten its type, so there'd be nothing to call <code>headers</code> on." },
      ],
      explain: "An associated function is called on the type, <code>T::headers()</code>. Inside a generic function the type is known, so that works. Through <code>dyn</code> it isn't, so it doesn't.",
    },
    {
      q: "<code>fn print_table&lt;T: TableRow&gt;(rows: &amp;[T])</code> is called with <code>&amp;books</code> and with <code>&amp;loans</code>. How many copies of <code>print_table</code> does the compiler generate?",
      options: [
        { t: "Two, one for each type", ok: true },
        { t: "One, which looks up the right <code>cells</code> at run time", why: "That's how <code>dyn TableRow</code> works, through a vtable. Generics are monomorphized: the compiler writes a copy per type, each calling its type's method directly." },
        { t: "None: generic functions are interpreted", why: "Rust has no interpreter. Generic code is compiled, once per type it's used with." },
      ],
      explain: "Monomorphization: fast calls the optimizer can inline, paid for in binary size and compile time. The optimizer may later inline or merge copies, so the finished binary can hold fewer.",
    },
    {
      q: "Priya says a column \"is useless if I can't SUM it\". Which way of writing <code>Cents(120450)</code> keeps her happy?",
      options: [
        { t: "<code>write_number_with_format(row, col, 1204.50, &amp;money)</code> with a currency number format", ok: true },
        { t: "<code>write_string(row, col, \"$1,204.50\")</code>", why: "It looks identical on screen, but it's text. A spreadsheet's SUM skips text, so the total comes out as 0 or as a sum of only the cells that happen to be numbers." },
        { t: "<code>write_number(row, col, 120450.0)</code>, and tell finance it's in cents", why: "It sums, but every reader now has to remember to divide by 100, and every chart and pivot table is off by a factor of a hundred. Store the value in the unit people read, and let the format do the display." },
      ],
      explain: "Numbers as numbers, display as format. The <code>f64</code> conversion happens only at this last step, after all the arithmetic was done in integer cents.",
    },
    {
      q: "Your pack prints <code>error: no such month</code> and then returns from <code>main</code> normally. The scheduler that runs it every night will:",
      options: [
        { t: "Record the run as a success, because the exit status was 0", ok: true },
        { t: "Read the message and mark the run as failed", why: "Schedulers, cron, CI and shells decide by the exit status alone. They don't read your text, and it may not even be kept." },
        { t: "Retry it until the message goes away", why: "A scheduler retries on a non-zero status, if it's set up to. With status 0 there's nothing to retry." },
      ],
      explain: "Failing properly means a non-zero exit status, with the message on standard error. <code>ExitCode::FAILURE</code>, or returning <code>Err</code> from <code>main</code>, both give status 1.",
    },
    {
      q: "Where should <code>--month</code> be checked for being a real month?",
      options: [
        { t: "In clap's <code>value_parser</code>, so a bad month never gets past parsing the arguments", ok: true },
        { t: "Just before writing the Spend sheet, where it's used", why: "By then the program has opened the database, loaded everything and reconciled. Worse, every other place that uses the month has to trust it or check again. Checking at the edge means one check, and an early, clear error." },
        { t: "Nowhere: an unknown month just gives an empty Spend sheet", why: "Then <code>2026-13</code> produces a valid-looking workbook with zero spend, and finance might file it. A silent wrong answer is worse than a loud failure." },
      ],
      explain: "Parse at the boundary, then trust the type: the same idea as ticket 4's parsers, applied to the command line.",
    },
    {
      q: "A <code>&amp;dyn Describe</code> is 16 bytes and a <code>&amp;Book</code> is 8. What are the extra 8?",
      options: [
        { t: "A pointer to the vtable: the table of that type's method implementations", ok: true },
        { t: "The size of the value, so it can be copied", why: "The vtable does record the size, but the reference itself carries a pointer to the vtable, not the size." },
        { t: "A reference count", why: "That's <code>Rc</code> and <code>Arc</code>, and the count lives with the value on the heap, not in the pointer." },
      ],
      explain: "A fat pointer: data pointer plus vtable pointer. Every method call through it reads the vtable and jumps.",
    },
  ],
  tryit: "Unzip your own pack (<code>unzip -o out/kettle_pack_2026-09.xlsx -d /tmp/pack</code>) and open <code>xl/worksheets/sheet2.xml</code>. Find one supplier's total. Then change your code to write that column with <code>write_string</code> and <code>format!</code>, rebuild, and look again: the <code>t=</code> attribute tells you what the spreadsheet will think it is. Change it back.",
};
