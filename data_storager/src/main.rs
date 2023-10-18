use data_storager::DataStorage;

fn main() {
    let storage = DataStorage::new("data.txt");

    // Write data to storage
    let data_to_store = "Hello, world!";
    storage.write_data(data_to_store).unwrap();

    // Read data from storage
    match storage.read_data() {
        Ok(data) => {
            println!("Read data: {}", data);
        }
        Err(error) => {
            println!("Error reading data: {:?}", error);
        }
    }
}
