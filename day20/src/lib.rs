pub use filelib::{load, split_lines_by_blanks};
use std::cmp::min;

type Pixel = bool;
const DARK_PIXEL: Pixel = false;
const LIGHT_PIXEL: Pixel = true;
const DARK_PIXEL_CHAR: char = '.';
const LIGHT_PIXEL_CHAR: char = '#';

/// Convert image enhancement algorithm into a bunch of pixels
///
/// ```
/// let s = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#";
/// let result = day20::parse_image_enhancement_algorithm(s);
/// assert_eq!(result[0], false);
/// assert_eq!(result[2], true);
/// assert_eq!(result.len(), 512);
/// ```
pub fn parse_image_enhancement_algorithm(input: &str) -> Vec<Pixel> {
    return input
        .chars()
        .filter(|&x| x == DARK_PIXEL_CHAR || x == LIGHT_PIXEL_CHAR)
        .map(|x| {
            if x == DARK_PIXEL_CHAR {
                return DARK_PIXEL;
            }
            return LIGHT_PIXEL;
        })
        .collect();
}

/// Convert image to a vector of vectors of pixels
///
/// ```
/// let s = "#..#.\n#....\n##..#\n..#..\n..###";
/// let vec_string: Vec<String> = s.lines().map(|x| x.to_string()).collect();
/// let result = day20::parse_image(&vec_string);
/// assert_eq!(result.len(), 5);
/// assert_eq!(result[0].len(), 5);
/// assert_eq!(result[0][0], true);
/// assert_eq!(result[0][1], false);
/// ```
pub fn parse_image(input: &Vec<String>) -> Vec<Vec<Pixel>> {
    let mut result = Vec::new();
    for x in input.iter() {
        result.push(parse_image_enhancement_algorithm(&x));
    }
    return result;
}

fn pixels_to_num(pixels: &Vec<Pixel>) -> u32 {
    let mut num: u32 = 0;
    let base: u32 = 2;
    for i in 0..pixels.len() {
        let pixel_num = if pixels[pixels.len() - i - 1] == LIGHT_PIXEL {
            1
        } else {
            0
        };
        num += pixel_num * base.pow(i.try_into().unwrap());
    }
    return num;
}

fn enhance_pixel(
    iec: &Vec<Pixel>,
    img: &Vec<Vec<Pixel>>,
    top_left_x: i32,
    mut top_left_y: i32,
    bottom_right_x: i32,
    mut bottom_right_y: i32,
    outside_is: Pixel
) -> Pixel {
    let mut img_pixels: Vec<Pixel> = Vec::new();
    while top_left_y < 0 {
        img_pixels.push(outside_is);
        img_pixels.push(outside_is);
        img_pixels.push(outside_is);
        top_left_y += 1;
    }
    let width = img[0].len();
    let height = img.len();
    let y_bound: usize = min(height - 1, bottom_right_y as usize);
    for y in top_left_y as usize..=y_bound {
        let mut cur_top_x = top_left_x;
        while cur_top_x < 0 {
            img_pixels.push(outside_is);
            cur_top_x += 1;
        }
        let cur_top_x: usize = cur_top_x as usize;
        for x in cur_top_x..=min(width - 1, bottom_right_x as usize) {
            let y_usize: usize = y.try_into().unwrap();
            let x_usize: usize = x.try_into().unwrap();
            img_pixels.push(img[y_usize][x_usize]);
        }
        let mut cur_bottom_x = bottom_right_x;
        while cur_bottom_x >= width.try_into().unwrap() {
            img_pixels.push(outside_is);
            cur_bottom_x -= 1;
        }
    }
    while bottom_right_y >= height.try_into().unwrap() {
        img_pixels.push(outside_is);
        img_pixels.push(outside_is);
        img_pixels.push(outside_is);
        bottom_right_y -= 1;
    }
    let num: usize = pixels_to_num(&img_pixels).try_into().unwrap();
    let iec_at_index = iec[num];
    return iec_at_index;
}

fn enhance_image(iec: &Vec<Pixel>, img: &Vec<Vec<Pixel>>, outside_is: Pixel) -> Vec<Vec<Pixel>> {
    // Moving 3x3 ceneted on a pixel in the image or outside the image.
    let width = img[0].len();
    let height = img.len();

    let mut result: Vec<Vec<Pixel>> = Vec::new();

    // No pixel will ever be at the height or lower
    let usize_offset: usize = 2;
    for top_left_y in 0..height + usize_offset {
        let top_left_y: i32 = top_left_y as i32 - usize_offset as i32;
        let mut row: Vec<Pixel> = Vec::new();
        // If the left is past the edge of the image, nothing will be added.
        // If we did add, we would have an infinite number of pixels to add...
        for top_left_x in 0..width + usize_offset {
            let top_left_x: i32 = top_left_x as i32 - usize_offset as i32;
            row.push(enhance_pixel(
                iec,
                img,
                top_left_x,
                top_left_y,
                top_left_x + 2,
                top_left_y + 2,
                outside_is
            ));
        }
        result.push(row);
    }
    return result;
}

