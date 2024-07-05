use nannou_audio::Buffer;

pub struct Audio {
    sounds: Vec<(audrey::read::BufFileReader, f32)>,
}

impl Audio {
    pub fn new() -> Self {
        Self {
            sounds: vec![]
        }
    }

    pub fn play_note(&mut self, note: audrey::read::BufFileReader, volume: f32) {
        self.sounds.push((note, volume));
    }
}

pub fn play_audio(audio: &mut Audio, buffer: &mut Buffer) {
    let mut have_ended = vec![];
    let len_frames = buffer.len_frames();

    for (i, sound) in audio.sounds.iter_mut().enumerate() {
        let mut frame_count = 0;
        let file_frames = sound.0.frames::<[f32; 2]>().filter_map(Result::ok);
        for (frame, file_frame) in buffer.frames_mut().zip(file_frames) {
            for (sample, file_sample) in frame.iter_mut().zip(&file_frame) {
                *sample += *file_sample * sound.1;
            }

            frame_count += 1;
        }

        if frame_count < len_frames {
            have_ended.push(i);
        }
    }

    for i in have_ended.into_iter().rev() {
        audio.sounds.remove(i);
    }
}
