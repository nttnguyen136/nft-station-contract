use cosmwasm_std::Deps;
use regex::Regex;

pub fn username_is_valid(_deps: Deps, username: &str) -> bool {
    // let username_length_valid = validate_username_length(deps, username);
    let username_characters_valid = validate_username_characters(username);

    let username_length_valid = true;
    username_characters_valid && username_length_valid
}

pub fn validate_username_length(_deps: Deps, username: &str) -> bool {
    username.chars().count() > 3 && username.chars().count() <= 20
}

pub fn validate_username_characters(username: &str) -> bool {
    // first check for any characters _other than_ allowed characters
    let invalid_characters: Regex = Regex::new(r"[^a-z0-9_\-]").unwrap();
    let first_check_passed = !invalid_characters.is_match(username);

    // then check for invalid sequence of hyphens or underscores
    // if is_match returns true, it is invalid
    let invalid_hyphens_underscores: Regex = Regex::new(r"[_\-]{2,}").unwrap();
    let second_check_passed = !invalid_hyphens_underscores.is_match(username);

    first_check_passed && second_check_passed
}
