use symphonia::core::codecs::{DecoderOptions, CODEC_TYPE_NULL};
use symphonia::core::errors::Error;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use symphonia::core::audio::SampleBuffer;
use plotters::prelude::*;

fn main() {
    //--------------------------------------------------------------------
    //  WEIRD MATH NERD STUFF
    //--------------------------------------------------------------------

    let args: Vec<String> = std::env::args().collect();
    let path = args.get(1).expect("file path not provided");

    let src = std::fs::File::open(path).expect("failed to open file");
    let mss = MediaSourceStream::new(Box::new(src), Default::default());

    let mut hint = Hint::new();
    hint.with_extension("flac");

    let meta_opts: MetadataOptions = Default::default();
    let fmt_opts: FormatOptions = Default::default();

    let probed = symphonia::default::get_probe()
        .format(&hint, mss, &fmt_opts, &meta_opts)
        .expect("unsupported format");

    let mut format = probed.format;

    let track = format
        .tracks()
        .iter()
        .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
        .expect("no supported audio tracks");

    let dec_opts: DecoderOptions = Default::default();

    let mut decoder = symphonia::default::get_codecs()
        .make(&track.codec_params, &dec_opts)
        .expect("unsupported codec");

    let track_id = track.id;
    
    let mut sample_count = 0;
    let mut sample_buf = None;

    // setup plotters
    let root = BitMapBackend::new("plot.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let mut chart_builder = ChartBuilder::on(&root);
    chart_builder.caption("Blackhole", ("sans-serif", 50).into_font());
    chart_builder.margin(10);
    chart_builder.x_label_area_size(30);
    chart_builder.y_label_area_size(30);
    
    let mut chart_context = chart_builder.build_cartesian_2d(0.0..10000.0, -0.1..0.1).unwrap();
    chart_context.configure_mesh().draw().unwrap();


    // Main decoding loop
    loop {
        let packet = match format.next_packet() {
            Ok(packet) => packet,
            Err(Error::ResetRequired) => {
                unimplemented!();
            },
            Err(Error::IoError(_)) => {
                break;
            },
            Err(err) => {
                panic!("{}", err);
            }
        };

        if packet.track_id() != track_id {
            continue;
        }

        match decoder.decode(&packet) {
            Ok(audio_buf) => {
                if sample_buf.is_none() {
                    let spec = *audio_buf.spec();
                    let duration = audio_buf.capacity() as u64;

                    sample_buf = Some(SampleBuffer::<f32>::new(duration, spec));
                }

                if let Some(buf) = &mut sample_buf {
                    buf.copy_interleaved_ref(audio_buf);

                    
                    sample_count += buf.samples().len();
//                    println!("Decoded {} samples", sample_count);                    
                        
                }
            }
            Err(Error::DecodeError(_)) => (),
            Err(_) => { 
                break;
            },
        }
    }
    //println!("{:?}", sample_buf.unwrap().samples());

    let points: Vec<_> = sample_buf.unwrap().samples().into_iter().enumerate()
        .map(|(i, sample)| (i as f64, *sample as f64))
        .collect();
    
    println!("{:?}", &points);

    chart_context.draw_series(LineSeries::new(points, BLACK)).unwrap();

    root.present().unwrap();

}
