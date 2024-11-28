#include <chrono>
#include <cmath>
#include <iostream>
#include <unordered_set>
#include <vector>

using namespace std;

class Solution {
private:
  bool checkSubBoxes(vector<vector<char>> &board, int startRow, int startCol) {
    unordered_set<char> subMatrix;
    for (int i = 0; i < 3; i++) {
      for (int j = 0; j < 3; j++) {
        char current = board[startRow + i][startCol + j];
        if (current != '.' && subMatrix.find(current) != subMatrix.end()) {
          return false;
        }
        if (current != '.') {
          subMatrix.insert(current);
        }
      }
    }
    return true;
  }

  bool checkColumns(vector<vector<char>> &board, int colIdx, int columnLen) {
    unordered_set<char> col_set;
    for (int i = 0; i < columnLen; i++) {
      char current = board[i][colIdx];
      if (current != '.' && col_set.find(current) != col_set.end()) {
        return false;
      }
      if (current != '.') {
        col_set.insert(current);
      }
    }
    return true;
  }

  bool checkRows(vector<char> &row) {
    unordered_set<char> row_set;
    for (char &i : row) {
      if (i != '.' && row_set.find(i) != row_set.end()) {
        return false;
      }
      if (i != '.') {
        row_set.insert(i);
      }
    }
    return true;
  }

public:
  bool isValidSudoku(vector<vector<char>> &board) {
    int len = board.size();
    int rowSize = board[0].size();

    for (int i = 0; i < len; i++) {
      for (int j = 0; j < rowSize; j++) {
        if (i % 3 == 0 && j % 3 == 0) {
          if (!checkSubBoxes(board, i, j)) {
            return false;
          }
        }
      }
      if (!checkRows(board[i]) || !checkColumns(board, i, rowSize)) {
        return false;
      }
    }
    return true;
  }
};

using namespace chrono;

int main() {
  Solution solution;
  vector<vector<char>> board = {{'5', '3', '.', '.', '7', '.', '.', '.', '.'},
                                {'6', '.', '.', '1', '9', '5', '.', '.', '.'},
                                {'.', '9', '8', '.', '.', '.', '.', '6', '.'},
                                {'8', '.', '.', '.', '6', '.', '.', '.', '3'},
                                {'4', '.', '.', '8', '.', '3', '.', '.', '1'},
                                {'7', '.', '.', '.', '2', '.', '.', '.', '6'},
                                {'.', '6', '.', '.', '.', '.', '2', '8', '.'},
                                {'.', '.', '.', '4', '1', '9', '.', '.', '5'},
                                {'.', '.', '.', '.', '8', '.', '.', '7', '9'}};
  auto start = high_resolution_clock::now();
  bool validSudoku = solution.isValidSudoku(board);
  auto end = high_resolution_clock::now();
  cout << (validSudoku ? "Valid Sudoku" : "Invalid Sudoku") << '\n';
  auto duration = duration_cast<microseconds>(end - start);
  cout << "Runtime: " << duration.count() << " microseconds" << '\n';
  return 0;
}
