use std::env;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    let output_file = args.remove(args.len()-1);
    let input_files = args;

    println!("{:?}",wav_concat::get_wav_header(&input_files[0]));
}



mod wav_concat{
    use std::fs::File;
    use std::io::Read;

    

    #[derive(Debug)]
    pub struct WAVHeaderData {
        chunk_id: String,
        chunk_size: u32,
        format: String,
        subchunk1_id: String,
        subchunk1_size: u32,
        audio_format: u16,
        num_channels: u16,
        sample_rate: u32,
        byte_rate: u32,
        block_align: u16,
        bits_per_sample: u16,
        subchunk2_id: String,
        subchunk2_size: u32,
        data_begin: usize
    }

    pub fn array_to_u32(array:[u8;4]) -> u32{
        return u32::from_ne_bytes(array);
    }

    pub fn array_to_u16(array:[u8;2])->u16{
        return u16::from_ne_bytes(array);
    }

    pub fn get_wav_header(path:&String)->WAVHeaderData{
        let mut headers: WAVHeaderData = WAVHeaderData {
            chunk_id: String::new(), 
            chunk_size: 0, 
            format: String::new(), 
            subchunk1_id: String::new(), 
            subchunk1_size: 0, 
            audio_format: 0, 
            num_channels: 0, 
            sample_rate: 0, 
            byte_rate: 0, 
            block_align: 0, 
            bits_per_sample: 0, 
            subchunk2_id: String::new(),
            subchunk2_size: 0,
            data_begin: 0 
        };


        let mut file = File::open(path).unwrap();
        let mut buffer: Vec<u8> = vec![];
        file.read_to_end(&mut buffer).unwrap();

        let is_wav = &buffer[0..=3] == vec![0x52, 0x49, 0x46, 0x46] && &buffer[8..=15] == vec![0x57, 0x41,0x56,0x45,0x66, 0x6d, 0x74, 0x20];

        if is_wav{
            headers.chunk_id = String::from_utf8(buffer[0..=3].to_vec()).unwrap();
            headers.chunk_size = array_to_u32(buffer[4..=7].try_into().unwrap());
            headers.format = String::from_utf8(buffer[8..=11].to_vec()).unwrap();
            headers.subchunk1_id = String::from_utf8(buffer[12..=15].to_vec()).unwrap();
            headers.subchunk1_size = array_to_u32(buffer[16..=19].try_into().unwrap());
            headers.audio_format = array_to_u16(buffer[20..=21].try_into().unwrap());
            headers.num_channels = array_to_u16(buffer[22..=23].try_into().unwrap());
            headers.sample_rate = array_to_u32(buffer[24..=27].try_into().unwrap());
            headers.byte_rate = array_to_u32(buffer[28..=31].try_into().unwrap());
            headers.block_align = array_to_u16(buffer[32..=33].try_into().unwrap());
            headers.bits_per_sample = array_to_u16(buffer[34..=35].try_into().unwrap());

            let mut index:usize = 0;
            for i in 0..buffer.len(){
                if buffer[i..=i+3] == vec![0x64,0x61,0x74,0x61]{
                    index = i;
                    break;
                }
            }
            if index == 0{
                panic!("Beginning of Data not found!");
            }
            headers.subchunk2_id = String::from_utf8(buffer[index..=index+3].to_vec()).unwrap();
            headers.subchunk2_size = array_to_u32(buffer[index+4..=index+7].try_into().unwrap());

            
            headers.data_begin = index+8;
        }
        else{
            panic!("File isn't WAV Format!!!")
        }

        return headers;
    }

    /*pub fn verify_wav_headers(header:WAVHeaderData)->bool{
        let check_list:Vec<bool> = vec![];
        if header.header_data.get("ChunkSize").unwrap() == 36 + header.header_data.get("SubChunk2Size"){

        }
    }*/
}