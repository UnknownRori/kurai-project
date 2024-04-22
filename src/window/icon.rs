use macroquad::{miniquad::conf::Icon as MiniquadIcon, texture::Image};

pub struct Icon {
    image16: Image,
    image32: Image,
    image64: Image,
}

impl Icon {
    pub fn new(image16: Image, image32: Image, image64: Image) -> Self {
        Self {
            image16,
            image32,
            image64,
        }
    }

    pub fn icon(self) -> MiniquadIcon {
        let mut small: [u8; 1024] = [0; 1024];
        let mut medium: [u8; 4096] = [0; 4096];
        let mut big: [u8; 16384] = [0; 16384];

        populate_array(&self.image16, &mut small);
        populate_array(&self.image32, &mut medium);
        populate_array(&self.image64, &mut big);

        MiniquadIcon { small, medium, big }
    }
}

fn populate_array(img: &Image, arr: &mut [u8]) {
    let mut index: usize = 0;
    for pixel in img.get_image_data() {
        for value in pixel.iter() {
            arr[index] = *value;
            index += 1;
        }
    }
}
