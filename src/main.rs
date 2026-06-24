use colored::Colorize;
use std::env;

const ASCII_ART: &str = "
                                     .++-.
                                    .++++-.
                                   .-+++++-
                      .--...       .-+++++-.         ....
                      .+++++-...  .-+++-+++-.   ...-++++-
                      .-++++++++---.-+++++--.----++++++-.
                       .-++++---.+############-----+++-.
                        ..++-.....-#########-......++-.
                          -+--#......+####......##.+-.
                        ..---#####+-+######--######--..
                   ..--++++-####-###+#####+########+-++--..
                .-++++++++--###-#...+#####+-..#.###+-++++++++-.
                .-++++++++--###-.....-###+......####---++++++-.
                   ..-+++++-####-..##.-++.-##..####+-++++-..
                        ..---####+##++###+++#+#####-+...
                          -+--####-##########+####.+-.
                         .-++--###++#+######++##+-+++..
                        .-+++-+--####++##-#####.++++++-.
                       .-++++++-#####+##########-++++++-.
                       -+++++-.+###-.######.-###+--+++++-
                       .-...   +###.-#.####..###-   ..--.
                              .+##-..-++++-..+#++.
                               .##-  .++++-  .+..
                                     .-++-.
                                       ..
               ";
fn bubble(input: &str) -> String {
    // Size of the bubble based on the lenght of the input plus some padding
    let bubble_size = input.len();
    let border = "-".repeat(bubble_size + 2);
    format!(" {}\n< {} >\n {}", border, input, border)
}

fn main() {
    let user_input = env::args().nth(1);

    match user_input {
        // at first the pointers ("\") where hardcoded and things went messy with diffrent size of text
        // so i added some padding like the bubble function
        Some(arg) => {
            let bubble_wraped = bubble(&arg);
            let pointer = " ".repeat(arg.len() / 2 + 1); //to point to the middle of the bubble
            println!(
                "{}\n{}\\\n{} \\\n{}  \\\n{}   \\\n{}",
                bubble_wraped,
                pointer,
                pointer,
                pointer,
                pointer,
                ASCII_ART.purple().bold()
            );
        }

        None => {
            let bubble_wrapped = bubble("Sosa, Bang Bang");
            let pointer = " ".repeat("Sosa, Bang Bang".len() / 2 + 1);
            println!(
                "{}\n{}\\\n{} \\\n{}  \\\n{}   \\\n{}",
                bubble_wrapped,
                pointer,
                pointer,
                pointer,
                pointer,
                ASCII_ART.purple().bold()
            );
        }
    }
}
