use todo::Todo;
use std::env;

fn main() {
    let mut todo = Todo::new();
    let args:Vec<String> = env::args().collect();
    if args.len()>1{
       let command:&str = &args[1];
       let mut params = &args[2..];
       match command{
           "add" => todo.add(&mut params),
           "done" => todo.done(&mut params),
           "list" => todo.list(),
           "rm" => todo.rm(&mut params),
           "reset"=>todo.reset(),
           "sort" =>todo.sort(),
           _ => todo.help()
       } 
    }else{
        todo.list();
    }
}