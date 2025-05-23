// #[derive(Debug)]
// enum Cereal {
//     Barley,
//     Millet,
//     Rice,
//     Rye,
//     Spelt,
//     Wheat,
// }
use std::sync::{Arc, Barrier, Mutex};
use std::thread;
use std::time::Duration;
fn main() {
    // let mut grains: Vec<Cereal> = vec![];

    // grains.push(Cereal::Rye);
    // grains.push(Cereal::Barley);

    // println!("{:?}", grains);

    // drop(grains);

    // [Arc] это умный указатель который позволяет
    //передать владение между потоками к одним и тем же данным
    //
    // [Mutex] зашищает данные от одновременного доступа из нескольких потоков .lock() дает доступ к чтению и записи
    // а drop() разблокирует доступ
    //
    // [Barrie]r могут иметь пары или нечетное количество группы потоков, позволяют парам встретиться
    // в разных потоках в одном месте где завершится их робота
    // Уберите задержки в потоках чтобы убедиться что барьеры действительно работают
    let thread_data = Arc::new(Mutex::new(100));

    let data1 = Arc::clone(&thread_data);

    let barrier1 = Arc::new(Barrier::new(2));
    let barrier2 = Arc::new(Barrier::new(2));
    // let barrier3 = Arc::new(Barrier::new(2));
    //Когда я разблокировал mutex, переменной thread_data начал заниматся второй поток
    //после автоматической разблокировки второго потока, я блокирую Mutex в первом и снова меняю данные
    // таким образом первый поток последним завершает свою работу
    let barrier1_1 = Arc::clone(&barrier1);
    let barrier2_1 = Arc::clone(&barrier2);

    let handle1 = thread::spawn(move || {
        let mut value = data1.lock().unwrap();
        println!("Thread 1 non changed data: {}", *value);
        *value = 500;
        println!("Thread 1 changed data: {}", *value);
        drop(value);
        println!("Thread 1: Mutex unlocked");
        barrier1_1.wait();

        barrier2_1.wait();
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

    println!("Main: {}", *thread_data.lock().unwrap());
    let data2 = Arc::clone(&thread_data);
    let barrier1_2 = Arc::clone(&barrier1);
    let barrier2_2 = Arc::clone(&barrier2);
    let handle2 = thread::spawn(move || {
        thread::sleep(Duration::from_millis(400));
        barrier1_2.wait();
        let mut value = data2.lock().unwrap();
        println!("Thread 2 non changed data: {}", *value);
        *value = 1000;
        println!("Thread 2 changed data: {}", *value);
        barrier2_2.wait();
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("Main: {}", *thread_data.lock().unwrap());
}
