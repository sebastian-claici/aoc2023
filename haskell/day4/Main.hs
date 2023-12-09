import Control.Arrow ((&&&))
import Control.Applicative ((*>), (<$>), (<*>), (<|>))
import Data.List (intersect)
import Data.Either (fromRight)
import Text.Parsec
import Text.Parsec.Char (digit, letter, space, string, char)
import Text.Parsec.Combinator (eof, many1, sepBy)
import Text.Parsec.String (Parser)

data Card = Card
  { 
    index :: Int,
    held :: [Int],
    winning :: [Int]
  }
  deriving (Show)

parseInt :: Parser Int
parseInt = read <$> many1 digit

parseList :: Parser [Int]
parseList = endBy parseInt spaces

parseCard :: Parser Card
parseCard = Card 
  <$> (string "Card" *> spaces *> parseInt <* char ':' <* spaces)
  <*> parseList <* char '|' <* spaces
  <*> parseList

parseLine :: String -> Card 
parseLine line = fromRight (Card 0 [] []) (parse parseCard "" line)

countMatches :: Card -> Int 
countMatches (Card _ held winning) = length $ intersect held winning

score :: Card -> Int
score card = if l == 0 then 0 else 2^(l - 1)
  where l = countMatches card

partA :: [Card] -> Int
partA = sum . map score 

partB :: [Card] -> Int
partB = sum . foldr (\card acc -> 1 + sum (take (countMatches card) acc) : acc) []

main :: IO ()
main = do
  content <- readFile "../input/day4.in"
  let cards = map parseLine . lines $ content
  print . (partA &&& partB) $ cards