/// Enhance the image twice and count number of lit pixels
///
/// ```
/// let img_s = "#..#.\n#....\n##..#\n..#..\n..###";
/// let vec_string: Vec<String> = img_s.lines().map(|x| x.to_string()).collect();
/// let img = day20::parse_image(&vec_string);
/// let iec_s = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#";
/// let iec = day20::parse_image_enhancement_algorithm(iec_s);
/// assert_eq!(day20::puzzle_a(&iec, &img), 35);
/// ```
pub fn puzzle_a(iec: &Vec<Pixel>, img: &Vec<Vec<Pixel>>) -> u32 {
    // So an infinite area blinks on and off due to our input.
    // As a hack, I've just figured out what this should be
    let mut outside_is = DARK_PIXEL;
    let zeroth_index = iec[0];
    let enhanced_once = enhance_image(iec, img, outside_is);
    if zeroth_index == LIGHT_PIXEL {
        outside_is = LIGHT_PIXEL;
    }
    let enhanced_twice = enhance_image(iec, &enhanced_once, outside_is);
    let mut num_light: u32 = 0;
    for row in enhanced_twice {
        let lights: Vec<Pixel> = row
            .iter()
            .filter(|x| **x == LIGHT_PIXEL)
            .map(|x| *x)
            .collect();
        num_light += lights.len() as u32;
    }
    return num_light;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_img() -> Vec<Vec<Pixel>> {
        let s = "#..#.\n#....\n##..#\n..#..\n..###";
        let vec_string: Vec<String> = s.lines().map(|x| x.to_string()).collect();
        return parse_image(&vec_string);
    }

    fn get_iec() -> Vec<Pixel> {
        let s = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#";
        return parse_image_enhancement_algorithm(s);
    }

    #[test]
    fn test_pixels_to_num() {
        let pixels = vec![
            DARK_PIXEL,
            DARK_PIXEL,
            DARK_PIXEL,
            LIGHT_PIXEL,
            DARK_PIXEL,
            DARK_PIXEL,
            DARK_PIXEL,
            LIGHT_PIXEL,
            DARK_PIXEL,
        ];
        assert_eq!(pixels_to_num(&pixels), 34);
    }

    #[test]
    fn test_enhance_pixel_off_map_top_left() {
        let img = get_img();
        let iec = get_iec();
        let enhance_pixel = enhance_pixel(&iec, &img, -2, -2, 0, 0, DARK_PIXEL);
        // This is ... ... ..#, so index 1, so should be a dark pixel
        assert_eq!(enhance_pixel, DARK_PIXEL);
    }

    #[test]
    fn test_enhance_pixel_off_map_top_right() {
        let img = get_img();
        let iec = get_iec();
        let enhance_pixel = enhance_pixel(&iec, &img, 4, -2, 6, 0, DARK_PIXEL);
        // This is ... ... ..., so index 0, so should be a dark pixel
        assert_eq!(enhance_pixel, DARK_PIXEL);
    }

    #[test]
    fn test_enhance_pixel_off_map_bottom_left() {
        let img = get_img();
        let iec = get_iec();
        let enhance_pixel = enhance_pixel(&iec, &img, -2, 4, 0, 6, DARK_PIXEL);
        // This is ... ... ..., so index 0, so should be a dark pixel
        assert_eq!(enhance_pixel, DARK_PIXEL);
    }

    #[test]
    fn test_enhance_pixel_off_map_bottom_right() {
        let img = get_img();
        let iec = get_iec();
        let enhance_pixel = enhance_pixel(&iec, &img, 4, 4, 6, 6, DARK_PIXEL);
        // This is #.. ... ..., so index 256, so should be ??
        assert_eq!(enhance_pixel, iec[256]);
    }

    #[test]
    fn test_enhance_pixel_mid() {
        let img = get_img();
        let iec = get_iec();
        let enhance_pixel = enhance_pixel(&iec, &img, 1, 1, 3, 3, DARK_PIXEL);
        // This is example case, should come to 34, which is light pixel.
        assert_eq!(enhance_pixel, LIGHT_PIXEL);
    }
}
