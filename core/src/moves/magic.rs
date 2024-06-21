use crate::board::{square::NUM_BOARD_SQUARES, Bitboard};

/*
* Reference: https://www.chessprogramming.org/Magic_Bitboards
*/

pub const ROOK_MAGIC_TABLE_SIZE: usize = 102_400;

#[rustfmt::skip]
pub const ROOK_MAGIC_NUMS: [u64; NUM_BOARD_SQUARES] = [
    324259448050975248u64, 162139001189302336u64, 4647750006529359880u64, 144121785691422736u64,
    16176938657641660544u64, 9367489423970945072u64, 36051338366288384u64, 36029147746665088u64,
    3518447965192208u64, 4614078830617822340u64, 9241949523864129664u64, 11540615780106252u64,
    730287067600519297u64, 144819425575437312u64, 1225261127674627584u64, 40814017656160512u64,
    594475700577118276u64, 283675082228259u64, 148058037853261952u64, 14411662294658320384u64,
    2394186703782912u64, 1157847866488718336u64, 2306407062973841412u64, 4576167411597460u64,
    2323857959626489888u64, 18860477004136448u64, 621497027752297522u64, 3027553647748714496u64,
    9241953785514295424u64, 1970363492082688u64, 1729664285938024960u64, 4836870457972064321u64,
    141012374650913u64, 4652253601601699840u64, 58687601506263040u64, 281543780081672u64,
    1157433900411130112u64, 81628378934806544u64, 2310366730829959192u64, 2900476768907429780u64,
    36558770110480u64, 9042384969023488u64, 180425597514743824u64, 5487636764434923528u64,
    5766860422494879764u64, 9224498487624761348u64, 41702298761822218u64, 45599234000551940u64,
    70370891935872u64, 19210671497487104u64, 387030266675328u64, 289215847808893056u64,
    576469550545240192u64, 1153216449143113729u64, 9350715278336u64, 288521763922764288u64,
    282782794268833u64, 595672521157161122u64, 436884352794689609u64, 9241667927690743809u64,
    5188428314494240769u64, 1157988067282792450u64, 1152939243166828548u64, 4611967569673330817u64,
];

pub const BISHOP_MAGIC_TABLE_SIZE: usize = 5_248;

#[rustfmt::skip]
pub const BISHOP_MAGIC_NUMS: [u64; NUM_BOARD_SQUARES] = [
    2310454429704290569u64, 37163502750244928u64, 145330200115150856u64, 573953659699200u64,
    9845999220824211456u64, 574016004032512u64, 10093699283674480640u64, 2306407060834902016u64,
    2883575003184432136u64, 1747410678824308864u64, 9259405249167245312u64, 936784527773139074u64,
    4629702641998381057u64, 201028145628315697u64, 4899992295377881088u64, 4630405483133404688u64,
    153474299838154784u64, 2286992943744036u64, 434597432802681416u64, 865817269052115456u64,
    9156750026475656u64, 599823317909770240u64, 4578375142474880u64, 2308525819264500224u64,
    18596057879421451u64, 18331093560345096u64, 2305880392877736000u64, 56602859688444160u64,
    5382084129205534724u64, 5767422822691897608u64, 283691220206592u64, 144398865845093376u64,
    1163523824685120u64, 20267333288223264u64, 325489801822240u64, 4755836425302245636u64,
    594475563668865152u64, 1162496335329427604u64, 9244765235704371236u64, 576667461564269056u64,
    146371454722771202u64, 426679365288452u64, 13724105480340736u64, 1152922330050364928u64,
    4620737202526097424u64, 1316476062695166464u64, 13981996823661781640u64, 12430506881068303489u64,
    5193780677221351424u64, 426612797737280u64, 37445932288049152u64, 1171147012042137601u64,
    504403227018657856u64, 4629845569785954560u64, 4686013077882208273u64, 1154056209263894528u64,
    613054853085794304u64, 9025075185721408u64, 9571249324951568u64, 10999715432448u64,
    290408795603472u64, 10664524198170591488u64, 5924513492108288u64, 90511840181764112u64,
];

/// 
pub const fn magic_index(mask: Bitboard, occupied: Bitboard, magic: u64, shift: u8, offset: u64) -> usize {
    let blockerboard = occupied & mask;
    ((blockerboard.wrapping_mul(magic) >> shift) + offset) as usize
}

/// Generates all permutaions of the given mask.
/// Takes a mask representing the squares a piece can move to, without the edges.
pub fn blocker_boards(mask: Bitboard) -> Vec<Bitboard> {
    let mut boards = vec![];
    let mut n: Bitboard = 0;
    // https://www.chessprogramming.org/Traversing_Subsets_of_a_Set
    loop {
        boards.push(n);
        n = n.wrapping_sub(mask) & mask;
        if n == 0 {
            break;
        }
    }
    boards
}
