type IntType = usize;
const BOARD_SIZE: usize = IntType::BITS as usize;

#[derive(Copy, Clone)] 
struct Board {
    data: [IntType; BOARD_SIZE],
}
impl Board {
    fn new() -> Self {
        let mut board = Board {
            data: [0; BOARD_SIZE],
        };
        
        // add some psudorandom data
        let mut acc: IntType= 383289243938892398;
        for line in &mut board.data {
            acc = acc.overflowing_mul(123).0.overflowing_add(561).0;
            *line = acc;
        }
        board
    }

    fn read(&self, x: usize, y: usize) -> bool {
        self.data[y]>>x & 1 == 1
    }
    fn write(&mut self, x: usize, y: usize, val: bool) {
        if val {
            self.data[y] |= 1 << x
        }
        else {
            self.data[y] &= !(1 << x)
        }
    }

    fn display(&self, debug: bool) {
        print!("||==");for _ in 0..BOARD_SIZE { print!("=="); }println!("==||");
        print!("||  ");for _ in 0..BOARD_SIZE { print!("  "); }println!("  ||");
        for y in 0..BOARD_SIZE {
            print!("||  ");
            for x in 0..BOARD_SIZE {

                if self.read(x,y) {
                    if !debug {print!("[]");}
                    else {print!("\x1B[7;31m[");}
                }
                else {
                    if !debug {print!("  ");}
                    else {print!(" ");}
                }
                if debug{print!("{}",self.full_sum_at(x,y));print!("\x1B[0m");}
            }
            print!("  ||");
            println!();
        }
        print!("||  ");for _ in 0..BOARD_SIZE { print!("  "); }println!("  ||");
        print!("||==");for _ in 0..BOARD_SIZE { print!("=="); }println!("==||");
    }

    fn update(&mut self, old_board: &Board) {
        for y in 0..BOARD_SIZE {
            if false { // iterative implementation
                for x in 0..BOARD_SIZE {self.update_point(old_board,x,y);}
            }
            else { // bitwise implementation
                self.data[y] = old_board.next_row_state(y);
            }
        }
    }



    fn next_row_state(&self, y: usize) -> IntType {
        let sum = Board::full_sum(self.data[(y+1)%BOARD_SIZE], self.data[y], self.data[(y+BOARD_SIZE-1)%BOARD_SIZE]);
        Board::next_row_state_from_sum(sum, self.data[y])
    }

    fn full_sum(a: IntType, b: IntType, c: IntType) -> (IntType, IntType, IntType, IntType) {
        Board::sum_of_partials(Board::partial_sum(a), Board::partial_sum(b), Board::partial_sum(c))
    }
    
    fn partial_sum(a: IntType) -> (IntType, IntType) {
        let b = a.rotate_left(1);
        let c = a.rotate_right(1);
        Board::full_add(a,b,c)
    }

    fn sum_of_partials(p1: (IntType, IntType), p2: (IntType, IntType), p3: (IntType, IntType)) -> (IntType, IntType, IntType, IntType) {
        let f1 = Board::full_add(p1.0,p2.0,p3.0); // (1,2)
        let s1 = f1.0;
        let f2 = Board::full_add(p1.1,p2.1,p3.1); // (2,4)
        let h1 = Board::half_add(f1.1,f2.0); // (2,4)
        let s2 = h1.0;
        let (s3,s4) = Board::half_add(f2.1,h1.1); // (4,8)
        (s1,s2,s3,s4)
    }
    
    fn full_add(a: IntType, b: IntType, c: IntType) -> (IntType, IntType) {
        let h1 = Board::half_add(a,b);
        let h2 = Board::half_add(h1.0,c);
        (h2.0, h1.1|h2.1)
    }

    fn half_add(a: IntType, b: IntType) -> (IntType, IntType) {
        (a^b, a&b)
    }

    fn next_row_state_from_sum(sum: (IntType, IntType, IntType, IntType), state: IntType) -> IntType {
        // (sum == 3) + state * (sum == 4) 
        // s0=1 * s1=1 * s2=0 * s3=0 + state * s0=0 * s1=0 * s2=1 * s3=0 
        // (s0=1 * s1=1 * s2=0 + state * s0=0 * s1=0 * s2=1) * s3=0 
        // (s0 * s1 * !s2 + state * !s0 * !s1 * s2) * s3=0 
        let (s0, s1, s2, s3) = sum;
        ((s0 & s1 & (!s2)) | (state & (!s0) & (!s1) & s2)) & (!s3)
    }

   
    
    fn full_sum_at(&self, x: usize, y: usize) -> usize {
        let (s1, s2, s4, s8) = Board::full_sum(self.data[(y+1)%BOARD_SIZE], self.data[y], self.data[(y+BOARD_SIZE-1)%BOARD_SIZE]);
        (s1>> x & 1) + (s2 >> x & 1)*2 + (s4 >> x & 1)*4 + (s8 >> x & 1)*8

    }

    fn neighborcount(&self, x: usize, y: usize) -> i32 {
        let mut acc = 0;
        for sy in -1..2 {
            for sx in -1..2 {
                let ty = (sy+y as i32 + BOARD_SIZE as i32)%BOARD_SIZE as i32;
                let tx = (sx+x as i32 + BOARD_SIZE as i32)%BOARD_SIZE as i32;
                if !(sx==0 && sy==0) && self.read(tx as usize,ty as usize) {acc += 1;};
            }
        };
        acc
    }

    fn update_point(&mut self, old_board: &Board, x: usize, y: usize) {
        let acc = old_board.neighborcount(x,y);
        let mut w = false;//old_board.read(x,y);
        if acc == 3 || (old_board.read(x,y) && acc == 2) {w = true;}
        self.write(x,y,w);
    }
}


fn main() {
    let mut board = Board::new();
    let mut board_other = Board::new();

    let iterations = 4096;

    println!("Starting board:");
    board.display(false);
    let now = std::time::Instant::now();
    for iteration in 0..iterations{

        board_other.update(&board);
        board = board_other;
        
        if false {
            print!("\x1B[0;0H");
            println!("iteration: {} ", iteration);
            board.display(false);
            std::thread::sleep(std::time::Duration::from_millis(50));
        }
    }
    let elapsed = now.elapsed();
    println!();
    println!("Final board:");
    board.display(false);
    println!("Done {} iterations in: {:.2?}", iterations, elapsed);
}
