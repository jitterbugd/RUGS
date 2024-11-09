use crate::*;

const RUGS_MAGIC_BYTES: &[u8; 4] = b"RUGS";
const HEADER_COUNT: usize = 3;

fn color_distance(c1: &Rgba, c2: &Rgba) -> f64 {
    let dr = c1.r as f64 - c2.r as f64;
    let dg = c1.g as f64 - c2.g as f64;
    let db = c1.b as f64 - c2.b as f64;
    let da = c1.a as f64 - c2.a as f64;
    (dr * dr + dg * dg + db * db + da * da).sqrt() // Euclidean distance
}

fn closest_color(input: &Rgba, palette: &[Rgba]) -> Rgba {
    let mut closest = &palette[0];
    let mut min_distance = f64::MAX;

    for color in palette {
        let distance = color_distance(&input, color);
        if distance < min_distance {
            min_distance = distance;
            closest = color;
        }
    }

    *closest
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Rgba {
    pub fn to_vec(&self) -> Vec<u8> {
        vec![self.r, self.g, self.b, self.a]
    }
    
    pub fn from_bytes(input: &[u8]) -> Rgba {
        Rgba {
            r: input[0],
            g: input[1],
            b: input[2],
            a: input[3]
        }
    }
}

#[derive(Debug)]
pub enum ComperssionAmnt {
    NONE,
    MIN,
    MED,
    HIGH,
    ULTRA,
}

pub struct Image {
    pub width: u32,
    pub height: u32,
    pub image_data: Vec<Rgba>,
}

impl Image {
    pub fn image_bytes(&self) -> Vec<u8> {
        let mut buffer: Vec<u8> = vec![];
        for instance in &self.image_data {buffer.append(&mut instance.to_vec())}
        buffer
    }

    pub fn deserialize(&self) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::new();
        let mut compressor = ZlibEncoder::new(Vec::new(), Compression::default());

        buffer.extend_from_slice(RUGS_MAGIC_BYTES);
        buffer.extend_from_slice(&self.width.to_be_bytes());
        buffer.extend_from_slice(&self.height.to_be_bytes());
        
        let mut image_bytes: Vec<u8> = vec![];
        for chunk in &self.image_data {image_bytes.append(&mut chunk.to_vec());}
        compressor.write_all(&image_bytes).unwrap();
        buffer.append(&mut compressor.finish().unwrap());

        buffer
    }

    pub fn serialize(raw_bytes: Vec<u8>) -> Result<Image, &'static str> {
        let header_length = HEADER_COUNT * 4;  // 4 bytes per header
        let header: Vec<&[u8]> = raw_bytes[..header_length].chunks(4).by_ref().take(HEADER_COUNT).collect();
        
        let (magic_bytes, width, height) = (
            header[0], 
            u32::from_be_bytes(header[1].try_into().unwrap()), 
            u32::from_be_bytes(header[2].try_into().unwrap()), 
        ); 
        
        if magic_bytes != RUGS_MAGIC_BYTES {return Err("Magic bytes not RUGS, wrong file format")}

        let image_bytes = {
            let mut decompressor = ZlibDecoder::new(Vec::new());
            decompressor.write_all(&raw_bytes[header_length..]).unwrap();
            decompressor.finish().unwrap()
        };

        let mut image_data: Vec<Rgba> = vec![];
        for chunk in image_bytes.chunks(4) {
            image_data.push(Rgba::from_bytes(chunk));
        }

        Ok(Image {
            width,
            height,
            image_data,
        })
    }

    pub fn lossy_compress(&mut self, how_much: ComperssionAmnt) -> Result<(), &'static str> {

        let compression_magnitude = match how_much {
            ComperssionAmnt::ULTRA => 250,
            ComperssionAmnt::HIGH => 1000,
            ComperssionAmnt::MED => 2000,
            ComperssionAmnt::MIN => 5000,
            ComperssionAmnt::NONE => 0,
        };
            
        let mut colors_parsed: Vec<Rgba> = vec![];
        let mut association_buffer: HashMap<Rgba, u32> = HashMap::new();

        for chunk in self.image_bytes().chunks(4){
            if let [r, g, b, a] = chunk[..] {
                let color = Rgba {r,g,b,a};
                colors_parsed.push(color);
                *association_buffer.entry(color).or_insert(0) += 1;
            }
        }

        // Handle when we don't want any lossy compression
        let compressed_data = { if compression_magnitude != 0 {
            let mut binding: Vec<(Rgba, u32)> = association_buffer.into_iter().collect();
            binding.sort_by(|a, b| b.1.cmp(&a.1));
    
            // Extract the keys of the top X entries
            let top_keys: Vec<Rgba> = binding.iter().take(compression_magnitude).map(|(key, _)| key.clone()).collect();
    
            // Find closest color to the pixel from the table and replace it
            let mut compressed_data: Vec<Rgba> = vec![];
            for color in colors_parsed.iter() {
                compressed_data.push(closest_color(color, &top_keys))
            }   

            compressed_data
        } else { colors_parsed }};

        self.image_data = compressed_data;

        Ok(())
    }
}