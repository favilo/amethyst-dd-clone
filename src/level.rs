use amethyst::{core::math::Point2, Result};
use ndarray::{Array, Array2, Ix2};
use nom::{branch::alt, character::complete::char, error::ErrorKind, multi::many_m_n, IResult};
use serde::{Deserialize, Serialize};
use std::{fmt, fs::File, io::BufReader, iter, path::Path};

use log::{debug, info};

#[cfg(profiler)]
use thread_profiler::profile_scope;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Levels {
    levels: Vec<Level>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
struct LevelData {
    level: i32,
    name: String,
    data: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Level {
    pub level: i32,
    pub name: String,
    pub width: usize,
    pub height: usize,
    data: Array2<LevelTile>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Copy)]
pub enum LevelTile {
    Empty,
    Plain,
    Grass,
    Fence,
}

impl LevelTile {
    pub fn new(c: char) -> Result<Self> {
        match c {
            '.' => Ok(Self::Plain),
            ',' => Ok(Self::Grass),
            ' ' => Ok(Self::Empty),
            '-' => Ok(Self::Fence),
            _ => Err(amethyst::Error::new(LevelError::LevelLoadError)),
        }
    }

    pub fn is_blocking(&self) -> bool {
        match self {
            Self::Fence | Self::Empty => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub enum LevelError {
    LevelLoadError,
    OutOfBoundsError,
}

impl std::error::Error for LevelError {}

impl fmt::Display for LevelError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        use self::LevelError::*;

        match *self {
            LevelLoadError => write!(fmt, "Failed load level data"),
            OutOfBoundsError => write!(fmt, "Index out of bounds"),
        }
    }
}

impl Default for LevelTile {
    fn default() -> Self {
        LevelTile::Empty
    }
}

impl Level {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        // Read the JSON contents of the file as an instance of `User`.
        let u = serde_yaml::from_reader(reader);
        if let Err(e) = u {
            debug!("{:?}", e.location());
            return Err(amethyst::Error::new(LevelError::LevelLoadError));
        }
        let u: LevelData = u.expect("Level data accurate");
        let lines = u.data.lines();
        let width = lines
            .clone()
            .map(|l| l.chars().count())
            .fold(usize::min_value(), std::cmp::max);
        let height = lines.count();

        let data = parse_level_string(width, height, &u.data).expect("Proper level parsing");

        Ok(Self {
            level: u.level,
            name: u.name,
            height: height,
            width: width,
            data: data,
        })
    }

    pub fn get_tile(&self, p: Point2<u32>) -> Result<LevelTile> {
        if !self.in_bounds(p) {
            return Err(amethyst::Error::new(LevelError::OutOfBoundsError));
        }
        Ok(self.data[(p.x as usize, p.y as usize)])
    }

    pub fn is_blocking(&self, p: Point2<u32>) -> bool {
        let t = self.get_tile(p);
        if let Ok(t) = t {
            t.is_blocking()
        } else {
            true
        }
    }

    pub fn in_bounds(&self, p: Point2<u32>) -> bool {
        !(p.x >= self.width as u32 || p.y >= self.height as u32)
    }
}

fn parse_level_string(width: usize, height: usize, data: &str) -> Result<Array<LevelTile, Ix2>> {
    #[cfg(profiler)]
    profile_scope!("parse_level_string");
    let lines = data.lines().map(line_parser(width)).map(|t| t.unwrap().1);
    let v: Vec<LevelTile> = lines.rev().flatten().collect();
    let array = Array::from_shape_vec((height, width), v)?.reversed_axes();
    info!("{:?}", array);
    Ok(array)
}

fn line_parser(width: usize) -> impl Fn(&str) -> IResult<&str, Vec<LevelTile>> {
    move |i| {
        let (r, mut d) = many_m_n(0, width, tile_parser)(i)?;
        if d.len() < width {
            d.extend(iter::repeat(LevelTile::Empty).take(width - d.len()));
        }
        Ok((r, d))
    }
}

fn tile_parser(i: &str) -> IResult<&str, LevelTile> {
    let (r, t) = alt((char('.'), char(' '), char(','), char('-')))(i)?;
    let tile =
        LevelTile::new(t).map_err(|_| nom::Err::Failure(("Error parsing", ErrorKind::Char)))?;
    Ok((r, tile))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_level() {}
}
