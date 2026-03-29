use lyo::prelude::*;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::sleep;

/// Represents a data item from a source
#[derive(Debug, Clone)]
struct DataItem {
    timestamp: u64,
    value: f64,
    source: String,
}

/// Represents processed data with analysis
#[derive(Debug)]
struct ProcessedData {
    original: DataItem,
    analysis: String,
    computed_value: f64,
}

/// Producer that simulates fetching data from external sources
struct DataSourceProducer {
    sources: Vec<String>,
    current_index: usize,
    items_per_source: u32,
    current_item_count: u32,
}

impl DataSourceProducer {
    fn new(sources: Vec<String>, items_per_source: u32) -> Self {
        Self {
            sources,
            current_index: 0,
            items_per_source,
            current_item_count: 0,
        }
    }
}

impl Producer<DataItem> for DataSourceProducer {
    async fn produce(&mut self) -> anyhow::Result<DataItem> {
        if self.current_index >= self.sources.len() {
            return Err(anyhow::anyhow!("All sources exhausted"));
        }

        // Simulate network delay for fetching data
        sleep(Duration::from_millis(100)).await;

        self.current_item_count += 1;

        let item = DataItem {
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_secs(),
            value: rand::random::<f64>() * 100.0,
            source: self.sources[self.current_index].clone(),
        };

        // Move to next source if current one is exhausted
        if self.current_item_count >= self.items_per_source {
            self.current_index += 1;
            self.current_item_count = 0;
        }

        Ok(item)
    }
}

/// Consumer that processes and analyzes data
struct DataProcessorConsumer {
    total_processed: std::sync::atomic::AtomicU32,
    sum_values: Arc<Mutex<f64>>,
}

impl DataProcessorConsumer {
    fn new() -> Self {
        Self {
            total_processed: std::sync::atomic::AtomicU32::new(0),
            sum_values: Arc::new(Mutex::new(0.0)),
        }
    }
}

impl Consumer<DataItem> for DataProcessorConsumer {
    async fn consume(&self, item: &DataItem) {
        // Simulate processing time
        sleep(Duration::from_millis(50)).await;

        let analysis = if item.value > 75.0 {
            "HIGH"
        } else if item.value > 50.0 {
            "MEDIUM"
        } else if item.value > 25.0 {
            "LOW"
        } else {
            "CRITICAL"
        };

        let computed = item.value * 1.5;

        let processed = ProcessedData {
            original: item.clone(),
            analysis: analysis.to_string(),
            computed_value: computed,
        };

        println!(
            "[{}] Value: {:.2} | Analysis: {} | Computed: {:.2}",
            item.source, item.value, analysis, computed
        );

        self.total_processed
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        *self.sum_values.lock().await += computed;
    }

    async fn stop(&self) {
        let total = self
            .total_processed
            .load(std::sync::atomic::Ordering::Relaxed);
        let sum = *self.sum_values.lock().await;
        println!("\n=== Processing Summary ===");
        println!("Total items processed: {}", total);
        println!("Sum of computed values: {:.2}", sum);
        if total > 0 {
            println!("Average computed value: {:.2}", sum / total as f64);
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    println!("=== Data Processing Pipeline Example ===\n");
    println!("Simulating data fetch from multiple sources...\n");

    let sources = vec![
        "Sensor-A".to_string(),
        "Sensor-B".to_string(),
        "Sensor-C".to_string(),
    ];

    let producer: Box<DataSourceProducer> = Box::new(DataSourceProducer::new(sources, 5));
    let consumer: Box<DataProcessorConsumer> = Box::new(DataProcessorConsumer::new());

    println!("Starting data processing pipeline...\n");

    let mut controller = Controller::new(consumer, producer);
    controller.run().await;

    println!("\n=== Pipeline completed ===");

    Ok(())
}
