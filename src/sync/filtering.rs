use crate::config;

pub fn find_closest_episode(episodes: Vec<String>, target_episode: u8) -> Option<String> {
    let possible_matches: Vec<String> = episodes.into_iter()
        .filter(|episode| episode.contains(&format!(" {:0>2}.", target_episode)))
        .collect();

    let perfect_match = apply_filters(&possible_matches);

    if perfect_match.is_some() {
        perfect_match
    } else {
        println!("Trying closest match...");
        possible_matches.into_iter().next()
    }
}


fn apply_filters(episodes: &Vec<String>) -> Option<String> {
    let filters = config::get_filters();

    let mut filtered = episodes.clone();
    for filter in filters.contains {
        let try_filter: Vec<&String> = filtered.iter().filter(|&episode| episode.contains(&filter)).collect();
        if !try_filter.is_empty() {
            filtered = try_filter.into_iter().map(|ep| ep.clone()).collect();
        }
    }

    filtered = filtered.into_iter().filter(|episode| {
        for banned_term in filters.not_contains.iter() {
           if episode.contains(banned_term) {
                return false;
           }
        }
        true
    }).collect();

    filtered.into_iter().next()
}