use nucleo_matcher::{
    Config, Matcher, Utf32Str,
    pattern::{CaseMatching, Normalization, Pattern},
};

pub struct FuzzyMatcher {
    matcher: Matcher,
    buf: Vec<char>,
}

impl FuzzyMatcher {
    pub fn new() -> Self {
        Self {
            matcher: Matcher::new(Config::DEFAULT),
            buf: Vec::new(),
        }
    }
    pub fn filter<'a>(&mut self, query: &str, names: impl Iterator<Item = &'a str>) -> Vec<usize> {
        if query.is_empty() {
            return names.enumerate().map(|(i, _)| i).collect();
        }

        let pattern = Pattern::parse(query, CaseMatching::Smart, Normalization::Smart);

        let mut scored: Vec<(usize, u32)> = names
            .enumerate()
            .filter_map(|(i, name)| {
                let hay = Utf32Str::new(name, &mut self.buf);
                pattern.score(hay, &mut self.matcher).map(|s| (i, s))
            })
            .collect();

        scored.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        scored.into_iter().map(|(i, _)| i).collect()
    }
}
