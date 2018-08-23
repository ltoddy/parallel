pub mod crypto {
    pub fn base64_encode(input: String) -> String {
        let base64_pad: char = '=';
        let base64en: Vec<char> = vec![
            'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',
            'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P',
            'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X',
            'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f',
            'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n',
            'o', 'p', 'q', 'r', 's', 't', 'u', 'v',
            'w', 'x', 'y', 'z', '0', '1', '2', '3',
            '4', '5', '6', '7', '8', '9', '+', '/'];
        let mut output = String::new();
        let input = input.as_bytes();

        for i in 0..input.len() {
            match i % 3 {
                0 => output.push(base64en[(input[i] >> 2 & 0x3F) as usize]),
                1 => output.push(base64en[(((input[i - 1] & 0x3) << 4) + ((input[i] >> 4) & 0xF)) as usize]),
                2 => {
                    output.push(base64en[(((input[i - 1] & 0xF) << 2) + ((input[i] >> 6) & 0x3)) as usize]);
                    output.push(base64en[(input[i] & 0x3F) as usize]);
                }
                _ => continue
            }
        }

        let i = input.len() - 1;
        if i % 3 == 0 {
            output.push(base64en[((input[i] & 0x3) << 4) as usize]);
            output.push(base64_pad);
            output.push(base64_pad);
        } else if i % 3 == 1 {
            output.push(base64en[((input[i] & 0xF) << 2) as usize]);
            output.push(base64_pad);
        }

        return output;
    }
}
