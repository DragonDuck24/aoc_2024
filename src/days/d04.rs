// https://adventofcode.com/2024/day/4

pub fn solve(input: String) -> (String, String) {
    let mut p1 = 0;

    let lines = input.lines().map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

    let rows = lines.len();
    let cols = lines[0].len();

    for i in 0..rows {
        for j in 0..cols {
            if lines[i][j] == 'X' {
                if i < rows - 3 && lines[i+1][j] == 'M' && lines[i+2][j] == 'A' && lines[i+3][j] == 'S' {
                    p1 += 1;
                }
                if i >= 3 && lines[i-1][j] == 'M' && lines[i-2][j] == 'A' && lines[i-3][j] == 'S' {
                    p1 += 1;
                }
                if j < cols - 3 && lines[i][j+1] == 'M' && lines[i][j+2] == 'A' && lines[i][j+3] == 'S' {
                    p1 += 1;
                }
                if j >= 3 && lines[i][j-1] == 'M' && lines[i][j-2] == 'A' && lines[i][j-3] == 'S' {
                    p1 += 1;
                }

                if i < rows - 3 && j < cols - 3 && lines[i+1][j+1] == 'M' && lines[i+2][j+2] == 'A' && lines[i+3][j+3] == 'S' {
                    p1 += 1;
                }
                if i >= 3 && j >= 3 && lines[i-1][j-1] == 'M' && lines[i-2][j-2] == 'A' && lines[i-3][j-3] == 'S' {
                    p1 += 1;
                }
                if i < rows - 3 && j >= 3 && lines[i+1][j-1] == 'M' && lines[i+2][j-2] == 'A' && lines[i+3][j-3] == 'S' {
                    p1 += 1;
                }
                if i >= 3 && j < cols - 3 && lines[i-1][j+1] == 'M' && lines[i-2][j+2] == 'A' && lines[i-3][j+3] == 'S' {
                    p1 += 1;
                }
            }
        }
    }


    let mut p2 = 0;

    for i in 0..rows {
        for j in 0..cols {
            if lines[i][j] == 'A' && i > 0 && j > 0 && i < rows-1 && j < cols-1 {
                let dl = lines[i-1][j-1] == 'M' && lines[i+1][j+1] == 'S' || lines[i-1][j-1] == 'S' && lines[i+1][j+1] == 'M';
                let dr = lines[i-1][j+1] == 'M' && lines[i+1][j-1] == 'S' || lines[i-1][j+1] == 'S' && lines[i+1][j-1] == 'M';
                if dr && dl {
                    p2 += 1;
                }
            }
        }
    }


    (p1.to_string(), p2.to_string())
}



