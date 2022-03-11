use std::fmt;

const PLATE_SIZE: usize = 81;
const KINDS_SIZE: usize = 3;
const GROUP_SIZE: usize = 9;
const SET_SIZE: usize = 9;

//The simple Soduko plate repr as u8 array, 0 represents not set yet.
pub struct Plate {
    pub array: [u8; PLATE_SIZE],
}
impl Plate {
    //function to initialize plate from an input string
    pub fn new(input_str: &str) -> Plate {
        //iter over string as chars into valid u8 ints
        let input_digits_iter = input_str
            .chars()
            .map(|x| if x == '.' { '0' } else { x })
            .map(|x| x.to_digit(10)) //convert if possible into Some 10-base digit
            .filter(|p| p.is_some()) //drop None (not some / invalid) digits
            .map(|x| x.unwrap() as u8); //unwrap option(maybe monad) and cast as u8

        //allocate fix sized u8_array and write in digits
        //not just collecting into vector as I fail to
        //to pre-specify vector capacity and I need to move out
        // an array anyways as prog is stack only
        let mut x: [u8; PLATE_SIZE] = [0; PLATE_SIZE];
        for (i, elem) in input_digits_iter.enumerate() {
            if i >= PLATE_SIZE {
                break;
            };
            x[i] = elem;
        }

        Plate { array: x }
    }
}

//describes all groups of one kind row/col/square (9 groups of each kind)
//each one group within a kind is 9 sized bool vector describing the set of values
struct GroupArray {
    arr2d: [[bool; SET_SIZE]; GROUP_SIZE],
}
impl GroupArray {
    fn new() -> GroupArray {
        GroupArray {
            arr2d: [[false; SET_SIZE]; GROUP_SIZE],
        }
    }
}
impl fmt::Display for GroupArray {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, x) in self.arr2d.iter().enumerate() {
            write!(f, "\n [{}]", i)?;
            for (j, y) in x.iter().enumerate() {
                if *y {
                    write!(f, " {}", j + 1)?
                };
            }
        }
        Ok(())
    }
}

//describes states of the 27 groups of a soduku plate
//groups are accessed via kind row/col/square
//a complete plate has all sets of all groups equal to {1,2,...,9}
//a not yet solved plate will subsets thereof
pub struct GroupArrayCollection {
    row: GroupArray,
    col: GroupArray,
    square: GroupArray,
}
impl GroupArrayCollection {
    fn new() -> GroupArrayCollection {
        GroupArrayCollection {
            row: GroupArray::new(),
            col: GroupArray::new(),
            square: GroupArray::new(),
        }
    }
}

impl fmt::Display for GroupArrayCollection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\nROW {} ", self.row)?;
        write!(f, "\nCOL {} ", self.col)?;
        write!(f, "\nSQUARE {} ", self.square)?;
        Ok(())
    }
}

//suport functions for Soduko::validate()
pub fn get_col_iter(i: usize) -> impl Iterator<Item = usize> {
    (0..PLATE_SIZE).filter(move |x| (x % GROUP_SIZE) == i)
}
pub fn get_row_iter(i: usize) -> impl Iterator<Item = usize> {
    (0..PLATE_SIZE).filter(move |x| (x / GROUP_SIZE) == i)
}
pub fn get_square_idx(i: usize) -> usize {
    KINDS_SIZE * ((i / GROUP_SIZE) / KINDS_SIZE) + (i % GROUP_SIZE) / KINDS_SIZE
}
pub fn get_square_iter(i: usize) -> impl Iterator<Item = usize> {
    (0..PLATE_SIZE).filter(move |x| get_square_idx(*x) == i)
}

pub struct GroupMembership {
    row: usize,
    col: usize,
    square: usize,
}

pub struct Soduko {
    plate: Plate,
    groups: GroupArrayCollection,
    solved: bool,
    valid: Option<bool>,
    steps_taken: usize,
}

