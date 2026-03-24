// Fixed version of your Local SR and Marathon Decay logic

fn difficulty_value_from_peaks(peaks: &[f64]) -> f64 {
    let mut v: Vec<f64> = peaks.iter().copied().filter(|x| *x > 0.0).collect();
    if v.is_empty() { return 0.0; }
    
    v.sort_by(|a, b| b.partial_cmp(a).unwrap());

    let reduced_section_count = 10usize;
    let reduced_baseline = 0.75;
    let decay_weight = 0.9;

    let take = reduced_section_count.min(v.len());
    for i in 0..take {
        // 1. Calculate progress through the top sections
        let t = i as f64 / reduced_section_count as f64;
        // 2. Apply the logarithmic reduction (OsuStrainSkill style)
        let scale = lerp(1.0, reduced_baseline, t).log10().abs();
        v[i] *= lerp(reduced_baseline, 1.0, scale.min(1.0));
    }

    v.sort_by(|a, b| b.partial_cmp(a).unwrap());

    let mut difficulty = 0.0;
    let mut w = 1.0;
    for s in v {
        difficulty += s * w;
        w *= decay_weight;
    }
    difficulty
}

fn star_from_aim_speed(aim_peaks: &[f64], speed_peaks: &[f64]) -> f64 {
    let aim_dv = difficulty_value_from_peaks(aim_peaks);
    let speed_dv = difficulty_value_from_peaks(speed_peaks);

    let aim_rating = aim_dv.sqrt() * DIFFICULTY_MULTIPLIER;
    let speed_rating = speed_dv.sqrt() * DIFFICULTY_MULTIPLIER;

    // Use your crate's actual conversion logic here
    let base_aim_perf = (5.0 * (aim_rating / 0.0675).max(1.0) - 4.0).powf(3.0) / 100_000.0;
    let base_speed_perf = (5.0 * (speed_rating / 0.0675).max(1.0) - 4.0).powf(3.0) / 100_000.0;

    let base_perf = (base_aim_perf.powf(1.1) + base_speed_perf.powf(1.1)).powf(1.0 / 1.1);

    if base_perf <= 0.00001 { return 0.0; }

    // Standard SR formula: PERFORMANCE_BASE_MULTIPLIER is usually 1.12... in modern osu
    1.12_f64.cbrt() 
        * 0.027 
        * ((100_000.0 / 2.0_f64.powf(1.0 / 1.1) * base_perf).cbrt() + 4.0)
}

pub fn local_sr_per_minute(strains_aim: &[f64], strains_speed: &[f64]) -> Vec<f64> {
    let peaks_per_min = (MINUTE_MS / PEAK_SECTION_LEN_MS).round() as usize; 
    let n_minutes = strains_aim.len().div_ceil(peaks_per_min);

    let mut out = Vec::with_capacity(n_minutes);
    for k in 0..n_minutes {
        let start = k * peaks_per_min; // Changed from 'star' to 'start'
        let end = ((k + 1) * peaks_per_min).min(strains_aim.len());
        
        let aim_slice = &strains_aim[start..end];
        let speed_slice = &strains_speed[start..end];

        out.push(star_from_aim_speed(aim_slice, speed_slice));
    }
    out
}
