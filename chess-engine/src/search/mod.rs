use chess_core::{board::{piece::{NUM_PIECE_KINDS, NUM_PIECE_SIDES}, Board}, moves::{MoveList, ShortMove}};

pub enum Searching {
    Infinite,
    Mate(usize),
    Depth(usize),
    Nodes(usize),
}

impl Searching {
    pub fn stop(&self) -> (ShortMove, Option<ShortMove>) {
        todo!()
    }
}

pub(crate) struct SearchTree {
    root: Board,
    pvs: Vec<SearchLeaf<'static>>,
    current_depth: u16,
}

impl SearchTree {
    pub fn new(board: Board) -> Self {
        Self {
            root: board,
            pvs: vec![],
            current_depth: 0,
        }
    }
}

struct SearchBranch<'a> {
    short: ShortMove,
    flags: u16,
    score: i32,
    children: Option<&'a[SearchBranch<'a>]>
}

struct SearchLeaf<'a> {
    variation: &'a [ShortMove],
    depth: u16,
    flags: u16,
    score: f32,
}

const FINAL_MASK: u16 = 0b0000_0000_0000_0011;
const DELTA: u16 = 0b0000_0000_0000_0000;
const WIN: u16 = 0b0000_0000_0000_0001;
const LOSS: u16 = 0b0000_0000_0000_0010;
const DRAW: u16 = 0b0000_0000_0000_0011;
const CHECK_EXTENSION: u16 = 0b0000_0000_0000_0100;
const PROMOTION_REEVAL: u16 = 0b0000_0000_0000_1000;

fn eval_moves(moves: MoveList, board: &Board) -> impl Iterator<Item = (i16, ShortMove)> {
    let mut board = board.clone();
    moves.into_iter().map(move |m| {
        if m.is_capturing() {
            let (side, piece) = board.square(m.dest()).expect("can not capture empty space");
            board.remove_piece(side, piece, m.dest());
        }
        // TODO: add promote, enpasant, castling, dont discard work to generate post move board
        let (side, piece) = board.square(m.src()).expect("can not move from empty space");
        board.move_piece(side, piece, m.src(), m.dest());
        let mut sum = 0i16;
        for side in 0..NUM_PIECE_SIDES {
            let mut score = 0i16;
            for piece in 0..NUM_PIECE_KINDS {
                let count = board.piece(side.into(), piece.into()).count_ones();
                score = count as i16 * MATERIAL_VALUES[piece as usize];
            };
            if side == 1 {
                sum -= score;
            } else {
                sum += score;
            }
        }
        (sum, m)
    })
}

fn sort_moves(input: impl Iterator<Item = (i16, ShortMove)>) -> Box<[(i16, ShortMove)]> {
    let mut x = input.collect::<Vec<_>>();
    x.sort_unstable_by(|a, b| b.0.cmp(&a.0));
    x.into_boxed_slice()
}

const MATERIAL_VALUES: [i16; NUM_PIECE_KINDS] = [ 10, 30, 40, 50, 90, 0 ];

fn ucb_score(depth_dist: u16, eval_score: f32) -> f32 {
    todo!()
}
