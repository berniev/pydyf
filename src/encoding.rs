pub fn f64_to_pdf_string(val: f64) -> String {
    if val.fract() == 0.0 {
        format!("{}", val as i64)
    } else {
        let s = format!("{:.4}", val);
        let trimmed = s.trim_end_matches('0').trim_end_matches('.');
        if trimmed.is_empty() || trimmed == "-0" {
            "0".to_string()
        } else {
            trimmed.to_string()
        }
    }
}

/// ASCII85 encode data (equivalent to Python's base64.a85encode)
pub fn ascii85_encode(data: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();

    for chunk in data.chunks(4) {
        if chunk.len() == 4 {
            let value = ((chunk[0] as u32) << 24)
                | ((chunk[1] as u32) << 16)
                | ((chunk[2] as u32) << 8)
                | (chunk[3] as u32);

            if value == 0 {
                result.push(b'z');
            } else {
                let mut encoded = [0u8; 5];
                let mut val = value;
                for i in (0..5).rev() {
                    encoded[i] = (val % 85) as u8 + 33;
                    val /= 85;
                }
                result.extend_from_slice(&encoded);
            }
        } else {
            // Handle partial chunk at end
            let mut value = 0u32;
            for (i, &byte) in chunk.iter().enumerate() {
                value |= (byte as u32) << (24 - i * 8);
            }

            let mut encoded = [0u8; 5];
            let mut val = value;
            for i in (0..5).rev() {
                encoded[i] = (val % 85) as u8 + 33;
                val /= 85;
            }
            result.extend_from_slice(&encoded[..chunk.len() + 1]);
        }
    }

    result
}
