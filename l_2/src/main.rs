fn main() {
    let penguin_data = "\
    common name, length (cm)
    Little penguin, 33
    Yellow-eyed penguin, 65
    Fiordland penguin, 60
    Invalid, data
    ";

    let records = penguin_data.lines();
    let records_as_vec: Vec<&str> = penguin_data.lines().collect();
    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0 {
            continue;
        }

        let fields: Vec<_> = record.split(",").map(|field| field.trim()).collect();

        if cfg!(debug_assertions) {
            eprintln!("debug: {:?} -> {:?}", record, fields);
        }

        let name = fields[0];

        if let Ok(mut length) = fields[1].parse::<f32>() {
            length = length + 1.0;
            println!("{}, {}cm", name, length);
        }
    }

    println!("{:?}", records_as_vec);
}