impl Soduko {
    pub fn new(input_str: &str) -> Soduko {
        Soduko {
            plate: Plate::new(input_str),
            groups: GroupArrayCollection::new(),
            solved: false,
            valid: None,
            steps_taken: 0,
        }
    }

    //check validity of plate
    fn validate(&mut self) -> bool {
        for i in 0..GROUP_SIZE {
            if !self.check_group(get_col_iter(i)) {
                println!("failed col {i}");
                self.valid = Some(false);
                return false;
            }
            if !self.check_group(get_row_iter(i)) {
                println!("failed row {i}");
                self.valid = Some(false);
                return false;
            }
            if !self.check_group(get_square_iter(i)) {
                println!("failed square {i}");
                self.valid = Some(false);
                return false;
            }
        }
        self.valid = Some(true);
        return true;
    }

    //support for validate
    fn check_group(&self, iter: impl Iterator<Item = usize>) -> bool {
        let mut col_count: [bool; SET_SIZE] = [false; SET_SIZE];
        for j in iter {
            let value_idx = self.plate.array[j] as usize;
            if value_idx == 0 {
                continue;
            }
            if col_count[value_idx - 1] {
                return false;
            }
            col_count[value_idx - 1] = true;
        }
        true
    }

    //support for validate
    fn options_group(&self, i_cell: usize) -> [bool; SET_SIZE] {
        let groupmem = self.get_group_membership(i_cell);
        let mut val_set: [bool; SET_SIZE] = [false; SET_SIZE];
        
        //combine three groups of i_cell into one iter
        let groups_iter = get_row_iter(groupmem.row)
            .chain(get_col_iter(groupmem.col))
            .chain(get_square_iter(groupmem.square));
        
        //lookup what numbers are already taken in this group and write to bool array
        for j in groups_iter {
            let j_plate_value: usize = self.plate.array[j] as usize;
            if j_plate_value != 0 {
                val_set[j_plate_value - 1] = true;
            }
        }

        return val_set
    }

    fn get_group_membership(&self, i: usize) -> GroupMembership {
        let i_row_group = i / GROUP_SIZE;
        let i_col_group = i % GROUP_SIZE;
        let i_square_group = KINDS_SIZE * (i_row_group / KINDS_SIZE) + i_col_group / KINDS_SIZE;

        GroupMembership {
            row: i_row_group,
            col: i_col_group,
            square: i_square_group,
        }
    }

    //transform plate in to groups of sets and check validity.
    //Important to check as the repr cannot describe any invalid plate
    fn prepare_groups(&mut self) -> bool {
        for (i, elem) in self.plate.array.iter().enumerate() {
            let i_group = self.get_group_membership(i);
            if *elem == 0 {
                continue;
            };
            let idx = *elem as usize - 1;

            if self.groups.col.arr2d[i_group.col][idx]
                || self.groups.row.arr2d[i_group.row][idx]
                || self.groups.square.arr2d[i_group.square][idx]
            {
                println!("validation failed at i={i}");
                self.valid = Some(false);
                return false;
            }

            self.groups.col.arr2d[i_group.col][idx] = true;
            self.groups.row.arr2d[i_group.row][idx] = true;
            self.groups.square.arr2d[i_group.square][idx] = true;
        }
        self.valid = Some(true);
        return true;
    }

    fn find_options_fast(&self, i_cell: usize) -> [bool; SET_SIZE] {
        let i_group = self.get_group_membership(i_cell);
        self.find_options_fast_backend(&i_group)
    }

    fn find_options_fast_backend(&self, i_group: &GroupMembership) -> [bool; SET_SIZE] {
        //print!("row{} col{} sqaure {}",i_group.row,i_group.col,i_group.square);
        let mut options: [bool; SET_SIZE] = [false; SET_SIZE];
        for j in 0..SET_SIZE {
            options[j] = self.groups.col.arr2d[i_group.col][j]
                || self.groups.row.arr2d[i_group.row][j]
                || self.groups.square.arr2d[i_group.square][j]
        }
        return options;
    }

