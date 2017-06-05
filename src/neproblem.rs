use rand::{Rng, StdRng, SeedableRng};

use ea::*;
use neuro::{NeuralNetwork};
use problem::*;

///
/// Baseline class for optimization problems evolving neural networks.
///
pub trait NeuroProblem: Problem{
    fn compute(&self, nn: &mut NeuralNetwork) -> f32;
    fn get_inputs_count(&self) -> usize;
    fn get_outputs_count(&self) -> usize;
}

// impl<T> Problem for T where T: NeuroProblem {
//     fn compute(&self, )
// }

//--------------------------------------------

///
/// Problems which are typically used to test GP algorithms.
/// See for details: Luke S. Essentials of metaheuristics.
///
#[allow(dead_code)]
pub struct SymbolicRegressionProblem {
    func: fn(&SymbolicRegressionProblem, f32) -> f32,
}

#[allow(dead_code)]
impl SymbolicRegressionProblem {
    pub fn new(problem_type: char) -> SymbolicRegressionProblem {
        match problem_type {
            'f' => SymbolicRegressionProblem::new_f(),
            'g' => SymbolicRegressionProblem::new_g(),
            'h' => SymbolicRegressionProblem::new_h(),
            _ => panic!(format!("Unknown problem type for symbolic regression problem: {}", problem_type))
        }
    }

    pub fn new_f() -> SymbolicRegressionProblem {
        SymbolicRegressionProblem{func: SymbolicRegressionProblem::f}
    }

    pub fn new_g() -> SymbolicRegressionProblem {
        SymbolicRegressionProblem{func: SymbolicRegressionProblem::g}
    }

    pub fn new_h() -> SymbolicRegressionProblem {
        SymbolicRegressionProblem{func: SymbolicRegressionProblem::h}
    }

    fn f(&self, x: f32) -> f32 {
        let x2 = x*x;
        x2*x2 + x2*x + x2 + x
    }

    fn g(&self, x: f32) -> f32 {
        let x2 = x*x;
        x2*x2*x - 2f32*x2*x + x
    }

    fn h(&self, x: f32) -> f32 {
        let x2 = x*x;
        x2*x2*x2 - 2f32*x2*x2 + x2
    }
}

impl Problem for SymbolicRegressionProblem {
    fn compute_from_ind<T: Individual>(&self, ind: &T) -> f32 {
       unimplemented!()
    }
}

impl NeuroProblem for SymbolicRegressionProblem {
    fn get_inputs_count(&self) -> usize {1}
    fn get_outputs_count(&self) -> usize {1}

    fn compute(&self, nn: &mut NeuralNetwork) -> f32 {
        const PTS_COUNT: u32 = 20;

        let mut er = 0f32;
        let mut input = vec![0f32];
        let mut output;
        
        let mut rng: StdRng = StdRng::from_seed(&[0]);
        for _ in 0..PTS_COUNT {
            let x = rng.gen::<f32>(); // sample from [-1, 1]
            let y = (self.func)(&self, x);

            input[0] = x;
            output = nn.compute(&input);

            er += (output[0] - y).abs();
        }
        er
    }
}