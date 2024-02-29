use apache_avro::{types::Value, Reader, Schema, Writer};
use criterion::{black_box, Criterion};

pub fn bench<T>(name: &'static str, c: &mut Criterion, data: &T, schema: &str)
where
    T: Serialize + TryFrom<Value, Error=apache_avro::Error>,
{
    const BUFFER_LEN: usize = 10_000_000;

    let mut group = c.benchmark_group(format!("{}/avro", name));

    let schema = Schema::parse_str(schema).unwrap();

    let mut serialize_buffer = vec![0; BUFFER_LEN];
    group.bench_function("serialize", |b| {
        b.iter(|| {
            let mut writer = Writer::new(&schema, black_box(serialize_buffer.as_mut_slice()));
            let value = T::serialize(data).unwrap();
            writer.append(value).unwrap();
            black_box(writer.into_inner().unwrap());
        })
    });

    // let schema = Schema::parse_str(schema).unwrap();
    let mut writer = Writer::new(&schema, Vec::new());
    let value = T::serialize(data).unwrap();
    writer.append(value).unwrap();
    let deserialize_buffer = writer.into_inner().unwrap();

    group.bench_function("deserialize", |b| {
        b.iter(|| {
            let mut reader = Reader::new(&*deserialize_buffer).unwrap();
            let value = reader.next().unwrap().unwrap();
            let data: T = value.try_into().unwrap();
            black_box(data);
        })
    });

    crate::bench_size(name, "avro", deserialize_buffer.as_slice());

    group.finish();
}

pub trait Serialize {
    fn serialize(&self) -> Result<Value, apache_avro::Error>;
}

pub fn get_schema_field(schema: &'static Schema, field: &str) -> &'static Schema {
    if let Schema::Record(record) = schema {
        &record.fields[record.lookup[field]].schema
    } else {
        panic!("Schema is not a record")
    }
}

pub fn uuid_slice_to_value(value: &[u32;4]) -> Value {
    Value::Uuid(uuid::Uuid::from_bytes(unsafe { std::mem::transmute(*value) }))
}