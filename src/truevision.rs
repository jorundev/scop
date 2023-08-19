use std::{fs::File, io::Read, path::PathBuf};

#[derive(Debug)]
pub struct Targa {
    pub width: usize,
    pub height: usize,
    pub bytes: Vec<u8>,
}

#[derive(Debug)]
pub enum TargaError {
    IoError(std::io::Error),
    InvalidHeader,
    UnsupportedImageType(TargaImageType),
    UnsupportedBitDepth(u8),
    UnsupportedOrdering(HorizontalOrdering, VerticalOrdering),
}

impl From<std::io::Error> for TargaError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TargaImageType {
    NoImage,
    UncompressedColorMapped,
    UncompressedTrueColor,
    UncompressedGrayscale,
    CompressedColorMapped,
    CompressedTrueColor,
    CompressedGrayscale,
}

#[derive(Debug)]
struct ColorMapSpec {
    first_entry_index: u16,
    color_map_length: u16,
    color_map_entry_size: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum HorizontalOrdering {
    LeftToRight,
    RightToLeft,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum VerticalOrdering {
    TopToBottom,
    BottomToTop,
}

#[derive(Debug)]
struct ImageSpec {
    x_origin: u16,
    y_origin: u16,
    width: u16,
    height: u16,
    bits_per_pixel: u8,
    alpha_depth: u8,
    horizontal_ordering: HorizontalOrdering,
    vertical_ordering: VerticalOrdering,
}

#[derive(Debug)]
struct TargaHeader {
    id_length: u8,
    color_map_included: bool,
    image_type: TargaImageType,
    color_map_specification: ColorMapSpec,
    image_specification: ImageSpec,
}

impl TargaHeader {
    fn from_bytes(bytes: &[u8]) -> Result<Self, TargaError> {
        if bytes.len() != 18 {
            return Err(TargaError::InvalidHeader);
        }

        let id_length = bytes[0];
        let color_map_included = bytes[1] != 0;
        let image_type = match bytes[2] {
            0 => TargaImageType::NoImage,
            1 => TargaImageType::UncompressedColorMapped,
            2 => TargaImageType::UncompressedTrueColor,
            3 => TargaImageType::UncompressedGrayscale,
            9 => TargaImageType::CompressedColorMapped,
            10 => TargaImageType::CompressedTrueColor,
            11 => TargaImageType::CompressedGrayscale,
            _ => return Err(TargaError::InvalidHeader),
        };

        let color_map_specification = {
            let first_entry_index = u16::from_le_bytes([bytes[3], bytes[4]]);
            let color_map_length = u16::from_le_bytes([bytes[5], bytes[6]]);
            let color_map_entry_size = bytes[7];

            ColorMapSpec {
                first_entry_index,
                color_map_length,
                color_map_entry_size,
            }
        };

        let image_specification = {
            let x_origin = u16::from_le_bytes([bytes[8], bytes[9]]);
            let y_origin = u16::from_le_bytes([bytes[10], bytes[11]]);
            let width = u16::from_le_bytes([bytes[12], bytes[13]]);
            let height = u16::from_le_bytes([bytes[14], bytes[15]]);
            let bits_per_pixel = bytes[16];

            let alpha_depth = bytes[17] & 0xF;
            let horizontal_ordering = if (bytes[17] & 0b10000) != 0 {
                HorizontalOrdering::RightToLeft
            } else {
                HorizontalOrdering::LeftToRight
            };

            let vertical_ordering = if (bytes[17] & 0b100000) != 0 {
                VerticalOrdering::TopToBottom
            } else {
                VerticalOrdering::BottomToTop
            };

            ImageSpec {
                x_origin,
                y_origin,
                width,
                height,
                bits_per_pixel,
                alpha_depth,
                horizontal_ordering,
                vertical_ordering,
            }
        };

        Ok(Self {
            id_length,
            color_map_included,
            image_type,
            color_map_specification,
            image_specification,
        })
    }
}

impl Targa {
    pub fn from_file<P: Into<PathBuf>>(path: P) -> Result<Self, TargaError> {
        let mut file = File::open(path.into())?;

        let mut data = Vec::with_capacity(4096);
        let mut bytes = Vec::with_capacity(4096);

        file.read_to_end(&mut data)?;

        let header = TargaHeader::from_bytes(&data[..18])?;

        match header.image_type {
            TargaImageType::UncompressedTrueColor => {}
            other => return Err(TargaError::UnsupportedImageType(other)),
        };

        let number_of_pixels =
            header.image_specification.width as usize * header.image_specification.height as usize;

        let number_of_bytes = match header.image_specification.bits_per_pixel {
            24 => 3 * number_of_pixels,
            32 => 4 * number_of_pixels,
            other => return Err(TargaError::UnsupportedBitDepth(other)),
        };

        if number_of_bytes > data.len() + 18 {
            return Err(TargaError::InvalidHeader);
        }

        let pixel_data = &data[18..(number_of_bytes + 18)];

        let window_size = header.image_specification.bits_per_pixel / 8;

        if header.image_specification.horizontal_ordering != HorizontalOrdering::LeftToRight
            || header.image_specification.vertical_ordering != VerticalOrdering::TopToBottom
        {
            return Err(TargaError::UnsupportedOrdering(
                header.image_specification.horizontal_ordering,
                header.image_specification.vertical_ordering,
            ));
        }

        for pixel in pixel_data.chunks(window_size as usize) {
            bytes.push(pixel[0]);
            bytes.push(pixel[1]);
            bytes.push(pixel[2]);
        }

        Ok(Self {
            bytes,
            width: header.image_specification.width as usize,
            height: header.image_specification.height as usize,
        })
    }
}
