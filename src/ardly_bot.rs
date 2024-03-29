use regex::Regex;

pub fn response(mut input: String) -> Option<String> {
    input.make_ascii_lowercase();

    // avoid self replies
    if input.contains("know 'er") {
        return None;
    }

    if input.contains("liquor")
    || input.contains("liqueur") {
        return Some("lick 'er? I 'ardly know 'er!".to_owned());
    }

    if input.contains("parappa")
    || input.contains("rappa") {
        return Some("Kick, punch, block, it's all in the mind.".to_owned());
    }

    // TODO make this a lazy static.
    let er_regex = Regex::new(r"(\P{White_Space}+)er(s|ed|ing)?(\p{White_Space}|[\.!?,]|$)").unwrap();

    let mut best_word = "";
    for captures in er_regex.captures_iter(&input) {
        // 0 is the whole match
        let Some(mut erless_word) = captures.get(1).map(|c| c.as_str()) else {
            continue
        };

        // This seems easier than changing the regex to handle "ererer"
        while erless_word.ends_with("er") {
            erless_word = &erless_word[0..erless_word.len() - 2];
        }

        // TODO? Better bestness criteria?
        if best_word.len() <= erless_word.len() {
            best_word = erless_word;
        }
    }

    if best_word.is_empty() {
        return None;
    }

    if best_word == "rap" {
        return Some("No means no.".to_owned());
    }

    if best_word == "rapp" {
        return Some("What is this? Friday Night Funkin'?".to_owned());
    }

    if best_word == "wrapp" {
        return Some("Like in a blanket? Is she cold?".to_owned());
    }

    if best_word == "jamm" {
        return Some("Um Jammer Lammy? My guitar is in my mind!".to_owned());
    }

    if best_word == "bon" {
        return Some("I'd rather leave 'er bones where they are.".to_owned());
    }

    let mut best_word = best_word.to_owned();

    if (
       // For example, collid-er => collide 'er
       best_word.ends_with("id")
       && !(
           // but also raid-er => raid 'er
           best_word.ends_with("aid")
           // Do these occur in English besides borrowed words where the joke
           // doesn't work anyway? {
           //|| best_word.ends_with("eid")
           //|| best_word.ends_with("iid")
           // }
           || best_word.ends_with("oid")
           // but also guid-er => guide 'er
           //|| best_word.ends_with("uid")
       )
    ) || (
        // pok-er => poke 'er
        best_word.ends_with("ok")
        // but, book-er => book 'er
        && !best_word.ends_with("ook")
    )
    {
        best_word.push('e');
    }

    best_word.push_str(" 'er? I 'ardly know 'er!");

    Some(best_word)
}

#[test]
fn response_works_on_these_examples() {
    macro_rules! a {
        ($input: literal, $expected: expr) => {
            let expected: Option<&str> = $expected;
            let expected: Option<String> = expected.map(|s| s.to_owned());
            assert_eq!(response($input.to_owned()), expected);
        }
    }
    a!("", None);
    a!("booker", Some("book 'er? I 'ardly know 'er!"));
    a!("liquor", Some("lick 'er? I 'ardly know 'er!"));
    a!("Large Hadron Collider", Some("collide 'er? I 'ardly know 'er!"));
    a!("cupholder", Some("cuphold 'er? I 'ardly know 'er!"));
    a!("poker", Some("poke 'er? I 'ardly know 'er!"));
    a!("hand me the fish boner", Some("I'd rather leave 'er bones where they are."));
    a!("raper", Some("No means no."));
    a!("rapper", Some("What is this? Friday Night Funkin'?"));
    a!("I'm gonna work as a present wrapper", Some("Like in a blanket? Is she cold?"));
    a!("PaRappa the Rappa", Some("Kick, punch, block, it's all in the mind."));
    a!("They turned on the radio jammer", Some("Um Jammer Lammy? My guitar is in my mind!"));
    a!("raider", Some("raid 'er? I 'ardly know 'er!"));
    a!("avoider", Some("avoid 'er? I 'ardly know 'er!"));
    // If this test is making things complicated, consider removing it, since this
    // is a rarely used word.
    a!("guider", Some("guide 'er? I 'ardly know 'er!"));
}