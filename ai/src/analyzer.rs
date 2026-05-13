//! Analizzatore MSAS e IRCM
//!
//! MSAS (Multi-Scale Autocorrelation Signature) - rilevamento struttura
//! IRCM (Iterative Relational Calculus with Memory) - proiezione multidimensionale

// Se non usi HashMap, rimuovilo. Se lo usi, tienilo.
#[derive(Debug)]
pub struct MsasAnalyzer {
    epsilon: f64,
}

#[derive(Debug)]
pub struct IrcmProjector {
    dimensions: usize,
}

#[derive(Debug, Clone)]
pub struct MsasResult {
    pub index: f64,
    pub has_structure: bool,
    pub structure_strength: f64,
}

#[derive(Debug, Clone)]
pub struct IrcmProjection {
    pub point: Vec<f64>,
    pub s_score: f64,
    pub is_equilibrium: bool,
}

impl MsasAnalyzer {
    pub fn new() -> Self {
        Self { epsilon: 0.001 }
    }

    pub fn with_epsilon(epsilon: f64) -> Self {
        Self { epsilon }
    }

    pub fn compute(&self, sequence: &[f64]) -> MsasResult {
        if sequence.is_empty() {
            return MsasResult { index: 0.0, has_structure: false, structure_strength: 0.0 };
        }
        let mean = sequence.iter().sum::<f64>() / sequence.len() as f64;
        let variance = sequence.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / sequence.len() as f64;
        let msas = variance.sqrt();
        MsasResult { index: msas, has_structure: msas > self.epsilon, structure_strength: msas }
    }

    pub fn correlate(&self, a: &[f64], b: &[f64]) -> f64 {
        let min_len = a.len().min(b.len());
        if min_len == 0 { return 0.0; }
        let a_mean = a[..min_len].iter().sum::<f64>() / min_len as f64;
        let b_mean = b[..min_len].iter().sum::<f64>() / min_len as f64;
        let numerator: f64 = (0..min_len).map(|i| (a[i] - a_mean) * (b[i] - b_mean)).sum();
        let a_denom: f64 = (0..min_len).map(|i| (a[i] - a_mean).powi(2)).sum();
        let b_denom: f64 = (0..min_len).map(|i| (b[i] - b_mean).powi(2)).sum();
        if a_denom == 0.0 || b_denom == 0.0 { 0.0 } else { numerator / (a_denom * b_denom).sqrt() }
    }
}

impl IrcmProjector {
    pub fn new() -> Self { Self { dimensions: 6 } }
    pub fn with_dimensions(dimensions: usize) -> Self { Self { dimensions } }
    pub fn project(&self, values: &[f64]) -> IrcmProjection {
        let mut point = vec![0.0; self.dimensions];
        for i in 0..self.dimensions.min(values.len()) { point[i] = values[i]; }
        let s_score = self.compute_s_score(&point);
        IrcmProjection { point, s_score, is_equilibrium: (s_score - 1.0).abs() < 0.01 }
    }
    fn compute_s_score(&self, point: &[f64]) -> f64 {
        let v = point.get(0).copied().unwrap_or(1.0);
        let i = point.get(1).copied().unwrap_or(1.0);
        let t = point.get(2).copied().unwrap_or(1.0);
        let k = point.get(3).copied().unwrap_or(1.0);
        if t * k == 0.0 { 1.0 } else { (v * i) / (t * k) }
    }
    pub fn find_equilibrium(&self, points: &[IrcmProjection]) -> Option<usize> {
        points.iter().enumerate().filter(|(_, p)| p.is_equilibrium).map(|(i, p)| (i, (p.s_score - 1.0).abs())).min_by(|a, b| a.1.partial_cmp(&b.1).unwrap()).map(|(i, _)| i)
    }
}

impl Default for MsasAnalyzer { fn default() -> Self { Self::new() } }
impl Default for IrcmProjector { fn default() -> Self { Self::new() } }
