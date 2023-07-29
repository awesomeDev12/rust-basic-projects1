use std::env::{args,Args};

fn main() {

    let mut args: Args = args();
    // println!("Hello, world!");
    // println!("{:?}",args);


    let first = args.nth(1).unwrap();
    let operator:char = args.nth(0).unwrap().chars().next().unwrap();
    let second = args.nth(0).unwrap();


    let first_number : f32 = first.parse().unwrap();
    let second_number = second.parse::<f32>().unwrap();

    // println!("{:?} {:?} {:?}",first_number,operator,second_number);


    fn operate1(operator:char, first_number:f32, second_number:f32) -> f32 {

        if operator == '+' {
            return first_number + second_number;
        } else if operator == '-' {
            return first_number - second_number;
        } else if operator == '*' {
            return first_number * second_number;
        } else if operator == '/' {
            return first_number / second_number;
        }else {
            println!("Please pass a valid parameter");
            return 0.0;
        }
    }


    fn operate2(operator:char, first_number:f32, second_number:f32) -> f32 {

        match operator {
            '+' => first_number + second_number,
            '-' => first_number - second_number,
            '*' | 'x' | 'X' => first_number * second_number,
            '/' => first_number / second_number,
            _ => panic!("Invalid Operator")
        }
    }

    // let result:f32 = operate1(operator,first_number,second_number);
    // println!("Result is {:?}", result);


    let result:f32 = operate2(operator,first_number,second_number);
    println!("Result is {:?}", result);

}
