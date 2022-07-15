use rand::Rng;
use fixed_vec_deque::FixedVecDeque;

pub const PLOT_DEPTH: usize = 128;
pub const PLOT_WIDTH: usize = 256;
pub type PlotCell = u8;
pub type PlotRow = Vec<PlotCell>;
pub type PlotData = FixedVecDeque<[PlotRow; PLOT_DEPTH]>;

pub fn new_row() -> PlotRow {
    let mut rng = rand::thread_rng();
    let mut row: PlotRow = Vec::with_capacity(PLOT_WIDTH);
    for _ in 0..PLOT_WIDTH {
        row.push(rng.gen_range(0..255));
    }
    row
}

pub fn new_plot_data() -> PlotData {
    FixedVecDeque::<[PlotRow; PLOT_DEPTH]>::new()
}
