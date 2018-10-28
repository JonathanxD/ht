/*
 *      ht - Grabs a specific line from stdin <https://github.com/JonathanxD/ht>
 *
 *         The MIT License (MIT)
 *
 *      Copyright (c) 2018 JonathanxD (https://github.com/JonathanxD/) <jhrldev@gmail.com>
 *      Copyright (c) contributors
 *
 *
 *      Permission is hereby granted, free of charge, to any person obtaining a copy
 *      of this software and associated documentation files (the "Software"), to deal
 *      in the Software without restriction, including without limitation the rights
 *      to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 *      copies of the Software, and to permit persons to whom the Software is
 *      furnished to do so, subject to the following conditions:
 *
 *      The above copyright notice and this permission notice shall be included in
 *      all copies or substantial portions of the Software.
 *
 *      THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 *      IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 *      FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 *      AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 *      LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 *      OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
 *      THE SOFTWARE.
 */
use std::env;
use std::io;
use std::io::BufRead;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let arg1: Option<&String> = args.get(1);
    let parse_size = arg1.map(|it| it.parse::<usize>());

    match parse_size {
        Some(Ok(size)) if size > 0 => {
            let std_in = io::stdin();
            let grab = std_in.lock().lines().into_iter().skip(size-1).next();

            match grab {
                Some(result) => {
                    match result {
                        Ok(line) => {
                            println!("{}", line);
                        }
                        _ => {
                            eprintln!("Failed to read stdin.");
                        }
                    }
                }
                _ => {
                    eprintln!("Line {} not found.", size);
                }
            }

        }
        _ => {
            print_help(&args);
        }
    }
}

fn print_help(args: &Vec<String>) {
    eprintln!(" Usage: {} [line]", args[0]);
    eprintln!(" - line: Line number (must be positive).");
}