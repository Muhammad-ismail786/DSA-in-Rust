
package main

import "fmt"

func totalNQueens(n int) int {

    count := 0

    cols := make(map[int]bool)
    diag1 := make(map[int]bool)
    diag2 := make(map[int]bool)

    var solve func(row int)

    solve = func(row int) {

        if row == n {
            count++
            return
        }

        for col := 0; col < n; col++ {

            d1 := row - col
            d2 := row + col

            if cols[col] || diag1[d1] || diag2[d2] {
                continue
            }

            cols[col] = true
            diag1[d1] = true
            diag2[d2] = true

            solve(row + 1)

            delete(cols, col)
            delete(diag1, d1)
            delete(diag2, d2)
        }
    }

    solve(0)

    return count
}

func main() {

    var n int

    fmt.Print("Enter n: ")
    fmt.Scan(&n)

    fmt.Println("Total Solutions:", totalNQueens(n))
}
