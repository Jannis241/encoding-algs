fn main() {
    let data = "hallo welt!";
    let rle_en = rle_encoding(&data.into());
    let rle_de = rle_decoding(&rle_en);
    println!("Rle encoded data: {:?}", &rle_en);
    println!("Rle decoded data: {:?}", rle_de);
}

fn rle_decoding(data: &Vec<u8>) -> Vec<u8> {
    let mut decoded = Vec::new();

    for i in 0..data.len() {
        if i % 2 == 0 {
            let times = data[i];
            let data = data[i + 1];

            for _ in 0..times {
                decoded.push(data);
            }
        }
    }

    decoded
}

fn rle_encoding(data: &Vec<u8>) -> Vec<u8> {
    let mut encoded = Vec::new();
    let mut i = 0;

    while i < data.len() {
        let current = data[i];
        let mut count = 1;

        while i + count < data.len() && data[i + count] == current {
            count += 1;
        }

        let mut remaining = count;
        while remaining > 0 {
            let run = remaining.min(255);
            encoded.push(run as u8);
            encoded.push(current);
            remaining -= run;
        }

        i += count;
    }

    return encoded;
}
