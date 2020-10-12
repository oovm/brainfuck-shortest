use shortest_brainfuck::shortest_map;
use std::collections::BTreeSet;

#[test]
fn test() {
    let mut heads = BTreeSet::new();
    for e in shortest_map().values() {
        heads.insert(e.trim_end_matches(|c| c != ']'));
    }
    println!("{:#?}", heads)
}

#[test]
fn tests() {
    let i = &[
        "-[[<+>->+>+<<]>]",
        "+[[<+>->+>+<<]>]",
        "--[[<+>->+>+<<]>]",
        "++[[<+>->+>+<<]>]",
        "-[[<+>->+>--->-<<<]>+++]",
        "+<-[[<+>->+>--->-<<<]>++]",
        "-<++[[<+>->->+++>+<<<]->]",
        "-<<+[+[<+>--->->->-<<<]>]",
        "-<<+[+[<+>--->->->-<<<]>]",
        "+[++[<+++>->+++<]>+++++++]",
        "+<-[[<+>->+>--->-<<<]>+++]",
        "++<-[[<+>->+>--->-<<<]>+++]",
        "-[++[<++>->+++>+++<<]--->+]",
        "-<-<<+[+[<+>--->->->-<<<]>]",
        "-[++[<++>->+++>+++<<]---->+]",
        "--<-<<+[+[<+>--->->->-<<<]>]",
        "+[+[<<<+>>>>]",
        "+[>>>->-[>->----<<<]>>]",
    ];

    let mut heads = BTreeSet::new();
    for e in i {
        heads.insert(e.trim_end_matches(|c| c != ']'));
    }
    for e in shortest_map().values() {
        heads.insert(e.trim_end_matches(|c| c != ']'));
    }
    println!("{:#?}", heads)
}
