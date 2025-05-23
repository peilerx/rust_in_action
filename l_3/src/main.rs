// #[derive(Debug)]
// enum Cereal {
//     Barley,
//     Millet,
//     Rice,
//     Rye,
//     Spelt,
//     Wheat,
// }
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
fn main() {
    // let mut grains: Vec<Cereal> = vec![];

    // grains.push(Cereal::Rye);
    // grains.push(Cereal::Barley);

    // println!("{:?}", grains);

    // drop(grains);

    //Arc это умный указатель который позволяет
    //передать владение между потоками к одним и тем же данным
    //Mutex зашищает данные от одновременного доступа из нескольких потоков .lock() дает доступ к чтению и записи
    // а drop() разблокирует доступ
    let thread_data = Arc::new(Mutex::new(100));

    let data1 = Arc::clone(&thread_data);
    //Когда я разблокировал mutex, переменной thread_data начал заниматся второй поток
    //после автоматической разблокировки второго потока, я блокирую Mutex в первом и снова меняю данные
    // таким образом первый поток последним завершает свою работу
    let handle1 = thread::spawn(move || {
        let mut value = data1.lock().unwrap();
        println!("Thread 1 non changed data: {}", *value);
        *value = 500;
        println!("Thread 1 changed data: {}", *value);
        drop(value);
        println!("Thread 1: Mutex unlocked");
        thread::sleep(Duration::from_millis(1000));
        println!("Thread 1: Still running after drop");
        thread::sleep(Duration::from_millis(800));
        value = data1.lock().unwrap();
        *value = 700;
        println!(
            "Thread 1: changes the value again after Thread 2: {}",
            *value
        );
        thread::sleep(Duration::from_millis(700));
    });

    let data2 = Arc::clone(&thread_data);

    let handle2 = thread::spawn(move || {
        thread::sleep(Duration::from_millis(400));
        let mut value = data2.lock().unwrap();
        println!("Thread 2 non changed data: {}", *value);
        *value = 1000;
        println!("Thread 2 changed data: {}", *value);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("Main: {}", *thread_data.lock().unwrap());
}
