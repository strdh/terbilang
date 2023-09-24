fn terbilang(number:usize) -> String {
    if number < 10 {
        let token_satuan = ["nol", "satu", "dua", "tiga", "empat", "lima", "enam", "tuju", "delapan", "sembilan"];
        return String::from(token_satuan[(number % 10) as usize]);
    } else if number >= 10 && number < 20 {
        let token_belasan = ["sepuluh", "sebelas", "dua belas", "tiga belas", "empat belas", "lima belas", "enam belas", "tuju belas", "delapan belas", "sembilan belas"];
        return String::from(token_belasan[(number % 10) as usize]);
    } else if number >= 20 && number < 100 {
        let token_puluhan = ["", "", "dua puluh", "tiga puluh", "empat puluh", "lima puluh", "enam puluh", "tuju puluh", "delapan puluh", "sembilan puluh"];

        let token_idx = number / 10;
        let mod_num = number % 10;
        let text = String::from(token_puluhan[token_idx]);

        if mod_num!= 0 {
            return text + " " + &terbilang(mod_num);
        }

        return text;
    } else if number >= 100 && number < 1000 {
        let token_idx = number / 100;
        let mod_num = number % 100;
        let mut text = String::from("");

        if token_idx == 1 {
            text = text + "seratus";
        } else {
            let token_satuan = ["nol", "satu", "dua", "tiga", "empat", "lima", "enam", "tuju", "delapan", "sembilan"];
            text = String::from(token_satuan[token_idx as usize]) + " ratus";
        }

        if mod_num != 0 {
            return text + " " + &terbilang(mod_num);
        }

        return text;
    } else if number >= 1000 && number < 1000000 {
        let token_idx = number / 1000;
        let mod_num = number % 1000;
        let mut text = String::from("");

        if token_idx == 1 {
            text = text + "seribu";
        } else {
            text = text + &terbilang(token_idx) + " ribu";
        }

        if mod_num != 0 {
            return text + " " + &terbilang(mod_num);
        }

        return text;
    } else if number >= 1000000 && number < 1000000000 {
        let token_idx = number / 1000000;
        let mod_num = number % 1000000;
        let mut text = String::from("");
        text = text + &terbilang(token_idx) + " juta ";

        if mod_num != 0 {
            return text + &terbilang(mod_num);
        }

        return text;
    } else if number >= 1000000000 && number < 1000000000000 {
        let token_idx = number / 1000000000;
        let mod_num = number % 1000000000;
        let mut text = String::from("");
        text = text + &terbilang(token_idx) + " miliar ";

        if mod_num != 0 {
            return text + &terbilang(mod_num);
        }

        return text;
    } else if number >= 1000000000000 && number < 1000000000000000 {
        let token_idx = number / 1000000000000;
        let mod_num = number % 1000000000000;
        let mut text = String::from("");
        text = text + &terbilang(token_idx) + " triliun ";

        if mod_num != 0 {
            return text + &terbilang(mod_num);
        }

        return text;
    } else if number >= 1000000000000000 && number < 1000000000000000000 {
        let token_idx = number / 1000000000000000;
        let mod_num = number % 1000000000000000;
        let mut text = String::from("");
        text = text + &terbilang(token_idx) + " quadriliun ";

        if mod_num != 0 {
            return text + &terbilang(mod_num);
        }

        return text;
    }

    return String::from("INVALID");
}

fn main() {
    let arr = [1, 11, 10, 13, 20, 54, 100, 333, 1000, 10000, 999999, 1234567, 888888888, 1000000000, 7898765432, 1000000000000, 9990000000000, 1000000000000000, 1001000000000009];

    for number in arr {
        println!("{number} => {}", terbilang(number));
    }
}
