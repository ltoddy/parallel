extern crate parallel;

#[cfg(test)]
mod test_crypto {
    use parallel::crypto::*;

    #[test]
    pub fn test_base64_encode() {
        assert_eq!(base64_encode(String::from("9153728a-074f-4510-966f-f3098ddb1bd9")).unwrap(),
                   String::from("OTE1MzcyOGEtMDc0Zi00NTEwLTk2NmYtZjMwOThkZGIxYmQ5"));

        assert_eq!(base64_encode(String::from("2c8d7b62-8dcf-4dfe-a6a2-efe65264a53bx")).unwrap(),
                   String::from("MmM4ZDdiNjItOGRjZi00ZGZlLWE2YTItZWZlNjUyNjRhNTNieA=="));

        assert_eq!(base64_encode(String::from("78b57816-41c5-4b9e-8ca4-32f41626f9ac")).unwrap(),
                   String::from("NzhiNTc4MTYtNDFjNS00YjllLThjYTQtMzJmNDE2MjZmOWFj"));
    }

    #[test]
    pub fn test_base64_decode() {
        assert_eq!(base64_decode(String::from("OTE1MzcyOGEtMDc0Zi00NTEwLTk2NmYtZjMwOThkZGIxYmQ5")).unwrap(),
                   String::from("9153728a-074f-4510-966f-f3098ddb1bd9"));

        assert_eq!(base64_decode(String::from("MmM4ZDdiNjItOGRjZi00ZGZlLWE2YTItZWZlNjUyNjRhNTNieA==")).unwrap(),
                   String::from("2c8d7b62-8dcf-4dfe-a6a2-efe65264a53bx"));

        assert_eq!(base64_decode(String::from("NzhiNTc4MTYtNDFjNS00YjllLThjYTQtMzJmNDE2MjZmOWFj")).unwrap(),
                   String::from("78b57816-41c5-4b9e-8ca4-32f41626f9ac"));
    }
}
