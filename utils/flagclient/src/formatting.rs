// The Leaderboard Table:
// ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
// ┇                Leaderboard                ┇
// ┇   1.  michael..................1024       ┇
// ┇   2.  krunk.....................500       ┇
// ┇   4.  adam.......................42       ┇
// ┇   5.  andrew......................5       ┇
// ┇   6.  janie.......................2       ┇
// ┇   33. a_long_username_ove.........1       ┇
// ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

const SMALL_BOARD: &'static str = "
┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
┇                Leaderboard                 ┇
┇ Nobody is in the lead! You could be first! ┇
┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
";

pub fn generate_leaderboard(start: usize, scores: &Vec<(i32, String)>) -> String {
    if scores.len() == 0 {
        return SMALL_BOARD.to_string();
    }

    let title_len = 11;

    let longest_entry = match scores.iter().map(|&(ref s, ref n)| n.len() + s.digits()).max() {
        Some(n) if n + 3 < title_len => title_len,
        Some(n) => n + 3,
        _ => title_len,
    };
    let longest_place = (start + scores.len()).digits() + 1;
    let width = longest_entry + (longest_place + 1) * 2 + 6;

    let header = format!("┏{}┓", '━'.times(width));

    let space  = width - title_len;
    let title  = format!("┇{}Leaderboard{}┇", ' '.times(space / 2), ' '.times(space - (space / 2)));

    let mut entries: Vec<String> = Vec::with_capacity(scores.len());
    let mut place = start;

    for &(ref score, ref user) in scores.iter() {
        place += 1;

        let place_str = format!("{}.{}", place, ' '.times(longest_place - place.digits()));
        let dots = longest_entry - (user.len() + score.digits());
        let entry = format!("{}{}{}", user, '.'.times(dots), score);

        let line = format!("┇   {}{}{}   ┇", place_str, entry, ' '.times(place_str.len()));
        entries.push(line);
    }

    let info = entries.connect("\n");

    let footer = format!("┗{}┛", '━'.times(width));

    format!("{}\n{}\n{}\n{}", header, title, info, footer)
}

trait Digits {
    fn digits(&self) -> usize;
}

impl Digits for i32 {
    fn digits(&self) -> usize {
        let mut val = *self;
        let mut bump = 1;
        if val < 0 {
            val *= -1;
            bump = 2;
        }
        (val as f64).log(10f64) as usize + bump
    }
}

impl Digits for usize {
    fn digits(&self) -> usize {
        (*self as f64).log(10f64) as usize + 1
    }
}

trait Times {
    fn times(&self, n: usize) -> String;
}

impl Times for char {
    fn times(&self, n: usize) -> String {
        let chain = vec![self.to_string(); n];
        chain.connect("")
    }
}
