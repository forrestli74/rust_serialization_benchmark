#[cfg(feature = "capnp")]
pub mod mesh_capnp;
#[cfg(feature = "flatbuffers")]
pub mod mesh_generated;
#[cfg(feature = "prost")]
pub mod mesh_prost {
    include!(concat!(env!("OUT_DIR"), "/prost.mesh.rs"));
}

use core::pin::Pin;
use crate::Generate;
#[cfg(feature = "capnp")]
use crate::bench_capnp;
#[cfg(feature = "flatbuffers")]
use crate::bench_flatbuffers;
#[cfg(feature = "prost")]
use crate::bench_prost;
#[cfg(feature = "flatbuffers")]
use flatbuffers::{FlatBufferBuilder, WIPOffset};
#[cfg(feature = "capnp")]
pub use mesh_capnp as cp;
#[cfg(feature = "flatbuffers")]
pub use mesh_generated::mesh as fb;
use rand::Rng;
#[cfg(feature = "rkyv")]
use rkyv::Archived;

#[derive(Clone, Copy)]
#[cfg_attr(feature = "abomonation", derive(abomonation_derive::Abomonation))]
#[cfg_attr(feature = "borsh", derive(borsh::BorshSerialize, borsh::BorshDeserialize))]
#[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize))]
#[cfg_attr(feature = "rkyv", archive_attr(derive(bytecheck::CheckBytes)))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "speedy", derive(speedy::Readable, speedy::Writable))]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Generate for Vector3 {
    fn generate<R: Rng>(rand: &mut R) -> Self {
        Self {
            x: rand.gen(),
            y: rand.gen(),
            z: rand.gen(),
        }
    }
}

#[cfg(feature = "flatbuffers")]
impl Into<fb::Vector3> for Vector3 {
    #[inline]
    fn into(self) -> fb::Vector3 {
        fb::Vector3::new(self.x, self.y, self.z)
    }
}

#[cfg(feature = "capnp")]
impl<'a> bench_capnp::Serialize<'a> for Vector3 {
    type Reader = cp::vector3::Reader<'a>;
    type Builder = cp::vector3::Builder<'a>;

    #[inline]
    fn serialize_capnp(&self, builder: &mut Self::Builder) {
        builder.set_x(self.x);
        builder.set_y(self.y);
        builder.set_z(self.z);
    }
}

#[cfg(feature = "prost")]
impl bench_prost::Serialize for Vector3 {
    type Message = mesh_prost::Vector3;

    #[inline]
    fn serialize_pb(&self) -> Self::Message {
        let mut result = Self::Message::default();
        result.x = self.x;
        result.y = self.y;
        result.z = self.z;
        result
    }
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "abomonation", derive(abomonation_derive::Abomonation))]
#[cfg_attr(feature = "borsh", derive(borsh::BorshSerialize, borsh::BorshDeserialize))]
#[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize))]
#[cfg_attr(feature = "rkyv", archive_attr(derive(bytecheck::CheckBytes)))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "speedy", derive(speedy::Readable, speedy::Writable))]
pub struct Triangle {
    pub v0: Vector3,
    pub v1: Vector3,
    pub v2: Vector3,
    pub normal: Vector3,
}

impl Generate for Triangle {
    fn generate<R: Rng>(rand: &mut R) -> Self {
        Self {
            v0: Vector3::generate(rand),
            v1: Vector3::generate(rand),
            v2: Vector3::generate(rand),
            normal: Vector3::generate(rand),
        }
    }
}

#[cfg(feature = "flatbuffers")]
impl Into<fb::Triangle> for Triangle {
    #[inline]
    fn into(self) -> fb::Triangle {
        fb::Triangle::new(
            &self.v0.into(),
            &self.v1.into(),
            &self.v2.into(),
            &self.normal.into()
        )
    }
}

#[cfg(feature = "capnp")]
impl<'a> bench_capnp::Serialize<'a> for Triangle {
    type Reader = cp::triangle::Reader<'a>;
    type Builder = cp::triangle::Builder<'a>;

    #[inline]
    fn serialize_capnp(&self, builder: &mut Self::Builder) {
        self.v0.serialize_capnp(&mut builder.reborrow().init_v0());
        self.v1.serialize_capnp(&mut builder.reborrow().init_v1());
        self.v2.serialize_capnp(&mut builder.reborrow().init_v2());
        self.normal.serialize_capnp(&mut builder.reborrow().init_normal());
    }
}

#[cfg(feature = "prost")]
impl bench_prost::Serialize for Triangle {
    type Message = mesh_prost::Triangle;

    #[inline]
    fn serialize_pb(&self) -> Self::Message {
        let mut result = Self::Message::default();
        result.v0 = Some(self.v0.serialize_pb());
        result.v1 = Some(self.v1.serialize_pb());
        result.v2 = Some(self.v2.serialize_pb());
        result.normal = Some(self.normal.serialize_pb());
        result
    }
}

#[derive(Clone)]
#[cfg_attr(feature = "abomonation", derive(abomonation_derive::Abomonation))]
#[cfg_attr(feature = "borsh", derive(borsh::BorshSerialize, borsh::BorshDeserialize))]
#[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize))]
#[cfg_attr(feature = "rkyv", archive_attr(derive(bytecheck::CheckBytes)))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "speedy", derive(speedy::Readable, speedy::Writable))]
pub struct Mesh {
    pub triangles: Vec<Triangle>,
}

#[cfg(feature = "rkyv")]
impl ArchivedMesh {
    pub fn triangles_pin(self: Pin<&mut Self>) -> Pin<&mut Archived<Vec<Triangle>>> {
        unsafe { self.map_unchecked_mut(|s| &mut s.triangles) }
    }
}

#[cfg(feature = "flatbuffers")]
impl<'a> bench_flatbuffers::Serialize<'a> for Mesh {
    type Target = fb::Mesh<'a>;

    #[inline]
    fn serialize_fb<'b>(&self, fbb: &'b mut FlatBufferBuilder<'a>) -> WIPOffset<Self::Target>
    where
        'a: 'b,
    {
        fbb.start_vector::<fb::Triangle>(self.triangles.len());
        for triangle in self.triangles.iter().cloned() {
            fbb.push::<fb::Triangle>(triangle.into());
        }
        let triangles = fbb.end_vector(self.triangles.len());

        let mut builder = fb::MeshBuilder::new(fbb);
        builder.add_triangles(triangles);
        builder.finish()
    }
}

#[cfg(feature = "capnp")]
impl<'a> bench_capnp::Serialize<'a> for Mesh {
    type Reader = cp::mesh::Reader<'a>;
    type Builder = cp::mesh::Builder<'a>;

    #[inline]
    fn serialize_capnp(&self, builder: &mut Self::Builder) {
        let mut mesh = builder.reborrow().init_triangles(self.triangles.len() as u32);
        for (i, value) in self.triangles.iter().enumerate() {
            value.serialize_capnp(&mut mesh.reborrow().get(i as u32));
        }
    }
}

#[cfg(feature = "prost")]
impl bench_prost::Serialize for Mesh {
    type Message = mesh_prost::Mesh;

    #[inline]
    fn serialize_pb(&self) -> Self::Message {
        let mut result = Self::Message::default();
        for triangle in self.triangles.iter() {
            result.triangles.push(triangle.serialize_pb());
        }
        result
    }
}