    pub fn solve_fast(&mut self) -> () {
        //init recursive
        self.prepare_groups();
        if self.valid.unwrap() {
            self.sf_fast_next(0);
        } else {
            println!("input plate is invalid");
        }
    }

    fn sf_fast_next(&mut self, mut i_cell: usize) -> () {
        self.steps_taken += 1;
        //if this cell in plate was preset, skip to next cell
        while i_cell < PLATE_SIZE && self.plate.array[i_cell] != 0 {
            i_cell = i_cell + 1;
        }

        //recursive completion bound, stop if out of plate
        if i_cell >= PLATE_SIZE {
            self.solved = true;
            return;
        }
        //print!("\n i_cell:{i_cell}");
        //find elems in groups that celll belongs to (one row, one col and one square)
        let i_group = self.get_group_membership(i_cell);

        //find elem options for this cell
        let options = self.find_options_fast_backend(&i_group);

        //try each option an move on to next cell
        for (j, j_bool) in options.iter().enumerate() {
            //if this set elem not in any group
            if *j_bool == false {
                //print!(" try:{}",j+1);
                //write to plate
                self.plate.array[i_cell] = j as u8 + 1;

                //write to group
                self.groups.col.arr2d[i_group.col][j] = true;
                self.groups.row.arr2d[i_group.row][j] = true;
                self.groups.square.arr2d[i_group.square][j] = true;

                //move to next cell
                self.sf_fast_next(i_cell + 1);

                //comming back, check if solved then return
                if self.solved {
                    return;
                } //check if solved then return

                //else not solved current option proved not to work

                //remove failed option from plate
                self.plate.array[i_cell] = 0;

                //remove failed option from groups
                self.groups.col.arr2d[i_group.col][j] = false;
                self.groups.row.arr2d[i_group.row][j] = false;
                self.groups.square.arr2d[i_group.square][j] = false;

                //try a new option for this cell
            }
        }
        //dead-end all options are exhausted in this cell, go back alter previous cells
        return;
    }

    pub fn solve(&mut self) -> () {
        //init recursive
        self.validate();
        if self.valid.unwrap() {
            self.solve_next(0);
        } else {
            println!("input plate is invalid");
        }
    }

    fn solve_next(&mut self, mut i_cell: usize) -> () {
        self.steps_taken += 1;
        //if this cell in plate was preset, skip to next cell
        while i_cell < PLATE_SIZE && self.plate.array[i_cell] != 0 {
            i_cell = i_cell + 1;
        }

        //recursive completion bound, stop if out of plate
        if i_cell >= PLATE_SIZE {
            self.solved = true;
            return;
        }

        //find elem options for this cell
        let options = self.options_group(i_cell);

        //try each option an move on to next cell
        for (j, j_bool) in options.iter().enumerate() {
            //if this set elem not in any group
            if *j_bool == false {
                //print!(" try:{}",j+1);
                //write to plate
                self.plate.array[i_cell] = j as u8 + 1;

                //move to next cell
                self.solve_next(i_cell + 1);

                //comming back, check if solved then return
                if self.solved {
                    return;
                } //check if solved then return

                //else not solved current option proved not to work

                //remove failed option from plate
                self.plate.array[i_cell] = 0;


                //try a new option for this cell
            }
        }
        //dead-end all options are exhausted in this cell, go back alter previous cells
        return;
    }
}
impl fmt::Display for Soduko {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n||soduko plate||\n")?;

        for (i, elem) in self.plate.array.iter().enumerate() {
            if i % 3 == 0 && i != 0 {
                write!(f, " ")?;
            }
            if i % 9 == 0 && i != 0 {
                write!(f, "\n")?;
            }
            if i % 27 == 0 && i != 0 {
                write!(f, "\n")?;
            }
            write!(f, "{}", elem)?;
        }

        write!(f, "\n plate solved: {}", self.solved)?;
        write!(f, "\n steps used: {}", self.steps_taken)?;

        Ok(())
    }
}

