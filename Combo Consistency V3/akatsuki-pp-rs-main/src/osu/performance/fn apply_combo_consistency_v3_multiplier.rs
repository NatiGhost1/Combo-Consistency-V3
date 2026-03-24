fn apply_combo_consistency_v3_multiplier(&self) -> f64 {
    let Some(base_p) = self.combo_consistency_v3_p else {
        return 1.0;
    };

    let combo = self.state.max_combo as f64;
    let misses = self.effective_miss_count();   // ← now using the helper!

    let mut p = base_p;

    // ────────────────────────────────────────────────
    // Keep ALL original conditions exactly as in your file
    // ────────────────────────────────────────────────

    if overall_combo_consistency_v3_enabled() {
        p = 0.98;
    }

    if self.mods.rx() {
        p = 0.97;
    }

    if self.mods.ap() {
        p = 0.96;
    }

    if self.mods.dt() && self.mods.hr() {
        p = 0.9825;
    }

    if self.mods.dt() && self.mods.ez() {
        p = 0.9828;
    }

    if self.state.max_combo <= 500 && self.mods.dt() {
        p = 0.96;
    }

    if self.state.max_combo <= 500 && self.mods.hr() && self.mods.dt() {
        p = 0.97;
    }

    if self.state.max_combo <= 500 && self.mods.rx() {
        p = 0.95;
    }

    if self.state.max_combo <= 250 && self.mods.fl() && self.mods.hr() && self.mods.dt() {
        p = 0.91;
    }

    if self.state.max_combo <= 250 && self.mods.rx() && self.mods.fl() && self.mods.hr() && self.mods.dt() {
        p = 0.88;
    }

    if self.state.max_combo >= 5000 && self.effective_miss_count() < 5.0 {
        p = 0.99;
    }

    if self.state.max_combo >= 5000
        && self.effective_miss_count() >= 5.0
        && self.effective_miss_count() != 10.0
    {
        p = 0.983;
    }

    if self.state.max_combo >= 5000
        && self.effective_miss_count() >= 10.0
        && self.effective_miss_count() != 15.0
    {
        p = 0.98;
    }

    if self.state.max_combo >= 5000
        && self.effective_miss_count() >= 15.0
        && self.effective_miss_count() != 20.0
    {
        p = 0.975;
    }

    if self.state.max_combo >= 5000
        && self.effective_miss_count() >= 20.0
        && self.effective_miss_count() != 31.0
    {
        p = 0.97;
    }

    if self.state.max_combo >= 5000
        && self.effective_miss_count() > 30.0
        && self.effective_miss_count() < 50.0
    {
        p = 0.90;
    }

    if self.state.max_combo >= 5000
        && self.effective_miss_count() >= 50.0
        && self.effective_miss_count() != (self.state.max_combo as f64 / 2.0)
    {
        p = 0.85;
    }

    if self.state.max_combo >= 5000 && self.effective_miss_count() >= (self.state.max_combo as f64 / 2.0) {
        p = 0.0000001;
    }

    // Short map tax (from your file)
    let short_tax = 0.5 + 0.5 * (combo / (combo + 500.0));
    p *= short_tax;

    // Final miss exponent
    p.powi(misses as i32)
}