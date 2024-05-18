use std::io;



// 1 defin to do item
// 2 add todo item to do todolist
//2 display todo list 
//4 filter to do list 
// update task with complete or not complete
//6 delete task
//7 save and store todo list data (json ?)
//8 deploy


struct Todoitem {
    id:u32,
    name:String,
    complete:bool
}

struct TodoList {
    items: Vec<Todoitem>
}

impl TodoList {

    fn new() -> TodoList {
        TodoList { items: Vec::new()}
    }


    fn add_item(&mut self, title:String) {
        let id = self.items.len() as u32 +1;
        let new_item:Todoitem = Todoitem{
            id,
            name : title.clone(),
            complete: false
        };
        self.items.push(new_item);
        println!("You just add a new item:");
        for item in &self.items {
            if item.id == self.items.len() as u32 {
                println!("id: {:?}",item.id);
                println!("name: {:?}",item.name);
                println!("complete: {:?}",item.complete);
            }

        }
    }

    

    fn display(&self, choice:u32) {
        if choice == 1 { 
        println!("Here the list of not competed task");
        for item in &self.items {
            if item.complete {
                let complete = "Yes";
                println!("id: {:?}",item.id);
                println!("name: {:?}",item.name);
                println!("complete: {:?}", complete);
                println!("-------")


            }
        }       
    } else if choice == 2 {
            println!("Here the list of not competed task");
            for item in &self.items {
                if item.complete == false {
                    let complete = "No";
                    println!("id: {:?}",item.id);
                    println!("name: {:?}",item.name);
                    println!("complete: {:?}", complete);
                    println!("-------")   
                }
            }           
        }
     else {
        println!("Here the list of not competed task");
            for item in &self.items {
                
                let complete = if item.complete == false {"No"} else {"Yes"};
                println!("id: {:?}",item.id);
                println!("name: {:?}",item.name);
                println!("complete: {:?}", complete);
                println!("-------")   
                }

    }
}



    fn switch_to_complete(&mut self, id:u32) {
        
            if let Some(item) = self.items.iter_mut().find(|i| i.id == id) {
                if item.complete {
                    println!("item : {:?} with id : {:?} already completed", item.name, item.id)
                }
                else {
                    item.complete = true;
                    println!("Completed: {}", item.name);

                }
                
        
    }
}

    fn delete(&mut self, sel_id: u32) {
        let index = self.items.iter().position(|r| r.id == sel_id).unwrap();
        self.items.remove(index);
        }
        
          
    }


fn main() {

    let mut todo_list = TodoList::new();

    loop {

       
        println!("PLease select your choice");
        println!("1. Add");
        println!("2. Update");
        println!("3. display");
        println!("4. Delete");
        println!("5. Exit");

        let mut choice = String::new();
        io::stdin()
        .read_line(&mut choice)
        .expect("Fail to read the line");
        
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please select one of the valid number");
                continue
            }
        };

        match choice {
            1 => {
                println!("Please, write the task you want to add");
                let mut name = String::new();
                io::stdin()
                .read_line(&mut name)
                .expect("Fail to read the line");

                todo_list.add_item(name)
                
            },
            2 => {
                println!("Please, write the id of the task you want to modfiy");               
                todo_list.display(2);
                let mut choice = String::new();
                io::stdin()
                .read_line(&mut choice)
                .expect("Fail to read the line");    
                let choice: u32 = match choice.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Please select one of the valid number");
                        continue
                }    
            };    
                todo_list.switch_to_complete(choice);
            

        
        },
            3 => {
                println!("What do you want to see:");
                println!("1. completed items");
                println!("2. Items to complete");
                println!("3. All items");

                let mut choice = String::new();
                io::stdin()
                .read_line(&mut choice)
                .expect("Fail to read the line");
                
                let choice: u32 = match choice.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Please select one of the valid number");
                        continue
                    }
                };

                todo_list.display(choice)
                
            },
            4 => {  
                
                println!("Pleas give the id of the task you want to delete");
                todo_list.display(3);


                let mut choice = String::new();
                io::stdin()
                .read_line(&mut choice)
                .expect("Fail to read the line");
                
                let choice: u32 = match choice.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Please select one of the valid number");
                        continue
                    }
                };

                todo_list.delete(choice);
                todo_list.display(3);

            },
            5 => {
                println!("exiting...");
                break;
            },
            _ => println!("Please select on of the options")
        }

    }

}