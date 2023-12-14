fn new_stack(max_size: usize) -> Vec<u32> {
    Vec::with_capacity(max_size)
}

fn pop(stack: &mut Vec<u32>) -> Option<u32> {
    let popped_value = stack.pop();
    println!("The popped value is: {popped_value:?}");
    popped_value
}

fn push(stack: &mut Vec<u32>, value: u32, max_size: usize) {
    if stack.len() == max_size {
        println!("Cannot add more elements");
    } else {
        stack.push(value);
        println!("Stack: {:?}", stack);
    }
}

fn input() -> u32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    let n = input.trim().parse().expect("Invalid input");
    n
}

fn size(stack: &Vec<u32>) -> usize {
    stack.len()
}

fn main() {
    println!("let us first create a stack for our use");
    println!("Please mention the size of the stack");

    let size_stack = input();
    let mut stack = new_stack(size_stack as usize);
    loop {
        println!("*********** Menu ***********");
        println!("{}: Add an element to the stack", "1");
        println!("{}: Pop an element from the stack", "2");
        println!("{}: Get the size of the stack", "3");
        println!("{}: Display", "4");
        println!("{}: Exit", "5");

        let choice = input();
        match choice {
            1 => {
                println!("Please enter the value you want to add");
                let value = input();
                push(&mut stack, value, size_stack as usize);
            }
            2 => {
                println!("{}: Popped value", pop(&mut stack).unwrap());
            }
            3 => {
                println!("The size of the stack is: {}", size(&stack));
            }
            4 => {
                println!("Stack: {:?}", stack);
            }
            _ => {
                println!("Invalid input");
            }
        }

        println!("Do you want to continue?");
        println!("{}: Yes", "1");
        println!("{}: No", "2");
        let status = input();
        if status == 1 {
            continue;
        } else {
            println!("Thank you for using our stack");
            break;
        }
    }
}
