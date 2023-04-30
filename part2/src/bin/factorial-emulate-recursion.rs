#[derive(Clone, Copy)]
enum ReturnAddr {
    Start,
    AfterRecursiveCall,
}

use ReturnAddr::*;

#[derive(Clone, Copy)]
struct Call {
    return_addr: ReturnAddr,
    number: u32,
}

fn main() {
    let mut call_stack = vec![];

    // "Call" the "factorial() function".
    call_stack.push(Call {
        return_addr: Start,
        number: 5,
    });

    let mut return_value = None;
    while !call_stack.is_empty() {
        // The body of the "factorial() function":

        let Call {
            return_addr,
            number,
        } = call_stack[call_stack.len() - 1];

        match return_addr {
            Start => {
                if number == 1 {
                    // BASE CASE
                    return_value = Some(1);
                    call_stack.pop(); // "Return" from "function call".
                } else {
                    // RECURSIVE CASE
                    let len = call_stack.len();
                    call_stack[len - 1].return_addr = AfterRecursiveCall;

                    // "Call" the "factorial() function":
                    call_stack.push(Call {
                        return_addr: Start,
                        number: number - 1,
                    });
                }
            }
            AfterRecursiveCall => {
                return_value = return_value.map(|value| number * value);
                call_stack.pop();
            }
        }
    }

    let return_value = return_value.expect(r#""Function call" should "return" a value."#);
    println!("{return_value}");
}
