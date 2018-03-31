use std::env;

fn all_combinations<'a>(choices: &Vec<Vec<&'a str>>) -> Vec<Vec<&'a str>> {
    let mut combs = vec![vec![]];

    for vals in choices {
        let mut new_combs = vec![];
        for val in vals {
            for comb in &combs {
                let mut new_comb = comb.clone();
                new_comb.push(val.clone());
                new_combs.push(new_comb);
            }
        }
        combs = new_combs;
    }
    combs
}

fn print_usage(bin: &AsRef<str>) {
    println!(r#"
Usage: {0} [choice1,...] [choice2,...] [choice3,...] ... [choiceN,...]

Example: {0} a,b,c 1,2,3
Output:
    a 1
    a 2
    a 3
    b 1
    b 2
    ...
    "#, bin.as_ref());
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.is_empty() || args[1] == "-h" || args[1] == "--help" {
        print_usage(&args[0]);
        return;
    }

    let choices: Vec<Vec<&str>> = args.iter()
        .skip(1)
        .map(|x| { x.split(",").collect() })
        .collect();

    let combs = all_combinations(&choices);

    for comb in &combs {
        println!("{}", comb.join(" "));
    }
}
