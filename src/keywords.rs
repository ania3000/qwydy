use crate::token::TokenKind;

/// Приводит два варианта буквы æ к одному написанию.
/// Исходный текст при этом не изменяется.
pub fn normalize_identifier(text: &str) -> String {
    text.chars()
        .map(|ch| match ch {
            'æ' => 'ӕ',
            other => other,
        })
        .collect()
}

/// Возвращает специальный токен, если имя — ключевое слово Qwydy.
pub fn keyword_kind(text: &str) -> Option<TokenKind> {
    match normalize_identifier(text).as_str() {
        "функци" => Some(TokenKind::Function),
        "ивгӕ" => Some(TokenKind::Let),
        "кӕд" => Some(TokenKind::If),
        "ӕндӕра" => Some(TokenKind::Else),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::TokenKind;

    #[test]
    fn recognizes_qwydy_keywords() {
        assert_eq!(keyword_kind("функци"), Some(TokenKind::Function));
        assert_eq!(keyword_kind("ивгӕ"), Some(TokenKind::Let));
        assert_eq!(keyword_kind("кӕд"), Some(TokenKind::If));
        assert_eq!(keyword_kind("æндæра"), Some(TokenKind::Else));
    }

    #[test]
    fn recognizes_both_ae_letters() {
        assert_eq!(keyword_kind("æндæра"), keyword_kind("ӕндӕра"));

        assert_eq!(normalize_identifier("кæд"), normalize_identifier("кӕд"));
    }

    #[test]
    fn ordinary_names_are_not_keywords() {
        assert_eq!(keyword_kind("хъуыды"), None);
    }
}
