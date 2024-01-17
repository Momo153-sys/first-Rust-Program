
fn main(){
    //We create a List instance 
let mut list = List::new(); 

//We create the different Task instance with List implementation
let task1 = list.add_task("Mop The Floor");
let task2 = list.add_task("Wipe The Glasses");
let task3 = list.add_task("Mow The Garden");

list.completed_task(task3.id);
list.completed_task(task2.id);

list.list_tasks();
}
#[derive(Clone)]
//We create a Task struct and implement it with a new() function that create Task new instances  
struct Task{
    id:usize,
    description:String,
    completed:bool,
}
impl Task{
    fn new(id:usize,description:&str,completed:bool) -> Self{
        Task { 
            id,
            description:String::from(description),
            completed,
             }
    }
}
//We create List struct that contains a vector that only stores Task instances and implement it 
struct List{
    to_do_list:Vec<Task>,
}
impl List{
    fn new()->Self{
        List{
            to_do_list:vec![],
        }
    }
    fn add_task(&mut self, description:&str) ->Task{
        let id = self.to_do_list.len()+1;
        let new_task = Task{
            id,
            description:String::from(description),
            completed:false,
        };
        self.to_do_list.push(new_task.clone());
        new_task
    }
    fn completed_task(&mut self, id:usize)-> Option<&Task>{
        if let Some(condition) = self.to_do_list.iter_mut().find(|t| t.id == id) {
            condition.completed = true;
            Some(condition)
        } else {
            None
        }
    }
    fn list_tasks(&mut self){
        for task in &self.to_do_list{
            println!("Task ID: {}, Task Description: {}, Task Completed: {}",task.id,task.description,task.completed);
        }
    }
}