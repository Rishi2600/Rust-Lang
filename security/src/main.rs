use std::borrow::Cow;

fn filter_profanity(input: &str) -> Cow<str> {
    if input.contains("bad_word") {
        // We MUST allocate a new string to change it
        Cow::Owned(input.replace("bad_word", "***"))
    } else {
        // No bad words? Just return the reference we were given! No allocation.
        Cow::Borrowed(input)
    }
}