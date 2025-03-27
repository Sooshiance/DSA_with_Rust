mod array;
mod queues;

pub fn ds_containership(){
    array::arrays();

    let mut queue = queues::Queue::new();
    
    queue.enqueue(1);
    queue.enqueue(2);
    queue.enqueue(3);

    while !queue.is_empty() {
        println!("{}", queue.dequeue().unwrap());
    }
}