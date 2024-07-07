use num::complex::*;
use num::integer::Roots;
use plotters::prelude::*;
use rustfft::{num_complex::Complex, FftPlanner};
use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::{DecoderOptions, CODEC_TYPE_NULL};
use symphonia::core::errors::Error;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

fn decode_signal(signal_path: String) -> SampleBuffer<f32> {
    let src = std::fs::File::open(signal_path).expect("failed to open file");
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

    loop {
        let packet = match format.next_packet() {
            Ok(packet) => packet,
            Err(Error::ResetRequired) => {
                unimplemented!();
            }
            Err(Error::IoError(_)) => {
                break;
            }
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
                }
            }
            Err(Error::DecodeError(_)) => (),
            Err(_) => {
                break;
            }
        }
    }

    sample_buf.expect("could not decode signal")
}

pub fn create_data_points(signal_path: String) -> Vec<Vec<[f64; 2]>> {
    let buf = decode_signal(signal_path);

    let mut points: Vec<Vec<_>> = vec![];

    // default sample points
    let points_1: Vec<_> = buf
        .samples()
        .into_iter()
        .enumerate()
        .map(|(i, sample)| [i as f64, *sample as f64])
        .collect();

    points.push(points_1);

    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_inverse(buf.len());

    let samples: Vec<f32> = buf.samples().to_vec();

    // Convert f32 samples to Complex<f32>
    let mut complex_samples: Vec<Complex<f32>> =
    samples.iter().map(|&x| Complex::new(x, 0.0)).collect();

    fft.process(&mut complex_samples);

    for i in 0..complex_samples.len() {
        complex_samples[i] = complex_samples[i] * 1. / complex_samples.len().sqrt() as f32;
    }

    // fft sample points
    let points_2: Vec<_> = complex_samples
        .iter()
        .enumerate()
        .map(|(i, sample)| [i as f64, sample.re() as f64])
        .collect();

    points.push(points_2);

    points
}
