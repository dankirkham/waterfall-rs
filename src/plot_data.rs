use fixed_vec_deque::FixedVecDeque;

pub const PLOT_DEPTH: usize = 256;
pub type PlotCell = u8;
pub type PlotRow = Vec<PlotCell>;
pub type PlotData = FixedVecDeque<[PlotRow; PLOT_DEPTH]>;

pub fn new_plot_data() -> PlotData {
    FixedVecDeque::<[PlotRow; PLOT_DEPTH]>::new()
}
