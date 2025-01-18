use crate::tune::speedtest::single::{single_download, single_upload};

pub async fn multi_download(url: &str, thread_count: u8) -> (f64, f64, Vec<Vec<f64>>) {
    let mut handles = Vec::new();
    for _ in 0..thread_count {
        let url = url.to_string();
        let handle = tokio::spawn(async move {
            let (avg_speed, max_speed, speeds) = single_download(url.as_str()).await;
            (avg_speed, max_speed, speeds)
        });
        handles.push(handle);
    }
    let mut avg_speeds = Vec::new();
    let mut speeds = Vec::new();
    for handle in handles {
        if let Ok((avg_speed, _max_speed, speed)) = handle.await {
            avg_speeds.push(avg_speed);
            speeds.push(speed);
        } else {
            avg_speeds.push(0.0);
            speeds.push(Vec::new());
        };
    }
    let avg_speed = avg_speeds.iter().map(|speed| speed).sum();

    let max_len = speeds.iter().map(|v| v.len()).max().unwrap_or(0);
    let mut speeds_sum = vec![0.0; max_len];
    for i in 0..max_len {
        for inner_vec in speeds.iter() {
            if let Some(&value) = inner_vec.get(i) {
                speeds_sum[i] += value;
            }
        }
    }

    let max = speeds_sum
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_else(|| &0.0);

    (avg_speed, *max, speeds)
}

pub async fn multi_upload(url: &str, thread_count: u8) -> (f64, f64, Vec<Vec<f64>>) {
    let mut handles = Vec::new();
    for _ in 0..thread_count {
        let url = url.to_string();
        let handle = tokio::spawn(async move {
            let (avg_speed, max_speed, speeds) = single_upload(url.as_str()).await;
            (avg_speed, max_speed, speeds)
        });
        handles.push(handle);
    }
    let mut avg_speeds = Vec::new();
    let mut speeds = Vec::new();
    for handle in handles {
        if let Ok((avg_speed, _max_speed, speed)) = handle.await {
            avg_speeds.push(avg_speed);
            speeds.push(speed);
        } else {
            avg_speeds.push(0.0);
            speeds.push(Vec::new());
        };
    }
    let avg_speed = avg_speeds.iter().map(|speed| speed).sum();

    let max_len = speeds.iter().map(|v| v.len()).max().unwrap_or(0);
    let mut speeds_sum = vec![0.0; max_len];
    for i in 0..max_len {
        for inner_vec in speeds.iter() {
            if let Some(&value) = inner_vec.get(i) {
                speeds_sum[i] += value;
            }
        }
    }

    let max = speeds_sum
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_else(|| &0.0);

    (avg_speed, *max, speeds)
}
