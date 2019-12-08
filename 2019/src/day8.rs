mod common;
use generic_array::typenum::{U25, U6};
use generic_array::{ArrayLength, GenericArray};
use std::str::FromStr;

#[derive(Debug)]
struct Layer<Width: ArrayLength<u8>, Height: ArrayLength<GenericArray<u8, Width>>> {
    pixels: GenericArray<GenericArray<u8, Width>, Height>,
}

#[derive(Debug)]
struct Image<Width: ArrayLength<u8>, Height: ArrayLength<GenericArray<u8, Width>>> {
    layers: Vec<Layer<Width, Height>>,
}

impl<Width: ArrayLength<u8>, Height: ArrayLength<GenericArray<u8, Width>>> FromStr
    for Layer<Width, Height>
{
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = Width::to_usize();
        let height = Height::to_usize();
        if s.len() != width * height {
            Err("size of string is not matching width and height")
        } else {
            let mut v = Vec::with_capacity(height);
            for row in s.as_bytes().chunks(width) {
                let p: GenericArray<u8, Width> =
                    GenericArray::from_exact_iter(row.iter().map(|c| c - b'0')).unwrap();
                v.push(p);
            }
            Ok(Layer {
                pixels: GenericArray::clone_from_slice(v.as_slice()),
            })
        }
    }
}

impl<Width: ArrayLength<u8>, Height: ArrayLength<GenericArray<u8, Width>>> Layer<Width, Height> {
    fn count_digits(&self, d: u8) -> usize {
        self.pixels
            .iter()
            .flat_map(|r| r.iter())
            .filter(|&&p| p == d)
            .count()
    }
}

impl<Width: ArrayLength<u8>, Height: ArrayLength<GenericArray<u8, Width>>> FromStr
    for Image<Width, Height>
{
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = Width::to_usize();
        let height = Height::to_usize();
        let layers: Vec<_> = s
            .as_bytes()
            .chunks(width * height)
            .map(|l| {
                std::str::from_utf8(l)
                    .unwrap()
                    .parse::<Layer<Width, Height>>()
                    .expect("could not parse layer")
            })
            .collect();
        Ok(Image { layers })
    }
}

impl<Width: ArrayLength<u8>, Height: ArrayLength<GenericArray<u8, Width>>> Image<Width, Height> {
    fn checksum(&self) -> usize {
        let min_zero_layer = self.layers.iter().map(|l| (l, l.count_digits(0))).min_by_key(|&(_, zeros)| zeros).unwrap().0;
        let ones = min_zero_layer.count_digits(1);
        let twos = min_zero_layer.count_digits(2);
        ones * twos
    }
}

fn main() {
    let input: Vec<_> = common::get_lines()
        .into_iter()
        .map(|l| l.parse::<Image<U25, U6>>().expect("could not parse image"))
        .collect();
    for image in input {
        let result1 = image.checksum();
        println!("Part1: image checksum: {}", result1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_parsing() {
        use generic_array::typenum::{U2, U3};
        let input = "123456789012";
        let image = input
            .parse::<Image<U3, U2>>()
            .expect("could not parse image");
        assert_eq!(image.layers[0].pixels[0].as_slice(), [1, 2, 3]);
        assert_eq!(image.layers[0].pixels[1].as_slice(), [4, 5, 6]);
        assert_eq!(image.layers[1].pixels[0].as_slice(), [7, 8, 9]);
        assert_eq!(image.layers[1].pixels[1].as_slice(), [0, 1, 2]);

        let checksum = image.checksum();
        assert_eq!(checksum, 1);
    }
}
