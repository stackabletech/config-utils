use super::Escape;

pub struct EnvVarEscaper;

impl Escape for EnvVarEscaper {
    fn escape(line: String) -> String {
        let mut escaped = String::new();
        for c in line.chars() {
            // currently nothing is replaced so this acts as a placeholder
            // for any future refinement
            escaped.push(c)
        }

        escaped
    }
}
