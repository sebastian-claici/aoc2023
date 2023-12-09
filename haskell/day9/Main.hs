module Main where

solve :: [Int] -> Int 
solve xs 
  | all (==0) xs = 0
  | otherwise = last xs + solve [y - x | (x, y) <- zip xs (drop 1 xs)]

parse :: String -> [Int]
parse = map read . words

main = do 
  content <- readFile "../input/day9.in"
  -- Part A
  print . sum . map (solve . parse) . lines $ content
  -- Part B
  print . sum . map (solve . reverse . parse) . lines $ content
