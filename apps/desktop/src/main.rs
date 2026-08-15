use qw_core::ProcessorConfig;
use qw_dsp::Processor;

fn main() {
    let config = ProcessorConfig::default();
    let processor = Processor::new(config, qw_core::StreamConfig::default());
    println!(
        "QuietWatch desktop UI is not implemented yet. DSP processor is constructed ({} Hz).",
        processor.stream_config().sample_rate
    );
}
