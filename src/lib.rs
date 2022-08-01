use std::env;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;

pub struct Todo{
   todos:Vec<String>,
   path:String
}

impl Todo{
    pub fn new()->Todo{
        let home_path = env::var("HOME").unwrap();
        let path =  format!("{}/.todo",home_path);
        let mut file = OpenOptions::new()
               .read(true)
               .write(true)
               .create(true)
               .open(&path)
               .unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let mut  todos:Vec<String> = Vec::new();
        if !contents.is_empty() {
            for line in contents.lines(){
                todos.push(line.to_string());
            }
        }
        Todo{
            todos,
            path
        }
    }

    pub fn add(&mut self,params:&[String]){
        let params = params.to_vec(); 
        self.todos.push(params.join(" "));
        self.save()
    }

    pub fn done(&mut self,params:&[String]){
        let mut nums:Vec<usize> = params.to_vec().iter().map(|val| val.parse().unwrap()).collect();
        while let Some(num) = nums.pop() {
            let mut res=String::new();
           for ch in self.todos[num-1].chars(){
              res.push('\u{0336}');
              res.push(ch);
           }
           self.todos[num-1] = res;
        }
        self.save()
    }

    pub fn list(&self){
        for (index,todo) in self.todos.iter().enumerate(){
            println!("\x1b[93m{}\x1b[0m {}",index+1,todo);
        }
    }

    pub fn rm(&mut self,params:&[String]){
        let mut nums:Vec<usize> = params.to_vec().iter().map(|val| val.parse().unwrap()).collect();
        while let Some(num) = nums.pop() {
            self.todos.remove(num-1);
        }
        self.save()
    }

    pub fn reset(&mut self){
        self.todos.clear();
        self.save();
    }

    pub fn sort(&mut self){
        self.todos.sort_by_key(|todo| todo.starts_with('\u{0336}'));
        self.save();
    }

    pub fn help(&self){
const TODO_HELP: &str = "Usage: todo [COMMAND] [ARGUMENTS]
Todo is a lightweight todo terminal tool written in rust
Example: todo list
Available commands:
    - add [TASK/s]
        adds new task/s
        Example: todo add buy apples
    - list
        lists all tasks
        Example: todo list
    - done [INDEX]
        marks task as done
        Example: todo done 2 3 (marks second and third tasks as completed)
    - rm [INDEX]
        removes a task
        Example: todo rm 4
    - reset
        deletes all tasks
    - sort
        sorts completed and uncompleted tasks
        Example: todo sort
";
            println!("{}", TODO_HELP);
    }

    pub fn save(&mut self){
       let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .truncate(true)
            .create(true)
            .open(&self.path)
            .unwrap();
        for todo in self.todos.iter_mut(){
            todo.push_str("\n");
            file.write(&todo.as_bytes()).unwrap();
        }
    }
}