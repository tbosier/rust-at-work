"""python3 tools/test_highlight.py: the tricky cases for tools/highlight.py."""
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from highlight import compiler, plain_text, rust, tokens  # noqa: E402


def kinds(code):
    """(class, text) for the colored tokens only."""
    return [(c, t) for c, t in tokens(code) if c]


class Rust(unittest.TestCase):
    def roundtrip(self, code):
        self.assertEqual(plain_text(rust(code)), code)

    def test_lifetime_and_char(self):
        code = "fn first<'a>(s: &'a str) -> char { let q = 'a'; let e = '\\''; q }"
        got = kinds(code)
        self.assertIn(("sx-ty", "'a"), got)
        self.assertIn(("sx-str", "'a'"), got)
        self.assertIn(("sx-str", "'\\''"), got)
        self.assertEqual([t for c, t in got if c == "sx-ty"].count("'a"), 2)
        self.roundtrip(code)

    def test_static_lifetime(self):
        self.assertIn(("sx-ty", "'static"), kinds("fn h() -> &'static [&'static str];"))

    def test_string_holding_comment_marker(self):
        code = 'let url = "https://example.com"; // real comment'
        got = kinds(code)
        self.assertIn(("sx-str", '"https://example.com"'), got)
        self.assertIn(("sx-com", "// real comment"), got)
        self.roundtrip(code)

    def test_escaped_quote_in_string(self):
        code = r'let s = "say \"hi\" // not a comment"; x'
        self.assertEqual(kinds(code)[1], ("sx-str", r'"say \"hi\" // not a comment"'))

    def test_raw_strings(self):
        code = 'let a = r#"he said "no" // still text"#; let b = r"c:\\dir"; let c = br"x";'
        strs = [t for c, t in kinds(code) if c == "sx-str"]
        self.assertEqual(strs, ['r#"he said "no" // still text"#', 'r"c:\\dir"', 'br"x"'])
        self.roundtrip(code)

    def test_nested_generics_escape(self):
        code = "let m: HashMap<String, Vec<Option<&'a u32>>> = HashMap::new();"
        out = rust(code)
        self.assertNotIn("<String", out)
        self.assertIn("&lt;", out)
        self.assertIn("&amp;", out)
        tys = [t for c, t in kinds(code) if c == "sx-ty"]
        self.assertEqual(tys, ["HashMap", "String", "Vec", "Option", "'a", "u32", "HashMap"])
        self.roundtrip(code)

    def test_macros_attributes_numbers(self):
        code = '#[derive(Debug, Clone)]\nfn main() { println!("{}", 1_000u32 + 0xff); assert!(x != 2.5e3); }'
        got = kinds(code)
        self.assertEqual(got[0], ("sx-attr", "#[derive(Debug, Clone)]"))
        self.assertIn(("sx-mac", "println!"), got)
        self.assertIn(("sx-mac", "assert!"), got)
        self.assertIn(("sx-num", "1_000u32"), got)
        self.assertIn(("sx-num", "0xff"), got)
        self.assertIn(("sx-num", "2.5e3"), got)
        self.roundtrip(code)

    def test_attribute_with_bracket_in_string(self):
        code = '#[error("bad ] row")]\nstruct E;'
        self.assertEqual(kinds(code)[0], ("sx-attr", '#[error("bad ] row")]'))

    def test_tuple_field_and_range(self):
        code = "let x = t.0 + v[1..10].len();"
        nums = [t for c, t in kinds(code) if c == "sx-num"]
        self.assertEqual(nums, ["1", "10"])

    def test_doc_and_block_comments(self):
        code = "/// Docs here.\n//! Crate docs.\n/* outer /* inner */ still */ fn f() {}"
        coms = [t for c, t in kinds(code) if c == "sx-com"]
        self.assertEqual(coms, ["/// Docs here.", "//! Crate docs.", "/* outer /* inner */ still */"])
        self.roundtrip(code)

    def test_identifiers_that_look_like_prefixes(self):
        code = "let b = br; let r = b + rb; let r2 = format!(\"{b}\");"
        got = kinds(code)
        self.assertNotIn("sx-str", [c for c, t in got if t in ("b", "br", "r", "rb")])
        self.roundtrip(code)

    def test_html_in_code_is_escaped(self):
        code = 'let tag = "<script>alert(1)</script>"; if a < b && c > d {}'
        out = rust(code)
        self.assertNotIn("<script>", out)
        self.roundtrip(code)

    def test_non_ascii(self):
        self.roundtrip('let s = "café"; let c = \'é\'; let naïve = 1;')


class Compiler(unittest.TestCase):
    SAMPLE = """error[E0382]: borrow of moved value: `names`
 --> src/main.rs:8:45
  |
6 |     let names = vec![String::from("Ada")];
  |         ----- move occurs because `names` has type `Vec<String>`
8 |     println!("{}", names.len());
  |                    ^^^^^ value borrowed here after move
  |
  = note: expected reference `&str`
  = help: the trait `Ord` is not implemented
help: consider cloning the value
...
warning: unused variable: `x`
error: could not compile `snippets` (bin "snippets") due to 1 previous error"""

    def test_roundtrip(self):
        self.assertEqual(plain_text(compiler(self.SAMPLE)), self.SAMPLE)

    def test_classes(self):
        out = compiler(self.SAMPLE).split("\n")
        self.assertTrue(out[0].startswith('<span class="cc-err">error[E0382]:</span>'))
        self.assertIn('<span class="cc-loc">--&gt;</span>', out[1])
        self.assertTrue(out[3].startswith('<span class="cc-loc">6 |</span>'))
        self.assertIn("vec!", out[3])  # source line inside output stays plain
        self.assertNotIn("sx-", out[3])
        self.assertEqual(out[6], '<span class="cc-loc">  |                    ^^^^^ value borrowed here after move</span>')
        self.assertIn('<span class="cc-note">note:</span>', out[8])
        self.assertIn('<span class="cc-help">help:</span>', out[9])
        self.assertTrue(out[10].startswith('<span class="cc-help">help:</span>'))
        self.assertEqual(out[11], '<span class="cc-loc">...</span>')
        self.assertTrue(out[12].startswith('<span class="cc-warn">warning:</span>'))
        self.assertTrue(out[13].startswith('<span class="cc-err">error:</span>'))
        self.assertIn("Vec&lt;String&gt;", out[4])


if __name__ == "__main__":
    unittest.main()
