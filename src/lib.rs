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
            '4', '5', '6', '7', '8', '9', '+', '/',
        ]; /* BASE 64 encode table */
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

    pub fn base64_decode(input: String) -> String {
        let base64de: Vec<isize> = vec![
            62, -1, -1, -1, 63, 52, 53, 54, /* '+', ',', '-', '.', '/', '0', '1', '2', */
            55, 56, 57, 58, 59, 60, 61, -1, /* '3', '4', '5', '6', '7', '8', '9', ':', */
            -1, -1, -1, -1, -1, -1, 0, 1, /* ';', '<', '=', '>', '?', '@', 'A', 'B', */
            2, 3, 4, 5, 6, 7, 8, 9, /* 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', */
            10, 11, 12, 13, 14, 15, 16, 17, /* 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', */
            18, 19, 20, 21, 22, 23, 24, 25, /* 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', */
            -1, -1, -1, -1, -1, -1, 26, 27, /* '[', '\', ']', '^', '_', '`', 'a', 'b', */
            28, 29, 30, 31, 32, 33, 34, 35, /* 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', */
            36, 37, 38, 39, 40, 41, 42, 43, /* 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', */
            44, 45, 46, 47, 48, 49, 50, 51, /* 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', */
        ]; /* ASCII order for BASE 64 decode, -1 in unused character */

        let (base64de_first, base64de_last): (char, char) = ('+', 'z');

        let mut index: usize = 0;
        let input: &[u8] = input.as_bytes();
        let mut output: Vec<usize> = vec![0; input.len() / 4 * 3];

        for i in 0..input.len() {
            if input[i] == 61 /* = */ {
                break;
            }

            let c: isize = base64de[(input[i] - base64de_first as u8) as usize];
            if input[i] < base64de_first as u8 || input[i] > base64de_last as u8 || c == -1 {
                panic!("invalid base64 code");
            }

            match i % 4 {
                /* from 8 / gcd(6, 8) */
                0 => {
                    output[index] = ((c as usize) << 2) & 0xFF;
                }
                1 => {
                    output[index] += ((c as usize) >> 4) & 0x3;
                    index += 1;

                    /* if not last char with padding */
                    if i < (input.len() - 3) || input[input.len() - 2] as char != '=' /* = */ {
                        output[index] = ((c as usize) & 0xF) << 4;
                    }
                }
                2 => {
                    output[index] += ((c as usize) >> 2) & 0xF;
                    index += 1;

                    /* if not last char with padding */
                    if i < input.len() - 2 || input[input.len() - 1] as char != '=' /* = */ {
                        output[index] = ((c as usize) & 0x3) << 6;
                    }
                }
                _ => {
                    output[index] += (c as u8) as usize;
                    index += 1;
                }
            }
        }

        let mut res = String::new();
        for i in 0..output.len() {
            if output[i] == 0 {
                break;
            }
            res.push(output[i] as u8 as char);
        }
        return res;
    }
}
