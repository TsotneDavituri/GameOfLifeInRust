use std::{thread};
use std::time::Duration;
use minifb::{Key, Window, WindowOptions};
use rand::Rng;

const WIDTH: usize = 1200;
const HEIGHT: usize = 1200;

fn main() {
    let rows = 200;
    let cols = 200;
    let mut window = Window::new("Game of Life - ESC to exit", WIDTH, HEIGHT, WindowOptions::default())
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut grid = generate_grid(rows, cols);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        draw_grid(&grid, &mut buffer, rows, cols);
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();

        thread::sleep(Duration::from_millis(500));
        update_grid(&mut grid, rows, cols);
    }
}

fn draw_grid(grid: &Vec<Vec<i8>>, buffer: &mut Vec<u32>, rows: usize, cols: usize) {
    let cell_width = WIDTH / cols;
    let cell_height = HEIGHT / rows;

    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            let color = if cell == 1 { 0xFFFFFF } else { 0x000000 };
            for i in 0..cell_height {
                for j in 0..cell_width {
                    let idx = (y * cell_height + i) * WIDTH + (x * cell_width + j);
                    buffer[idx] = color;
                }
            }
        }
    }
}

fn generate_grid(rows: usize, cols: usize) -> Vec<Vec<i8>> {
    let mut rng = rand::thread_rng();
    (0..rows).map(|_| {
        (0..cols).map(|_| {
            rng.gen_range(0..=1)
        }).collect()
    }).collect()
}

fn count_neighbours(grid: &Vec<Vec<i8>>, row: isize, col: isize, rows: usize, cols: usize) -> i8 {
    let mut count = 0;
    for i in -1..=1 {
        for j in -1..=1 {
            if i == 0 && j == 0 {
                continue;
            }
            let new_row = row + i;
            let new_col = col + j;
            if new_row >= 0 && new_row < rows as isize && new_col >= 0 && new_col < cols as isize {
                count += grid[new_row as usize][new_col as usize];
            }
        }
    }
    count
}

fn update_grid(grid: &mut Vec<Vec<i8>>, rows: usize, cols: usize) {
    let mut new_grid = grid.clone();

    for row in 0..rows {
        for col in 0..cols {
            let neighbours = count_neighbours(grid, row as isize, col as isize, rows, cols);
            if neighbours > 3 &&  grid[row][col] == 1 {
                new_grid[row][col] = 0
            }
            if neighbours < 2 && grid[row][col] == 1 {
                new_grid[row][col] = 0
            }
            if neighbours == 3 && grid[row][col] == 0 {
                new_grid[row][col] = 1
            }
        }
    }
    *grid = new_grid;
}
