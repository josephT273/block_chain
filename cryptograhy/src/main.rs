pub mod cipher {
    pub mod transposition {
        pub mod rail_fence {
            pub fn encrypt(text: &str, key: usize) -> String {
                if key == 1 {
                    return text.to_string();
                }

                let chars: Vec<char> = text.chars().collect();
                let len = chars.len();

                let mut matrix = vec![vec!['\n'; len]; key];

                let mut dir_down: bool = false;
                let mut row: usize = 0;
                let mut col: usize = 0;

                for i in 0..len {
                    if row == 0 || row == key - 1 {
                        dir_down = !dir_down;
                    }

                    matrix[row][col] = chars[i];
                    col += 1;

                    if dir_down {
                        row += 1
                    } else {
                        row -= 1;
                    }
                }

                let mut result = String::new();
                for r in 0..key {
                    for c in 0..len {
                        if matrix[r][c] != '\n' {
                            result.push(matrix[r][c]);
                        }
                    }
                }

                result
            }
            pub fn decrypt(text: &str, key: usize) -> String {
                if key == 1 {
                    return text.to_string();
                }

                // k = L / (2(N - 1)) -> N = number of rails, L = length of the word
                let chars: Vec<char> = text.chars().collect();
                let len = chars.len();

                let mut matrix = vec![vec!['\n'; len]; key];

                let mut dir_down: bool = false;
                let mut row: usize = 0;
                let mut col: usize = 0;

                for _ in 0..len {
                    if row == 0 || row == key - 1 {
                        dir_down = !dir_down;
                    }

                    matrix[row][col] = '*';
                    col += 1;

                    if dir_down {
                        row += 1;
                    } else {
                        row -= 1;
                    }
                }

                let mut index = 0;

                for r in 0..key {
                    for c in 0..len {
                        if matrix[r][c] == '*' && index < len {
                            matrix[r][c] = chars[index];
                            index += 1;
                        }
                    }
                }

                let mut result = String::new();
                row = 0;
                col = 0;
                dir_down = false;

                for _ in 0..len {
                    if row == 0 || row == key - 1 {
                        dir_down = !dir_down;
                    }
                    result.push(matrix[row][col]);
                    col += 1;

                    if dir_down {
                        row += 1;
                    } else {
                        row -= 1;
                    }
                }

                result
            }
        }

        pub mod route {
            pub fn encrypt(text: &str, key: usize) -> String {
                if key == 1 {
                    return text.to_string();
                }

                let chars: Vec<char> = text.chars().collect();

                let len = chars.len();
                let cols = (len + key - 1) / key;

                let mut index = 0;

                let mut matrix = vec![vec!['*'; cols]; key];
                for i in 0..cols {
                    for r in 0..key {
                        if index < len {
                            matrix[r][i] = chars[index];
                            index += 1;
                        }
                    }
                }

                let mut top = 0;
                let mut left = 0;
                let mut bottom = key - 1;
                let mut right = cols - 1;

                let mut result = String::new();

                while top <= bottom && left <= right {
                    for c in (left..=right).rev() {
                        if matrix[top][c] != '*' {
                            result.push(matrix[top][c]);
                        }
                    }
                    top += 1;

                    for r in top..=bottom {
                        if matrix[r][left] != '*' {
                            result.push(matrix[r][left]);
                        }
                    }
                    left += 1;

                    if top <= bottom {
                        for c in left..=right {
                            if matrix[bottom][c] != '*' {
                                result.push(matrix[bottom][c]);
                            }
                        }

                        bottom -= 1;
                    }

                    if left <= right {
                        for r in (top..=bottom).rev() {
                            if matrix[r][right] != '*' {
                                result.push(matrix[r][right]);
                            }
                        }
                        right -= 1;
                    }
                }

                result
            }
            // pub fn decrypt(text: &str, key: usize) -> String {
            //     return String::new();
            // }
        }
    }
}

fn main() {
    let key: usize = 3;
    let text = "Hello World";

    let data = cipher::transposition::rail_fence::encrypt(text, key);
    cipher::transposition::rail_fence::decrypt(&data, key);

    println!("{}", cipher::transposition::route::encrypt(text, key));
}
