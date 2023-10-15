const COLS: usize = 100;
const ROWS: usize = 100;
fn main() {
    let mut grid = vec![vec![' '; COLS]; ROWS];

    let centerx = 30;
    let centery = 30;
    let radius = 20;
    
    
    for degree in 0..360 {

        let y = centery as f64 + (degree as f64).to_radians().sin() * radius as f64;
        let x = centerx as f64 + (degree as f64).to_radians().cos() * radius as f64;
        println!("{}  {}", x as usize, y as usize);
        grid[y as usize][x as usize] = '*';
    }
    print_grid(&grid);
    

}

fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        let mut substr = String::new();
        for &value in row {
            //println!("{}", value);
            substr.push_str(&value.to_string());
        }
        println!("{}", substr);

    }
}
