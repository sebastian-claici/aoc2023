module Day1 where

import Data.Char (isNumber, digitToInt)
import Data.Maybe (mapMaybe)
import Data.List (tails, isPrefixOf)

read :: String -> [String]
read = lines

-- Part A

calibrate :: [Int] -> Int
calibrate xs = head xs * 10 + last xs

solveA :: [String] -> Int
solveA = sum . map (calibrate . map digitToInt . filter isNumber)

-- Part B

mapDigit :: String -> Maybe Int
mapDigit [] = Nothing
mapDigit s 
  | isPrefixOf "one" s = Just 1
  | isPrefixOf "two" s = Just 2
  | isPrefixOf "three" s = Just 3
  | isPrefixOf "four" s = Just 4
  | isPrefixOf "five" s = Just 5
  | isPrefixOf "six" s = Just 6
  | isPrefixOf "seven" s = Just 7
  | isPrefixOf "eight" s = Just 8
  | isPrefixOf "nine" s = Just 9
  | isNumber $ head s = Just $ digitToInt $ head s
  | otherwise = Nothing
 
solveB :: [String] -> Int
solveB = sum . map (calibrate . mapMaybe mapDigit . tails)
