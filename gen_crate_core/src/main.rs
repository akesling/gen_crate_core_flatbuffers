use schema::example::{HelloFlatBuffersTable, HelloFlatBuffersTableArgs};

fn main() {
    let mut builder = flatbuffers::FlatBufferBuilder::with_capacity(1024);

    let greeting = builder.create_string("Hello");
    let object = builder.create_string("Generated Crate Core");
    let root = HelloFlatBuffersTable::create(&mut builder, &HelloFlatBuffersTableArgs {
        greeting: Some(greeting),
        object: Some(object),
    });

    builder.finish(root, None);
    let buf = builder.finished_data();

    let hello = flatbuffers::root::<HelloFlatBuffersTable>(buf).unwrap();
    println!("Built flatbuffer: {:?}", hello);
}
