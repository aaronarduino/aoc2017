fn main() {
    let mut curr = 8;
    let test = 325489; // problem input
    let mut last_total = 1+curr;
    loop {
        let curr_total = (curr+8) + last_total;
        if last_total < test && curr_total >= test {
            curr = curr+8;
            break;
        }
        curr = curr+8;
        last_total = curr_total;
    }

    let side_len = curr/4;
    let arc_len = test-last_total;
    let side_div = arc_len as f32/side_len as f32;
    let side_index:i32 = side_div.ceil() as i32;
    let tmp_diff_num = side_len*(side_index-1);
    let len_of_last_side = arc_len-tmp_diff_num;
    let len_from_center = side_len/2;
    let output = get_coord(len_from_center, side_index, len_of_last_side, side_len);

    println!("{:?}", distance(output));
}

fn get_coord(len_from_center: i32, side_index: i32, len_of_last_side: i32, side_len: i32) -> (i32, i32) {
    match side_index {
        1 => {
            if len_of_last_side < len_from_center {
                return (len_from_center, (0-len_from_center)+len_of_last_side);
            }
            return (len_from_center, (0+len_from_center)-(side_len-len_of_last_side));
        }
        2 => {
            if len_of_last_side < len_from_center {
                return ((0+len_from_center)-len_of_last_side, 0+len_from_center);
            }
            return ((0-len_from_center)+(side_len-len_of_last_side), 0+len_from_center);
        }
        3 => {
            if len_of_last_side < len_from_center {
                return (0-len_from_center, (0-len_from_center)+(side_len-len_of_last_side));
            }
           return (0-len_from_center, (0+len_from_center)-len_of_last_side);
        }
        4 => {
            if len_of_last_side < len_from_center {
                return ((0-len_from_center)+len_of_last_side, 0-len_from_center);
            }
            return ((0+len_from_center)-(side_len-len_of_last_side), 0-len_from_center);
        }
        _ => panic!("side was not in range 1-4"),
    }
}

fn distance(input: (i32, i32)) -> i32 {
    (input.0-0).abs()+(input.1-0).abs()
}