fn main() {
    let mut s = Soduko::new(
        "
    12. ..6 7..
    ..3 ..8 ...
    ... ... ...
    
    ... ..1 ...
    ... 78. ...
    .9. ... ...
    
    .3. ... .2.
    .7. .9. ..8
    2.. 3.. 5.1
    
    ",
    );

    println!("s is {}", s);

    // let pos_set = s.find_possible_set(0);
    // println!("pos set is {:?}", pos_set);

    s.validate();
    if let Some(x) = s.valid {
        println!("this plate is valid: {}", x);
    } else {
        println!("this plate is valid: Unknown");
    }

    s.solve_fast();
    println!("s is {}", s);
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn valid_plate() {
        let mut s = Soduko::new(
            "
            123 456 78.
            ... ... ...
            ... ... ...
            
            .8. ..1 ...
            ... 78. .5.
            ... ... ...
            
            ..1 ... .2.
            ... .9. ...
            ... ... ..1",
        );
        assert_eq!(s.validate(), true);

        assert_eq!(s.prepare_groups(), true)
    }

    #[test]
    fn invalid_row() {
        let mut inv_row = Soduko::new(
            "
            123 156 78.
            ... ... ...
            ... ... ...
            
            .8. ..1 ...
            ... 78. .5.
            ... ... ...
            
            ..1 ... .2.
            ... .9. ...
            ... ... ..1",
        );
        assert_eq!(inv_row.validate(), false);

        assert_eq!(inv_row.prepare_groups(), false);
    }

    #[test]
    fn invalid_col() {
        let mut inv_col = Soduko::new(
            "
            123 456 78.
            ... ... ...
            ... ... .2.
            
            .8. ..1 ...
            ... 78. .5.
            ... ... ...
            
            ..1 ... .2.
            ... .9. ...
            ... ... ..1",
        );
        assert_eq!(inv_col.validate(), false);

        assert_eq!(inv_col.prepare_groups(), false);
    }

    #[test]
    fn invalid_square() {
        let mut inv_square = Soduko::new(
            "
            123 456 78.
            ... ... ...
            ... ... .2.
            
            .8. ..1 ...
            ... 78. .5.
            ... ... ...
            
            ..1 ... ...
            ... .9. 1..
            ... ... ..1",
        );
        assert_eq!(inv_square.validate(), false);

        assert_eq!(inv_square.prepare_groups(), false);
    }

    #[test]
    fn empty_plate() {
        let mut inv_square = Soduko::new("no digits in string");
        assert_eq!(inv_square.validate(), true);

        assert_eq!(inv_square.prepare_groups(), true);
    }

    #[test]
    fn find_options_fast() {
        let mut s = Soduko::new(
            "
            123 456 78.
            ... ... ...
            ... ... ..9
            
            ... ... ...
            .9. ... ...
            ... ... ...
            
            ... ... ...
            ... ... ...
            ... ... ...",
        );

        s.prepare_groups();
        println!("s is {}", s);
        println!("g is {}", s.groups);

        assert_eq!(
            s.find_options_fast(0),
            [true, true, true, true, true, true, true, true, false]
        );
        assert_eq!(
            s.find_options_fast(1),
            [true, true, true, true, true, true, true, true, true]
        );
        assert_eq!(
            s.find_options_fast(7),
            [true, true, true, true, true, true, true, true, true]
        );
        assert_eq!(
            s.find_options_fast(9),
            [true, true, true, false, false, false, false, false, false]
        );
    }
}


#[test]
fn two_solvers_same_solutions() {

    let input_string = "
    12. ..6 7..
    ..3 ..8 ...
    ... ... ...
    
    ... ..1 ...
    ... 78. ...
    .9. ... ...
    
    .3. ... .2.
    .7. .9. ..8
    2.. 3.. 5.1
    ";

   let mut s1 = Soduko::new(input_string);
   let mut s2 = Soduko::new(input_string);
   s1.solve_fast();
   s2.solve();
   println!("s1 is {} \ns2 is {}", s1, s2);

   assert_eq!(s1.solved, true);
   assert_eq!(s2.solved, true);

   assert_eq!(
       s1.plate.array,
       s2.plate.array
    );

}