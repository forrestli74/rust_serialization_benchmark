use apache_avro::{Schema, Writer};
use rust_serialization_benchmark::datasets::mesh::{Mesh, Triangle};
// use rust_serialization_benchmark::datasets::minecraft_savedata::{Player, Players};
use rust_serialization_benchmark::{datasets, generate_vec};
use rust_serialization_benchmark::datasets::log::{Log, Logs};
use rust_serialization_benchmark::Generate;



fn main() {
    let schema = &*datasets::mesh::AVRO_SCHEMA;
    let mut writer = Writer::new(&schema, Vec::new());
    let mut rng = rand::thread_rng();
    const TRIANGLES: usize = 500;
    let data = Mesh {
        triangles: generate_vec::<_, Triangle>(&mut rng, TRIANGLES..TRIANGLES + 1),
    };
    let data2 = apache_avro::to_value(data).unwrap();
    // match data2 {
    //     apache_avro::types::Value::Record(r) => {
    //         let tmp = r.into_iter().next().unwrap().1;
    //         println!("{:?}", tmp);

    //     },
    //     _ => todo!(),
    // }
    writer.append(data2).unwrap();
    println!("!!!!!!!!!!!!!!!!");
    
    // writer.append_ser(data).unwrap();
}