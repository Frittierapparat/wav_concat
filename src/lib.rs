pub mod wav_concat{
    use std::fs::File;
    use std::io::{Read, Write};
    use std::vec;


#[cfg(test)]
mod tests {
    use std::time::Instant;
    use super::*;

    #[test]
    fn it_works() {
        let file_list = [
        "Track 10.wav", "Track 17.wav", "Track 23.wav", "Track 2.wav", "Track 7.wav", 
        "Track 11.wav", "Track 18.wav", "Track 24.wav", "Track 30.wav", "Track 8.wav",
        "Track 12.wav", "Track 19.wav", "Track 25.wav", "Track 31.wav", "Track 9.wav",
        "Track 13.wav", "Track 1.wav", "Track 26.wav", "Track 3.wav", "Track 14.wav", 
        "Track 20.wav", "Track 27.wav", "Track 4.wav", "Track 15.wav", "Track 21.wav", 
        "Track 28.wav", "Track 5.wav", "Track 16.wav", "Track 22.wav", "Track 29.wav", 
        "Track 6.wav"];
        let mut file_pos_list: Vec<String> = vec![];
        for file in file_list{
            file_pos_list.push(format!("/home/mia/Music/tmp000/{}", file))
        }
        println!("{:?}", file_pos_list);
        wav_concat(file_pos_list, "tmp.wav".to_string())
    }
}

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

    pub fn get_wav_header(buffer:&Vec<u8>)->WAVHeaderData{
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


        let is_wav = &buffer[0..=3] == vec![0x52, 0x49, 0x46, 0x46] && &buffer[8..=15] == vec![0x57, 0x41,0x56,0x45,0x66, 0x6d, 0x74, 0x20];

        if is_wav{
            headers.chunk_id = String::from_utf8(buffer[0..=3].to_vec()).unwrap();
            headers.chunk_size = u32::from_le_bytes(buffer[4..=7].try_into().unwrap());
            headers.format = String::from_utf8(buffer[8..=11].to_vec()).unwrap();
            headers.subchunk1_id = String::from_utf8(buffer[12..=15].to_vec()).unwrap();
            headers.subchunk1_size = u32::from_le_bytes(buffer[16..=19].try_into().unwrap());
            headers.audio_format = u16::from_le_bytes(buffer[20..=21].try_into().unwrap());
            headers.num_channels = u16::from_le_bytes(buffer[22..=23].try_into().unwrap());
            headers.sample_rate = u32::from_le_bytes(buffer[24..=27].try_into().unwrap());
            headers.byte_rate = u32::from_le_bytes(buffer[28..=31].try_into().unwrap());
            headers.block_align = u16::from_le_bytes(buffer[32..=33].try_into().unwrap());
            headers.bits_per_sample = u16::from_le_bytes(buffer[34..=35].try_into().unwrap());

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
            headers.subchunk2_size = u32::from_le_bytes(buffer[index+4..=index+7].try_into().unwrap());

            
            headers.data_begin = index+8;
        }
        else{
            panic!("File isn't WAV Format!!!")
        }

        return headers;
    }

    fn get_wav_header_from_file(path:String)->WAVHeaderData{
        let mut file = File::open(path).unwrap();
        let mut buf:[u8;100] = [0;100];
        file.read(&mut buf).unwrap();
        return  get_wav_header(&buf.to_vec());
    }

    pub fn verify_wav_header(header:&WAVHeaderData)->bool{
        let mut check_list:Vec<bool> = vec![];

        check_list.push(header.byte_rate == header.sample_rate * u32::from(header.num_channels) * u32::from(header.bits_per_sample) / 8);
        check_list.push(header.block_align == header.num_channels * header.bits_per_sample / 8);

        return check_list.into_iter().all(|x| x);
    }

    pub fn verify_wav_header_compatibility(header:&WAVHeaderData, reference:&WAVHeaderData)->bool{
        let mut check_list:Vec<bool> = vec![];

        //Verify data in the new header is valid
        check_list.push(header.byte_rate == header.sample_rate * u32::from(header.num_channels) * u32::from(header.bits_per_sample) / 8);
        check_list.push(header.block_align == header.num_channels * header.bits_per_sample / 8);

        //compare important data for equality
        check_list.push(header.audio_format == reference.audio_format);
        check_list.push(header.num_channels == reference.num_channels);
        check_list.push(header.sample_rate == reference.sample_rate);
        check_list.push(header.byte_rate == reference.byte_rate);
        check_list.push(header.block_align == reference.block_align);

        //println!("{:?}", check_list);
        return check_list.into_iter().all(|x| x);
    }

    pub fn wav_concat(mut files:Vec<String>, output_file:String){
        let ref_file_name = String::from(&files[0]);
        let mut ref_file_header = get_wav_header_from_file(files.remove(0));

        let mut final_subchunk2_size:u32 = 0;
        let mut final_chunk_size: u32 = 0;

        final_chunk_size += ref_file_header.chunk_size;
        final_subchunk2_size += ref_file_header.subchunk2_size;

        let mut file_checklist: Vec<bool> = vec![];
        if verify_wav_header(&ref_file_header){
            for file in &files{
                let header_data = get_wav_header_from_file(file.to_string());
                if verify_wav_header_compatibility(&header_data, &ref_file_header){
                    file_checklist.push(true);
                    final_subchunk2_size += header_data.subchunk2_size;
                    final_chunk_size += header_data.subchunk2_size;
                }
                else{
                    file_checklist.push(false);
                }
            }

            if file_checklist.into_iter().all(|x| x){
                ref_file_header.subchunk2_size = final_subchunk2_size;
                ref_file_header.chunk_size = final_chunk_size;
                let mut final_file = File::create(output_file).unwrap();
                let mut ref_file: Vec<u8> = vec![];
                File::open(ref_file_name).unwrap().read_to_end(&mut ref_file).unwrap();
                final_file.write(&overwrite_wav_header(ref_file, ref_file_header)).unwrap();
                for file in &files{
                    let mut file_buf: Vec<u8> = vec![];
                    File::open(file).unwrap().read_to_end(&mut file_buf).unwrap();
                    let header = get_wav_header(&file_buf);
                    final_file.write(&remove_wav_header(file_buf, header.data_begin)).unwrap();
                }
            }
        }


    }


    pub fn remove_wav_header(bytes:Vec<u8>, data_offset:usize)->Vec<u8>{
        return bytes[data_offset..].to_vec();
    }

    fn overwrite_wav_header(bytes:Vec<u8>, new_header_data:WAVHeaderData)->Vec<u8>{
        let mut temp_bytes:Vec<u8> = bytes.clone();
        temp_bytes.splice(0..=3, new_header_data.chunk_id.into_bytes());
        temp_bytes.splice(4..=7, new_header_data.chunk_size.to_le_bytes());
        temp_bytes.splice(8..=11, new_header_data.format.into_bytes());
        temp_bytes.splice(12..=15, new_header_data.subchunk1_id.into_bytes());
        temp_bytes.splice(16..=19, new_header_data.subchunk1_size.to_le_bytes());
        temp_bytes.splice(20..=21, new_header_data.audio_format.to_le_bytes());
        temp_bytes.splice(22..=23, new_header_data.num_channels.to_le_bytes());
        temp_bytes.splice(24..=27, new_header_data.sample_rate.to_le_bytes());
        temp_bytes.splice(28..=31, new_header_data.byte_rate.to_le_bytes());
        temp_bytes.splice(32..=33, new_header_data.block_align.to_le_bytes());
        temp_bytes.splice(34..=35, new_header_data.bits_per_sample.to_le_bytes());
        temp_bytes.splice(36..=39, new_header_data.subchunk2_id.into_bytes());
        temp_bytes.splice(40..=43, new_header_data.subchunk2_size.to_le_bytes());
        return temp_bytes;
    }
}