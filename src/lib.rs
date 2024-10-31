pub struct CodeBlock {
    start_bracket: usize,
    end_bracket: usize,
    newline: usize,
}

pub fn get_top_level_brackets(input: &str) -> Vec<CodeBlock> {
    let mut blocks = Vec::new();

    let mut bracket_counter = 0;
    let mut start_bracket = 0;
    let mut end_bracket = 0;
    let mut char_counter = 0;
    let mut bracked_ended: bool = false;
    for c in input.chars() {
        if c == '{' {
            if bracket_counter == 0 {
                start_bracket = char_counter;
            }
            bracket_counter += 1;
        } else if c == '}' {
            bracket_counter -= 1;
            end_bracket = char_counter + 1;
            if bracket_counter == 0 {
                bracked_ended = true;
            }
        } else if c == '\n' {
            if bracked_ended {
                blocks.push(CodeBlock {
                    start_bracket: start_bracket,
                    end_bracket: end_bracket,
                    newline: char_counter,
                });
                bracked_ended = false;
            }
        }
        char_counter += 1;
    }

    if bracked_ended {
        blocks.push(CodeBlock {
            start_bracket: start_bracket,
            end_bracket: end_bracket,
            newline: char_counter-1,
        });
    }

    blocks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let c_code = String::from(
            r#"
            int a;
            int b;
            struct Example {
                int aa { {} };
                int y;
            };

            typedef struct{
            int one;
            int two;
            }myStruct;

        
            struct AnotherExample {
                float a;
                float b;
            };

            int main() {
                printf("Hello World!");
                return 0;
            }

            "#,
        );

        let blocks = get_top_level_brackets(&c_code);

        let mut start_char = 0;
        for block in blocks {
            println!("============\n");
            println!("{}", &c_code[start_char..block.newline]);
            println!("============\n");
            start_char = block.newline;
        }
    }
}
