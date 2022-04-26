mod comments;

pub struct Interpreter {}

impl Interpreter {
    pub fn run() -> Result<(), ()> {
        let s = comments::strip_comments(
            "This is a//// Comment\n
            \rHi// You shouldn't see this\nRetard\n\"This is a string and not a // Comment\"\n
            \rTest test\n
            \rTest test \"Escaped \\\"test// 1st fake comment\\\" // 2nd fake comment\" // Shouldn't see this hi",
        );
        println!("{}", s);

        Ok(())
    }
}
