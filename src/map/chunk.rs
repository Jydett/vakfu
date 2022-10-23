use std::borrow::Cow;

use bevy::prelude::Color;
use byte::ctx::Bytes;
use byte::{BytesExt, TryRead};

use super::sprite::MapSprite;

#[derive(Debug)]
pub struct MapChunk {
    pub map_x: i32,
    pub map_y: i32,
    pub min_x: i32,
    pub min_y: i32,
    pub min_z: i16,
    pub max_x: i32,
    pub max_y: i32,
    pub max_z: i16,
    pub sprites: Vec<MapSprite>,
}

impl<'a> TryRead<'a> for MapChunk {
    fn try_read(bytes: &'a [u8], _ctx: ()) -> byte::Result<(Self, usize)> {
        let offset = &mut 0;

        let min_x: i32 = bytes.read(offset)?;
        let min_y: i32 = bytes.read(offset)?;
        let min_z: i16 = bytes.read(offset)?;
        let max_x: i32 = bytes.read(offset)?;
        let max_y: i32 = bytes.read(offset)?;
        let max_z: i16 = bytes.read(offset)?;
        let map_x: i32 = bytes.read(offset)?;
        let map_y: i32 = bytes.read(offset)?;
        let rects: u16 = bytes.read(offset)?;
        let mut sprites: Vec<MapSprite> = Vec::with_capacity(rects as usize * 2);

        for _ in 0..rects {
            let rect_min_x = map_x + bytes.read::<u8>(offset)? as i32;
            let rect_max_x = map_x + bytes.read::<u8>(offset)? as i32;
            let rect_min_y = map_y + bytes.read::<u8>(offset)? as i32;
            let rect_max_y = map_y + bytes.read::<u8>(offset)? as i32;

            for cell_x in rect_min_x..rect_max_x {
                for cell_y in rect_min_y..rect_max_y {
                    let count: u8 = bytes.read(offset)?;
                    for _ in 0..count {
                        let typ: u8 = bytes.read(offset)?;
                        let cell_z = bytes.read(offset)?;
                        let height = bytes.read(offset)?;
                        let altitude_order = bytes.read(offset)?;
                        // let tag = bytes.read(offset)?;
                        let group_key: i32 = bytes.read(offset)?;
                        let layer: u8 = bytes.read(offset)?;
                        let group_id: i32 = bytes.read(offset)?;
                        let _occluder: bool = bytes.read(offset)?;
                        let element_id = bytes.read(offset)?;
                        let colors: Colors = bytes.read_with(offset, typ)?;
                        let color = colors.get(0);
                        let element = MapSprite {
                            cell_x,
                            cell_y,
                            cell_z,
                            height,
                            altitude_order,
                            // tag,
                            element_id,
                            group_key,
                            group_id,
                            layer,
                            color,
                        };
                        sprites.push(element);
                    }
                }
            }
        }
        let chunk = MapChunk {
            map_x,
            map_y,
            min_x,
            min_y,
            min_z,
            max_x,
            max_y,
            max_z,
            sprites,
        };
        Ok((chunk, *offset))
    }
}

struct Colors<'a> {
    table: Vec<Cow<'a, [u8]>>,
}

impl<'a> Colors<'a> {
    fn get(&self, idx: u16) -> Color {
        match self.table.get(idx as usize).map(|buf| &buf[..]) {
            Some([r, g, b, a]) => Color::rgba_linear(
                teint(i8::from_ne_bytes([*r])) * 2.0,
                teint(i8::from_ne_bytes([*g])) * 2.0,
                teint(i8::from_ne_bytes([*b])) * 2.0,
                teint(i8::from_ne_bytes([*a])),
            ),
            Some([r, g, b]) => Color::rgb_linear(
                teint(i8::from_ne_bytes([*r])) * 2.0,
                teint(i8::from_ne_bytes([*g])) * 2.0,
                teint(i8::from_ne_bytes([*b])) * 2.0,
            ),
            _ => Color::rgb_linear(1.0, 1.0, 1.0),
        }
    }
}

impl<'a> TryRead<'a, u8> for Colors<'a> {
    fn try_read(bytes: &'a [u8], _ctx: u8) -> byte::Result<(Self, usize)> {
        let offset = &mut 0;

        let count: u8 = 1;
        let mut table = Vec::with_capacity(count as usize);
        // for _ in 0..count as usize {
            let size = size_from_tag(_ctx);
            let bytes = bytes.read_with(offset, Bytes::Len(size))?;
            table.push(Cow::Borrowed(bytes));
        // }
        Ok((Colors { table }, *offset))
    }
}

fn size_from_tag(tag: u8) -> usize {
    let mut n2 = 0;
    if (tag & 2) == 2 { n2 += 3 };
    if (tag & 8) == 8 {n2 += 1 };
    if (tag & 0x10) == 16 { n2 *= 2 };
    if (tag & 1) == 1 { n2 += 3 };
    if (tag & 4) == 4 { n2 += 3 };
    n2
}

#[inline]
fn teint(v: i8) -> f32 {
    (v as f32 / 255.0f32) + 0.5f32
}
