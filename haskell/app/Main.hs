module Main where

import qualified Day1

main :: IO ()
main = do
  content <- readFile "../input/day1.in"
  print . Day1.solveA . Day1.read $ content
  print . Day1.solveB . Day1.read $ content

