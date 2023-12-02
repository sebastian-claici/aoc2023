module Main where

import Data.Char (isNumber, digitToInt)
import Data.Maybe (mapMaybe)
import Data.List (tails, isPrefixOf)

-- Part A

calibrate :: [Int] -> Int
calibrate xs = head xs * 10 + last xs

solveA :: [String] -> Int
solveA = sum . map (calibrate . map digitToInt . filter isNumber)

-- Part B

mapDigit :: String -> Maybe Int
mapDigit [] = Nothing
mapDigit s
  | "one" `isPrefixOf` s = Just 1
  | "two" `isPrefixOf` s = Just 2
  | "three" `isPrefixOf` s = Just 3
  | "four" `isPrefixOf` s = Just 4
  | "five" `isPrefixOf` s = Just 5
  | "six" `isPrefixOf` s = Just 6
  | "seven" `isPrefixOf` s = Just 7
  | "eight" `isPrefixOf` s = Just 8
  | "nine" `isPrefixOf` s = Just 9
  | isNumber $ head s = Just $ digitToInt $ head s
  | otherwise = Nothing

solveB :: [String] -> Int
solveB = sum . map (calibrate . mapMaybe mapDigit . tails)

main :: IO ()
main = do
  content <- readFile "../input/day1.in"
  print . solveA . lines $ content
  print . solveB . lines $ content

