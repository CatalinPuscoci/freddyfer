pub fn clean_all(string: &str) -> String {
    let chars = string.chars();
    chars.fold("".to_string(), |acc, c| acc + &find_char_match(c))
}

fn find_char_match(c: char) -> String {
    match c {
        'Ă' | 'À' | 'Á' | 'Â' | 'Ã' | 'Ä' | 'Å' | 'Æ' => "A".to_string(),
        'Þ' => "B".to_string(),
        'Ç' | 'Č' => "C".to_string(),
        'Ď' | 'Ð' => "D".to_string(),
        'Ě' | 'È' | 'É' | 'Ê' | 'Ë' => "E".to_string(),
        'Ƒ' => "F".to_string(),
        'Ì' | 'Í' | 'Î' | 'Ï' => "I".to_string(),
        'Ň' | 'Ñ' => "N".to_string(),
        'Ò' | 'Ó' | 'Ô' | 'Õ' | 'Ö' | 'Ø' => "O".to_string(),
        'Ř' => "R".to_string(),
        'ß' => "ss".to_string(),
        'Ș' | 'Š' => "S".to_string(),
        'Ț' | 'Ť' => "T".to_string(),
        'Ů' | 'Ù' | 'Ú' | 'Û' | 'Ü' => "U".to_string(),
        'Ý' => "Y".to_string(),
        'Ž' => "Z".to_string(),

        'ă' | 'à' | 'á' | 'â' | 'ã' | 'ä' | 'å' | 'æ' => "a".to_string(),
        'þ' => "b".to_string(),
        'ç' | 'č' => "c".to_string(),
        'ď' | 'ð' => "d".to_string(),
        'ě' | 'è' | 'é' | 'ê' | 'ë' => "e".to_string(),
        'ƒ' => "f".to_string(),
        'ì' | 'í' | 'î' | 'ï' => "i".to_string(),
        'ñ' | 'ň' => "n".to_string(),
        'ò' | 'ó' | 'ô' | 'õ' | 'ö' | 'ø' => "o".to_string(),
        'ř' => "r".to_string(),
        'ș' | 'š' => "s".to_string(),
        'ț' | 'ť' => "t".to_string(),
        'ů' | 'ù' | 'ú' | 'û' | 'ü' => "u".to_string(),
        'ý' | 'ÿ' => "y".to_string(),
        'ž' => "z".to_string(),
        _ => c.to_string(),
    }
}
