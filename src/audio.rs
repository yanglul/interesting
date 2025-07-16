use std::fs::File;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use crossbeam_channel::{bounded, Receiver, Sender};
use log::{error, info, warn};
use mp4::{Mp4Reader, TrackType};
use ringbuf::RingBuffer;
 
 

type AudioFrame = Vec<u8>;

struct AudioStreamConfig {
    file_path: PathBuf,
    buffer_size: usize,
    sample_rate: u32,
    channels: u16,
}

struct AudioStream {
    producer_thread: thread::JoinHandle<()>,
    consumer_thread: thread::JoinHandle<()>,
    ctrl_sender: Sender<()>,
}

impl AudioStream {
    pub fn start(
        file_path: impl AsRef<Path>,
        buffer_size: usize,
        frame_handler: impl Fn(AudioFrame) + Send + 'static,
    ) -> Result<Self, AudioStreamError> {
        // 初始化日志
        simplelog::TermLogger::init(
            simplelog::LevelFilter::Info,
            simplelog::Config::default(),
            simplelog::TerminalMode::Mixed,
            simplelog::ColorChoice::Auto,
        )?;

        let file_path = file_path.as_ref().to_path_buf();
        
        // 创建环形缓冲区和控制通道
        let rb = RingBuffer::new(buffer_size);
        let (mut producer, mut consumer) = rb.split();
        let (ctrl_sender, ctrl_receiver) = bounded(1);

        // 读取MP4文件并获取配置
        let config = Self::read_config(&file_path)?;
        info!("Audio stream config: {:?}", config);

        // 启动生产者线程
        let producer_thread = Self::spawn_producer_thread(
            file_path,
            producer,
            ctrl_receiver,
        )?;

        // 启动消费者线程
        let consumer_thread = thread::spawn(move || {
            info!("Consumer thread started");
            while let Some(audio_frame) = consumer.pop() {
                frame_handler(audio_frame);
            }
            info!("Consumer thread finished");
        });

        Ok(Self {
            producer_thread,
            consumer_thread,
            ctrl_sender,
        })
    }

    fn read_config(file_path: &Path) -> Result<AudioStreamConfig, AudioStreamError> {
        let file = File::open(file_path)?;
        let mp4 = Mp4Reader::read_header(file)?;

        // 获取音频轨道信息
        let audio_track = mp4
            .tracks()
            .values()
            .find(|t| t.track_type() == TrackType::Audio)
            .ok_or(AudioStreamError::NoAudioTrack)?;

        let sample_rate = audio_track.timescale();
        let channels = audio_track.channels().unwrap_or(2); // 默认为立体声

        Ok(AudioStreamConfig {
            file_path: file_path.to_path_buf(),
            buffer_size: 100, // 默认值，可以在调用时覆盖
            sample_rate,
            channels,
        })
    }

    fn spawn_producer_thread(
        file_path: PathBuf,
        mut producer: ringbuf::Producer<AudioFrame>,
        ctrl_receiver: Receiver<()>,
    ) -> Result<thread::JoinHandle<()>, AudioStreamError> {
        let handle = thread::spawn(move || {
            info!("Producer thread started for file: {:?}", file_path);
            
            let file = match File::open(&file_path) {
                Ok(f) => f,
                Err(e) => {
                    error!("Failed to open file: {}", e);
                    return;
                }
            };

            let mp4 = match Mp4Reader::read_header(file) {
                Ok(m) => m,
                Err(e) => {
                    error!("Failed to read MP4 header: {}", e);
                    return;
                }
            };

            let audio_track = match mp4
                .tracks()
                .values()
                .find(|t| t.track_type() == TrackType::Audio)
            {
                Some(t) => t,
                None => {
                    error!("No audio track found");
                    return;
                }
            };

            let mut samples = match mp4.read_samples(audio_track.track_id()) {
                Ok(s) => s,
                Err(e) => {
                    error!("Failed to read samples: {}", e);
                    return;
                }
            };

            'producer_loop: loop {
                // 检查是否有停止信号
                if ctrl_receiver.try_recv().is_ok() {
                    info!("Producer received stop signal");
                    break 'producer_loop;
                }

                match samples.next() {
                    Ok(Some(sample)) => {
                        if producer.push(sample.bytes).is_err() {
                            warn!("Buffer full, dropping frame");
                            continue;
                        }
                    }
                    Ok(None) => {
                        info!("No more audio frames to read");
                        break 'producer_loop;
                    }
                    Err(e) => {
                        error!("Error reading sample: {}", e);
                        break 'producer_loop;
                    }
                }

                // 稍微休息一下，避免占用太多CPU
                thread::sleep(Duration::from_millis(1));
            }
            info!("Producer thread finished");
        });

        Ok(handle)
    }

    pub fn stop(self) -> Result<()> {
        info!("Stopping audio stream...");
        self.ctrl_sender.send(())?;
        
        self.producer_thread.join().map_err(|_| AudioStreamError::ThreadError)?;
        self.consumer_thread.join().map_err(|_| AudioStreamError::ThreadError)?;
        
        info!("Audio stream stopped successfully");
        Ok(())
    }
}

fn main() -> Result<(), AudioStreamError> {
    // 示例：处理音频帧的函数
    let frame_handler = |frame: AudioFrame| {
        // 这里可以添加实际的音频处理逻辑
        println!("Processing frame of size: {}", frame.len());
        // 模拟处理时间
        thread::sleep(Duration::from_millis(5));
    };

    // 启动音频流
    let audio_stream = AudioStream::start("audio.mp4", 100, frame_handler)?;
    
    // 主线程可以做其他事情...
    thread::sleep(Duration::from_secs(5));
    
    // 停止音频流
    audio_stream.stop()?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    #[test]
    fn test_audio_stream() {
        let frame_count = Arc::new(AtomicUsize::new(0));
        let frame_count_clone = frame_count.clone();
        
        let frame_handler = move |_frame: AudioFrame| {
            frame_count_clone.fetch_add(1, Ordering::SeqCst);
        };

        let audio_stream = AudioStream::start("test.mp4", 50, frame_handler)
            .expect("Failed to start audio stream");
        
        thread::sleep(Duration::from_secs(2));
        
        audio_stream.stop().expect("Failed to stop audio stream");
        
        assert!(frame_count.load(Ordering::SeqCst) > 0);
    }
}