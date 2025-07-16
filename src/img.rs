use image::io::Reader as ImageReader;
use jpeg_encoder::{Encoder, ColorType};
use std::fs::File;
use std::io::BufWriter;

fn convert_png_to_jpg_with_dpi(input_path: &str, output_path: &str, dpi: u16) -> Result<(), Box<dyn std::error::Error>> {
    // 打开并解码PNG图片
    let img = ImageReader::open(input_path)?.decode()?;
    
    // 转换为RGB格式
    let rgb_img = img.to_rgb8();
    
    // 创建输出文件
    let output_file = File::create(output_path)?;
    let mut writer = BufWriter::new(output_file);
    
    // 创建JPEG编码器，设置质量为90
    let mut encoder = Encoder::new(&mut writer, 90);
    
    // 设置DPI (转换为每米的点数)
    // let dots_per_meter = (dpi as f32 / 0.0254).round() as u16;
    encoder.set_density(jpeg_encoder::Density::Inch { x: (dpi), y: (dpi) });
    
    // 编码图片
    encoder.encode(
        &rgb_img,
        rgb_img.width() as u16,
        rgb_img.height() as u16,
        ColorType::Rgb,
    )?;
    
    Ok(())
}

pub fn run(s :String, t:String) {
    let input_path = s.as_str();
    let output_path = t.as_str();
    let target_dpi = 200;
    
    match convert_png_to_jpg_with_dpi(input_path, output_path, target_dpi) {
        Ok(_) => println!("成功将{}转换为{}，DPI设置为{}", input_path, output_path, target_dpi),
        Err(e) => eprintln!("转换失败: {}", e),
    }
}