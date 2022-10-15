pub mod helpers{
    use extendr_api::Robj;
    use extendr_api::TryFrom;
    /// convert an Hash Map of Robj to Hash Map of String
    pub fn convert_robj_hashmap_string_hashmap(input_hashmap:std::collections::HashMap<&'static str, Robj>)->std::collections::HashMap<String,String>{
        let mut output_map:std::collections::HashMap<String,String> = std::collections::HashMap::new();
        for (key, value) in input_hashmap {
            if key != ""{
                let try_res_string = String::try_from(value);
                if !try_res_string.is_err(){
                    output_map.insert(
                        String::from(key),
                        String::from(try_res_string.as_ref().unwrap())
                    );
                }
            }
        }
        return output_map;
    }
    /// takes string input and convert it to image filter
    pub fn convert_string_to_image_filter(input_string:String)->image::imageops::FilterType{
        let res = match input_string.as_str() {
            "Nearest" => image::imageops::FilterType::Nearest,
            "Triangle" => image::imageops::FilterType::Triangle,
            "CatmullRom" => image::imageops::FilterType::CatmullRom,
            "Gaussian" => image::imageops::FilterType::Gaussian,
            "Lanczos3" => image::imageops::FilterType::Lanczos3,
            _ => image::imageops::FilterType::Lanczos3,
        };
        return res;
    }
}