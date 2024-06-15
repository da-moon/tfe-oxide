use {rand::Rng, regex::Regex};
// ────────────────────────────────────────────────────────────
/// the following is the standard ascii character range
const STANDARD_ASCII_RANGE: std::ops::Range<usize> = 0..128;
/// the following Struct allows generation a random strings based on a given regex character class.
#[derive(Clone, PartialEq, Eq)]
pub struct Random {
    /// regex character set in which random characters are picked from
    charset: String,
    _internal: (),
}
impl Random {
    /// generates a new random string generator that uses a custom regex character class
    pub fn new(charset: String) -> Self {
        Random {
            charset,
            _internal: (),
        }
    }
    /// generates a random string of the specified length
    #[allow(dead_code)]
    pub fn generate(&self, length: usize) -> String {
        let mut rng = rand::thread_rng();
        let distr = rand::distributions::Uniform::new_inclusive(
            STANDARD_ASCII_RANGE.start,
            STANDARD_ASCII_RANGE.end,
        );
        let re = Regex::new(self.charset.as_str()).unwrap();
        (0..)
            .into_iter()
            .map(|_| ((rng.sample(distr) as u8) as char).to_string())
            .filter(|input| re.is_match(input.as_str()))
            .take(length)
            .collect::<String>()
    }
}
// ────────────────────────────────────────────────────────────
impl Default for Random {
    /// Generates a new random string generator that uses alpha-numeric character set
    fn default() -> Self {
        Random {
            charset: "[[:alpha:]]".to_string(),
            _internal: (),
        }
    }
}
// ────────────────────────────────────────────────────────────
/// This function generates a random alphanumeric string. it is not exported as it is deprecated and kept here just in case.
fn _generate_random_string(length: usize) -> String {
    #[allow(unused_mut)]
    let mut rng = rand::thread_rng();
    // Generate a random string of the specified length
    rng.sample_iter(&rand::distributions::Alphanumeric)
        .take(length)
        .map(char::from)
        .collect::<String>()
}
#[cfg(test)]
mod tests {
    // cargo test --all-targets -- "utils::random::tests" --nocapture
    // cargo watch -cx 'test --all-targets -- "utils::random::tests" --nocapture'
    use super::*;
    use regex::Regex;
    #[test]
    fn random_alphanumeric_passing() {
        let length: usize = 10;
        let charset: &str = r"^[a-zA-Z0-9]+$";
        let re = Regex::new(charset).unwrap();
        let generator = Random::default();
        let actual = generator.generate(length);
        assert_eq!(length, actual.len());
        if length > 0 {
            assert!(re.is_match(actual.as_str()));
        }
    }
    #[test]
    fn random_special_chars_passing() {
        let length: usize = 10;
        let charset: &str = r"^[&#@!%]+$";
        let re = Regex::new(charset).unwrap();
        let generator = Random::new(charset.to_string());
        let actual = generator.generate(length);
        assert_eq!(length, actual.len());
        if length > 0 {
            assert!(re.is_match(actual.as_str()));
        }
    }
    #[test]
    fn random_custom_regex_passing() {
        let length: usize = 200;
        let charset: &str = r"^[[&^#$@!%][:alnum:]]+$";
        let re = Regex::new(charset).unwrap();
        let generator = Random::new(charset.to_string());
        let actual = generator.generate(length);
        assert_eq!(length, actual.len());
        if length > 0 {
            assert!(re.is_match(actual.as_str()));
        }
    }
}
// ────────────────────────────────────────────────────────────
// vim: filetype=rust syntax=rust softtabstop=4 tabstop=4 shiftwidth=4 textwidth=79 fileencoding=utf-8 expandtab
// code: language=rust insertSpaces=true tabSize=4
