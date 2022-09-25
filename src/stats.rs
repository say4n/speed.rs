use std::time::Duration;

pub fn minimum(durations: Vec<Duration>) -> Duration {
    let mut md = Duration::MAX;
    for d in durations {
        if d < md {
            md = d;
        }
    }

    return md;
}

pub fn maximum(durations: Vec<Duration>) -> Duration {
    let mut md = Duration::ZERO;
    for d in durations {
        if d > md {
            md = d;
        }
    }

    return md;
}

pub fn average(durations: Vec<Duration>) -> Duration {
    let mut sum = Duration::ZERO;
    for d in durations.clone() {
        sum += d;
    }

    return sum / durations.len().try_into().unwrap();
}

pub fn median(durations: Vec<Duration>) -> Duration {
    let l = durations.len();
    let mut dd = durations.clone();

    dd.sort();

    if l % 2 == 0 {
        return (dd[l / 2] + dd[l / 2 - 1]) / 2;
    } else {
        return dd[l / 2];
    }
}

pub fn jitter(durations: Vec<Duration>) -> Duration {
    let mut jj: Vec<Duration> = Vec::new();

    for i in 1..durations.len() {
        if durations[i - 1] > durations[i] {
            jj.push(durations[i - 1] - durations[i])
        } else {
            jj.push(durations[i] - durations[i - 1])
        }
    }

    return average(jj);
}
